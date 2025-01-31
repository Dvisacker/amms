use std::{cmp::min, sync::Arc, vec};

use alloy::{
    dyn_abi::{DynSolType, DynSolValue},
    network::Network,
    primitives::{aliases::I24, Address, U256},
    providers::Provider,
    sol,
    sol_types::SolType,
    transports::Transport,
};
use tracing::instrument;

use crate::{
    amm::{AutomatedMarketMaker, AMM},
    bindings::{
        getclpoolticksinrange::{
            GetCLPoolTicksInRange::{self},
            PoolUtils::PopulatedTick,
        },
        getuniswapv3pooldatabatchrequest::GetUniswapV3PoolDataBatchRequest,
        getuniv3pooldata::GetUniV3PoolData,
        syncuniswapv3poolbatchrequest::SyncUniswapV3PoolBatchRequest,
    },
    errors::AMMError,
};

use super::UniswapV3Pool;

sol! {
    struct TickData {
        bool initialized;
        int24 tick;
        uint128 liquidityGross;
        int128 liquidityNet;
    }

    struct TicksWithBlock {
        TickData[] ticks;
        uint256 blockNumber;
    }

    struct PopulatedTicksWithBlock {
        PopulatedTick[] ticks;
        uint256 blockNumber;
    }
}

#[inline]
fn populate_pool_data_from_tokens(
    pool: &mut UniswapV3Pool,
    tokens: &[DynSolValue],
) -> Result<(), AMMError> {
    pool.token_a = tokens[0]
        .as_address()
        .ok_or(AMMError::BatchRequestError(pool.address))?;
    pool.token_a_decimals = tokens[1]
        .as_uint()
        .ok_or(AMMError::BatchRequestError(pool.address))?
        .0
        .to::<u8>();
    pool.token_b = tokens[2]
        .as_address()
        .ok_or(AMMError::BatchRequestError(pool.address))?;
    pool.token_b_decimals = tokens[3]
        .as_uint()
        .ok_or(AMMError::BatchRequestError(pool.address))?
        .0
        .to::<u8>();
    pool.liquidity = tokens[4]
        .as_uint()
        .ok_or(AMMError::BatchRequestError(pool.address))?
        .0
        .to::<u128>();
    pool.sqrt_price = tokens[5]
        .as_uint()
        .ok_or(AMMError::BatchRequestError(pool.address))?
        .0;
    pool.tick = tokens[6]
        .as_int()
        .ok_or(AMMError::BatchRequestError(pool.address))?
        .0
        .as_i32();
    pool.tick_spacing = tokens[7]
        .as_int()
        .ok_or(AMMError::BatchRequestError(pool.address))?
        .0
        .as_i32();
    pool.fee = tokens[8]
        .as_uint()
        .ok_or(AMMError::BatchRequestError(pool.address))?
        .0
        .to::<u32>();

    Ok(())
}

fn bytes32_to_string(bytes: &[u8]) -> String {
    let mut result = String::from_utf8_lossy(bytes).into_owned();
    result.truncate(result.trim_end_matches('\0').len());
    result
}

#[inline]
pub fn populate_v3_pool_data(
    pool: &mut UniswapV3Pool,
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
    pool.factory = tokens[6]
        .as_address()
        .ok_or(AMMError::BatchRequestError(pool.address))?;
    pool.liquidity = tokens[7]
        .as_uint()
        .ok_or(AMMError::BatchRequestError(pool.address))?
        .0
        .to::<u128>();
    pool.sqrt_price = tokens[8]
        .as_uint()
        .ok_or(AMMError::BatchRequestError(pool.address))?
        .0;
    pool.tick = tokens[9]
        .as_int()
        .ok_or(AMMError::BatchRequestError(pool.address))?
        .0
        .as_i32();
    pool.tick_spacing = tokens[10]
        .as_int()
        .ok_or(AMMError::BatchRequestError(pool.address))?
        .0
        .as_i32();
    pool.fee = tokens[11]
        .as_uint()
        .ok_or(AMMError::BatchRequestError(pool.address))?
        .0
        .to::<u32>();

    pool.liquidity_net = i128::try_from(
        tokens[12]
            .as_int()
            .ok_or(AMMError::BatchRequestError(pool.address))?
            .0,
    )
    .unwrap();

    Ok(())
}

pub async fn fetch_v3_pool_data_batch_request<T, N, P>(
    addresses: &[Address],
    block_number: Option<u64>,
    provider: Arc<P>,
) -> Result<DynSolValue, AMMError>
where
    T: Transport + Clone,
    N: Network,
    P: Provider<T, N>,
{
    let deployer = GetUniV3PoolData::deploy_builder(provider, addresses.to_vec());
    let res = if let Some(block_number) = block_number {
        deployer.block(block_number.into()).call_raw().await?
    } else {
        deployer.call_raw().await?
    };

    let constructor_return = DynSolType::Array(Box::new(DynSolType::Tuple(vec![
        DynSolType::Address,
        DynSolType::FixedBytes(32),
        DynSolType::Uint(8),
        DynSolType::Address,
        DynSolType::FixedBytes(32),
        DynSolType::Uint(8),
        DynSolType::Address,
        DynSolType::Uint(128),
        DynSolType::Uint(160),
        DynSolType::Int(24),
        DynSolType::Int(24),
        DynSolType::Uint(24),
        DynSolType::Int(128),
    ])));
    let return_data = constructor_return.abi_decode_sequence(&res)?;

    Ok(return_data)
}

