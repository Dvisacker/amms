use alloy::{
    dyn_abi::DynSolType,
    network::Network,
    primitives::{Address, U256},
    providers::Provider,
};
use std::sync::Arc;

use crate::{
    bindings::{
        getve33pooldata::GetVe33PoolData, getve33pooldatabatchrequest::GetVe33PoolDataBatchRequest,
        getve33poolsbatchrequest::GetVe33PoolsBatchRequest,
    },
    errors::AMMError,
};

use alloy::dyn_abi::DynSolValue;

use crate::amm::{AutomatedMarketMaker, AMM};

use super::Ve33Pool;

#[inline]
// This is the older version of the function that is to be deprecated
fn populate_pool_data_from_tokens(mut pool: Ve33Pool, tokens: &[DynSolValue]) -> Option<Ve33Pool> {
    pool.token_a = tokens[0].as_address()?;
    pool.token_a_decimals = tokens[1].as_uint()?.0.to::<u8>();
    pool.token_b = tokens[2].as_address()?;
    pool.token_b_decimals = tokens[3].as_uint()?.0.to::<u8>();
    pool.reserve_0 = tokens[4].as_uint()?.0.to::<u128>();
    pool.reserve_1 = tokens[5].as_uint()?.0.to::<u128>();
    pool.stable = tokens[6].as_bool()?;

    Some(pool)
}

fn bytes32_to_string(bytes: &[u8]) -> String {
    let mut result = String::from_utf8_lossy(bytes).into_owned();
    result.truncate(result.trim_end_matches('\0').len());
    result
}

pub fn populate_ve33_pool_data(
    pool: &mut Ve33Pool,
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

    pool.stable = tokens[9].as_bool().unwrap_or(false);

    if pool.stable {
        pool.fee = 5; // 0.05%
    } else {
        pool.fee = 30; // 0.3%
    }

    Ok(())
}

pub async fn get_pools_batch_request<N, P>(
    factory: Address,
    from: U256,
    step: U256,
    provider: Arc<P>,
) -> Result<Vec<Address>, AMMError>
where
    N: Network,
    P: Provider<N>,
{
    let deployer = GetVe33PoolsBatchRequest::deploy_builder(provider, from, step, factory);
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

    let deployer = GetVe33PoolDataBatchRequest::deploy_builder(provider, target_addresses);
    let res = deployer.call().await?;

    let constructor_return = DynSolType::Array(Box::new(DynSolType::Tuple(vec![
        DynSolType::Address,
        DynSolType::Uint(8),
        DynSolType::Address,
        DynSolType::Uint(8),
        DynSolType::Uint(112),
        DynSolType::Uint(112),
        DynSolType::Bool,
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
                        if let AMM::Ve33Pool(ve33_pool) = amms
                            .get_mut(pool_idx)
                            .expect("Pool idx should be in bounds")
                        {
                            if let Some(pool) =
                                populate_pool_data_from_tokens(ve33_pool.to_owned(), pool_data)
                            {
                                tracing::trace!(?pool);
                                *ve33_pool = pool;
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

pub async fn fetch_ve33_pool_data_batch_request<N, P>(
    addresses: &[Address],
    provider: Arc<P>,
) -> Result<DynSolValue, AMMError>
where
    N: Network,
    P: Provider<N>,
{
    let deployer = GetVe33PoolData::deploy_builder(provider, addresses.to_vec());
    let res = deployer.call_raw().await?;

    let constructor_return = DynSolType::Array(Box::new(DynSolType::Tuple(vec![
        DynSolType::Address,        //token a
        DynSolType::FixedBytes(32), //token a symbol
        DynSolType::Uint(8),        //token a decimals
        DynSolType::Address,        //token b
        DynSolType::FixedBytes(32), //token b symbol
        DynSolType::Uint(8),        //token b decimals
        DynSolType::Uint(256),      //reserve 0
        DynSolType::Uint(256),      //reserve 1
        DynSolType::Address,        //factory address
        DynSolType::Bool,           //stable
    ])));

    let return_data = constructor_return.abi_decode_sequence(&res)?;

    Ok(return_data)
}

pub async fn get_ve33_pool_data_batch_request<N, P>(
    pools: &mut [Ve33Pool],
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

    let return_data = fetch_ve33_pool_data_batch_request(&target_addresses, provider).await?;

    let mut pool_idx = 0;
    if let Some(tokens_arr) = return_data.as_array() {
        for token in tokens_arr {
            if let Some(pool_data) = token.as_tuple() {
                // If the pool token A is not zero, signaling that the pool data was polulated
                if let Some(address) = pool_data[0].as_address() {
                    if !address.is_zero() {
                        // Update the pool data
                        let ve33_pool = pools
                            .get_mut(pool_idx)
                            .expect("Pool idx should be in bounds");

                        populate_ve33_pool_data(ve33_pool, pool_data)?;
                    }
                }

                pool_idx += 1;
            }
        }
    }

    Ok(())
}
