pub mod batch_request;
pub mod factory;

use crate::{
    amm::{consts::*, AutomatedMarketMaker, IErc20},
    bindings::iaerodromepool::IAerodromePool,
    errors::{AMMError, ArithmeticError, EventLogError, SwapSimulationError},
};
use alloy::{
    network::Network,
    primitives::{Address, Bytes, B256, U256},
    providers::Provider,
    rpc::types::eth::Log,
    sol,
    sol_types::{SolCall, SolEvent},
    transports::Transport,
};
use alloy_chains::NamedChain;
use async_trait::async_trait;
use db::models::{NewDbPool, NewDbUniV2Pool};
use num_bigfloat::BigFloat;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::instrument;
use types::chain_serde;
use types::exchange::{ExchangeName, ExchangeType};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ve33Pool {
    pub address: Address,
    pub token_a: Address,
    pub token_a_decimals: u8,
    pub token_a_symbol: String,
    pub token_b: Address,
    pub token_b_decimals: u8,
    pub token_b_symbol: String,
    pub reserve_0: u128,
    pub reserve_1: u128,
    pub fee: u32,
    pub stable: bool,
    pub exchange_name: ExchangeName,
    pub exchange_type: ExchangeType,
    #[serde(with = "chain_serde")]
    pub chain: NamedChain,
    pub factory: Address,
}

impl From<NewDbUniV2Pool> for Ve33Pool {
    fn from(pool: NewDbUniV2Pool) -> Self {
        Ve33Pool {
            address: pool.address.parse().unwrap_or(Address::ZERO),
            token_a: pool.token_a.parse().unwrap_or(Address::ZERO),
            token_a_decimals: pool.token_a_decimals as u8,
            token_a_symbol: pool.token_a_symbol.to_string(),
            token_b: pool.token_b.parse().unwrap_or(Address::ZERO),
            token_b_decimals: pool.token_b_decimals as u8,
            token_b_symbol: pool.token_b_symbol.to_string(),
            reserve_0: pool.reserve_0.parse().unwrap_or(0),
            reserve_1: pool.reserve_1.parse().unwrap_or(0),
            fee: pool.fee as u32,
            stable: false,
            factory: pool
                .factory_address
                .unwrap()
                .parse()
                .unwrap_or(Address::ZERO),
            exchange_name: ExchangeName::from_str(&pool.exchange_name.unwrap()).unwrap(),
            exchange_type: ExchangeType::from_str(&pool.exchange_type.unwrap()).unwrap(),
            chain: pool.chain.parse::<NamedChain>().unwrap(),
        }
    }
}

impl From<Ve33Pool> for NewDbUniV2Pool {
    fn from(pool: Ve33Pool) -> Self {
        NewDbUniV2Pool {
            address: pool.address.to_string(),
            chain: pool.chain.as_str().to_string(),
            exchange_name: Some(pool.exchange_name.as_str().to_string()),
            exchange_type: Some(pool.exchange_type.as_str().to_string()),
            token_a: pool.token_a.to_string(),
            token_a_symbol: pool.token_a_symbol.clone(),
            token_a_decimals: pool.token_a_decimals as i32,
            token_b: pool.token_b.to_string(),
            token_b_symbol: pool.token_b_symbol.clone(),
            token_b_decimals: pool.token_b_decimals as i32,
            reserve_0: pool.reserve_0.to_string(),
            reserve_1: pool.reserve_1.to_string(),
            fee: pool.fee as i32,
            factory_address: Some(pool.factory.to_string()),
            active: None,
        }
    }
}

#[async_trait]
impl AutomatedMarketMaker for Ve33Pool {
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
        let (reserve_0, reserve_1) = self.get_reserves(provider.clone()).await?;
        tracing::info!(?reserve_0, ?reserve_1, address = ?self.address, "UniswapV2 sync");

        self.reserve_0 = reserve_0;
        self.reserve_1 = reserve_1;

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
        batch_request::get_pools_batch_request(
            self.factory,
            U256::from(0),
            U256::from(1000),
            provider.clone(),
        )
        .await?;

