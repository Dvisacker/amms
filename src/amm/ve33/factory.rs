use std::sync::Arc;

use alloy::{
    network::Network,
    primitives::{Address, B256, U256},
    providers::Provider,
    rpc::types::eth::Log,
    sol,
    sol_types::SolEvent,
    transports::Transport,
};
use alloy_chains::NamedChain;
use async_trait::async_trait;
use types::exchange::{ExchangeName, ExchangeType};

use crate::{
    amm::{
        consts::U256_1,
        factory::AutomatedMarketMakerFactory,
        uniswap_v2::{self, UniswapV2Pool},
        ve33, AMM,
    },
    errors::AMMError,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;

sol! {
    /// Interface of the Ve33Factory contract
    #[derive(Debug, PartialEq, Eq)]
    #[sol(rpc)]
    contract IVe33Factory {
        event PairCreated(address indexed token0, address indexed token1, address pair, uint256 index);
        function getPool(address tokenA, address tokenB) external view returns (address pair);
        function allPools(uint256 index) external view returns (address pool);
        function allPoolsLength() external view returns (uint256 length);
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Ve33Factory {
    pub address: Address,
    pub creation_block: u64,
    pub fee: u32,
}

impl Ve33Factory {
    pub fn new(address: Address, creation_block: u64, fee: u32) -> Ve33Factory {
        Ve33Factory {
            address,
            creation_block,
            fee,
        }
    }

    pub async fn get_all_pools_batched<T, N, P>(
        &self,
        provider: Arc<P>,
    ) -> Result<Vec<AMM>, AMMError>
    where
        T: Transport + Clone,
        N: Network,
        P: Provider<T, N>,
    {
        let factory = IVe33Factory::new(self.address, provider.clone());

        let IVe33Factory::allPoolsLengthReturn {
            length: pools_length,
        } = factory.allPoolsLength().call().await?;

        tracing::info!("Found {:?} pools", pools_length.to::<usize>());

        let mut pools = vec![];
        // NOTE: max batch size for this call until codesize is too large
        let step = 766;
        let mut idx_from = U256::ZERO;
        let mut idx_to = if step > pools_length.to::<usize>() {
            pools_length
        } else {
            U256::from(step)
        };

        for _ in (0..pools_length.to::<usize>()).step_by(step) {
            tracing::info!("Getting pools from index {:?} to {:?}", idx_from, idx_to);
            pools.append(
                &mut ve33::batch_request::get_pools_batch_request(
                    self.address,
                    idx_from,
                    idx_to,
                    provider.clone(),
                )
                .await?,
            );

            idx_from = idx_to;

            if idx_to + U256::from(step) > pools_length {
                idx_to = pools_length - U256_1
            } else {
                idx_to += U256::from(step);
            }
        }

        let mut amms = vec![];

        // Create new empty pools for each pair
        for addr in pools {
            let amm = UniswapV2Pool {
                address: addr,
                ..Default::default()
            };

            amms.push(AMM::UniswapV2Pool(amm));
        }

        Ok(amms)
    }
}

#[async_trait]
impl AutomatedMarketMakerFactory for Ve33Factory {
    fn address(&self) -> Address {
        self.address
    }

    fn amm_created_event_signature(&self) -> B256 {
        IVe33Factory::PairCreated::SIGNATURE_HASH
    }

    async fn new_amm_from_log<T, N, P>(&self, log: Log, provider: Arc<P>) -> Result<AMM, AMMError>
    where
        T: Transport + Clone,
        N: Network,
        P: Provider<T, N>,
    {
        let pair_created_event = IVe33Factory::PairCreated::decode_log(log.as_ref(), true)?;
        Ok(AMM::UniswapV2Pool(
            UniswapV2Pool::new_from_address(pair_created_event.pair, self.fee, provider).await?,
        ))
    }

    fn new_empty_amm_from_log(&self, log: Log) -> Result<AMM, alloy::sol_types::Error> {
        let pair_created_event = IVe33Factory::PairCreated::decode_log(log.as_ref(), true)?;

        Ok(AMM::UniswapV2Pool(UniswapV2Pool {
            address: pair_created_event.pair,
            token_a: pair_created_event.token0,
            token_a_symbol: String::new(),
            token_b: pair_created_event.token1,
            token_b_symbol: String::new(),
            token_a_decimals: 0,
            token_b_decimals: 0,
            reserve_0: 0,
            reserve_1: 0,
            fee: 0,
            exchange_name: ExchangeName::Unknown,
            exchange_type: ExchangeType::Unknown,
            chain: NamedChain::Mainnet,
            factory: Address::ZERO,
        }))
    }

    #[instrument(skip(self, provider) level = "debug")]
    async fn get_all_amms<T, N, P>(
        &self,
        _to_block: Option<u64>,
        provider: Arc<P>,
        _step: u64,
    ) -> Result<Vec<AMM>, AMMError>
    where
        T: Transport + Clone,
        N: Network,
        P: Provider<T, N>,
    {
        self.get_all_pools_batched(provider).await
    }

    async fn populate_amm_data<T, N, P>(
        &self,
        amms: &mut [AMM],
        _block_number: Option<u64>,
        provider: Arc<P>,
    ) -> Result<(), AMMError>
    where
        T: Transport + Clone,
        N: Network,
        P: Provider<T, N>,
    {
        // Max batch size for call
        let step = 127;
        for amm_chunk in amms.chunks_mut(step) {
            uniswap_v2::batch_request::get_amm_data_batch_request(amm_chunk, provider.clone())
                .await?;
        }
        Ok(())
    }

    fn creation_block(&self) -> u64 {
        self.creation_block
    }
}