pub async fn get_v3_pool_data_batch_request<T, N, P>(
    pools: &mut [UniswapV3Pool],
    block_number: Option<u64>,
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

    let return_data =
        fetch_v3_pool_data_batch_request(&target_addresses, block_number, provider).await?;

    let mut pool_idx = 0;
    if let Some(tokens_arr) = return_data.as_array() {
        for token in tokens_arr {
            if let Some(pool_data) = token.as_tuple() {
                if let Some(address) = pool_data[0].as_address() {
                    if !address.is_zero() {
                        let uniswap_v3_pool = pools
                            .get_mut(pool_idx)
                            .expect("Pool idx should be in bounds");

                        populate_v3_pool_data(uniswap_v3_pool, pool_data)?;
                    }
                }

                pool_idx += 1;
            }
        }
    }

    Ok(())
}

pub async fn get_uniswap_v3_tick_data_batch_request<T, N, P>(
    pool: &UniswapV3Pool,
    tick_start: i32,
    num_ticks: u32,
    block_number: Option<u64>,
    provider: Arc<P>,
) -> Result<(Vec<PopulatedTick>, U256), AMMError>
where
    T: Transport + Clone,
    N: Network,
    P: Provider<T, N>,
{
    let mut all_tick_data: Vec<PopulatedTick> = vec![];
    let mut last_tick = tick_start;
    let range = min(100, num_ticks);
    let bn = block_number.unwrap_or(0);

    while last_tick < tick_start + num_ticks as i32 {
        let deployer = GetCLPoolTicksInRange::deploy_builder(
            provider.clone(),
            1, //uniswap v3
            pool.address,
            I24::unchecked_from(last_tick),
            I24::unchecked_from(last_tick + range as i32),
        );

        let data = match block_number {
            Some(number) => deployer.block(number.into()).call_raw().await?,
            None => deployer.call_raw().await?,
        };

        let result = PopulatedTicksWithBlock::abi_decode(&data, true)?;

        let tick_data: Vec<PopulatedTick> = result
            .ticks
            .iter()
            .map(|tick| PopulatedTick {
                tick: tick.tick,
                liquidityNet: tick.liquidityNet,
                liquidityGross: tick.liquidityGross,
                feeGrowthOutside0X128: tick.feeGrowthOutside0X128,
                feeGrowthOutside1X128: tick.feeGrowthOutside1X128,
            })
            .collect();

        println!("Adding tick_data: {:?}", tick_data[0].tick);

        all_tick_data.extend(tick_data);
        last_tick += range as i32;
    }

    Ok((all_tick_data, U256::from(bn)))
}

pub async fn sync_v3_pool_batch_request<T, N, P>(
    pool: &mut UniswapV3Pool,
    provider: Arc<P>,
) -> Result<(), AMMError>
where
    T: Transport + Clone,
    N: Network,
    P: Provider<T, N>,
{
    let deployer = SyncUniswapV3PoolBatchRequest::deploy_builder(provider, vec![pool.address]);
    let res = deployer.call_raw().await?;

    let constructor_return = DynSolType::Array(Box::new(DynSolType::Tuple(vec![
        DynSolType::Uint(128),
        DynSolType::Uint(160),
        DynSolType::Int(24),
        DynSolType::Int(128),
    ])));

    let return_data_tokens = constructor_return.abi_decode_sequence(&res)?;

    if let Some(tokens_arr) = return_data_tokens.as_array() {
        if tokens_arr.len() == 1 {
            if let Some(tokens_tup) = tokens_arr[0].as_tuple() {
                if tokens_tup[1]
                    .as_uint()
                    .ok_or(AMMError::BatchRequestError(pool.address))?
                    .0
                    .is_zero()
                {
                    return Err(AMMError::BatchRequestError(pool.address));
                } else {
                    pool.liquidity = tokens_tup[0]
                        .as_uint()
                        .ok_or(AMMError::BatchRequestError(pool.address))?
                        .0
                        .to::<u128>();
                    pool.sqrt_price = tokens_tup[1]
                        .as_uint()
                        .ok_or(AMMError::BatchRequestError(pool.address))?
                        .0;
                    pool.tick = tokens_tup[2]
                        .as_int()
                        .ok_or(AMMError::BatchRequestError(pool.address))?
                        .0
                        .as_i32();
                }
            }
        } else {
            return Err(AMMError::EyreError(eyre::eyre!(
                "Unexpected length of the batch static call"
            )));
        }
    }

    Ok(())
}

#[instrument(skip(provider) level = "debug")]
pub async fn get_amm_data_batch_request<T, N, P>(
    amms: &mut [AMM],
    block_number: u64,
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

    let deployer = GetUniswapV3PoolDataBatchRequest::deploy_builder(provider, target_addresses);
    let res = deployer.block(block_number.into()).call_raw().await?;

    let constructor_return = DynSolType::Array(Box::new(DynSolType::Tuple(vec![
        DynSolType::Address,
        DynSolType::Uint(8),
        DynSolType::Address,
        DynSolType::Uint(8),
        DynSolType::Uint(128),
        DynSolType::Uint(160),
        DynSolType::Int(24),
        DynSolType::Int(24),
        DynSolType::Uint(24),
        DynSolType::Int(128),
    ])));
    let return_data_tokens = constructor_return.abi_decode_sequence(&res)?;

    let mut pool_idx = 0;
    if let Some(tokens_arr) = return_data_tokens.as_array() {
        for token in tokens_arr {
            if let Some(pool_data) = token.as_tuple() {
                if let Some(address) = pool_data[0].as_address() {
                    if !address.is_zero() {
                        // Update pool data
                        if let AMM::UniswapV3Pool(uniswap_v3_pool) = amms
                            .get_mut(pool_idx)
                            .expect("Pool idx should be in bounds")
                        {
                            populate_pool_data_from_tokens(uniswap_v3_pool, pool_data)?;
                        }
                    }
                }

                pool_idx += 1;
            }
        }
    }

    Ok(())
}
