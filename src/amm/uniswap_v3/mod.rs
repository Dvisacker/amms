pub mod batch_request;
pub mod factory;

use crate::{
    amm::{consts::*, AutomatedMarketMaker, IErc20},
    bindings::getclpoolticksinrange::PoolHelpers::PopulatedTick,
    errors::{AMMError, ArithmeticError, EventLogError, SwapSimulationError},
};
use alloy::{
    network::Network,
    primitives::{aliases::I24, Address, Bytes, B256, I256, U256},
    providers::Provider,
    rpc::types::eth::{Filter, Log},
    sol,
    sol_types::{SolCall, SolEvent},
};
use alloy_chains::NamedChain;
use async_recursion::async_recursion;
use async_trait::async_trait;
use db::models::{NewDbPool, NewDbUniV3Pool};
use futures::{stream::FuturesOrdered, StreamExt};
use num_bigfloat::BigFloat;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    sync::Arc,
};
use tracing::instrument;
use types::chain_serde;
use types::exchange::{ExchangeName, ExchangeType};
use uniswap_v3_math::tick_math::{MAX_SQRT_RATIO, MAX_TICK, MIN_SQRT_RATIO, MIN_TICK};

pub const POPULATE_TICK_DATA_STEP: u64 = 100000;

use self::factory::IUniswapV3Factory;