        Ok(())
    }

    fn sync_on_event_signatures(&self) -> Vec<B256> {
        vec![IAerodromePool::Sync::SIGNATURE_HASH]
    }

    #[instrument(skip(self), level = "debug")]
    fn sync_from_log(&mut self, log: Log) -> Result<(), EventLogError> {
        let event_signature = log.topics()[0];

        if event_signature == IAerodromePool::Sync::SIGNATURE_HASH {
            let sync_event = IAerodromePool::Sync::decode_log(log.as_ref(), true)?;

            let (reserve_0, reserve_1) = (
                sync_event.reserve0.to::<u128>(),
                sync_event.reserve1.to::<u128>(),
            );

            tracing::info!(reserve_0, reserve_1, address = ?self.address, "UniswapV2 sync event");

            self.reserve_0 = reserve_0;
            self.reserve_1 = reserve_1;
            Ok(())
        } else {
            Err(EventLogError::InvalidEventSignature)
        }
    }

    // Calculates base/quote, meaning the price of base token per quote (ie. exchange rate is X base per 1 quote)
    fn calculate_price(&self, base_token: Address) -> Result<f64, ArithmeticError> {
        Ok(q64_to_f64(self.calculate_price_64_x_64(base_token)?))
    }

    fn tokens(&self) -> Vec<Address> {
        vec![self.token_a, self.token_b]
    }

    fn simulate_swap(
        &self,
        token_in: Address,
        amount_in: U256,
        _token_out: Address,
    ) -> Result<U256, SwapSimulationError> {
        if self.token_a == token_in {
            Ok(self.get_amount_out(
                amount_in,
                U256::from(self.reserve_0),
                U256::from(self.reserve_1),
                self.stable,
            ))
        } else {
            Ok(self.get_amount_out(
                amount_in,
                U256::from(self.reserve_1),
                U256::from(self.reserve_0),
                self.stable,
            ))
        }
    }

    fn simulate_swap_mut(
        &mut self,
        token_in: Address,
        amount_in: U256,
        _token_out: Address,
    ) -> Result<U256, SwapSimulationError> {
        if self.token_a == token_in {
            let amount_out = self.get_amount_out(
                amount_in,
                U256::from(self.reserve_0),
                U256::from(self.reserve_1),
                self.stable,
            );

            tracing::trace!(?amount_out);
            tracing::trace!(?self.reserve_0, ?self.reserve_1, "pool reserves before");

            self.reserve_0 += amount_in.to::<u128>();
            self.reserve_1 -= amount_out.to::<u128>();

            tracing::trace!(?self.reserve_0, ?self.reserve_1, "pool reserves after");

            Ok(amount_out)
        } else {
            let amount_out = self.get_amount_out(
                amount_in,
                U256::from(self.reserve_1),
                U256::from(self.reserve_0),
                self.stable,
            );

            tracing::trace!(?amount_out);
            tracing::trace!(?self.reserve_0, ?self.reserve_1, "pool reserves before");

            self.reserve_0 -= amount_out.to::<u128>();
            self.reserve_1 += amount_in.to::<u128>();

            tracing::trace!(?self.reserve_0, ?self.reserve_1, "pool reserves after");

            Ok(amount_out)
        }
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

    fn to_new_db_pool(&self) -> NewDbPool {
        NewDbPool::UniV2(NewDbUniV2Pool {
            address: self.address.to_string(),
            chain: self.chain.as_str().to_string(),
            token_a: self.token_a.to_string(),
            token_a_symbol: self.token_a_symbol.clone(),
            token_a_decimals: self.token_a_decimals as i32,
            token_b: self.token_b.to_string(),
            token_b_symbol: self.token_b_symbol.clone(),
            token_b_decimals: self.token_b_decimals as i32,
            reserve_0: self.reserve_0.to_string(),
            reserve_1: self.reserve_1.to_string(),
            fee: self.fee as i32,
            exchange_name: Some(self.exchange_name.as_str().to_string()),
            exchange_type: Some(self.exchange_type.as_str().to_string()),
            factory_address: Some(self.factory.to_string()),
            active: None,
        })
    }
}

