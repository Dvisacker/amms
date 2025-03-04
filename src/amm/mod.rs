pub mod camelot_v3;
pub mod consts;
pub mod curve;
pub mod erc_4626;
pub mod factory;
// pub mod slipstream;
pub mod uniswap_v2;
pub mod uniswap_v3;
pub mod ve33;

use std::{
    hash::{Hash, Hasher},
    sync::Arc,
};

use alloy::{
    network::Network,
    primitives::{Address, B256, U256},
    providers::Provider,
    rpc::types::eth::Log,
    sol,
};
use alloy_chains::NamedChain;
use async_trait::async_trait;
use db::models::NewDbPool;
use serde::{Deserialize, Serialize};
use types::exchange::{ExchangeName, ExchangeType};

use crate::errors::{AMMError, ArithmeticError, EventLogError, SwapSimulationError};

use self::{
    camelot_v3::CamelotV3Pool, curve::CurvePool, erc_4626::ERC4626Vault, uniswap_v2::UniswapV2Pool,
    uniswap_v3::UniswapV3Pool, ve33::Ve33Pool,
};

sol! {
    /// Interface of the ERC20
    #[derive(Debug, PartialEq, Eq)]
    #[sol(rpc)]
    contract IErc20 {
        function balanceOf(address account) external view returns (uint256);
        function decimals() external view returns (uint8);
        function symbol() external view returns (string memory);
    }
}

#[async_trait]
pub trait AutomatedMarketMaker {
    /// Returns the address of the AMM.
    fn address(&self) -> Address;

    /// Syncs the AMM data on chain via batched static calls.
    async fn sync<N, P>(&mut self, provider: Arc<P>) -> Result<(), AMMError>
    where
        N: Network,
        P: Provider<N>;

    /// Returns the vector of event signatures subscribed to when syncing the AMM.
    fn sync_on_event_signatures(&self) -> Vec<B256>;

    /// Returns a vector of tokens in the AMM.
    fn tokens(&self) -> Vec<Address>;

    fn token_symbols(&self) -> Vec<String>;

    /// Calculates a f64 representation of base token price in the AMM.
    fn calculate_price(&self, base_token: Address) -> Result<f64, ArithmeticError>;

    /// Updates the AMM data from a log.
    fn sync_from_log(&mut self, log: Log) -> Result<(), EventLogError>;

    /// Populates the AMM data via batched static calls.
    async fn populate_data<N, P>(
        &mut self,
        block_number: Option<u64>,
        middleware: Arc<P>,
    ) -> Result<(), AMMError>
    where
        N: Network,
        P: Provider<N>;

    /// Locally simulates a swap in the AMM.
    ///
    /// Returns the amount received for `amount_in` of `token_in`.
    fn simulate_swap(
        &self,
        token_in: Address,
        amount_in: U256,
        token_out: Address,
    ) -> Result<U256, SwapSimulationError>;

    /// Locally simulates a swap in the AMM.
    /// Mutates the AMM state to the state of the AMM after swapping.
    /// Returns the amount received for `amount_in` of `token_in`.
    fn simulate_swap_mut(
        &mut self,
        token_in: Address,
        amount_in: U256,
        token_out: Address,
    ) -> Result<U256, SwapSimulationError>;

    /// Returns the token out of the AMM for a given `token_in`.
    // fn get_token_out(&self, token_in: Address) -> Address;

    fn to_new_db_pool(&self, tag: Option<String>) -> NewDbPool;

    fn exchange_name(&self) -> ExchangeName;
    fn exchange_type(&self) -> ExchangeType;
    fn chain(&self) -> NamedChain;
    fn name(&self) -> String;
}

macro_rules! amm {
    ($($pool_type:ident),+ $(,)?) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum AMM {
            $($pool_type($pool_type),)+
        }

        #[async_trait]
        impl AutomatedMarketMaker for AMM {
            fn address(&self) -> Address{
                match self {
                    $(AMM::$pool_type(pool) => pool.address(),)+
                }
            }

            async fn sync<N, P>(&mut self, middleware: Arc<P>) -> Result<(), AMMError>
            where
                N: Network,
                P: Provider<N>,
            {
                match self {
                    $(AMM::$pool_type(pool) => pool.sync(middleware).await,)+
                }
            }

            fn sync_on_event_signatures(&self) -> Vec<B256> {
                match self {
                    $(AMM::$pool_type(pool) => pool.sync_on_event_signatures(),)+
                }
            }

            fn sync_from_log(&mut self, log: Log) -> Result<(), EventLogError> {
                match self {
                    $(AMM::$pool_type(pool) => pool.sync_from_log(log),)+
                }
            }

            fn simulate_swap(&self, token_in: Address, amount_in: U256, token_out: Address) -> Result<U256, SwapSimulationError> {
                match self {
                    $(AMM::$pool_type(pool) => pool.simulate_swap(token_in, amount_in, token_out),)+
                }
            }

            fn simulate_swap_mut(&mut self, token_in: Address, amount_in: U256, token_out: Address) -> Result<U256, SwapSimulationError> {
                match self {
                    $(AMM::$pool_type(pool) => pool.simulate_swap_mut(token_in, amount_in, token_out),)+
                }
            }

            fn to_new_db_pool(&self, tag: Option<String>) -> NewDbPool {
                match self {
                    $(AMM::$pool_type(pool) => pool.to_new_db_pool(tag),)+
                }
            }

            // fn get_token_out(&self, token_in: Address) -> Address {
            //     match self {
            //         $(AMM::$pool_type(pool) => pool.get_token_out(token_in),)+
            //     }
            // }

            async fn populate_data<N, P>(&mut self, block_number: Option<u64>, middleware: Arc<P>) -> Result<(), AMMError>
            where
                N: Network,
                P: Provider<N>,
            {
                match self {
                    $(AMM::$pool_type(pool) => pool.populate_data(block_number, middleware).await,)+
                }
            }

            fn tokens(&self) -> Vec<Address> {
                match self {
                    $(AMM::$pool_type(pool) => pool.tokens(),)+
                }
            }

            fn calculate_price(&self, base_token: Address) -> Result<f64, ArithmeticError> {
                match self {
                    $(AMM::$pool_type(pool) => pool.calculate_price(base_token),)+
                }
            }

            fn token_symbols(&self) -> Vec<String> {
                match self {
                    $(AMM::$pool_type(pool) => pool.token_symbols(),)+
                }
            }

            fn exchange_name(&self) -> ExchangeName {
                match self {
                    $(AMM::$pool_type(pool) => pool.exchange_name(),)+
                }
            }

            fn exchange_type(&self) -> ExchangeType {
                match self {
                    $(AMM::$pool_type(pool) => pool.exchange_type(),)+
                }
            }

            fn chain(&self) -> NamedChain {
                match self {
                    $(AMM::$pool_type(pool) => pool.chain(),)+
                }
            }

            fn name(&self) -> String {
                match self {
                    $(AMM::$pool_type(pool) => {
                        let symbols = pool.token_symbols();
                        let exchange_name = pool.exchange_name();
                        format!("{}:{}-{}", exchange_name, symbols[0], symbols[1])
                    },)+
                }
            }
        }

        impl Hash for AMM {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.address().hash(state);
            }
        }

        impl PartialEq for AMM {
            fn eq(&self, other: &Self) -> bool {
                self.address() == other.address()
            }
        }

        impl Eq for AMM {}
    };
}

amm!(
    UniswapV2Pool,
    Ve33Pool,
    UniswapV3Pool,
    ERC4626Vault,
    CamelotV3Pool,
    CurvePool
);