sol! {
    /// Interface of the IUniswapV3Pool
    #[derive(Debug, PartialEq, Eq)]
    #[sol(rpc)]
    contract IUniswapV3Pool {
        event Swap(address indexed sender, address indexed recipient, int256 amount0, int256 amount1, uint160 sqrtPriceX96, uint128 liquidity, int24 tick);
        event Burn(address indexed owner, int24 indexed tickLower, int24 indexed tickUpper, uint128 amount, uint256 amount0, uint256 amount1);
        event Mint(address sender, address indexed owner, int24 indexed tickLower, int24 indexed tickUpper, uint128 amount, uint256 amount0, uint256 amount1);
        function token0() external view returns (address);
        function token1() external view returns (address);
        function liquidity() external view returns (uint128);
        function slot0() external view returns (uint160, int24, uint16, uint16, uint16, uint8, bool);
        function fee() external view returns (uint24);
        function tickSpacing() external view returns (int24);
        function ticks(int24 tick) external view returns (uint128, int128, uint256, uint256, int56, uint160, uint32, bool);
        function tickBitmap(int16 wordPosition) external view returns (uint256);
        function swap(address recipient, bool zeroForOne, int256 amountSpecified, uint160 sqrtPriceLimitX96, bytes calldata data) external returns (int256, int256);
        function factory() external view returns (address);
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UniswapV3Pool {
    pub address: Address,
    pub token_a: Address,
    pub token_a_decimals: u8,
    pub token_a_symbol: String,
    pub token_b: Address,
    pub token_b_decimals: u8,
    pub token_b_symbol: String,
    pub liquidity: u128,
    pub liquidity_net: i128,
    pub factory: Address,
    pub sqrt_price: U256,
    pub fee: u32,
    pub tick: i32,
    pub tick_spacing: i32,
    pub tick_bitmap: HashMap<i16, U256>,
    pub ticks: HashMap<i32, Info>,
    pub exchange_name: ExchangeName,
    pub exchange_type: ExchangeType,
    #[serde(with = "chain_serde")]
    pub chain: NamedChain,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Info {
    pub liquidity_gross: u128,
    pub liquidity_net: i128,
    pub initialized: bool,
}

impl Info {
    pub fn new(liquidity_gross: u128, liquidity_net: i128, initialized: bool) -> Self {
        Info {
            liquidity_gross,
            liquidity_net,
            initialized,
        }
    }
}

pub fn json_to_tickbitmap(json: JsonValue) -> HashMap<i16, U256> {
    let mut tick_bitmap: HashMap<i16, U256> = HashMap::new();
    json.as_object().map(|map| {
        for (tick, value) in map.iter() {
            let tick = tick.as_str().parse::<i16>().unwrap();
            let value = U256::from_str_radix(value.as_str().unwrap(), 2).unwrap();
            tick_bitmap.insert(tick, value);
        }
    });
    tick_bitmap
}

pub fn json_to_ticks(json: JsonValue) -> HashMap<i32, Info> {
    let mut ticks: HashMap<i32, Info> = HashMap::new();
    json.as_object().map(|map| {
        for (tick, value) in map.iter() {
            let tick = tick.as_str().parse::<i32>().unwrap();
            let value: Info = serde_json::from_value(value.clone()).unwrap();
            ticks.insert(tick, value);
        }
    });
    ticks
}

// cast a NewDbUniV3Pool into a UniswapV3Pool
impl From<NewDbUniV3Pool> for UniswapV3Pool {
    fn from(pool: NewDbUniV3Pool) -> Self {
        UniswapV3Pool {
            chain: pool.chain.parse::<NamedChain>().unwrap(),
            exchange_name: ExchangeName::from_str(&pool.exchange_name.unwrap()).unwrap(),
            exchange_type: ExchangeType::from_str(&pool.exchange_type.unwrap()).unwrap(),
            address: pool.address.parse().unwrap_or(Address::ZERO),
            token_a: pool.token_a.parse().unwrap_or(Address::ZERO),
            token_a_symbol: pool.token_a_symbol.to_string(),
            token_a_decimals: pool.token_a_decimals as u8,
            token_b: pool.token_b.parse().unwrap_or(Address::ZERO),
            token_b_symbol: pool.token_b_symbol.to_string(),
            token_b_decimals: pool.token_b_decimals as u8,
            factory: Address::ZERO, // TODO
            liquidity: pool.liquidity.unwrap().parse().unwrap_or(0),
            liquidity_net: 0, // TODO
            sqrt_price: U256::from_str_radix(&pool.sqrt_price.unwrap(), 10).unwrap_or(U256::ZERO),
            tick: pool.tick.unwrap_or(0),
            tick_spacing: pool.tick_spacing.unwrap_or(0),
            tick_bitmap: json_to_tickbitmap(pool.tick_bitmap.unwrap()),
            ticks: json_to_ticks(pool.ticks.unwrap()),
            fee: pool.fee.unwrap() as u32,
        }
    }
}

// cast a UniswapV3Pool into a NewDbUniV3Pool
impl From<UniswapV3Pool> for NewDbUniV3Pool {
    fn from(pool: UniswapV3Pool) -> Self {
        NewDbUniV3Pool {
            address: pool.address.to_string(),
            chain: pool.chain.as_str().to_string(),
            exchange_name: Some(pool.exchange_name.as_str().to_string()),
            exchange_type: Some(pool.exchange_type.as_str().to_string()),
            token_a: pool.token_a.to_string(),
            token_a_symbol: pool.token_a_symbol,
            token_a_decimals: pool.token_a_decimals as i32,
            token_b: pool.token_b.to_string(),
            token_b_symbol: pool.token_b_symbol,
            token_b_decimals: pool.token_b_decimals as i32,
            sqrt_price: Some(pool.sqrt_price.to_string()),
            liquidity: Some(pool.liquidity.to_string()),
            tick: Some(pool.tick),
            fee: Some(pool.fee as i32),
            tick_spacing: Some(pool.tick_spacing),
            tick_bitmap: Some(serde_json::to_value(&pool.tick_bitmap).unwrap()),
            ticks: Some(serde_json::to_value(&pool.ticks).unwrap()),
            factory_address: Some(pool.factory.to_string()),
            tag: None,
        }
    }
}

#[async_trait]
impl AutomatedMarketMaker for UniswapV3Pool {
    fn address(&self) -> Address {
        self.address
    }

    fn name(&self) -> String {
        let symbols = self.token_symbols();
        let exchange_name = self.exchange_name();
        format!("{}:{}-{}", exchange_name, symbols[0], symbols[1])
    }

    #[instrument(skip(self, provider), level = "debug")]
    async fn sync<N, P>(&mut self, provider: Arc<P>) -> Result<(), AMMError>
    where
        N: Network,
        P: Provider<N>,
    {
        batch_request::sync_v3_pool_batch_request(self, provider.clone()).await?;
        Ok(())
    }

    // This defines the event signatures to listen to that will produce events to be passed into AMM::sync_from_log()
    fn sync_on_event_signatures(&self) -> Vec<B256> {
        vec![
            IUniswapV3Pool::Swap::SIGNATURE_HASH,
            IUniswapV3Pool::Mint::SIGNATURE_HASH,
            IUniswapV3Pool::Burn::SIGNATURE_HASH,
        ]
    }

    #[instrument(skip(self), level = "debug")]
    fn sync_from_log(&mut self, log: Log) -> Result<(), EventLogError> {
        let event_signature = log.topics()[0];

        if event_signature == IUniswapV3Pool::Burn::SIGNATURE_HASH {
            self.sync_from_burn_log(log)?;
        } else if event_signature == IUniswapV3Pool::Mint::SIGNATURE_HASH {
            self.sync_from_mint_log(log)?;
        } else if event_signature == IUniswapV3Pool::Swap::SIGNATURE_HASH {
            self.sync_from_swap_log(log)?;
        } else {
            Err(EventLogError::InvalidEventSignature)?
        }

        Ok(())
    }

    fn tokens(&self) -> Vec<Address> {
        vec![self.token_a, self.token_b]
    }

    fn calculate_price(&self, base_token: Address) -> Result<f64, ArithmeticError> {
        let tick = uniswap_v3_math::tick_math::get_tick_at_sqrt_ratio(self.sqrt_price)?;
        let shift = self.token_a_decimals as i8 - self.token_b_decimals as i8;

        let price = match shift.cmp(&0) {
            Ordering::Less => 1.0001_f64.powi(tick) / 10_f64.powi(-shift as i32),
            Ordering::Greater => 1.0001_f64.powi(tick) * 10_f64.powi(shift as i32),
            Ordering::Equal => 1.0001_f64.powi(tick),
        };

        if base_token == self.token_a {
            Ok(price)
        } else {
            Ok(1.0 / price)
        }
    }
    // NOTE: This function will not populate the tick_bitmap and ticks, if you want to populate those, you must call populate_tick_data on an initialized pool
    async fn populate_data<N, P>(
        &mut self,
        block_number: Option<u64>,
        provider: Arc<P>,
    ) -> Result<(), AMMError>
    where
        N: Network,
        P: Provider<N>,
    {
        batch_request::get_v3_pool_data_batch_request(
            std::slice::from_mut(self),
            block_number,
            provider.clone(),
        )
        .await?;
        Ok(())
    }

    fn simulate_swap(
        &self,
        token_in: Address,
        amount_in: U256,
        _token_out: Address,
    ) -> Result<U256, SwapSimulationError> {
        if amount_in.is_zero() {
            return Ok(U256::ZERO);
        }

        let zero_for_one = token_in == self.token_a;

        // Set sqrt_price_limit_x_96 to the max or min sqrt price in the pool depending on zero_for_one
        let sqrt_price_limit_x_96 = if zero_for_one {
            MIN_SQRT_RATIO + U256_1
        } else {
            MAX_SQRT_RATIO - U256_1
        };

        // Initialize a mutable state state struct to hold the dynamic simulated state of the pool
        let mut current_state = CurrentState {
            sqrt_price_x_96: self.sqrt_price, //Active price on the pool
            amount_calculated: I256::ZERO,    //Amount of token_out that has been calculated
            amount_specified_remaining: I256::from_raw(amount_in), //Amount of token_in that has not been swapped
            tick: self.tick,                                       //Current i24 tick of the pool
            liquidity: self.liquidity, //Current available liquidity in the tick range
        };

        while current_state.amount_specified_remaining != I256::ZERO
            && current_state.sqrt_price_x_96 != sqrt_price_limit_x_96
        {
            // Initialize a new step struct to hold the dynamic state of the pool at each step
            let mut step = StepComputations {
                // Set the sqrt_price_start_x_96 to the current sqrt_price_x_96
                sqrt_price_start_x_96: current_state.sqrt_price_x_96,
                ..Default::default()
            };

            // Get the next tick from the current tick
            (step.tick_next, step.initialized) =
                uniswap_v3_math::tick_bitmap::next_initialized_tick_within_one_word(
                    &self.tick_bitmap,
                    current_state.tick,
                    self.tick_spacing,
                    zero_for_one,
                )?;

            // ensure that we do not overshoot the min/max tick, as the tick bitmap is not aware of these bounds
            // Note: this could be removed as we are clamping in the batch contract
            step.tick_next = step.tick_next.clamp(MIN_TICK, MAX_TICK);

            // Get the next sqrt price from the input amount
            step.sqrt_price_next_x96 =
                uniswap_v3_math::tick_math::get_sqrt_ratio_at_tick(step.tick_next)?;

            // Target spot price
            let swap_target_sqrt_ratio = if zero_for_one {
                if step.sqrt_price_next_x96 < sqrt_price_limit_x_96 {
                    sqrt_price_limit_x_96
                } else {
                    step.sqrt_price_next_x96
                }
            } else if step.sqrt_price_next_x96 > sqrt_price_limit_x_96 {
                sqrt_price_limit_x_96
            } else {
                step.sqrt_price_next_x96
            };

            // Compute swap step and update the current state
            (
                current_state.sqrt_price_x_96,
                step.amount_in,
                step.amount_out,
                step.fee_amount,
            ) = uniswap_v3_math::swap_math::compute_swap_step(
                current_state.sqrt_price_x_96,
                swap_target_sqrt_ratio,
                current_state.liquidity,
                current_state.amount_specified_remaining,
                self.fee,
            )?;

            // Decrement the amount remaining to be swapped and amount received from the step
            current_state.amount_specified_remaining = current_state
                .amount_specified_remaining
                .overflowing_sub(I256::from_raw(
                    step.amount_in.overflowing_add(step.fee_amount).0,
                ))
                .0;

            current_state.amount_calculated -= I256::from_raw(step.amount_out);

            // If the price moved all the way to the next price, recompute the liquidity change for the next iteration
            if current_state.sqrt_price_x_96 == step.sqrt_price_next_x96 {
                if step.initialized {
                    let mut liquidity_net = if let Some(info) = self.ticks.get(&step.tick_next) {
                        info.liquidity_net
                    } else {
                        0
                    };

                    // we are on a tick boundary, and the next tick is initialized, so we must charge a protocol fee
                    if zero_for_one {
                        liquidity_net = -liquidity_net;
                    }

                    current_state.liquidity = if liquidity_net < 0 {
                        if current_state.liquidity < (-liquidity_net as u128) {
                            return Err(SwapSimulationError::LiquidityUnderflow);
                        } else {
                            current_state.liquidity - (-liquidity_net as u128)
                        }
                    } else {
                        current_state.liquidity + (liquidity_net as u128)
                    };
                }
                // Increment the current tick
                current_state.tick = if zero_for_one {
                    step.tick_next.wrapping_sub(1)
                } else {
                    step.tick_next
                }
                // If the current_state sqrt price is not equal to the step sqrt price, then we are not on the same tick.
                // Update the current_state.tick to the tick at the current_state.sqrt_price_x_96
            } else if current_state.sqrt_price_x_96 != step.sqrt_price_start_x_96 {
                current_state.tick = uniswap_v3_math::tick_math::get_tick_at_sqrt_ratio(
                    current_state.sqrt_price_x_96,
                )?;
            }
        }

        let amount_out = (-current_state.amount_calculated).into_raw();

        tracing::trace!(?amount_out);

        Ok(amount_out)
    }

    fn simulate_swap_mut(
        &mut self,
        token_in: Address,
        amount_in: U256,
        _token_out: Address,
    ) -> Result<U256, SwapSimulationError> {
        if amount_in.is_zero() {
            return Ok(U256::ZERO);
        }

        let zero_for_one = token_in == self.token_a;

        // Set sqrt_price_limit_x_96 to the max or min sqrt price in the pool depending on zero_for_one
        let sqrt_price_limit_x_96 = if zero_for_one {
            MIN_SQRT_RATIO + U256_1
        } else {
            MAX_SQRT_RATIO - U256_1
        };

        // Initialize a mutable state state struct to hold the dynamic simulated state of the pool
        let mut current_state = CurrentState {
            // Active price on the pool
            sqrt_price_x_96: self.sqrt_price,
            // Amount of token_out that has been calculated
            amount_calculated: I256::ZERO,
            // Amount of token_in that has not been swapped
            amount_specified_remaining: I256::from_raw(amount_in),
            // Current i24 tick of the pool
            tick: self.tick,
            // Current available liquidity in the tick range
            liquidity: self.liquidity,
        };

        while current_state.amount_specified_remaining != I256::ZERO
            && current_state.sqrt_price_x_96 != sqrt_price_limit_x_96
        {
            // Initialize a new step struct to hold the dynamic state of the pool at each step
            let mut step = StepComputations {
                // Set the sqrt_price_start_x_96 to the current sqrt_price_x_96
                sqrt_price_start_x_96: current_state.sqrt_price_x_96,
                ..Default::default()
            };

            // Get the next tick from the current tick
            (step.tick_next, step.initialized) =
                uniswap_v3_math::tick_bitmap::next_initialized_tick_within_one_word(
                    &self.tick_bitmap,
                    current_state.tick,
                    self.tick_spacing,
                    zero_for_one,
                )?;

            // ensure that we do not overshoot the min/max tick, as the tick bitmap is not aware of these bounds
            // Note: this could be removed as we are clamping in the batch contract
            step.tick_next = step.tick_next.clamp(MIN_TICK, MAX_TICK);

            // Get the next sqrt price from the input amount
            step.sqrt_price_next_x96 =
                uniswap_v3_math::tick_math::get_sqrt_ratio_at_tick(step.tick_next)?;

            // Target spot price
            let swap_target_sqrt_ratio = if zero_for_one {
                if step.sqrt_price_next_x96 < sqrt_price_limit_x_96 {
                    sqrt_price_limit_x_96
                } else {
                    step.sqrt_price_next_x96
                }
            } else if step.sqrt_price_next_x96 > sqrt_price_limit_x_96 {
                sqrt_price_limit_x_96
            } else {
                step.sqrt_price_next_x96
            };

            // Compute swap step and update the current state
            (
                current_state.sqrt_price_x_96,
                step.amount_in,
                step.amount_out,
                step.fee_amount,
            ) = uniswap_v3_math::swap_math::compute_swap_step(
                current_state.sqrt_price_x_96,
                swap_target_sqrt_ratio,
                current_state.liquidity,
                current_state.amount_specified_remaining,
                self.fee,
            )?;

            // Decrement the amount remaining to be swapped and amount received from the step
            current_state.amount_specified_remaining = current_state
                .amount_specified_remaining
                .overflowing_sub(I256::from_raw(
                    step.amount_in.overflowing_add(step.fee_amount).0,
                ))
                .0;

            current_state.amount_calculated -= I256::from_raw(step.amount_out);

            // If the price moved all the way to the next price, recompute the liquidity change for the next iteration
            if current_state.sqrt_price_x_96 == step.sqrt_price_next_x96 {
                if step.initialized {
                    let mut liquidity_net = if let Some(info) = self.ticks.get(&step.tick_next) {
                        info.liquidity_net
                    } else {
                        0
                    };

                    // we are on a tick boundary, and the next tick is initialized, so we must charge a protocol fee
                    if zero_for_one {
                        liquidity_net = -liquidity_net;
                    }

                    current_state.liquidity = if liquidity_net < 0 {
                        if current_state.liquidity < (-liquidity_net as u128) {
                            return Err(SwapSimulationError::LiquidityUnderflow);
                        } else {
                            current_state.liquidity - (-liquidity_net as u128)
                        }
                    } else {
                        current_state.liquidity + (liquidity_net as u128)
                    };
                }
                // Increment the current tick
                current_state.tick = if zero_for_one {
                    step.tick_next.wrapping_sub(1)
                } else {
                    step.tick_next
                }
                // If the current_state sqrt price is not equal to the step sqrt price, then we are not on the same tick.
                // Update the current_state.tick to the tick at the current_state.sqrt_price_x_96
            } else if current_state.sqrt_price_x_96 != step.sqrt_price_start_x_96 {
                current_state.tick = uniswap_v3_math::tick_math::get_tick_at_sqrt_ratio(
                    current_state.sqrt_price_x_96,
                )?;
            }
        }

        // Update the pool state
        self.liquidity = current_state.liquidity;
        self.sqrt_price = current_state.sqrt_price_x_96;
        self.tick = current_state.tick;

        let amount_out = (-current_state.amount_calculated).into_raw();

        tracing::trace!(?amount_out);

        Ok(amount_out)
    }

    // fn get_token_out(&self, token_in: Address) -> Address {
    //     if self.token_a == token_in {
    //         self.token_b
    //     } else {
    //         self.token_a
    //     }
    // }

    fn token_symbols(&self) -> Vec<String> {
        vec![self.token_a_symbol.clone(), self.token_b_symbol.clone()]
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

    fn to_new_db_pool(&self, tag: Option<String>) -> NewDbPool {
        NewDbPool::UniV3(NewDbUniV3Pool {
            address: self.address.to_string(),
            chain: self.chain.as_str().to_string(),
            exchange_name: Some(self.exchange_name.as_str().to_string()),
            exchange_type: Some(self.exchange_type.as_str().to_string()),
            token_a: self.token_a.to_string(),
            token_a_symbol: self.token_a_symbol.clone(),
            token_a_decimals: self.token_a_decimals as i32,
            token_b: self.token_b.to_string(),
            token_b_symbol: self.token_b_symbol.clone(),
            token_b_decimals: self.token_b_decimals as i32,
            sqrt_price: Some(self.sqrt_price.to_string()),
            liquidity: Some(self.liquidity.to_string()),
            tick: Some(self.tick as i32),
            fee: Some(self.fee as i32),
            tick_spacing: Some(self.tick_spacing as i32),
            tick_bitmap: None,
            ticks: None,
            factory_address: Some(self.factory.to_string()),
            tag: tag,
        })
    }
}

impl UniswapV3Pool {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        address: Address,
        token_a: Address,
        token_a_decimals: u8,
        token_a_symbol: String,
        token_b: Address,
        token_b_decimals: u8,
        token_b_symbol: String,
        fee: u32,
        liquidity: u128,
        liquidity_net: i128,
        factory: Address,
        sqrt_price: U256,
        tick: i32,
        tick_spacing: i32,
        tick_bitmap: HashMap<i16, U256>,
        ticks: HashMap<i32, Info>,
        exchange_name: ExchangeName,
        exchange_type: ExchangeType,
        chain: NamedChain,
    ) -> UniswapV3Pool {
        UniswapV3Pool {
            address,
            token_a,
            token_a_decimals,
            token_a_symbol,
            token_b,
            token_b_decimals,
            token_b_symbol,
            factory,
            liquidity,
            liquidity_net,
            fee,
            sqrt_price,
            tick,
            tick_spacing,
            tick_bitmap,
            ticks,
            exchange_name,
            exchange_type,
            chain,
        }
    }

    pub fn get_token_out(&self, token_in: Address) -> Address {
        if self.token_a == token_in {
            self.token_b
        } else {
            self.token_a
        }
    }

    pub fn new_empty(address: Address, chain: NamedChain) -> Result<Self, AMMError> {
        let pool = UniswapV3Pool {
            address,
            factory: Address::ZERO,
            liquidity_net: 0,
            token_a: Address::ZERO,
            token_a_decimals: 0,
            token_a_symbol: String::new(),
            token_b: Address::ZERO,
            token_b_decimals: 0,
            token_b_symbol: String::new(),
            liquidity: 0,
            sqrt_price: U256::ZERO,
            tick: 0,
            tick_spacing: 0,
            fee: 0,
            tick_bitmap: HashMap::new(),
            ticks: HashMap::new(),
            exchange_name: ExchangeName::Unknown,
            exchange_type: ExchangeType::Unknown,
            chain,
        };

        Ok(pool)
    }

    /// Creates a new instance of the pool from the pair address.
    ///
    /// This function will populate all pool data.
    pub async fn new_from_address<N, P>(
        pair_address: Address,
        creation_block: u64,
        provider: Arc<P>,
    ) -> Result<Self, AMMError>
    where
        N: Network,
        P: Provider<N>,
    {
        let mut pool = UniswapV3Pool {
            address: pair_address,
            factory: Address::ZERO,
            liquidity_net: 0,
            token_a: Address::ZERO,
            token_a_decimals: 0,
            token_a_symbol: String::new(),
            token_b: Address::ZERO,
            token_b_decimals: 0,
            token_b_symbol: String::new(),
            liquidity: 0,
            sqrt_price: U256::ZERO,
            tick: 0,
            tick_spacing: 0,
            fee: 0,
            tick_bitmap: HashMap::new(),
            ticks: HashMap::new(),
            exchange_name: ExchangeName::UniswapV3,
            exchange_type: ExchangeType::UniV3,
            chain: NamedChain::Mainnet,
        };

        // We need to get tick spacing before populating tick data because tick spacing can not be uninitialized when syncing burn and mint logs
        tracing::info!("Getting tick spacing");
        pool.tick_spacing = pool.get_tick_spacing(provider.clone()).await?;

        tracing::info!("Populating tick data");
        let synced_block = pool
            .populate_tick_data(creation_block, provider.clone())
            .await?;

        // TODO: break this into two threads so it can happen concurrently
        tracing::info!("Populating data");
        pool.populate_data(Some(synced_block), provider).await?;

        if !pool.data_is_populated() {
            return Err(AMMError::PoolDataError);
        }

        Ok(pool)
    }

    /// Creates a new instance of the pool from a log.
    ///
    /// This function will populate all pool data.
    pub async fn new_from_log<N, P>(log: Log, provider: Arc<P>) -> Result<Self, AMMError>
    where
        N: Network,
        P: Provider<N>,
    {
        let event_signature = log.topics()[0];

        if event_signature == IUniswapV3Factory::PoolCreated::SIGNATURE_HASH {
            if let Some(block_number) = log.block_number {
                let pool_created_event =
                    IUniswapV3Factory::PoolCreated::decode_log(&log.inner, true)?;

                UniswapV3Pool::new_from_address(pool_created_event.pool, block_number, provider)
                    .await
            } else {
                Err(EventLogError::LogBlockNumberNotFound)?
            }
        } else {
            Err(EventLogError::InvalidEventSignature)?
        }
    }
    /// Creates a new instance of the pool from a log.
    ///
    /// This function will not populate all pool data.
    pub fn new_empty_pool_from_log(log: Log) -> Result<Self, EventLogError> {
        let event_signature = log.topics()[0];

        if event_signature == IUniswapV3Factory::PoolCreated::SIGNATURE_HASH {
            let pool_created_event =
                IUniswapV3Factory::PoolCreated::decode_log(log.as_ref(), true)?;

            Ok(UniswapV3Pool {
                address: pool_created_event.pool,
                token_a: pool_created_event.token0,
                token_a_decimals: 0,
                token_a_symbol: String::new(),
                token_b: pool_created_event.token1,
                token_b_decimals: 0,
                token_b_symbol: String::new(),
                fee: pool_created_event.fee.to(),
                liquidity: 0,
                sqrt_price: U256::ZERO,
                tick_spacing: 0,
                tick: 0,
                tick_bitmap: HashMap::new(),
                ticks: HashMap::new(),
                exchange_name: ExchangeName::Unknown,
                exchange_type: ExchangeType::Unknown,
                chain: NamedChain::Mainnet,
                factory: Address::ZERO,
                liquidity_net: 0,
            })
        } else {
            Err(EventLogError::InvalidEventSignature)
        }
    }

    #[async_recursion]
    pub async fn get_batch_logs<N, P>(
        provider: Arc<P>,
        address: Address,
        from_block: u64,
        to_block: u64,
    ) -> Result<Vec<Log>, AMMError>
    where
        N: Network,
        P: Provider<N>,
    {
        let mut logs = vec![];
        let mut step_block = to_block - from_block;
        let filter = Filter::new()
            .event_signature(vec![
                IUniswapV3Pool::Burn::SIGNATURE_HASH,
                IUniswapV3Pool::Mint::SIGNATURE_HASH,
            ])
            .address(address)
            .from_block(from_block)
            .to_block(to_block);
        let res = provider.get_logs(&filter).await;
        if res.is_err() {
            if step_block == 0 {
                return Err(AMMError::TransportError(res.err().unwrap()));
            }
            tracing::debug!(
                "addr: {:?}, start_block: {}, end_block: {}, step: {}, err: {:?}",
                address,
                from_block,
                to_block,
                step_block,
                res.err()
            );
            step_block /= 2;
            let left_lgos = Self::get_batch_logs(
                provider.clone(),
                address,
                from_block,
                from_block + step_block,
            )
            .await?;
            let right_logs = Self::get_batch_logs(
                provider.clone(),
                address,
                from_block + step_block + 1,
                to_block,
            )
            .await?;
            logs.extend(left_lgos);
            logs.extend(right_logs);
        } else {
            logs = res.unwrap();
            tracing::debug!(
                "addr: {:?}, start_block: {}, end_block: {}, step: {}, logs: {:?}",
                address,
                from_block,
                to_block,
                step_block,
                logs.len()
            );
        }
        Ok(logs)
    }

    /// Populates the `tick_bitmap` and `ticks` fields of the pool to the current block.
    ///
    /// Returns the last synced block number.
    pub async fn populate_tick_data_sync<N, P>(
        &mut self,
        mut from_block: u64,
        provider: Arc<P>,
    ) -> Result<u64, AMMError>
    where
        N: Network,
        P: Provider<N>,
    {
        let current_block = provider
            .get_block_number()
            .await
            .map_err(AMMError::TransportError)?;

        let mut ordered_logs: BTreeMap<u64, Vec<Log>> = BTreeMap::new();
        let pool_address: Address = self.address;

        while from_block <= current_block {
            let provider = provider.clone();

            tracing::info!("Getting logs from block {:?}", from_block);

            let mut target_block = from_block + POPULATE_TICK_DATA_STEP - 1;
            if target_block > current_block {
                target_block = current_block;
            }

            let result =
                Self::get_batch_logs(provider, pool_address, from_block, target_block).await;

            let logs = result?;

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

            from_block += POPULATE_TICK_DATA_STEP;
        }

        for (_, log_group) in ordered_logs {
            for log in log_group {
                self.sync_from_log(log)?;
            }
        }

        Ok(current_block)
    }

    /// Populates the `tick_bitmap` and `ticks` fields of the pool to the current block.
    ///
    /// Returns the last synced block number.
    pub async fn populate_tick_data<N, P>(
        &mut self,
        mut from_block: u64,
        provider: Arc<P>,
    ) -> Result<u64, AMMError>
    where
        N: Network,
        P: Provider<N>,
    {
        let current_block = provider
            .get_block_number()
            .await
            .map_err(AMMError::TransportError)?;

        let mut futures = FuturesOrdered::new();

        let mut ordered_logs: BTreeMap<u64, Vec<Log>> = BTreeMap::new();

        let pool_address: Address = self.address;

        while from_block <= current_block {
            let provider = provider.clone();

            tracing::info!("Getting logs from block {:?}", from_block);

            let mut target_block = from_block + POPULATE_TICK_DATA_STEP - 1;
            if target_block > current_block {
                target_block = current_block;
            }

            futures.push_back(async move {
                Self::get_batch_logs(provider, pool_address, from_block, target_block).await
            });

            from_block += POPULATE_TICK_DATA_STEP;
        }

        // TODO: this could be more dry since we use this in another place
        while let Some(result) = futures.next().await {
            let logs = result?;

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
        }

        for (_, log_group) in ordered_logs {
            for log in log_group {
                self.sync_from_log(log)?;
            }
        }

        Ok(current_block)
    }

    /// Returns the swap fee of the pool.
    pub fn fee(&self) -> u32 {
        self.fee
    }

    /// Returns whether the pool data is populated.
    pub fn data_is_populated(&self) -> bool {
        !(self.token_a.is_zero() || self.token_b.is_zero())
    }

    /// Returns the word position of a tick in the `tick_bitmap`.
    pub async fn get_tick_word<N, P>(&self, tick: i32, provider: Arc<P>) -> Result<U256, AMMError>
    where
        N: Network,
        P: Provider<N>,
    {
        let v3_pool = IUniswapV3Pool::new(self.address, provider);
        let (word_position, _) = uniswap_v3_math::tick_bitmap::position(tick);
        let IUniswapV3Pool::tickBitmapReturn { _0: bm } =
            v3_pool.tickBitmap(word_position).call().await?;
        Ok(bm)
    }

    /// Returns the next word in the `tick_bitmap` after a given word position.
    pub async fn get_next_word<N, P>(
        &self,
        word_position: i16,
        provider: Arc<P>,
    ) -> Result<U256, AMMError>
    where
        N: Network,
        P: Provider<N>,
    {
        let v3_pool = IUniswapV3Pool::new(self.address, provider);
        let IUniswapV3Pool::tickBitmapReturn { _0: bm } =
            v3_pool.tickBitmap(word_position).call().await?;
        Ok(bm)
    }

    /// Returns the tick spacing of the pool.
    pub async fn get_tick_spacing<N, P>(&self, provider: Arc<P>) -> Result<i32, AMMError>
    where
        N: Network,
        P: Provider<N>,
    {
        let v3_pool = IUniswapV3Pool::new(self.address, provider);
        let IUniswapV3Pool::tickSpacingReturn { _0: ts } = v3_pool.tickSpacing().call().await?;
        Ok(ts.unchecked_into())
    }

    /// Fetches the current tick of the pool via static call.
    pub async fn get_tick<N, P>(&self, provider: Arc<P>) -> Result<i32, AMMError>
    where
        N: Network,
        P: Provider<N>,
    {
        Ok(self.get_slot_0(provider).await?.1)
    }

    /// Fetches the tick info of a given tick via static call.
    pub async fn get_tick_info<N, P>(
        &self,
        tick: i32,
        provider: Arc<P>,
    ) -> Result<(u128, i128, U256, U256, i64, U256, u32, bool), AMMError>
    where
        N: Network,
        P: Provider<N>,
    {
        let v3_pool = IUniswapV3Pool::new(self.address, provider.clone());

        let tick_info = v3_pool.ticks(I24::unchecked_from(tick)).call().await?;

        Ok((
            tick_info._0,
            tick_info._1,
            tick_info._2,
            tick_info._3,
            tick_info._4.unchecked_into(),
            U256::from(tick_info._5),
            tick_info._6,
            tick_info._7,
        ))
    }

    /// Fetches `liquidity_net` at a given tick via static call.
    pub async fn get_liquidity_net<N, P>(
        &self,
        tick: i32,
        provider: Arc<P>,
    ) -> Result<i128, AMMError>
    where
        N: Network,
        P: Provider<N>,
    {
        let tick_info = self.get_tick_info(tick, provider).await?;
        Ok(tick_info.1)
    }

    /// Fetches whether a specified tick is initialized via static call.
    pub async fn get_initialized<N, P>(&self, tick: i32, provider: Arc<P>) -> Result<bool, AMMError>
    where
        N: Network,
        P: Provider<N>,
    {
        let tick_info = self.get_tick_info(tick, provider).await?;
        Ok(tick_info.7)
    }

    /// Fetches the current slot 0 of the pool via static call.
    pub async fn get_slot_0<N, P>(
        &self,
        provider: Arc<P>,
    ) -> Result<(U256, i32, u16, u16, u16, u8, bool), AMMError>
    where
        N: Network,
        P: Provider<N>,
    {
        let v3_pool = IUniswapV3Pool::new(self.address, provider);
        let IUniswapV3Pool::slot0Return {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
        } = v3_pool.slot0().call().await?;

        Ok((_0.to(), _1.unchecked_into(), _2, _3, _4, _5, _6))
    }

    /// Fetches the current liquidity of the pool via static call.
    pub async fn get_liquidity<N, P>(&self, provider: Arc<P>) -> Result<u128, AMMError>
    where
        N: Network,
        P: Provider<N>,
    {
        let v3_pool = IUniswapV3Pool::new(self.address, provider);
        let IUniswapV3Pool::liquidityReturn { _0: liquidity } = v3_pool.liquidity().call().await?;
        Ok(liquidity)
    }

    /// Fetches the current sqrt price of the pool via static call.
    pub async fn get_sqrt_price<N, P>(&self, provider: Arc<P>) -> Result<U256, AMMError>
    where
        N: Network,
        P: Provider<N>,
    {
        Ok(self.get_slot_0(provider).await?.0)
    }

    /// Updates the pool state from a burn event log.
    pub fn sync_from_burn_log(&mut self, log: Log) -> Result<(), alloy::dyn_abi::Error> {
        let burn_event = IUniswapV3Pool::Burn::decode_log(log.as_ref(), true)?;

        self.modify_position(
            burn_event.tickLower.unchecked_into(),
            burn_event.tickUpper.unchecked_into(),
            -(burn_event.amount as i128),
        );

        tracing::debug!(?burn_event, address = ?self.address, sqrt_price = ?self.sqrt_price, liquidity = ?self.liquidity, tick = ?self.tick, "UniswapV3 burn event");

        Ok(())
    }

    /// Updates the pool state from a mint event log.
    pub fn sync_from_mint_log(&mut self, log: Log) -> Result<(), alloy::dyn_abi::Error> {
        let mint_event = IUniswapV3Pool::Mint::decode_log(log.as_ref(), true)?;

        self.modify_position(
            mint_event.tickLower.unchecked_into(),
            mint_event.tickUpper.unchecked_into(),
            mint_event.amount as i128,
        );

        tracing::debug!(?mint_event, address = ?self.address, sqrt_price = ?self.sqrt_price, liquidity = ?self.liquidity, tick = ?self.tick, "UniswapV3 mint event");

        Ok(())
    }

    pub fn populate_ticks_from_tick_data(&mut self, tick_data: Vec<PopulatedTick>) {
        self.ticks = HashMap::new();
        self.tick_bitmap = HashMap::new();
        for tick in tick_data {
            self.ticks.insert(
                tick.tick.as_i32(),
                Info {
                    liquidity_gross: tick.liquidityGross,
                    liquidity_net: tick.liquidityNet,
                    initialized: tick.liquidityGross > 0,
                },
            );

            if tick.liquidityGross > 0 {
                self.flip_tick(tick.tick.as_i32(), self.tick_spacing);
            }
        }
    }

    /// Modifies a positions liquidity in the pool.
    pub fn modify_position(&mut self, tick_lower: i32, tick_upper: i32, liquidity_delta: i128) {
        //We are only using this function when a mint or burn event is emitted,
        //therefore we do not need to checkTicks as that has happened before the event is emitted
        self.update_position(tick_lower, tick_upper, liquidity_delta);

        if liquidity_delta != 0 {
            //if the tick is between the tick lower and tick upper, update the liquidity between the ticks
            if self.tick > tick_lower && self.tick < tick_upper {
                self.liquidity = if liquidity_delta < 0 {
                    self.liquidity - ((-liquidity_delta) as u128)
                } else {
                    self.liquidity + (liquidity_delta as u128)
                }
            }
        }
    }

    pub fn update_position(&mut self, tick_lower: i32, tick_upper: i32, liquidity_delta: i128) {
        let mut flipped_lower = false;
        let mut flipped_upper = false;

        if liquidity_delta != 0 {
            flipped_lower = self.update_tick(tick_lower, liquidity_delta, false);
            flipped_upper = self.update_tick(tick_upper, liquidity_delta, true);
            if flipped_lower {
                self.flip_tick(tick_lower, self.tick_spacing);
            }
            if flipped_upper {
                self.flip_tick(tick_upper, self.tick_spacing);
            }
        }

        if liquidity_delta < 0 {
            if flipped_lower {
                self.ticks.remove(&tick_lower);
            }

            if flipped_upper {
                self.ticks.remove(&tick_upper);
            }
        }
    }

    pub fn update_tick(&mut self, tick: i32, liquidity_delta: i128, upper: bool) -> bool {
        let info = match self.ticks.get_mut(&tick) {
            Some(info) => info,
            None => {
                self.ticks.insert(tick, Info::default());
                self.ticks
                    .get_mut(&tick)
                    .expect("Tick does not exist in ticks")
            }
        };

        let liquidity_gross_before = info.liquidity_gross;

        let liquidity_gross_after = if liquidity_delta < 0 {
            liquidity_gross_before - ((-liquidity_delta) as u128)
        } else {
            liquidity_gross_before + (liquidity_delta as u128)
        };

        // we do not need to check if liqudity_gross_after > maxLiquidity because we are only calling update tick on a burn or mint log.
        // this should already be validated when a log is
        let flipped = (liquidity_gross_after == 0) != (liquidity_gross_before == 0);

        if liquidity_gross_before == 0 {
            info.initialized = true;
        }

        info.liquidity_gross = liquidity_gross_after;

        info.liquidity_net = if upper {
            info.liquidity_net - liquidity_delta
        } else {
            info.liquidity_net + liquidity_delta
        };

        flipped
    }

    pub fn flip_tick(&mut self, tick: i32, tick_spacing: i32) {
        let (word_pos, bit_pos) = uniswap_v3_math::tick_bitmap::position(tick / tick_spacing);
        let mask = U256::from(1) << bit_pos;

        if let Some(word) = self.tick_bitmap.get_mut(&word_pos) {
            *word ^= mask;
        } else {
            self.tick_bitmap.insert(word_pos, mask);
        }
    }

    /// Updates the pool state from a swap event log.
    pub fn sync_from_swap_log(&mut self, log: Log) -> Result<(), alloy::sol_types::Error> {
        let swap_event = IUniswapV3Pool::Swap::decode_log(log.as_ref(), true)?;

        self.sqrt_price = swap_event.sqrtPriceX96.to();
        self.liquidity = swap_event.liquidity;
        self.tick = swap_event.tick.unchecked_into();

        tracing::debug!(?swap_event, address = ?self.address, sqrt_price = ?self.sqrt_price, liquidity = ?self.liquidity, tick = ?self.tick, "UniswapV3 swap event");

        Ok(())
    }

    pub async fn get_token_decimals<N, P>(&mut self, provider: Arc<P>) -> Result<(u8, u8), AMMError>
    where
        N: Network,
        P: Provider<N>,
    {
        let IErc20::decimalsReturn {
            _0: token_a_decimals,
        } = IErc20::new(self.token_a, provider.clone())
            .decimals()
            .call()
            .await?;

        let IErc20::decimalsReturn {
            _0: token_b_decimals,
        } = IErc20::new(self.token_b, provider)
            .decimals()
            .call()
            .await?;

        Ok((token_a_decimals, token_b_decimals))
    }

    pub async fn get_fee<N, P>(&mut self, provider: Arc<P>) -> Result<u32, AMMError>
    where
        N: Network,
        P: Provider<N>,
    {
        let IUniswapV3Pool::feeReturn { _0: fee } = IUniswapV3Pool::new(self.address, provider)
            .fee()
            .call()
            .await?;

        Ok(fee.to::<u32>())
    }

    pub async fn get_token_0<N, P>(&self, provider: Arc<P>) -> Result<Address, AMMError>
    where
        N: Network,
        P: Provider<N>,
    {
        let v3_pool = IUniswapV3Pool::new(self.address, provider);

        let IUniswapV3Pool::token0Return { _0: token_0 } = match v3_pool.token0().call().await {
            Ok(result) => result,
            Err(contract_error) => return Err(AMMError::ContractError(contract_error)),
        };

        Ok(token_0)
    }

    pub async fn get_token_1<N, P>(&self, provider: Arc<P>) -> Result<Address, AMMError>
    where
        N: Network,
        P: Provider<N>,
    {
        let v3_pool = IUniswapV3Pool::new(self.address, provider);

        let IUniswapV3Pool::token1Return { _0: token_1 } = match v3_pool.token1().call().await {
            Ok(result) => result,
            Err(contract_error) => return Err(AMMError::ContractError(contract_error)),
        };

        Ok(token_1)
    }
    /* Legend:
       sqrt(price) = sqrt(y/x)
       L = sqrt(x*y)
       ==> x = L^2/price
       ==> y = L^2*price
    */
    pub fn calculate_virtual_reserves(&self) -> Result<(u128, u128), ArithmeticError> {
        let tick = uniswap_v3_math::tick_math::get_tick_at_sqrt_ratio(self.sqrt_price)?;
        let price = 1.0001_f64.powi(tick);

        let sqrt_price = BigFloat::from_f64(price.sqrt());

        //Sqrt price is stored as a Q64.96 so we need to left shift the liquidity by 96 to be represented as Q64.96
        //We cant right shift sqrt_price because it could move the value to 0, making division by 0 to get reserve_x
        let liquidity = BigFloat::from_u128(self.liquidity);

        let (reserve_0, reserve_1) = if !sqrt_price.is_zero() {
            let reserve_x = liquidity.div(&sqrt_price);
            let reserve_y = liquidity.mul(&sqrt_price);

            (reserve_x, reserve_y)
        } else {
            (BigFloat::from(0), BigFloat::from(0))
        };

        Ok((
            reserve_0
                .to_u128()
                .ok_or(ArithmeticError::U128ConversionError)?,
            reserve_1
                .to_u128()
                .ok_or(ArithmeticError::U128ConversionError)?,
        ))
    }

    pub fn calculate_compressed(&self, tick: i32) -> i32 {
        if tick < 0 && tick % self.tick_spacing != 0 {
            (tick / self.tick_spacing) - 1
        } else {
            tick / self.tick_spacing
        }
    }

    pub fn calculate_word_pos_bit_pos(&self, compressed: i32) -> (i16, u8) {
        uniswap_v3_math::tick_bitmap::position(compressed)
    }

    /// Returns the call data for a swap.
    pub fn swap_calldata(
        &self,
        recipient: Address,
        zero_for_one: bool,
        amount_specified: I256,
        sqrt_price_limit_x_96: U256,
        calldata: Vec<u8>,
    ) -> Result<Bytes, alloy::dyn_abi::Error> {
        Ok(IUniswapV3Pool::swapCall {
            recipient,
            zeroForOne: zero_for_one,
            amountSpecified: amount_specified,
            sqrtPriceLimitX96: sqrt_price_limit_x_96.to(),
            data: calldata.into(),
        }
        .abi_encode()
        .into())
    }

    pub fn token_symbols(&self) -> Vec<String> {
        vec![self.token_a_symbol.clone(), self.token_b_symbol.clone()]
    }

    pub fn exchange_name(&self) -> ExchangeName {
        self.exchange_name
    }

    pub fn exchange_type(&self) -> ExchangeType {
        self.exchange_type
    }

    pub fn chain(&self) -> NamedChain {
        self.chain
    }
}

pub struct CurrentState {
    amount_specified_remaining: I256,
    amount_calculated: I256,
    sqrt_price_x_96: U256,
    tick: i32,
    liquidity: u128,
}

#[derive(Default)]
pub struct StepComputations {
    pub sqrt_price_start_x_96: U256,
    pub tick_next: i32,
    pub initialized: bool,
    pub sqrt_price_next_x96: U256,
    pub amount_in: U256,
    pub amount_out: U256,
    pub fee_amount: U256,
}

pub struct Tick {
    pub liquidity_gross: u128,
    pub liquidity_net: i128,
    pub fee_growth_outside_0_x_128: U256,
    pub fee_growth_outside_1_x_128: U256,
    pub tick_cumulative_outside: U256,
    pub seconds_per_liquidity_outside_x_128: U256,
    pub seconds_outside: u32,
    pub initialized: bool,
}

#[cfg(test)]
mod test {

    use super::*;

    use alloy::{
        primitives::{address, aliases::U24, U160},
        providers::ProviderBuilder,
    };
    use batch_request::get_uniswap_v3_tick_data_batch_request;

    sol! {
        /// Interface of the Quoter
        #[derive(Debug, PartialEq, Eq)]
        #[sol(rpc)]
        contract IQuoter {
            function quoteExactInputSingle(address tokenIn, address tokenOut,uint24 fee, uint256 amountIn, uint160 sqrtPriceLimitX96) external returns (uint256 amountOut);
        }
    }

    async fn initialize_pool<N, P>(
        provider: Arc<P>,
        address: Address,
    ) -> eyre::Result<(UniswapV3Pool, u64)>
    where
        N: Network,
        P: Provider<N>,
    {
        let mut pool = UniswapV3Pool {
            address: address,
            ..Default::default()
        };

        pool.tick_spacing = pool.get_tick_spacing(provider.clone()).await?;
        let synced_block = provider.get_block_number().await?;

        pool.populate_data(Some(synced_block), provider.clone())
            .await?;
        let current_tick = pool.tick;
        let num_ticks = 2000 * pool.tick_spacing;
        let tick_start = current_tick - num_ticks / 2;

        let (tick_data, _) = get_uniswap_v3_tick_data_batch_request(
            &pool,
            tick_start,
            num_ticks,
            Some(synced_block),
            provider.clone(),
            1, //uniswap v3
        )
        .await?;

        pool.populate_ticks_from_tick_data(tick_data);

        Ok((pool, synced_block))
    }

    #[tokio::test]
    #[ignore] // Ignoring to not throttle the Provider on workflows
    async fn test_simulate_swap_usdc_weth() {
        let rpc_endpoint = std::env::var("MAINNET_RPC_URL").unwrap();
        let provider = Arc::new(ProviderBuilder::new().on_http(rpc_endpoint.parse().unwrap()));

        let (pool, synced_block) = initialize_pool(
            provider.clone(),
            address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640"),
        )
        .await
        .unwrap();
        let quoter = IQuoter::new(
            address!("b27308f9f90d607463bb33ea1bebb41c27ce5ab6"),
            provider.clone(),
        );

        let amount_in = U256::from(100000000); // 100 USDC
        let amount_out = pool
            .simulate_swap(pool.token_a, amount_in, pool.token_b)
            .unwrap();
        let expected_amount_out = quoter
            .quoteExactInputSingle(
                pool.token_a,
                pool.token_b,
                U24::from(pool.fee),
                amount_in,
                U160::ZERO,
            )
            .block(synced_block.into())
            .call()
            .await
            .unwrap();

        assert_eq!(
            amount_out, expected_amount_out.amountOut,
            "invalid amount_out: {}, expected_amount_out: {}",
            amount_out, expected_amount_out.amountOut
        );

        let amount_in_1 = U256::from(10000000000_u64); // 10_000 USDC
        let amount_out_1 = pool
            .simulate_swap(pool.token_a, amount_in_1, pool.token_b)
            .unwrap();
        let expected_amount_out_1 = quoter
            .quoteExactInputSingle(
                pool.token_a,
                pool.token_b,
                U24::from(pool.fee),
                amount_in_1,
                U160::ZERO,
            )
            .block(synced_block.into())
            .call()
            .await
            .unwrap();
        assert_eq!(
            amount_out_1, expected_amount_out_1.amountOut,
            "invalid amount_out_1: {}, expected_amount_out_1: {}",
            amount_out_1, expected_amount_out_1.amountOut
        );

        let amount_in_2 = U256::from(1000000000000_u128); // 1_000_000 USDC
        let amount_out_2 = pool
            .simulate_swap(pool.token_a, amount_in_2, pool.token_b)
            .unwrap();
        let expected_amount_out_2 = quoter
            .quoteExactInputSingle(
                pool.token_a,
                pool.token_b,
                U24::from(pool.fee),
                amount_in_2,
                U160::ZERO,
            )
            .block(synced_block.into())
            .call()
            .await
            .unwrap();

        assert_eq!(
            amount_out_2, expected_amount_out_2.amountOut,
            "invalid amount_out_2: {}, expected_amount_out_2: {}",
            amount_out_2, expected_amount_out_2.amountOut
        );
    }

    #[tokio::test]
    #[ignore] // Ignoring to not throttle the Provider on workflows
    async fn test_simulate_swap_weth_usdc() {
        let rpc_endpoint = std::env::var("MAINNET_RPC_URL").unwrap();
        let provider = Arc::new(ProviderBuilder::new().on_http(rpc_endpoint.parse().unwrap()));

        let (pool, synced_block) = initialize_pool(
            provider.clone(),
            address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640"),
        )
        .await
        .unwrap();
        let quoter = IQuoter::new(
            address!("b27308f9f90d607463bb33ea1bebb41c27ce5ab6"),
            provider.clone(),
        );

        let amount_in = U256::from(1000000000000000000_u128); // 1 ETH
        let amount_out = pool
            .simulate_swap(pool.token_b, amount_in, pool.token_a)
            .unwrap();
        let expected_amount_out = quoter
            .quoteExactInputSingle(
                pool.token_b,
                pool.token_a,
                U24::from(pool.fee),
                amount_in,
                U160::ZERO,
            )
            .block(synced_block.into())
            .call()
            .await
            .unwrap();

        assert_eq!(
            amount_out, expected_amount_out.amountOut,
            "invalid amount_out: {}, expected_amount_out: {}",
            amount_out, expected_amount_out.amountOut
        );

        let amount_in_1 = U256::from(10000000000000000000_u128); // 10 ETH
        let amount_out_1 = pool
            .simulate_swap(pool.token_b, amount_in_1, pool.token_a)
            .unwrap();
        let expected_amount_out_1 = quoter
            .quoteExactInputSingle(
                pool.token_b,
                pool.token_a,
                U24::from(pool.fee),
                amount_in_1,
                U160::ZERO,
            )
            .block(synced_block.into())
            .call()
            .await
            .unwrap();

        assert_eq!(
            amount_out_1, expected_amount_out_1.amountOut,
            "invalid amount_out_1: {}, expected_amount_out_1: {}",
            amount_out_1, expected_amount_out_1.amountOut
        );
    }

    #[tokio::test]
    #[ignore] // Ignoring to not throttle the Provider on workflows
    async fn test_simulate_swap_link_weth() {
        let rpc_endpoint = std::env::var("MAINNET_RPC_URL").unwrap();
        let provider = Arc::new(ProviderBuilder::new().on_http(rpc_endpoint.parse().unwrap()));

        let (pool, synced_block) = initialize_pool(
            provider.clone(),
            address!("a6Cc3C2531FdaA6Ae1A3CA84c2855806728693e8"),
        )
        .await
        .unwrap();
        let quoter = IQuoter::new(
            address!("b27308f9f90d607463bb33ea1bebb41c27ce5ab6"),
            provider.clone(),
        );

        let amount_in = U256::from(1000000000000000000_u128); // 1 LINK
        let amount_out = pool
            .simulate_swap(pool.token_a, amount_in, pool.token_b)
            .unwrap();
        let expected_amount_out = quoter
            .quoteExactInputSingle(
                pool.token_a,
                pool.token_b,
                U24::from(pool.fee),
                amount_in,
                U160::ZERO,
            )
            .block(synced_block.into())
            .call()
            .await
            .unwrap();

        assert_eq!(amount_out, expected_amount_out.amountOut);

        let amount_in_1 = U256::from(100000000000000000000_u128); // 100 LINK
        let amount_out_1 = pool
            .simulate_swap(pool.token_a, amount_in_1, pool.token_b)
            .unwrap();
        let expected_amount_out_1 = quoter
            .quoteExactInputSingle(
                pool.token_a,
                pool.token_b,
                U24::from(pool.fee),
                amount_in_1,
                U160::ZERO,
            )
            .block(synced_block.into())
            .call()
            .await
            .unwrap();

        assert_eq!(amount_out_1, expected_amount_out_1.amountOut);

        let amount_in_2 = U256::from(10000000000000000000000_u128); // 10_000 LINK
        let amount_out_2 = pool
            .simulate_swap(pool.token_a, amount_in_2, pool.token_b)
            .unwrap();
        let expected_amount_out_2 = quoter
            .quoteExactInputSingle(
                pool.token_a,
                pool.token_b,
                U24::from(pool.fee),
                amount_in_2,
                U160::ZERO,
            )
            .block(synced_block.into())
            .call()
            .await
            .unwrap();

        assert_eq!(amount_out_2, expected_amount_out_2.amountOut);
    }

    #[tokio::test]
    #[ignore] // Ignoring to not throttle the Provider on workflows
    async fn test_simulate_swap_weth_link() {
        let rpc_endpoint = std::env::var("MAINNET_RPC_URL").unwrap();
        let provider = Arc::new(ProviderBuilder::new().on_http(rpc_endpoint.parse().unwrap()));

        let (pool, synced_block) = initialize_pool(
            provider.clone(),
            address!("a6Cc3C2531FdaA6Ae1A3CA84c2855806728693e8"),
        )
        .await
        .unwrap();
        let quoter = IQuoter::new(
            address!("b27308f9f90d607463bb33ea1bebb41c27ce5ab6"),
            provider.clone(),
        );

        let amount_in = U256::from(1000000000000000000_u128); // 1 ETH
        let amount_out = pool
            .simulate_swap(pool.token_b, amount_in, pool.token_a)
            .unwrap();
        let expected_amount_out = quoter
            .quoteExactInputSingle(
                pool.token_b,
                pool.token_a,
                U24::from(pool.fee),
                amount_in,
                U160::ZERO,
            )
            .block(synced_block.into())
            .call()
            .await
            .unwrap();

        assert_eq!(amount_out, expected_amount_out.amountOut);

        let amount_in_1 = U256::from(10000000000000000000_u128); // 10 ETH
        let amount_out_1 = pool
            .simulate_swap(pool.token_b, amount_in_1, pool.token_a)
            .unwrap();
        let expected_amount_out_1 = quoter
            .quoteExactInputSingle(
                pool.token_b,
                pool.token_a,
                U24::from(pool.fee),
                amount_in_1,
                U160::ZERO,
            )
            .block(synced_block.into())
            .call()
            .await
            .unwrap();

        assert_eq!(amount_out_1, expected_amount_out_1.amountOut);

        let amount_in_2 = U256::from(100000000000000000000_u128); // 100 ETH
        let amount_out_2 = pool
            .simulate_swap(pool.token_b, amount_in_2, pool.token_a)
            .unwrap();
        let expected_amount_out_2 = quoter
            .quoteExactInputSingle(
                pool.token_b,
                pool.token_a,
                U24::from(pool.fee),
                amount_in_2,
                U160::ZERO,
            )
            .block(synced_block.into())
            .call()
            .await
            .unwrap();

        assert_eq!(amount_out_2, expected_amount_out_2.amountOut);
    }

    #[tokio::test]
    #[ignore] // Ignoring to not throttle the Provider on workflows
    async fn test_simulate_swap_mut_usdc_weth() {
        let rpc_endpoint = std::env::var("MAINNET_RPC_URL").unwrap();
        let provider = Arc::new(ProviderBuilder::new().on_http(rpc_endpoint.parse().unwrap()));

        let (pool, synced_block) = initialize_pool(
            provider.clone(),
            address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640"),
        )
        .await
        .unwrap();
        let quoter = IQuoter::new(
            address!("b27308f9f90d607463bb33ea1bebb41c27ce5ab6"),
            provider.clone(),
        );

        let amount_in = U256::from(100000000_u64); // 100 USDC
        let amount_out = pool
            .simulate_swap(pool.token_a, amount_in, pool.token_b)
            .unwrap();
        let expected_amount_out = quoter
            .quoteExactInputSingle(
                pool.token_a,
                pool.token_b,
                U24::from(pool.fee),
                amount_in,
                U160::ZERO,
            )
            .block(synced_block.into())
            .call()
            .await
            .unwrap();

        assert_eq!(amount_out, expected_amount_out.amountOut);

        let amount_in_1 = U256::from(10000000000_u128); // 10_000 USDC
        let amount_out_1 = pool
            .simulate_swap(pool.token_a, amount_in_1, pool.token_b)
            .unwrap();
        let expected_amount_out_1 = quoter
            .quoteExactInputSingle(
                pool.token_a,
                pool.token_b,
                U24::from(pool.fee),
                amount_in_1,
                U160::ZERO,
            )
            .block(synced_block.into())
            .call()
            .await
            .unwrap();

        assert_eq!(amount_out_1, expected_amount_out_1.amountOut);

        let amount_in_2 = U256::from(10000000000000_u128); // 10_000_000 USDC
        let amount_out_2 = pool
            .simulate_swap(pool.token_a, amount_in_2, pool.token_b)
            .unwrap();
        let expected_amount_out_2 = quoter
            .quoteExactInputSingle(
                pool.token_a,
                pool.token_b,
                U24::from(pool.fee),
                amount_in_2,
                U160::ZERO,
            )
            .block(synced_block.into())
            .call()
            .await
            .unwrap();

        assert_eq!(amount_out_2, expected_amount_out_2.amountOut);
    }

    #[tokio::test]
    #[ignore] // Ignoring to not throttle the Provider on workflows
    async fn test_simulate_swap_mut_weth_usdc() {
        let rpc_endpoint = std::env::var("MAINNET_RPC_URL").unwrap();
        let provider = Arc::new(ProviderBuilder::new().on_http(rpc_endpoint.parse().unwrap()));

        let (pool, synced_block) = initialize_pool(
            provider.clone(),
            address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640"),
        )
        .await
        .unwrap();
        let quoter = IQuoter::new(
            address!("b27308f9f90d607463bb33ea1bebb41c27ce5ab6"),
            provider.clone(),
        );

        let amount_in = U256::from(1000000000000000000_u128); // 1 ETH
        let amount_out = pool
            .simulate_swap(pool.token_b, amount_in, pool.token_a)
            .unwrap();
        let expected_amount_out = quoter
            .quoteExactInputSingle(
                pool.token_b,
                pool.token_a,
                U24::from(pool.fee),
                amount_in,
                U160::ZERO,
            )
            .block(synced_block.into())
            .call()
            .await
            .unwrap();

        assert_eq!(amount_out, expected_amount_out.amountOut);

        let amount_in_1 = U256::from(10000000000000000000_u128); // 10 ETH
        let amount_out_1 = pool
            .simulate_swap(pool.token_b, amount_in_1, pool.token_a)
            .unwrap();
        let expected_amount_out_1 = quoter
            .quoteExactInputSingle(
                pool.token_b,
                pool.token_a,
                U24::from(pool.fee),
                amount_in_1,
                U160::ZERO,
            )
            .block(synced_block.into())
            .call()
            .await
            .unwrap();

        assert_eq!(amount_out_1, expected_amount_out_1.amountOut);

        let amount_in_2 = U256::from(100000000000000000000_u128); // 100 ETH
        let amount_out_2 = pool
            .simulate_swap(pool.token_b, amount_in_2, pool.token_a)
            .unwrap();
        let expected_amount_out_2 = quoter
            .quoteExactInputSingle(
                pool.token_b,
                pool.token_a,
                U24::from(pool.fee),
                amount_in_2,
                U160::ZERO,
            )
            .block(synced_block.into())
            .call()
            .await
            .unwrap();

        assert_eq!(amount_out_2, expected_amount_out_2.amountOut);
    }

    #[tokio::test]
    #[ignore] // Ignoring to not throttle the Provider on workflows
    async fn test_simulate_swap_mut_link_weth() {
        let rpc_endpoint = std::env::var("MAINNET_RPC_URL").unwrap();
        let provider = Arc::new(ProviderBuilder::new().on_http(rpc_endpoint.parse().unwrap()));

        let (pool, synced_block) = initialize_pool(
            provider.clone(),
            address!("a6Cc3C2531FdaA6Ae1A3CA84c2855806728693e8"),
        )
        .await
        .unwrap();
        let quoter = IQuoter::new(
            address!("b27308f9f90d607463bb33ea1bebb41c27ce5ab6"),
            provider.clone(),
        );

        let amount_in = U256::from(1000000000000000000_u128); // 1 LINK
        let amount_out = pool
            .simulate_swap(pool.token_a, amount_in, pool.token_b)
            .unwrap();
        let expected_amount_out = quoter
            .quoteExactInputSingle(
                pool.token_a,
                pool.token_b,
                U24::from(pool.fee),
                amount_in,
                U160::ZERO,
            )
            .block(synced_block.into())
            .call()
            .await
            .unwrap();

        assert_eq!(amount_out, expected_amount_out.amountOut);

        let amount_in_1 = U256::from(100000000000000000000_u128); // 100 LINK
        let amount_out_1 = pool
            .simulate_swap(pool.token_a, amount_in_1, pool.token_b)
            .unwrap();
        let expected_amount_out_1 = quoter
            .quoteExactInputSingle(
                pool.token_a,
                pool.token_b,
                U24::from(pool.fee),
                amount_in_1,
                U160::ZERO,
            )
            .block(synced_block.into())
            .call()
            .await
            .unwrap();

        assert_eq!(amount_out_1, expected_amount_out_1.amountOut);

        let amount_in_2 = U256::from(10000000000000000000000_u128); // 10_000 LINK
        let amount_out_2 = pool
            .simulate_swap(pool.token_a, amount_in_2, pool.token_b)
            .unwrap();
        let expected_amount_out_2 = quoter
            .quoteExactInputSingle(
                pool.token_a,
                pool.token_b,
                U24::from(pool.fee),
                amount_in_2,
                U160::ZERO,
            )
            .block(synced_block.into())
            .call()
            .await
            .unwrap();

        assert_eq!(amount_out_2, expected_amount_out_2.amountOut);

        let amount_in_3 = U256::from(10000000000000000000000_u128); // 1_000_000 LINK
        let amount_out_3 = pool
            .simulate_swap(pool.token_a, amount_in_3, pool.token_b)
            .unwrap();
        let expected_amount_out_3 = quoter
            .quoteExactInputSingle(
                pool.token_a,
                pool.token_b,
                U24::from(pool.fee),
                amount_in_3,
                U160::ZERO,
            )
            .block(synced_block.into())
            .call()
            .await
            .unwrap();

        assert_eq!(amount_out_3, expected_amount_out_3.amountOut);
    }

    #[tokio::test]
    #[ignore] // Ignoring to not throttle the Provider on workflows
    async fn test_simulate_swap_mut_weth_link() {
        let rpc_endpoint = std::env::var("MAINNET_RPC_URL").unwrap();
        let provider = Arc::new(ProviderBuilder::new().on_http(rpc_endpoint.parse().unwrap()));

        let (pool, synced_block) = initialize_pool(
            provider.clone(),
            address!("a6Cc3C2531FdaA6Ae1A3CA84c2855806728693e8"),
        )
        .await
        .unwrap();
        let quoter = IQuoter::new(
            address!("b27308f9f90d607463bb33ea1bebb41c27ce5ab6"),
            provider.clone(),
        );

        let amount_in = U256::from(1000000000000000000_u128); // 1 ETH
        let amount_out = pool
            .simulate_swap(pool.token_b, amount_in, pool.token_a)
            .unwrap();
        let expected_amount_out = quoter
            .quoteExactInputSingle(
                pool.token_b,
                pool.token_a,
                U24::from(pool.fee),
                amount_in,
                U160::ZERO,
            )
            .block(synced_block.into())
            .call()
            .await
            .unwrap();

        assert_eq!(amount_out, expected_amount_out.amountOut);

        let amount_in_1 = U256::from(10000000000000000000_u128); // 10 ETH
        let amount_out_1 = pool
            .simulate_swap(pool.token_b, amount_in_1, pool.token_a)
            .unwrap();
        let expected_amount_out_1 = quoter
            .quoteExactInputSingle(
                pool.token_b,
                pool.token_a,
                U24::from(pool.fee),
                amount_in_1,
                U160::ZERO,
            )
            .block(synced_block.into())
            .call()
            .await
            .unwrap();

        assert_eq!(amount_out_1, expected_amount_out_1.amountOut);

        let amount_in_2 = U256::from(100000000000000000000_u128); // 100 ETH
        let amount_out_2 = pool
            .simulate_swap(pool.token_b, amount_in_2, pool.token_a)
            .unwrap();
        let expected_amount_out_2 = quoter
            .quoteExactInputSingle(
                pool.token_b,
                pool.token_a,
                U24::from(pool.fee),
                amount_in_2,
                U160::ZERO,
            )
            .block(synced_block.into())
            .call()
            .await
            .unwrap();

        assert_eq!(amount_out_2, expected_amount_out_2.amountOut);

        let amount_in_3 = U256::from(100000000000000000000_u128); // 100_000 ETH
        let amount_out_3 = pool
            .simulate_swap(pool.token_b, amount_in_3, pool.token_a)
            .unwrap();
        let expected_amount_out_3 = quoter
            .quoteExactInputSingle(
                pool.token_b,
                pool.token_a,
                U24::from(pool.fee),
                amount_in_3,
                U160::ZERO,
            )
            .block(synced_block.into())
            .call()
            .await
            .unwrap();

        assert_eq!(amount_out_3, expected_amount_out_3.amountOut);
    }

    #[tokio::test]
    #[ignore] // Ignoring to not throttle the Provider on workflows
    async fn test_get_new_from_address() {
        let rpc_endpoint = std::env::var("MAINNET_RPC_URL").unwrap();
        let provider = Arc::new(ProviderBuilder::new().on_http(rpc_endpoint.parse().unwrap()));

        let pool = UniswapV3Pool::new_from_address(
            address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640"),
            12369620,
            provider.clone(),
        )
        .await
        .unwrap();

        assert_eq!(
            pool.address,
            address!("88e6a0c2ddd26feeb64f039a2c41296fcb3f5640")
        );
        assert_eq!(
            pool.token_a,
            address!("a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")
        );
        assert_eq!(pool.token_a_decimals, 6);
        assert_eq!(
            pool.token_b,
            address!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2")
        );
        assert_eq!(pool.token_b_decimals, 18);
        assert_eq!(pool.fee, 500);
        assert!(pool.tick != 0);
        assert_eq!(pool.tick_spacing, 10);
    }

    #[tokio::test]
    #[ignore] // Ignoring to not throttle the Provider on workflows
    async fn test_get_pool_data() {
        let rpc_endpoint = std::env::var("MAINNET_RPC_URL").unwrap();
        let provider = Arc::new(ProviderBuilder::new().on_http(rpc_endpoint.parse().unwrap()));

        let (pool, _synced_block) = initialize_pool(
            provider.clone(),
            address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640"),
        )
        .await
        .unwrap();

        assert_eq!(
            pool.address,
            address!("88e6a0c2ddd26feeb64f039a2c41296fcb3f5640")
        );
        assert_eq!(
            pool.token_a,
            address!("a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")
        );
        assert_eq!(pool.token_a_decimals, 6);
        assert_eq!(
            pool.token_b,
            address!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2")
        );
        assert_eq!(pool.token_b_decimals, 18);
        assert_eq!(pool.fee, 500);
        assert!(pool.tick != 0);
        assert_eq!(pool.tick_spacing, 10);
    }

    #[tokio::test]
    async fn test_sync_pool() {
        let rpc_endpoint = std::env::var("MAINNET_RPC_URL").unwrap();
        let provider = Arc::new(ProviderBuilder::new().on_http(rpc_endpoint.parse().unwrap()));

        let mut pool = UniswapV3Pool {
            address: address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640"),
            ..Default::default()
        };

        pool.sync(provider).await.unwrap();

        //TODO: need to assert values
    }

    #[tokio::test]
    async fn test_calculate_virtual_reserves() {
        let rpc_endpoint = std::env::var("MAINNET_RPC_URL").unwrap();
        let provider = Arc::new(ProviderBuilder::new().on_http(rpc_endpoint.parse().unwrap()));

        let mut pool = UniswapV3Pool {
            address: address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640"),
            ..Default::default()
        };

        pool.populate_data(None, provider.clone()).await.unwrap();

        let pool_at_block = IUniswapV3Pool::new(
            address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640"),
            provider.clone(),
        );

        let sqrt_price = pool_at_block
            .slot0()
            .block(16515398.into())
            .call()
            .await
            .unwrap();

        let liquidity = pool_at_block
            .liquidity()
            .block(16515398.into())
            .call()
            .await
            .unwrap();

        pool.sqrt_price = sqrt_price._0.to();
        pool.liquidity = liquidity._0;

        let (r_0, r_1) = pool.calculate_virtual_reserves().unwrap();

        assert_eq!(1067543429906214, r_0);
        assert_eq!(649198362624067343572319, r_1);
    }

    #[tokio::test]
    async fn test_calculate_price() {
        let rpc_endpoint = std::env::var("MAINNET_RPC_URL").unwrap();
        let provider = Arc::new(ProviderBuilder::new().on_http(rpc_endpoint.parse().unwrap()));

        let mut pool = UniswapV3Pool {
            address: address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640"),
            ..Default::default()
        };

        pool.populate_data(None, provider.clone()).await.unwrap();

        let block_pool = IUniswapV3Pool::new(
            address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640"),
            provider.clone(),
        );

        let sqrt_price = block_pool
            .slot0()
            .block(16515398.into())
            .call()
            .await
            .unwrap();

        pool.sqrt_price = sqrt_price._0.to();

        let float_price_a = pool.calculate_price(pool.token_a).unwrap();
        let float_price_b = pool.calculate_price(pool.token_b).unwrap();

        assert_eq!(float_price_a, 0.0006081236083117488);
        assert_eq!(float_price_b, 1644.4025299004006);
    }

    #[tokio::test]
    #[ignore] // Ignoring to not throttle the Provider on workflows
    async fn test_simulate_swap_slipstream_usdc_weth() {
        dotenv::dotenv().ok();
        let rpc_endpoint = std::env::var("BASE_RPC_URL").expect("Missing RPC url");
        let provider = Arc::new(ProviderBuilder::new().on_http(rpc_endpoint.parse().unwrap()));

        let (_pool, _synced_block) = initialize_pool(
            provider.clone(),
            address!("0x70aCDF2Ad0bf2402C957154f944c19Ef4e1cbAE1"),
        )
        .await
        .unwrap();
        // let quoter = IQuoter::new(
        //     address!("b27308f9f90d607463bb33ea1bebb41c27ce5ab6"),
        //     provider.clone(),
        // );

        // let amount_in = U256::from(100000000); // 100 USDC
        // let amount_out = pool
        //     .simulate_swap(pool.token_a, amount_in, pool.token_b)
        //     .unwrap();
        // let expected_amount_out = quoter
        //     .quoteExactInputSingle(
        //         pool.token_a,
        //         pool.token_b,
        //         U24::from(pool.fee),
        //         amount_in,
        //         U160::ZERO,
        //     )
        //     .block(synced_block.into())
        //     .call()
        //     .await
        //     .unwrap();

        // assert_eq!(
        //     amount_out, expected_amount_out.amountOut,
        //     "invalid amount_out: {}, expected_amount_out: {}",
        //     amount_out, expected_amount_out.amountOut
        // );

        // let amount_in_1 = U256::from(10000000000_u64); // 10_000 USDC
        // let amount_out_1 = pool
        //     .simulate_swap(pool.token_a, amount_in_1, pool.token_b)
        //     .unwrap();
        // let expected_amount_out_1 = quoter
        //     .quoteExactInputSingle(
        //         pool.token_a,
        //         pool.token_b,
        //         U24::from(pool.fee),
        //         amount_in_1,
        //         U160::ZERO,
        //     )
        //     .block(synced_block.into())
        //     .call()
        //     .await
        //     .unwrap();
        // assert_eq!(
        //     amount_out_1, expected_amount_out_1.amountOut,
        //     "invalid amount_out_1: {}, expected_amount_out_1: {}",
        //     amount_out_1, expected_amount_out_1.amountOut
        // );

        // let amount_in_2 = U256::from(1000000000000_u128); // 1_000_000 USDC
        // let amount_out_2 = pool
        //     .simulate_swap(pool.token_a, amount_in_2, pool.token_b)
        //     .unwrap();
        // let expected_amount_out_2 = quoter
        //     .quoteExactInputSingle(
        //         pool.token_a,
        //         pool.token_b,
        //         U24::from(pool.fee),
        //         amount_in_2,
        //         U160::ZERO,
        //     )
        //     .block(synced_block.into())
        //     .call()
        //     .await
        //     .unwrap();

        // assert_eq!(
        //     amount_out_2, expected_amount_out_2.amountOut,
        //     "invalid amount_out_2: {}, expected_amount_out_2: {}",
        //     amount_out_2, expected_amount_out_2.amountOut
        // );
    }
}