impl Ve33Pool {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        address: Address,
        token_a: Address,
        token_a_decimals: u8,
        token_a_symbol: String,
        token_b: Address,
        token_b_decimals: u8,
        token_b_symbol: String,
        reserve_0: u128,
        reserve_1: u128,
        stable: bool,
        fee: u32,
        factory: Address,
        exchange_name: ExchangeName,
        exchange_type: ExchangeType,
        chain: NamedChain,
    ) -> Ve33Pool {
        Ve33Pool {
            address,
            token_a,
            token_a_decimals,
            token_a_symbol,
            token_b,
            token_b_decimals,
            token_b_symbol,
            reserve_0,
            reserve_1,
            stable,
            fee,
            factory,
            exchange_name,
            exchange_type,
            chain,
        }
    }

    /// Creates a new instance of the pool from the pair address, and syncs the pool data.
    pub async fn new_from_address<T, N, P>(
        pair_address: Address,
        fee: u32,
        provider: Arc<P>,
    ) -> Result<Self, AMMError>
    where
        T: Transport + Clone,
        N: Network,
        P: Provider<T, N>,
    {
        let mut pool = Ve33Pool {
            address: pair_address,
            token_a: Address::ZERO,
            token_a_decimals: 0,
            token_a_symbol: String::new(),
            token_b: Address::ZERO,
            token_b_decimals: 0,
            token_b_symbol: String::new(),
            reserve_0: 0,
            reserve_1: 0,
            stable: false,
            fee,
            factory: Address::ZERO,
            exchange_name: ExchangeName::Unknown,
            exchange_type: ExchangeType::Unknown,
            chain: NamedChain::Mainnet,
        };

        pool.populate_data(None, provider.clone()).await?;

        if !pool.data_is_populated() {
            return Err(AMMError::PoolDataError);
        }

        Ok(pool)
    }

    /// Creates a new instance of a the pool from a `PairCreated` event log.
    ///
    /// This method syncs the pool data.
    pub async fn new_from_log<T, N, P>(
        log: Log,
        fee: u32,
        provider: Arc<P>,
    ) -> Result<Self, AMMError>
    where
        T: Transport + Clone,
        N: Network,
        P: Provider<T, N>,
    {
        let event_signature = log.data().topics()[0];

        if event_signature == factory::IVe33Factory::PairCreated::SIGNATURE_HASH {
            let pair_created_event =
                factory::IVe33Factory::PairCreated::decode_log(log.as_ref(), true)?;
            Ve33Pool::new_from_address(pair_created_event.pair, fee, provider).await
        } else {
            Err(EventLogError::InvalidEventSignature)?
        }
    }

    /// Creates a new instance of a the pool from a `PairCreated` event log.
    ///
    /// This method does not sync the pool data.
    pub fn new_empty_pool_from_log(log: Log) -> Result<Self, EventLogError> {
        let event_signature = log.topics()[0];

        if event_signature == factory::IVe33Factory::PairCreated::SIGNATURE_HASH {
            let pair_created_event =
                factory::IVe33Factory::PairCreated::decode_log(log.as_ref(), true)?;

            Ok(Ve33Pool {
                address: pair_created_event.pair,
                token_a: pair_created_event.token0,
                token_a_symbol: String::new(),
                token_b: pair_created_event.token1,
                token_b_symbol: String::new(),
                token_a_decimals: 0,
                token_b_decimals: 0,
                reserve_0: 0,
                reserve_1: 0,
                stable: false,
                fee: 0,
                factory: Address::ZERO,
                exchange_name: ExchangeName::UniswapV2,
                exchange_type: ExchangeType::UniV2,
                chain: NamedChain::Mainnet,
            })
        } else {
            Err(EventLogError::InvalidEventSignature)?
        }
    }

    /// Returns the swap fee of the pool.
    pub fn fee(&self) -> u32 {
        self.fee
    }

    /// Returns whether the pool data is populated.
    pub fn data_is_populated(&self) -> bool {
        !(self.token_a.is_zero()
            || self.token_b.is_zero()
            || self.reserve_0 == 0
            || self.reserve_1 == 0)
    }

    /// Returns the reserves of the pool.
    pub async fn get_reserves<T, N, P>(&self, provider: Arc<P>) -> Result<(u128, u128), AMMError>
    where
        T: Transport + Clone,
        N: Network,
        P: Provider<T, N>,
    {
        tracing::trace!("getting reserves of {}", self.address);

        // Initialize a new instance of the Pool
        let pool = IAerodromePool::new(self.address, provider);

        // Make a call to get the reserves
        let IAerodromePool::getReservesReturn {
            _reserve0: reserve_0,
            _reserve1: reserve_1,
            ..
        } = match pool.getReserves().call().await {
            Ok(result) => result,
            Err(contract_error) => return Err(AMMError::ContractError(contract_error)),
        };

        let (reserve_0, reserve_1) = (reserve_0.to::<u128>(), reserve_1.to::<u128>());

        tracing::trace!(reserve_0, reserve_1);

        Ok((reserve_0, reserve_1))
    }

    pub async fn get_token_decimals<T, N, P>(
        &mut self,
        provider: Arc<P>,
    ) -> Result<(u8, u8), AMMError>
    where
        T: Transport + Clone,
        N: Network,
        P: Provider<T, N>,
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

        tracing::trace!(token_a_decimals, token_b_decimals);

        Ok((token_a_decimals, token_b_decimals))
    }

    pub async fn get_token_0<T, N, P>(
        &self,
        pair_address: Address,
        provider: Arc<P>,
    ) -> Result<Address, AMMError>
    where
        T: Transport + Clone,
        N: Network,
        P: Provider<T, N>,
    {
        let pool = IAerodromePool::new(pair_address, provider);

        let IAerodromePool::token0Return { _0: token0 } = match pool.token0().call().await {
            Ok(result) => result,
            Err(contract_error) => return Err(AMMError::ContractError(contract_error)),
        };

        Ok(token0)
    }

    pub async fn get_token_1<T, N, P>(
        &self,
        pair_address: Address,
        middleware: Arc<P>,
    ) -> Result<Address, AMMError>
    where
        T: Transport + Clone,
        N: Network,
        P: Provider<T, N>,
    {
        let pool = IAerodromePool::new(pair_address, middleware);

        let IAerodromePool::token1Return { _0: token1 } = match pool.token1().call().await {
            Ok(result) => result,
            Err(contract_error) => return Err(AMMError::ContractError(contract_error)),
        };

        Ok(token1)
    }

    /// Calculates the price of the base token in terms of the quote token.
    ///
    /// Returned as a Q64 fixed point number.
    pub fn calculate_price_64_x_64(&self, base_token: Address) -> Result<u128, ArithmeticError> {
        let decimal_shift = self.token_a_decimals as i8 - self.token_b_decimals as i8;

        let (r_0, r_1) = if decimal_shift < 0 {
            (
                U256::from(self.reserve_0)
                    * U256::from(10u128.pow(decimal_shift.unsigned_abs() as u32)),
                U256::from(self.reserve_1),
            )
        } else {
            (
                U256::from(self.reserve_0),
                U256::from(self.reserve_1) * U256::from(10u128.pow(decimal_shift as u32)),
            )
        };

        if base_token == self.token_a {
            if r_0.is_zero() {
                Ok(U128_0X10000000000000000)
            } else {
                div_uu(r_1, r_0)
            }
        } else if r_1.is_zero() {
            Ok(U128_0X10000000000000000)
        } else {
            div_uu(r_0, r_1)
        }
    }

    pub fn get_amount_out(
        &self,
        amount_in: U256,
        reserve_in: U256,
        reserve_out: U256,
        stable: bool,
    ) -> U256 {
        tracing::trace!(?amount_in, ?reserve_in, ?reserve_out, ?stable);

        if amount_in.is_zero() || reserve_in.is_zero() || reserve_out.is_zero() {
            return U256::ZERO;
        }

        // Aerodrome fees:
        // STABLE_FEE = 10 (0.01%)
        // VOLATILE_FEE = 300 (0.3%)
        // FEE_DENOMINATOR = 100000
        let fee = if stable { 10 } else { 300 };

        // Apply fee: amount_in * (FEE_DENOMINATOR - fee) / FEE_DENOMINATOR
        let amount_in_with_fee = amount_in * U256::from(100000 - fee) / U256::from(100000);

        if stable {
            self.get_amount_out_stable(amount_in_with_fee, reserve_in, reserve_out)
                .expect("Failed to get amount out")
        } else {
            self.get_amount_out_volatile(amount_in_with_fee, reserve_in, reserve_out)
                .expect("Failed to get amount out")
        }
    }

    fn get_amount_out_volatile(
        &self,
        amount_in: U256,
        reserve_in: U256,
        reserve_out: U256,
    ) -> Result<U256, AMMError> {
        let numerator = amount_in * reserve_out;
        let denominator = reserve_in + amount_in;
        Ok(numerator / denominator)
    }

    fn get_amount_out_stable(
        &self,
        amount_in: U256,
        token_in: Address,
        reserve0: U256,
        reserve1: U256,
    ) -> Result<U256, AMMError> {
        if self.stable {
            let xy = self._k(reserve0, reserve1)?;

            // Scale reserves to 18 decimals
            let reserve0 = (reserve0 * U256::from(10).pow(U256::from(18)))
                / U256::from(10).pow(U256::from(self.token_a_decimals));
            let reserve1 = (reserve1 * U256::from(10).pow(U256::from(18)))
                / U256::from(10).pow(U256::from(self.token_b_decimals));

            let (reserve_a, reserve_b) = if token_in == self.token_a {
                (reserve0, reserve1)
            } else {
                (reserve1, reserve0)
            };

            // Scale amount_in to 18 decimals
            let amount_in = if token_in == self.token_a {
                (amount_in * U256::from(10).pow(U256::from(18)))
                    / U256::from(10).pow(U256::from(self.token_a_decimals))
            } else {
                (amount_in * U256::from(10).pow(U256::from(18)))
                    / U256::from(10).pow(U256::from(self.token_b_decimals))
            };

            let y = reserve_b - self._get_y(amount_in + reserve_a, xy, reserve_b)?;

            // Scale back to token decimals
            Ok((y * if token_in == self.token_a {
                U256::from(10).pow(U256::from(self.token_b_decimals))
            } else {
                U256::from(10).pow(U256::from(self.token_a_decimals))
            }) / U256::from(10).pow(U256::from(18)))
        } else {
            let (reserve_a, reserve_b) = if token_in == self.token_a {
                (reserve0, reserve1)
            } else {
                (reserve1, reserve0)
            };

            Ok((amount_in * reserve_b) / (reserve_a + amount_in))
        }
    }

    fn _k(&self, x: U256, y: U256) -> Result<U256, AMMError> {
        if self.stable {
            let x = (x * U256::from(10).pow(U256::from(18)))
                / U256::from(10).pow(U256::from(self.token_a_decimals));
            let y = (y * U256::from(10).pow(U256::from(18)))
                / U256::from(10).pow(U256::from(self.token_b_decimals));

            let a = (x * y) / U256::from(10).pow(U256::from(18));
            let b = ((x * x) / U256::from(10).pow(U256::from(18))
                + (y * y) / U256::from(10).pow(U256::from(18)));

            Ok((a * b) / U256::from(10).pow(U256::from(18))) // x3y+y3x >= k
        } else {
            Ok(x * y) // xy >= k
        }
    }

    fn _f(&self, x0: U256, y: U256) -> U256 {
        let a = (x0 * y) / U256::from(10).pow(U256::from(18));
        let b = ((x0 * x0) / U256::from(10).pow(U256::from(18))
            + (y * y) / U256::from(10).pow(U256::from(18)));

        (a * b) / U256::from(10).pow(U256::from(18))
    }

    fn _d(&self, x0: U256, y: U256) -> U256 {
        let term1 = (U256::from(3) * x0 * ((y * y) / U256::from(10).pow(U256::from(18))))
            / U256::from(10).pow(U256::from(18));
        let term2 = ((x0 * x0) / U256::from(10).pow(U256::from(18)) * x0)
            / U256::from(10).pow(U256::from(18));

        term1 + term2
    }

    fn _get_y(&self, x0: U256, xy: U256, y: U256) -> Result<U256, AMMError> {
        let mut y = y;

        for _ in 0..255 {
            let k = self._f(x0, y);

            if k < xy {
                let dy = ((xy - k) * U256::from(10).pow(U256::from(18))) / self._d(x0, y);

                if dy.is_zero() {
                    if k == xy {
                        return Ok(y);
                    }
                    if self._k(x0, y + U256::from(1))? > xy {
                        return Ok(y + U256::from(1));
                    }
                    y += U256::from(1);
                } else {
                    y += dy;
                }
            } else {
                let dy = ((k - xy) * U256::from(10).pow(U256::from(18))) / self._d(x0, y);

                if dy.is_zero() {
                    if k == xy || self._f(x0, y - U256::from(1)) < xy {
                        return Ok(y);
                    }
                    y -= U256::from(1);
                } else {
                    y -= dy;
                }
            }
        }

        Err(AMMError::FailedToConvergeToY)
    }

    /// Returns the calldata for a swap.
    pub fn swap_calldata(
        &self,
        amount_0_out: U256,
        amount_1_out: U256,
        to: Address,
        calldata: Vec<u8>,
    ) -> Result<Bytes, alloy::dyn_abi::Error> {
        Ok(IAerodromePool::swapCall {
            amount0Out: amount_0_out,
            amount1Out: amount_1_out,
            to,
            data: calldata.into(),
        }
        .abi_encode()
        .into())
    }
}

