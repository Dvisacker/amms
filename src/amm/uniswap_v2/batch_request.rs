use alloy::{
    dyn_abi::{DynSolType, DynSolValue},
    network::Network,
    primitives::{address, Address, U256},
    providers::Provider,
    sol,
    transports::Transport,
};
use std::sync::Arc;

use crate::{
    amm::{AutomatedMarketMaker, AMM},
    bindings::{
        self, getuniswapv2pooldatabatchrequest::GetUniswapV2PoolDataBatchRequest,
        getuniv2pooldata::GetUniV2PoolData,
    },
    errors::AMMError,
};

use super::UniswapV2Pool;

#[inline]
// This is the older version of the function that is to be deprecated
fn populate_pool_data_from_tokens(
    mut pool: UniswapV2Pool,
    tokens: &[DynSolValue],
) -> Option<UniswapV2Pool> {
    pool.token_a = tokens[0].as_address()?;
    pool.token_a_decimals = tokens[1].as_uint()?.0.to::<u8>();
    pool.token_b = tokens[2].as_address()?;
    pool.token_b_decimals = tokens[3].as_uint()?.0.to::<u8>();
    pool.reserve_0 = tokens[4].as_uint()?.0.to::<u128>();
    pool.reserve_1 = tokens[5].as_uint()?.0.to::<u128>();

    Some(pool)
}

fn bytes32_to_string(bytes: &[u8]) -> String {
    let mut result = String::from_utf8_lossy(bytes).into_owned();
    result.truncate(result.trim_end_matches('\0').len());
    result
}

pub fn populate_v2_pool_data(
    pool: &mut UniswapV2Pool,
    tokens: &[DynSolValue],
) -> Result<(), AMMError> {
    pool.token_a = tokens[0]
        .as_address()
        .ok_or(AMMError::BatchRequestError(pool.address))?;
    pool.token_a_symbol = bytes32_to_string(
        tokens[1]
            .as_fixed_bytes()
            .ok_or(AMMError::BatchRequestError(pool.address))?
            .0,
    );
    pool.token_a_decimals = tokens[2]
        .as_uint()
        .ok_or(AMMError::BatchRequestError(pool.address))?
        .0
        .to::<u8>();
    pool.token_b = tokens[3]
        .as_address()
        .ok_or(AMMError::BatchRequestError(pool.address))?;
    pool.token_b_symbol = bytes32_to_string(
        tokens[4]
            .as_fixed_bytes()
            .ok_or(AMMError::BatchRequestError(pool.address))?
            .0,
    );
    pool.token_b_decimals = tokens[5]
        .as_uint()
        .ok_or(AMMError::BatchRequestError(pool.address))?
        .0
        .to::<u8>();
    pool.reserve_0 = tokens[6]
        .as_uint()
        .ok_or(AMMError::BatchRequestError(pool.address))?
        .0
        .to::<u128>();
    pool.reserve_1 = tokens[7]
        .as_uint()
        .ok_or(AMMError::BatchRequestError(pool.address))?
        .0
        .to::<u128>();
    pool.factory = tokens[8]
        .as_address()
        .ok_or(AMMError::BatchRequestError(pool.address))?;

    Ok(())
}

pub async fn get_pairs_batch_request<T, N, P>(
    factory: Address,
    from: U256,
    step: U256,
    provider: Arc<P>,
) -> Result<Vec<Address>, AMMError>
where
    T: Transport + Clone,
    N: Network,
    P: Provider<T, N>,
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

