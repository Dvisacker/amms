use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use alloy::{
    network::Network,
    primitives::{Address, B256, U256},
    providers::Provider,
    rpc::types::eth::{Filter, Log},
    sol,
    sol_types::SolEvent,
    transports::Transport,
};
use alloy_chains::NamedChain;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use types::exchange::{ExchangeName, ExchangeType};

use crate::{
    amm::{factory::AutomatedMarketMakerFactory, AutomatedMarketMaker, AMM},
    errors::{AMMError, EventLogError},
};

use super::{batch_request, CamelotV3Pool, ICamelotV3Pool};

sol! {
    /// Interface of the UniswapV3Factory contract
    #[derive(Debug, PartialEq, Eq)]
    #[sol(rpc)]
    contract ICamelotV3Factory {
    event Owner(address indexed newOwner);
      event VaultAddress(address indexed newVaultAddress);
    event Pool(address indexed token0, address indexed token1, address pool);
  event FarmingAddress(address indexed newFarmingAddress);
  event DefaultCommunityFee(uint8 newDefaultCommunityFee);
  event FeeConfiguration(
    uint16 alpha1,
    uint16 alpha2,
    uint32 beta1,
    uint32 beta2,
    uint16 gamma1,
    uint16 gamma2,
    uint32 volumeBeta,
    uint16 volumeGamma,
    uint16 baseFee
  );
  function owner() external view returns (address);
  function poolDeployer() external view returns (address);
  function farmingAddress() external view returns (address);
  function defaultCommunityFee() external view returns (uint8);
  function vaultAddress() external view returns (address);
  function poolByPair(address tokenA, address tokenB) external view returns (address pool);
  function createPool(address tokenA, address tokenB) external returns (address pool);
  function setOwner(address _owner) external;
  function setFarmingAddress(address _farmingAddress) external;
  function setDefaultCommunityFee(uint8 newDefaultCommunityFee) external;
  function setVaultAddress(address _vaultAddress) external;
  function setBaseFeeConfiguration(
    uint16 alpha1,
    uint16 alpha2,
    uint32 beta1,
    uint32 beta2,
    uint16 gamma1,
    uint16 gamma2,
    uint32 volumeBeta,
    uint16 volumeGamma,
    uint16 baseFee
  ) external;
    }
}

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CamelotV3Factory {
    pub address: Address,
    pub creation_block: u64,
}

#[async_trait]
impl AutomatedMarketMakerFactory for CamelotV3Factory {
    fn address(&self) -> Address {
        self.address
    }

    fn creation_block(&self) -> u64 {
        self.creation_block
    }

    fn amm_created_event_signature(&self) -> B256 {
        ICamelotV3Factory::Pool::SIGNATURE_HASH
    }

    async fn new_amm_from_log<T, N, P>(&self, log: Log, provider: Arc<P>) -> Result<AMM, AMMError>
    where
        T: Transport + Clone,
        N: Network,
        P: Provider<T, N>,
    {
        if let Some(block_number) = log.block_number {
            let pool_created_filter = ICamelotV3Factory::Pool::decode_log(&log.inner, true)?;
            Ok(AMM::CamelotV3Pool(
                CamelotV3Pool::new_from_address(pool_created_filter.pool, block_number, provider)
                    .await?,
            ))
        } else {
            return Err(AMMError::BlockNumberNotFound);
        }
    }

    async fn get_all_amms<T, N, P>(
        &self,
        to_block: Option<u64>,
        provider: Arc<P>,
        step: u64,
    ) -> Result<Vec<AMM>, AMMError>
    where
        T: Transport + Clone,
        N: Network,
        P: Provider<T, N>,
    {
        if let Some(block) = to_block {
            self.get_all_pools_from_logs(block, step, provider).await
        } else {
            return Err(AMMError::BlockNumberNotFound);
        }
    }

    #[instrument(skip(self, amms, provider) level = "debug")]
    async fn populate_amm_data<T, N, P>(
        &self,
        amms: &mut [AMM],
        block_number: Option<u64>,
        provider: Arc<P>,
    ) -> Result<(), AMMError>
    where
        T: Transport + Clone,
        N: Network,
        P: Provider<T, N>,
    {
        if let Some(block_number) = block_number {
            // Max batch size for call
            let step = 76;
            for amm_chunk in amms.chunks_mut(step) {
                batch_request::get_amm_data_batch_request(
                    amm_chunk,
                    block_number,
                    provider.clone(),
                )
                .await?;
            }
        } else {
            return Err(AMMError::BlockNumberNotFound);
        }

        Ok(())
    }

    fn new_empty_amm_from_log(&self, log: Log) -> Result<AMM, alloy::sol_types::Error> {
        let pool_created_event = ICamelotV3Factory::Pool::decode_log(&log.inner, true)?;

        Ok(AMM::CamelotV3Pool(CamelotV3Pool {
            address: pool_created_event.pool,
            token_a: pool_created_event.token0,
            token_b: pool_created_event.token1,
            token_a_decimals: 0,
            token_b_decimals: 0,
            token_a_symbol: String::new(),
            token_b_symbol: String::new(),
            fee: 0,
            liquidity: 0,
            sqrt_price: U256::ZERO,
            tick_spacing: 0,
            tick: 0,
            tick_bitmap: HashMap::new(),
            ticks: HashMap::new(),
            exchange_name: ExchangeName::Unknown,
            exchange_type: ExchangeType::Unknown,
            chain: NamedChain::Mainnet,
        }))
    }
}