pub fn div_uu(x: U256, y: U256) -> Result<u128, ArithmeticError> {
    if !y.is_zero() {
        let mut answer;

        if x <= U256_0XFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF {
            answer = (x << U256_64) / y;
        } else {
            let mut msb = U256_192;
            let mut xc = x >> U256_192;

            if xc >= U256_0X100000000 {
                xc >>= U256_32;
                msb += U256_32;
            }

            if xc >= U256_0X10000 {
                xc >>= U256_16;
                msb += U256_16;
            }

            if xc >= U256_0X100 {
                xc >>= U256_8;
                msb += U256_8;
            }

            if xc >= U256_16 {
                xc >>= U256_4;
                msb += U256_4;
            }

            if xc >= U256_4 {
                xc >>= U256_2;
                msb += U256_2;
            }

            if xc >= U256_2 {
                msb += U256_1;
            }

            answer = (x << (U256_255 - msb)) / (((y - U256_1) >> (msb - U256_191)) + U256_1);
        }

        if answer > U256_0XFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF {
            return Ok(0);
        }

        let hi = answer * (y >> U256_128);
        let mut lo = answer * (y & U256_0XFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF);

        let mut xh = x >> U256_192;
        let mut xl = x << U256_64;

        if xl < lo {
            xh -= U256_1;
        }

        xl = xl.overflowing_sub(lo).0;
        lo = hi << U256_128;

        if xl < lo {
            xh -= U256_1;
        }

        xl = xl.overflowing_sub(lo).0;

        if xh != hi >> U256_128 {
            return Err(ArithmeticError::RoundingError);
        }

        answer += xl / y;

        if answer > U256_0XFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF {
            return Ok(0_u128);
        }

        Ok(answer.to::<u128>())
    } else {
        Err(ArithmeticError::YIsZero)
    }
}

/// Converts a Q64 fixed point to a Q16 fixed point -> f64
pub fn q64_to_f64(x: u128) -> f64 {
    BigFloat::from(x)
        .div(&BigFloat::from(U128_0X10000000000000000))
        .to_f64()
}

// TODO: Rewrite tests
