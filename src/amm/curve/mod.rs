use crate::{
    amm::AutomatedMarketMaker,
    errors::{AMMError, ArithmeticError, EventLogError, SwapSimulationError},
};
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
use db::models::{NewDbCurvePool, NewDbPool};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::instrument;
use types::chain_serde;
use types::exchange::{ExchangeName, ExchangeType};
pub mod batch_request;

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
        _fee: u32,
        provider: Arc<P>,
    ) -> Result<Self, AMMError>
    where
        T: Transport + Clone,
        N: Network,
        P: Provider<T, N>,
    {
        let mut pool = CurvePool::new(
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
            let pool_created_event = IStableSwapPool::TokenExchange::decode_log(&log.inner, true)?;
            let pool_address = pool_created_event.address;
            CurvePool::new_from_address(pool_address, 0, provider).await
        } else {
            Err(EventLogError::InvalidEventSignature)?
        }
    }

    // async fn get_coin_data<T, N, P>(
    //     &self,
    //     coin_address: Address,
    //     provider: Arc<P>,
    // ) -> Result<(String, u8), AMMError>
    // where
    //     T: Transport + Clone,
    //     N: Network,
    //     P: Provider<T, N>,
    // {
    //     let erc20 = IErc20::new(coin_address, provider.clone());

    //     let symbol = erc20.symbol().call().await?;
    //     let decimals = erc20.decimals().call().await?;

    //     Ok((symbol, decimals))
    // }
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
        let pool = IStableSwapPool::new(self.address, provider.clone());

        // Update balances
        for (i, _) in self.coins.iter().enumerate() {
            let i = U256::from(i);
            let IStableSwapPool::balancesReturn { _0: balance } = pool.balances(i).call().await?;
            let i = i.to::<usize>();
            self.balances[i] = balance;
        }

        tracing::info!(address = ?self.address, "Curve pool synced");

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
        batch_request::get_curve_pool_data_batch_request(self, provider.clone()).await?;
        Ok(())
    }

    fn sync_on_event_signatures(&self) -> Vec<B256> {
        vec![IStableSwapPool::TokenExchange::SIGNATURE_HASH]
    }

    #[instrument(skip(self), level = "debug")]
    fn sync_from_log(&mut self, log: Log) -> Result<(), EventLogError> {
        let event_signature = log.topics()[0];

        if event_signature == IStableSwapPool::TokenExchange::SIGNATURE_HASH {
            let exchange_event = IStableSwapPool::TokenExchange::decode_log(log.as_ref(), true)?;

            let sold_id = exchange_event.sold_id;
            let bought_id = exchange_event.bought_id;

            if sold_id >= self.balances.len() as i128 || bought_id >= self.balances.len() as i128 {
                return Err(EventLogError::InvalidEventData);
            }

            // Update balances based on the exchange
            self.balances[sold_id as usize] += exchange_event.tokens_sold;
            self.balances[bought_id as usize] = self.balances[bought_id as usize]
                .checked_sub(exchange_event.tokens_bought)
                .ok_or(EventLogError::ArithmeticError)?;

            tracing::info!(
                sold_id = sold_id,
                bought_id = bought_id,
                tokens_sold = ?exchange_event.tokens_sold,
                tokens_bought = ?exchange_event.tokens_bought,
                address = ?self.address,
                "Curve pool sync event"
            );

            Ok(())
        } else {
            Err(EventLogError::InvalidEventSignature)
        }
    }

    // Calculates base/quote, meaning the price of base token per quote (ie. exchange rate is X base per 1 quote)
    fn calculate_price(&self, _base_token: Address) -> Result<f64, ArithmeticError> {
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
        self.get_amount_out(token_in, amount_in, token_out)
    }

    fn simulate_swap_mut(
        &mut self,
        token_in: Address,
        amount_in: U256,
        token_out: Address,
    ) -> Result<U256, SwapSimulationError> {
        let amount_out = self.get_amount_out(token_in, amount_in, token_out)?;

        let i = self
            .coins
            .iter()
            .position(|&coin| coin == token_in)
            .ok_or(SwapSimulationError::CurveSwapError)?;
        let j = self
            .coins
            .iter()
            .position(|&coin| coin == token_out)
            .ok_or(SwapSimulationError::CurveSwapError)?;

        self.balances[i] += amount_in;
        self.balances[j] -= amount_out;

        Ok(amount_out)
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

    fn to_new_db_pool(&self) -> NewDbPool {
        NewDbPool::Curve(NewDbCurvePool {
            address: self.address.to_string(),
            chain: self.chain.as_str().to_string(),
            exchange_name: Some(self.exchange_name.as_str().to_string()),
            exchange_type: Some(self.exchange_type.as_str().to_string()),
            token_a: self.coins[0].to_string(),
            token_a_symbol: self.coin_symbols[0].clone(),
            token_a_decimals: self.coin_decimals[0] as i32,
            token_a_balance: self.balances[0].to_string(),
            token_b: self.coins[1].to_string(),
            token_b_symbol: self.coin_symbols[1].clone(),
            token_b_decimals: self.coin_decimals[1] as i32,
            token_b_balance: self.balances[1].to_string(),
            token_c: Some(self.coins[2].to_string()),
            token_c_symbol: Some(self.coin_symbols[2].clone()),
            token_c_decimals: Some(self.coin_decimals[2] as i32),
            token_c_balance: Some(self.balances[2].to_string()),
            token_d: Some(self.coins[3].to_string()),
            token_d_symbol: Some(self.coin_symbols[3].clone()),
            token_d_decimals: Some(self.coin_decimals[3] as i32),
            token_d_balance: Some(self.balances[3].to_string()),
        })
    }
}

impl CurvePool {
    fn get_amount_out(
        &self,
        token_in: Address,
        amount_in: U256,
        token_out: Address,
    ) -> Result<U256, SwapSimulationError> {
        let i = self
            .coins
            .iter()
            .position(|&coin| coin == token_in)
            .ok_or(SwapSimulationError::CurveSwapError)?;
        let j = self
            .coins
            .iter()
            .position(|&coin| coin == token_out)
            .ok_or(SwapSimulationError::CurveSwapError)?;

        // Simplified stable swap formula
        let x = self.balances[i] + amount_in;
        let y = self.balances[j];
        let k = x * y; // Constant product for simplicity

        let new_y = k / x;
        let amount_out = y - new_y;

        Ok(amount_out)
    }
}
