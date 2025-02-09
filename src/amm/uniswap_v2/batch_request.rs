use alloy::{
    dyn_abi::{DynSolType, SolType},
    network::Network,
    primitives::{Address, U256},
    providers::Provider,
    sol,
};
use std::sync::Arc;

use crate::{
    amm::{uniswap_v3::batch_request::fixed_bytes_to_string, AutomatedMarketMaker, AMM},
    bindings::{
        self,
        getuniv2pooldata::{GetUniV2PoolData, PoolHelpers::UniswapV2PoolData},
    },
    errors::AMMError,
};

use super::UniswapV2Pool;

pub type SolArray<T> = sol! { T[] };

pub fn populate_v2_pool_data(
    pool: &mut UniswapV2Pool,
    data: UniswapV2PoolData,
) -> Result<(), AMMError> {
    pool.token_a = data.tokenA;
    pool.token_a_decimals = data.tokenADecimals;
    pool.token_a_symbol = fixed_bytes_to_string(&data.tokenASymbol);
    pool.token_b = data.tokenB;
    pool.token_b_decimals = data.tokenBDecimals;
    pool.token_b_symbol = fixed_bytes_to_string(&data.tokenBSymbol);
    pool.reserve_0 = data.reserve0.to::<u128>();
    pool.reserve_1 = data.reserve1.to::<u128>();

    Ok(())
}

pub async fn get_pairs_batch_request<N, P>(
    factory: Address,
    from: U256,
    step: U256,
    provider: Arc<P>,
) -> Result<Vec<Address>, AMMError>
where
    N: Network,
    P: Provider<N>,
{
    let deployer =
        bindings::getuniswapv2pairsbatchrequest::GetUniswapV2PairsBatchRequest::deploy_builder(
            provider, from, step, factory,
        );
    let res = deployer.call_raw().await?;

    let constructor_return = DynSolType::Array(Box::new(DynSolType::Address));
    let return_data_tokens = constructor_return.abi_decode_sequence(&res)?;

    let mut pairs = vec![];
    if let Some(tokens_arr) = return_data_tokens.as_array() {
        for token in tokens_arr {
            if let Some(addr) = token.as_address() {
                if !addr.is_zero() {
                    pairs.push(addr);
                }
            }
        }
    };

    Ok(pairs)
}

pub async fn get_amm_data_batch_request<N, P>(
    amms: &mut [AMM],
    provider: Arc<P>,
) -> Result<(), AMMError>
where
    N: Network,
    P: Provider<N>,
{
    let mut target_addresses = vec![];
    for amm in amms.iter() {
        target_addresses.push(amm.address());
    }

    let return_data = fetch_v2_pool_data_batch_request(&target_addresses, provider).await?;

    let mut pool_idx = 0;
    for pool_data in return_data {
        if !pool_data.tokenA.is_zero() {
            if let AMM::UniswapV2Pool(uniswap_v2_pool) = amms
                .get_mut(pool_idx)
                .expect("Pool idx should be in bounds")
            {
                populate_v2_pool_data(uniswap_v2_pool, pool_data)?;
            }
        }

        pool_idx += 1;
    }

    Ok(())
}

pub async fn fetch_v2_pool_data_batch_request<N, P>(
    addresses: &[Address],
    provider: Arc<P>,
) -> Result<Vec<UniswapV2PoolData>, AMMError>
where
    N: Network,
    P: Provider<N>,
{
    let deployer = GetUniV2PoolData::deploy_builder(provider, addresses.to_vec());
    let res = deployer.call_raw().await?;

    let return_data = SolArray::<UniswapV2PoolData>::abi_decode(&res, false)?;

    Ok(return_data)
}

pub async fn get_v2_pool_data_batch_request<N, P>(
    pools: &mut [UniswapV2Pool],
    provider: Arc<P>,
) -> Result<(), AMMError>
where
    N: Network,
    P: Provider<N>,
{
    let mut target_addresses = vec![];
    for pool in pools.iter() {
        target_addresses.push(pool.address);
    }

    let return_data = fetch_v2_pool_data_batch_request(&target_addresses, provider).await?;

    let mut pool_idx = 0;
    for pool_data in return_data {
        // If the pool token A is not zero, signaling that the pool data was polulated
        if !pool_data.tokenA.is_zero() {
            // Update the pool data
            let uniswap_v2_pool = pools
                .get_mut(pool_idx)
                .expect("Pool idx should be in bounds");

            populate_v2_pool_data(uniswap_v2_pool, pool_data)?;
        }

        pool_idx += 1;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::amm::ExchangeType;
    use alloy_chains::{Chain, NamedChain};
    use provider::get_basic_provider;
    use std::str::FromStr;
    use types::exchange::ExchangeName;

    const POOL_WITH_LARGE_RESERVES: &str = "0x8ff6c0958199a79ac0a619a88ceb6c0c96f16f89";
    const WETH_USDC: &str = "0x3548029694fbb241d45fb24ba0cd9c9d4e745f16";

    #[tokio::test]
    async fn test_get_amm_data_batch_request() {
        // Get RPC URL from environment variable, or use default
        dotenv::dotenv().ok();
        let provider = get_basic_provider(Chain::from_named(NamedChain::Base)).await;

        // Create a test pool
        let pool_address = Address::from_str(WETH_USDC).expect("Invalid address");
        let test_pool = UniswapV2Pool {
            address: pool_address,
            token_a: Address::ZERO,
            token_a_symbol: String::new(),
            token_a_decimals: 0,
            token_b: Address::ZERO,
            token_b_symbol: String::new(),
            token_b_decimals: 0,
            reserve_0: 0,
            reserve_1: 0,
            factory: Address::ZERO,
            fee: 300, // 0.3% fee for UniswapV2
            exchange_name: ExchangeName::Aerodrome,
            exchange_type: ExchangeType::UniV2,
            chain: NamedChain::Base,
        };

        let mut amms = vec![AMM::UniswapV2Pool(test_pool)];

        // Execute the batch request
        get_amm_data_batch_request(&mut amms, provider)
            .await
            .expect("Batch request failed");

        // Verify the results
        if let AMM::UniswapV2Pool(updated_pool) = &amms[0] {
            assert_ne!(updated_pool.token_a, Address::ZERO, "token_a should be set");
            assert_ne!(updated_pool.token_b, Address::ZERO, "token_b should be set");
            assert_ne!(
                updated_pool.token_a_decimals, 0,
                "token_a_decimals should be set"
            );
            assert_ne!(
                updated_pool.token_b_decimals, 0,
                "token_b_decimals should be set"
            );

            // Print pool details for verification
            println!("Pool details:");
            println!("Token A: {:?}", updated_pool.token_a);
            println!("Token A symbol: {}", updated_pool.token_a_symbol);
            println!("Token A decimals: {}", updated_pool.token_a_decimals);
            println!("Token B: {:?}", updated_pool.token_b);
            println!("Token B symbol: {}", updated_pool.token_b_symbol);
            println!("Token B decimals: {}", updated_pool.token_b_decimals);
            println!("Reserve 0: {}", updated_pool.reserve_0);
            println!("Reserve 1: {}", updated_pool.reserve_1);
            println!("Factory: {:?}", updated_pool.factory);
        } else {
            panic!("Expected UniswapV2Pool variant");
        }
    }
}
