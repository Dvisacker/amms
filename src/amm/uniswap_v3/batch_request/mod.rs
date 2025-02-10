use alloy::{
    hex,
    network::Network,
    primitives::{aliases::I24, Address, FixedBytes, U256},
    providers::Provider,
    sol,
    sol_types::SolType,
};
use std::{cmp::min, sync::Arc, vec};
use tracing::instrument;

use crate::{
    amm::{AutomatedMarketMaker, AMM},
    bindings::{
        getclpoolticksinrange::{
            GetCLPoolTicksInRange::{self},
            PoolHelpers::PopulatedTick,
        },
        getuniv3pooldata::{GetUniV3PoolData, PoolHelpers::UniswapV3PoolData},
        syncuniswapv3poolbatchrequest::{
            PoolHelpers::UniswapV3PoolPriceData, SyncUniswapV3PoolBatchRequest,
        },
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

    struct PopulatedTicks {
        PopulatedTick[] ticks;
    }
}

type SolArray<T> = sol! { T[] };

pub fn fixed_bytes_to_string(bytes: &FixedBytes<32>) -> String {
    let mut result = String::from_utf8_lossy(bytes.as_slice()).into_owned();
    result.truncate(result.trim_end_matches('\0').len());
    result
}

#[inline]
pub fn populate_v3_pool_data(
    pool: &mut UniswapV3Pool,
    data: &UniswapV3PoolData,
) -> Result<(), AMMError> {
    pool.token_a = data.tokenA;
    pool.token_a_symbol = fixed_bytes_to_string(&data.tokenASymbol);
    pool.token_a_decimals = data.tokenADecimals;
    pool.token_b = data.tokenB;
    pool.token_b_symbol = fixed_bytes_to_string(&data.tokenBSymbol);
    pool.token_b_decimals = data.tokenBDecimals;
    pool.factory = data.factory;
    pool.liquidity = data.liquidity;
    pool.sqrt_price = U256::from(data.sqrtPrice);
    pool.tick = data.tick.try_into().unwrap();
    pool.tick_spacing = data.tickSpacing.try_into().unwrap();
    pool.fee = data.fee.try_into().unwrap();
    pool.liquidity_net = data.liquidityNet;
    Ok(())
}

pub async fn fetch_v3_pool_data_batch_request<N, P>(
    addresses: &[Address],
    block_number: Option<u64>,
    provider: Arc<P>,
) -> Result<Vec<UniswapV3PoolData>, AMMError>
where
    N: Network,
    P: Provider<N>,
{
    let deployer = GetUniV3PoolData::deploy_builder(provider, addresses.to_vec());
    let res = if let Some(block_number) = block_number {
        deployer.block(block_number.into()).call_raw().await?
    } else {
        deployer.call_raw().await?
    };

    let return_data = SolArray::<UniswapV3PoolData>::abi_decode(&res, false)?;

    Ok(return_data)
}

pub async fn get_v3_pool_data_batch_request<N, P>(
    pools: &mut [UniswapV3Pool],
    block_number: Option<u64>,
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

    let return_data =
        fetch_v3_pool_data_batch_request(&target_addresses, block_number, provider).await?;

    let mut pool_idx = 0;
    for pool_data in return_data {
        if !pool_data.tokenA.is_zero() {
            let uniswap_v3_pool = pools
                .get_mut(pool_idx)
                .expect("Pool idx should be in bounds");

            populate_v3_pool_data(uniswap_v3_pool, &pool_data)?;
        }

        pool_idx += 1;
    }

    Ok(())
}

pub async fn get_uniswap_v3_tick_data_batch_request<N, P>(
    pool: &UniswapV3Pool,
    tick_start: i32,
    num_ticks: i32,
    block_number: Option<u64>,
    provider: Arc<P>,
    exchange_id: u8, // 1 for uniswap v3, 2 for pancake v3, 3 for slipstream
) -> Result<(Vec<PopulatedTick>, U256), AMMError>
where
    N: Network,
    P: Provider<N>,
{
    let mut all_tick_data: Vec<PopulatedTick> = vec![];
    let mut last_tick = tick_start;
    let range = min(2000, num_ticks);
    let bn = block_number.unwrap_or(0);

    while last_tick < tick_start + num_ticks {
        let deployer = GetCLPoolTicksInRange::deploy_builder(
            provider.clone(),
            exchange_id,
            pool.address,
            I24::unchecked_from(last_tick),
            I24::unchecked_from(last_tick + range),
        );

        // The tick range contract returns a revert with the data in the error message
        // This allows us to bypass the codesize limit
        let data = match deployer.call_raw().await {
            Err(err) => {
                let err_string = err.to_string();

                if let Some(data_start) = err_string.find("data: ") {
                    let data_hex = &err_string[data_start + 7..].trim_matches('"');
                    hex::decode(&data_hex[2..]).unwrap()
                } else {
                    return Ok((vec![], U256::from(bn)));
                }
            }
            _ => vec![],
        };

        let result = PopulatedTicks::abi_decode(&data, true)?;

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

        all_tick_data.extend(tick_data);
        last_tick += range;
    }

    Ok((all_tick_data, U256::from(bn)))
}

// TODO: This function is confusing and not very useful since it only syncs the
// tick and liquidity (and not the entire data). Remove or rename this function
pub async fn sync_v3_pool_batch_request<N, P>(
    pool: &mut UniswapV3Pool,
    provider: Arc<P>,
) -> Result<(), AMMError>
where
    N: Network,
    P: Provider<N>,
{
    let deployer = SyncUniswapV3PoolBatchRequest::deploy_builder(provider, vec![pool.address]);
    let res = deployer.call_raw().await?;
    let return_data = SolArray::<UniswapV3PoolPriceData>::abi_decode(&res, false)?;

    let pool_data = return_data[0].clone();
    pool.liquidity = pool_data.liquidity;
    pool.sqrt_price = U256::from(pool_data.sqrtPrice);
    pool.tick = pool_data.tick.try_into().expect("Tick should be in bounds");
    pool.liquidity_net = pool_data.liquidityNet;

    Ok(())
}

#[instrument(skip(provider) level = "debug")]
pub async fn get_amm_data_batch_request<N, P>(
    amms: &mut [AMM],
    block_number: u64,
    provider: Arc<P>,
    full_sync: bool,
) -> Result<(), AMMError>
where
    N: Network,
    P: Provider<N>,
{
    let mut target_addresses = vec![];

    for amm in amms.iter() {
        target_addresses.push(amm.address());
    }

    let return_data =
        fetch_v3_pool_data_batch_request(&target_addresses, Some(block_number), provider.clone())
            .await?;

    let mut pool_idx = 0;
    for pool_data in return_data {
        if !pool_data.tokenA.is_zero() {
            // Update pool data
            if let AMM::UniswapV3Pool(uniswap_v3_pool) = amms
                .get_mut(pool_idx)
                .expect("Pool idx should be in bounds")
            {
                populate_v3_pool_data(uniswap_v3_pool, &pool_data)?;

                /*
                Full syncing uniswap v3 involves fetching/retrieving tick data which
                can be intensive RPC wise so we only do this if required

                Currently we fetch 1000 (initialized) ticks around the current price tick
                but there should probably be an arg to control the number of ticks to fetch
                */
                if full_sync {
                    let num_ticks = uniswap_v3_pool.tick_spacing * 100;
                    let tick_start = uniswap_v3_pool.tick - num_ticks / 2;
                    let (tick_data, _) = get_uniswap_v3_tick_data_batch_request(
                        uniswap_v3_pool,
                        tick_start,
                        num_ticks,
                        Some(block_number),
                        provider.clone(),
                        1, // uniswap v3
                    )
                    .await?;

                    uniswap_v3_pool.populate_ticks_from_tick_data(tick_data);
                }
            }

            pool_idx += 1;
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

    use alloy::{primitives::address, providers::ProviderBuilder};

    #[tokio::test]
    #[ignore]
    async fn test_uniswap_v3_batch_full_sync() {
        {
            dotenv::dotenv().ok();
            let rpc_endpoint = std::env::var("BASE_RPC_URL").expect("Missing RPC url");
            let provider = Arc::new(ProviderBuilder::new().on_http(rpc_endpoint.parse().unwrap()));

            let addresses = vec![
                address!("e5b5f522e98b5a2baae212d4da66b865b781db97"),
                address!("d0b53D9277642d899DF5C87A3966A349A798F224"),
                address!("00bf864f6bb2466fa875b97715f3b7e0cb76198c"),
                address!("20e068d76f9e90b90604500b84c7e19dcb923e7e"),
            ];

            let pools = addresses.iter().map(|addr| UniswapV3Pool {
                address: *addr,
                ..Default::default()
            });

            let synced_block = provider.get_block_number().await.unwrap();

            for mut pool in pools.clone() {
                pool.populate_data(Some(synced_block), provider.clone())
                    .await
                    .unwrap();
            }

            let mut amms: Vec<AMM> = pools
                .into_iter()
                .map(|pool| AMM::UniswapV3Pool(pool))
                .collect();

            get_amm_data_batch_request(&mut amms, synced_block, provider, true)
                .await
                .unwrap();

            for amm in amms.iter() {
                if let AMM::UniswapV3Pool(pool) = amm {
                    assert!(!pool.ticks.is_empty());
                }
            }
        }
    }
}
