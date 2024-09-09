use crate::{
    amm::{consts::*, AutomatedMarketMaker, IErc20},
    errors::{AMMError, ArithmeticError, EventLogError, SwapSimulationError},
};
use alloy::{
    network::Network,
    primitives::{Address, Bytes, B256, I256, U256},
    providers::Provider,
    rpc::types::eth::{Filter, Log},
    sol,
    sol_types::{SolCall, SolEvent},
    transports::Transport,
};
use alloy_chains::NamedChain;
use async_trait::async_trait;
use futures::{stream::FuturesOrdered, StreamExt};
use num_bigfloat::BigFloat;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    sync::Arc,
};
use tracing::instrument;
use types::chain_serde;
use types::exchange::{ExchangeName, ExchangeType};
use uniswap_v3_math::tick_math::{MAX_SQRT_RATIO, MAX_TICK, MIN_SQRT_RATIO, MIN_TICK};

sol! {
    /// Interface of the IUniswapV3Pool
    #[derive(Debug, PartialEq, Eq)]
    #[sol(rpc)]
    contract IStableSwapPool {
         // Basic pool information
        function name() external view returns (string memory);
        function symbol() external view returns (string memory);
        function balances(uint256 i) external view returns (uint256);
        function get_virtual_price() external view returns (uint256);
        function coins(uint256 i) external view returns (address);
        function get_dy(int128 i, int128 j, uint256 dx) external view returns (uint256);
        function get_dy_underlying(int128 i, int128 j, uint256 dx) external view returns (uint256);
        function calc_token_amount(uint256[2] memory amounts, bool deposit) external view returns (uint256);

        // Trading functions
        function exchange(int128 i, int128 j, uint256 dx, uint256 min_dy) external returns (uint256);
        function exchange_underlying(int128 i, int128 j, uint256 dx, uint256 min_dy) external returns (uint256);

        // Liquidity provision
        function add_liquidity(uint256[2] memory amounts, uint256 min_mint_amount) external returns (uint256);
        function remove_liquidity(uint256 _amount, uint256[2] memory min_amounts) external returns (uint256[2] memory);
        function remove_liquidity_one_coin(uint256 _token_amount, int128 i, uint256 min_amount) external returns (uint256);

        // Fee information
        function fee() external view returns (uint256);
        function admin_fee() external view returns (uint256);

        // Pool parameters
        function A() external view returns (uint256);
        function get_parameters() external view returns (uint256 A, uint256 future_A, uint256 fee, uint256 admin_fee, uint256 future_fee, uint256 future_admin_fee, address future_owner, uint256 initial_A, uint256 initial_A_time, uint256 future_A_time);

        // Events
        event TokenExchange(address indexed buyer, int128 sold_id, uint256 tokens_sold, int128 bought_id, uint256 tokens_bought);
        event AddLiquidity(address indexed provider, uint256[2] token_amounts, uint256[2] fees, uint256 invariant, uint256 token_supply);
        event RemoveLiquidity(address indexed provider, uint256[2] token_amounts, uint256[2] fees, uint256 token_supply);
        event RemoveLiquidityOne(address indexed provider, uint256 token_amount, uint256 coin_amount);
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CurvePool {
    pub address: Address,
    pub coins: Vec<Address>,
    pub coin_symbols: Vec<String>,
    pub coin_decimals: Vec<u8>,
    pub balances: Vec<U256>,
    pub exchange_name: ExchangeName,
    pub exchange_type: ExchangeType,
    #[serde(with = "chain_serde")]
    pub chain: NamedChain,
}

impl CurvePool {
    pub fn new(
        address: Address,
        coins: Vec<Address>,
        coin_symbols: Vec<String>,
        coin_decimals: Vec<u8>,
        balances: Vec<U256>,
        exchange_name: ExchangeName,
        exchange_type: ExchangeType,
        chain: NamedChain,
    ) -> Self {
        Self {
            address,
            coins,
            coin_symbols,
            coin_decimals,
            balances,
            exchange_name,
            exchange_type,
            chain,
        }
    }

    pub async fn new_from_address<T, N, P>(
        address: Address,
        provider: Arc<P>,
    ) -> Result<Self, AMMError>
    where
        T: Transport + Clone,
        N: Network,
        P: Provider<T, N>,
    {
        let pool = CurvePool::new(
            address,
            vec![],
            vec![],
            vec![],
            vec![],
            ExchangeName::Curve,
            ExchangeType::Curve,
            NamedChain::Mainnet,
        );
        pool.populate_data(None, provider).await?;
        Ok(pool)
    }

    pub async fn new_from_log<T, N, P>(log: Log, provider: Arc<P>) -> Result<Self, AMMError>
    where
        T: Transport + Clone,
        N: Network,
        P: Provider<T, N>,
    {
        let event_signature = log.topics()[0];

        if event_signature == IStableSwapPool::TokenExchange::SIGNATURE_HASH {
            if let Some(block_number) = log.block_number {
                let pool_created_event =
                    IStableSwapPool::TokenExchange::decode_log(&log.inner, true)?;

                CurvePool::new_from_address(pool_created_event.pool, block_number, provider).await
            } else {
                Err(EventLogError::LogBlockNumberNotFound)?
            }
        } else {
            Err(EventLogError::InvalidEventSignature)?
        }
    }
}

#[async_trait]
impl AutomatedMarketMaker for CurvePool {
    fn address(&self) -> Address {
        self.address
    }

    fn name(&self) -> String {
        let symbols = self.token_symbols();
        let exchange_name = self.exchange_name();
        format!("{}:{}-{}", exchange_name, symbols[0], symbols[1])
    }

    #[instrument(skip(self, provider), level = "debug")]
    async fn sync<T, N, P>(&mut self, provider: Arc<P>) -> Result<(), AMMError>
    where
        T: Transport + Clone,
        N: Network,
        P: Provider<T, N>,
    {
        // let (reserve_0, reserve_1) = self.get_reserves(provider.clone()).await?;
        // tracing::info!(?reserve_0, ?reserve_1, address = ?self.address, "UniswapV2 sync");

        // self.reserve_0 = reserve_0;
        // self.reserve_1 = reserve_1;

        Ok(())
    }

    #[instrument(skip(self, provider), level = "debug")]
    async fn populate_data<T, N, P>(
        &mut self,
        _block_number: Option<u64>,
        provider: Arc<P>,
    ) -> Result<(), AMMError>
    where
        T: Transport + Clone,
        N: Network,
        P: Provider<T, N>,
    {
        // batch_request::get_v2_pool_data_batch_request(self, provider.clone()).await?;

        Ok(())
    }

    fn sync_on_event_signatures(&self) -> Vec<B256> {
        vec![IStableSwapPool::TokenExchange::SIGNATURE_HASH]
    }

    #[instrument(skip(self), level = "debug")]
    fn sync_from_log(&mut self, log: Log) -> Result<(), EventLogError> {
        let event_signature = log.topics()[0];

        if event_signature == IStableSwapPool::TokenExchange::SIGNATURE_HASH {
            let sync_event = IStableSwapPool::TokenExchange::decode_log(log.as_ref(), true)?;
            // tracing::info!(reserve_0 = sync_event.reserve0, reserve_1 = sync_event.reserve1, address = ?self.address, "UniswapV2 sync event");

            // self.reserve_0 = sync_event.reserve0;
            // self.reserve_1 = sync_event.reserve1;

            Ok(())
        } else {
            Err(EventLogError::InvalidEventSignature)
        }
    }

    // Calculates base/quote, meaning the price of base token per quote (ie. exchange rate is X base per 1 quote)
    fn calculate_price(&self, base_token: Address) -> Result<f64, ArithmeticError> {
        Ok(0.0)
        // Ok(q64_to_f64(self.calculate_price_64_x_64(base_token)?))
    }

    fn tokens(&self) -> Vec<Address> {
        self.coins.clone()
    }

    fn simulate_swap(
        &self,
        token_in: Address,
        amount_in: U256,
        token_out: Address,
    ) -> Result<U256, SwapSimulationError> {
        // if self.token_a == token_in {
        //     Ok(self.get_amount_out(
        //         amount_in,
        //         U256::from(self.reserve_0),
        //         U256::from(self.reserve_1),
        //     ))
        // } else {
        //     Ok(self.get_amount_out(
        //         amount_in,
        //         U256::from(self.reserve_1),
        //         U256::from(self.reserve_0),
        //     ))
        // }
    }

    fn simulate_swap_mut(
        &mut self,
        token_in: Address,
        amount_in: U256,
        token_out: Address,
    ) -> Result<U256, SwapSimulationError> {
        // if self.token_a == token_in {
        //     let amount_out = self.get_amount_out(
        //         amount_in,
        //         U256::from(self.reserve_0),
        //         U256::from(self.reserve_1),
        //     );

        //     tracing::trace!(?amount_out);
        //     tracing::trace!(?self.reserve_0, ?self.reserve_1, "pool reserves before");

        //     self.reserve_0 += amount_in.to::<u128>();
        //     self.reserve_1 -= amount_out.to::<u128>();

        //     tracing::trace!(?self.reserve_0, ?self.reserve_1, "pool reserves after");

        //     Ok(amount_out)
        // } else {
        //     let amount_out = self.get_amount_out(
        //         amount_in,
        //         U256::from(self.reserve_1),
        //         U256::from(self.reserve_0),
        //     );

        //     tracing::trace!(?amount_out);
        //     tracing::trace!(?self.reserve_0, ?self.reserve_1, "pool reserves before");

        //     self.reserve_0 -= amount_out.to::<u128>();
        //     self.reserve_1 += amount_in.to::<u128>();

        //     tracing::trace!(?self.reserve_0, ?self.reserve_1, "pool reserves after");

        //     Ok(amount_out)
        // }
    }

    fn token_symbols(&self) -> Vec<String> {
        self.coin_symbols.clone()
    }

    fn exchange_name(&self) -> ExchangeName {
        self.exchange_name
    }

    fn exchange_type(&self) -> ExchangeType {
        self.exchange_type
    }

    fn chain(&self) -> NamedChain {
        self.chain
    }
}
