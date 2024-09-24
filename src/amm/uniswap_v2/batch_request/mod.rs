use alloy::{
    dyn_abi::{DynSolType, DynSolValue},
    network::Network,
    primitives::{Address, U256},
    providers::Provider,
    sol,
    transports::Transport,
};
use std::sync::Arc;

use crate::{
    amm::{AutomatedMarketMaker, AMM},
    errors::AMMError,
};

use super::UniswapV2Pool;

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    IGetUniswapV2PairsBatchRequest,
    "src/amm/uniswap_v2/batch_request/GetUniswapV2PairsBatchRequestABI.json"
}

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    IGetUniswapV2PoolDataBatchRequest,
    "src/amm/uniswap_v2/batch_request/GetUniswapV2PoolDataBatchRequestABI.json"
}

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    IGetUniV2PoolData,
    "src/amm/uniswap_v2/batch_request/GetUniV2PoolData.json"
}

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
    let deployer = IGetUniswapV2PairsBatchRequest::deploy_builder(provider, from, step, factory);
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

    let deployer = IGetUniswapV2PoolDataBatchRequest::deploy_builder(provider, target_addresses);
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
    let deployer = IGetUniV2PoolData::deploy_builder(provider, addresses.to_vec());
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