pub async fn get_amm_data_batch_request<T, N, P>(
    amms: &mut [AMM],
    provider: Arc<P>,
) -> Result<(), AMMError>
where
    T: Transport + Clone,
    N: Network,
    P: Provider<T, N>,
{
    let mut target_addresses = vec![];
    for amm in amms.iter() {
        target_addresses.push(amm.address());
    }

    println!("All target addresses: {:?}", target_addresses);

    let deployer = GetUniswapV2PoolDataBatchRequest::deploy_builder(provider, target_addresses);
    let res = deployer.call().await?;

    let constructor_return = DynSolType::Array(Box::new(DynSolType::Tuple(vec![
        DynSolType::Address,
        DynSolType::Uint(8),
        DynSolType::Address,
        DynSolType::Uint(8),
        DynSolType::Uint(112),
        DynSolType::Uint(112),
    ])));
    let return_data_tokens = constructor_return.abi_decode_sequence(&res)?;

    let mut pool_idx = 0;
    if let Some(tokens_arr) = return_data_tokens.as_array() {
        for token in tokens_arr {
            if let Some(pool_data) = token.as_tuple() {
                // If the pool token A is not zero, signaling that the pool data was polulated
                if let Some(address) = pool_data[0].as_address() {
                    if !address.is_zero() {
                        // Update the pool data
                        if let AMM::UniswapV2Pool(uniswap_v2_pool) = amms
                            .get_mut(pool_idx)
                            .expect("Pool idx should be in bounds")
                        {
                            if let Some(pool) = populate_pool_data_from_tokens(
                                uniswap_v2_pool.to_owned(),
                                pool_data,
                            ) {
                                tracing::trace!(?pool);
                                *uniswap_v2_pool = pool;
                            }
                        }
                    }
                }

                pool_idx += 1;
            }
        }
    }

    Ok(())
}

pub async fn fetch_v2_pool_data_batch_request<T, N, P>(
    addresses: &[Address],
    provider: Arc<P>,
) -> Result<DynSolValue, AMMError>
where
    T: Transport + Clone,
    N: Network,
    P: Provider<T, N>,
{
    let deployer = GetUniV2PoolData::deploy_builder(provider, addresses.to_vec());
    let res = deployer.call_raw().await?;

    let constructor_return = DynSolType::Array(Box::new(DynSolType::Tuple(vec![
        DynSolType::Address,        //token a
        DynSolType::FixedBytes(32), //token a symbol
        DynSolType::Uint(8),        //token a decimals
        DynSolType::Address,        //token b
        DynSolType::FixedBytes(32), //token b symbol
        DynSolType::Uint(8),        //token b decimals
        DynSolType::Uint(112),      //reserve 0
        DynSolType::Uint(112),      //reserve 1
        DynSolType::Address,        //factory address
    ])));

    let return_data = constructor_return.abi_decode_sequence(&res)?;

    Ok(return_data)
}

pub async fn get_v2_pool_data_batch_request<T, N, P>(
    pools: &mut [UniswapV2Pool],
    provider: Arc<P>,
) -> Result<(), AMMError>
where
    T: Transport + Clone,
    N: Network,
    P: Provider<T, N>,
{
    let mut target_addresses = vec![];
    for pool in pools.iter() {
        target_addresses.push(pool.address);
    }

    let return_data = fetch_v2_pool_data_batch_request(&target_addresses, provider).await?;

    let mut pool_idx = 0;
    if let Some(tokens_arr) = return_data.as_array() {
        for token in tokens_arr {
            if let Some(pool_data) = token.as_tuple() {
                // If the pool token A is not zero, signaling that the pool data was polulated
                if let Some(address) = pool_data[0].as_address() {
                    if !address.is_zero() {
                        // Update the pool data
                        let uniswap_v2_pool = pools
                            .get_mut(pool_idx)
                            .expect("Pool idx should be in bounds");

                        populate_v2_pool_data(uniswap_v2_pool, pool_data)?;
                        // {
                        //     tracing::trace!(?pool);
                        //     *uniswap_v2_pool = pool;
                        // }
                    }
                }

                pool_idx += 1;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::amm::ExchangeType;
    use alloy::providers::ProviderBuilder;
    use alloy_chains::{Chain, NamedChain};
    use config::get_chain_config;
    use std::{env, str::FromStr};
    use types::exchange::ExchangeName;

    const VE33_POOL_ADDRESS: &str = "0x8ff6c0958199a79ac0a619a88ceb6c0c96f16f89";
    const AEROUSDC_ADDRESS: &str = "0x6cdcb1c4a4d1c3c6d054b27ac5b77e89eafb971d";

    #[tokio::test]
    async fn test_get_amm_data_batch_request() {
        // Get RPC URL from environment variable, or use default
        dotenv::dotenv().ok();
        let config = get_chain_config(Chain::from_named(NamedChain::Base)).await;
        let provider = config.ws;

        // Create a test pool
        let pool_address = Address::from_str(VE33_POOL_ADDRESS).expect("Invalid address");
        let mut test_pool = UniswapV2Pool {
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

        println!("{:?}", amms);

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