impl CamelotV3Factory {
    pub fn new(address: Address, creation_block: u64) -> CamelotV3Factory {
        CamelotV3Factory {
            address,
            creation_block,
        }
    }

    pub async fn get_all_pools_from_logs<T, N, P>(
        self,
        to_block: u64,
        step: u64,
        provider: Arc<P>,
    ) -> Result<Vec<AMM>, AMMError>
    where
        T: Transport + Clone,
        N: Network,
        P: Provider<T, N>,
    {
        // Unwrap can be used here because the creation block was verified within `Dex::new()`
        let mut from_block = self.creation_block;
        let mut aggregated_amms: HashMap<Address, AMM> = HashMap::new();
        let mut ordered_logs: BTreeMap<u64, Vec<Log>> = BTreeMap::new();
        // let mut futures = FuturesOrdered::new();

        while from_block < to_block {
            let mut target_block = from_block + step - 1;
            println!(
                "Getting logs from block: {} to block: {}",
                from_block, target_block
            );
            if target_block > to_block {
                target_block = to_block;
            }

            let logs = provider
                .get_logs(
                    &Filter::new()
                        .event_signature(vec![
                            ICamelotV3Factory::Pool::SIGNATURE_HASH,
                            ICamelotV3Pool::Burn::SIGNATURE_HASH,
                            ICamelotV3Pool::Mint::SIGNATURE_HASH,
                        ])
                        .from_block(from_block)
                        .to_block(target_block),
                )
                .await
                .map_err(AMMError::TransportError)?;

            println!("Got {:?} Logs", logs.len());

            for log in logs {
                if let Some(log_block_number) = log.block_number {
                    if let Some(log_group) = ordered_logs.get_mut(&log_block_number) {
                        log_group.push(log);
                    } else {
                        ordered_logs.insert(log_block_number, vec![log]);
                    }
                } else {
                    return Err(EventLogError::LogBlockNumberNotFound)?;
                }
            }

            from_block += step;
        }

        for (_, log_group) in ordered_logs {
            for log in log_group {
                let event_signature = log.topics()[0];

                //If the event sig is the pool created event sig, then the log is coming from the factory
                if event_signature == ICamelotV3Factory::Pool::SIGNATURE_HASH {
                    if log.address() == self.address {
                        let mut new_pool = self.new_empty_amm_from_log(log)?;
                        if let AMM::UniswapV3Pool(ref mut pool) = new_pool {
                            pool.tick_spacing = pool.get_tick_spacing(provider.clone()).await?;
                        }

                        aggregated_amms.insert(new_pool.address(), new_pool);
                    }
                } else if event_signature == ICamelotV3Pool::Burn::SIGNATURE_HASH {
                    //If the event sig is the BURN_EVENT_SIGNATURE log is coming from the pool
                    if let Some(AMM::UniswapV3Pool(pool)) = aggregated_amms.get_mut(&log.address())
                    {
                        pool.sync_from_burn_log(log)?;
                    }
                } else if event_signature == ICamelotV3Pool::Mint::SIGNATURE_HASH {
                    if let Some(AMM::UniswapV3Pool(pool)) = aggregated_amms.get_mut(&log.address())
                    {
                        pool.sync_from_mint_log(log)?;
                    }
                }
            }
        }

        Ok(aggregated_amms.into_values().collect::<Vec<AMM>>())
    }

    // Function to get all pair created events for a given Dex factory address and sync pool data
    pub async fn get_pools_from_logs<T, N, P>(
        self,
        start_block: u64,
        to_block: u64,
        step: u64,
        provider: Arc<P>,
    ) -> Result<Vec<AMM>, AMMError>
    where
        T: Transport + Clone,
        N: Network,
        P: Provider<T, N>,
    {
        // Unwrap can be used here because the creation block was verified within `Dex::new()`
        let mut aggregated_amms: HashMap<Address, AMM> = HashMap::new();
        let mut ordered_logs: BTreeMap<u64, Vec<Log>> = BTreeMap::new();
        let mut from_block = start_block;
        // let mut futures = FuturesOrdered::new();

        while from_block < to_block {
            let mut target_block = from_block + step - 1;
            println!(
                "Getting logs from block: {} to block: {}",
                from_block, target_block
            );
            if target_block > to_block {
                target_block = to_block;
            }

            let logs = provider
                .get_logs(
                    &Filter::new()
                        .event_signature(vec![ICamelotV3Factory::Pool::SIGNATURE_HASH])
                        .from_block(from_block)
                        .to_block(target_block),
                )
                .await
                .map_err(AMMError::TransportError)?;

            println!("Got {:?} Logs", logs.len());

            for log in logs {
                if let Some(log_block_number) = log.block_number {
                    if let Some(log_group) = ordered_logs.get_mut(&log_block_number) {
                        log_group.push(log);
                    } else {
                        ordered_logs.insert(log_block_number, vec![log]);
                    }
                } else {
                    return Err(EventLogError::LogBlockNumberNotFound)?;
                }
            }

            from_block += step;
        }

        for (_, log_group) in ordered_logs {
            for log in log_group {
                let event_signature = log.topics()[0];

                //If the event sig is the pool created event sig, then the log is coming from the factory
                if event_signature == ICamelotV3Factory::Pool::SIGNATURE_HASH {
                    if log.address() == self.address {
                        let new_pool = self.new_empty_amm_from_log(log)?;
                        aggregated_amms.insert(new_pool.address(), new_pool);
                    }
                }
            }
        }

        Ok(aggregated_amms.into_values().collect::<Vec<AMM>>())
    }
}
