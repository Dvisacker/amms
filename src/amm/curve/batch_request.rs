use alloy::{
    dyn_abi::DynSolType,
    network::Network,
    primitives::{Address, U256},
    providers::Provider,
    transports::Transport,
};
use std::sync::Arc;

use crate::{bindings, errors::AMMError};

use super::CurvePool;

#[derive(Debug)]
struct PoolData {
    coins: [Address; 8],
    balances: [U256; 8],
    decimals: [u8; 8],
    #[allow(dead_code)]
    symbols: [String; 8],
    #[allow(dead_code)]
    virtual_price: U256,
    #[allow(dead_code)]
    a: U256,
    #[allow(dead_code)]
    fee: U256,
}

fn populate_pool_data_from_tokens(
    mut pool: CurvePool,
    pool_data: PoolData,
) -> Result<CurvePool, AMMError> {
    pool.coins = pool_data
        .coins
        .into_iter()
        .filter(|&c| c != Address::ZERO)
        .collect();
    pool.balances = pool_data
        .balances
        .into_iter()
        .take(pool.coins.len())
        .collect();
    pool.coin_decimals = pool_data
        .decimals
        .into_iter()
        .take(pool.coins.len())
        .collect();
    pool.coin_symbols = pool_data
        .symbols
        .into_iter()
        .take(pool.coins.len())
        .collect();

    // You might want to store these additional parameters in your CurvePool struct
    // pool.virtual_price = pool_data.virtual_price;
    // pool.a = pool_data.a;
    // pool.fee = pool_data.fee;

    Ok(pool)
}

pub async fn get_curve_pool_data_batch_request<T, N, P>(
    pool: &mut CurvePool,
    provider: Arc<P>,
) -> Result<(), AMMError>
where
    T: Transport + Clone,
    N: Network,
    P: Provider<T, N>,
{
    let deployer = bindings::getcurvepoolbatchrequest::GetCurvePoolBatchRequest::deploy_builder(
        provider,
        vec![pool.address],
    );
    let res = deployer.call_raw().await?;

    let constructor_return = DynSolType::Array(Box::new(DynSolType::Tuple(vec![
        DynSolType::FixedArray(Box::new(DynSolType::Address), 8),
        DynSolType::FixedArray(Box::new(DynSolType::Uint(256)), 8),
        DynSolType::FixedArray(Box::new(DynSolType::Uint(8)), 8),
        DynSolType::FixedArray(Box::new(DynSolType::FixedBytes(32)), 8),
        DynSolType::Uint(256),
        DynSolType::Uint(256),
        DynSolType::Uint(256),
    ])));
    let return_data_tokens = constructor_return.abi_decode_sequence(&res)?;

    if let Some(tokens_arr) = return_data_tokens.as_array() {
        for token in tokens_arr {
            if let Some(pool_data_tuple) = token.as_tuple() {
                let pool_data = PoolData {
                    coins: pool_data_tuple[0]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|v| v.as_address().unwrap())
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap(),
                    balances: pool_data_tuple[1]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|v| v.as_uint().unwrap().0)
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap(),
                    decimals: pool_data_tuple[2]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|v| v.as_uint().unwrap().0.to::<u8>())
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap(),
                    symbols: pool_data_tuple[3]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|v| {
                            String::from_utf8(v.as_fixed_bytes().unwrap().0.to_vec())
                                .unwrap_or_default()
                        })
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap(),
                    virtual_price: pool_data_tuple[4].as_uint().unwrap().0,
                    a: pool_data_tuple[5].as_uint().unwrap().0,
                    fee: pool_data_tuple[6].as_uint().unwrap().0,
                };

                *pool = populate_pool_data_from_tokens(pool.to_owned(), pool_data)?;
            }
        }
    }

    Ok(())
}
