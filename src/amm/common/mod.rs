use alloy::{
    dyn_abi::{DynSolType, DynSolValue},
    network::Network,
    primitives::Address,
    providers::Provider,
    sol,
    transports::Transport,
};
use std::sync::Arc;
use types::pool::DetailedPool;

use crate::errors::AMMError;

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    IGetDetailedPoolDataBatchRequest,
    "src/amm/common/batch_request/GetDetailedPoolDataBatchRequestABI.json"
}

pub async fn fetch_pool_data_batch_request<T, N, P>(
    addresses: Vec<Address>,
    provider: Arc<P>,
) -> Result<DynSolValue, AMMError>
where
    T: Transport + Clone,
    N: Network,
    P: Provider<T, N>,
{
    let deployer = IGetDetailedPoolDataBatchRequest::deploy_builder(provider, addresses);
    let res = deployer.call().await?;

    let constructor_return = DynSolType::Array(Box::new(DynSolType::Tuple(vec![
        DynSolType::Address,        //token a
        DynSolType::FixedBytes(32), //token a symbol
        DynSolType::Uint(8),        //token a decimals
        DynSolType::Address,        //token b
        DynSolType::FixedBytes(32), //token b symbol
        DynSolType::Uint(8),        //token b decimals
        DynSolType::Address,        //factory address
        DynSolType::Uint(112),      //reserve 0
        DynSolType::Uint(112),      //reserve 1
        DynSolType::Uint(24),       //fee
    ])));

    let return_data_tokens = constructor_return.abi_decode_sequence(&res)?;

    Ok(return_data_tokens)
}

pub async fn get_detailed_pool_data_batch_request<T, N, P>(
    amms: &mut [DetailedPool],
    provider: Arc<P>,
) -> Result<(), AMMError>
where
    T: Transport + Clone,
    N: Network,
    P: Provider<T, N>,
{
    let mut target_addresses = vec![];
    for amm in amms.iter() {
        target_addresses.push(amm.address);
    }

    let deployer = IGetDetailedPoolDataBatchRequest::deploy_builder(provider, target_addresses);
    let res = deployer.call().await?;

    let constructor_return = DynSolType::Array(Box::new(DynSolType::Tuple(vec![
        DynSolType::Address,        //token a
        DynSolType::FixedBytes(32), //token a symbol
        DynSolType::Uint(8),        //token a decimals
        DynSolType::Address,        //token b
        DynSolType::FixedBytes(32), //token b symbol
        DynSolType::Uint(8),        //token b decimals
        DynSolType::Address,        //factory address
        DynSolType::Uint(112),      //reserve 0
        DynSolType::Uint(112),      //reserve 1
        DynSolType::Uint(24),       //fee
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
                        let pool = amms
                            .get_mut(pool_idx)
                            .expect("Pool idx should be in bounds");

                        if let Some(detailed_pool) =
                            populate_pool_data_from_tokens(pool.to_owned(), pool_data)
                        {
                            *pool = detailed_pool;
                        }
                    }
                }

                pool_idx += 1;
            }
        }
    }

    Ok(())
}

#[inline]
fn populate_pool_data_from_tokens(
    mut pool: DetailedPool,
    tokens: &[DynSolValue],
) -> Option<DetailedPool> {
    pool.address = pool.address;
    pool.token_a = tokens[0].as_address()?;
    pool.token_a_symbol = bytes32_to_string(tokens[1].as_fixed_bytes()?.0);
    pool.token_a_decimals = tokens[2].as_uint()?.0.to::<u8>();
    pool.token_b = tokens[3].as_address()?;
    pool.token_b_symbol = bytes32_to_string(tokens[4].as_fixed_bytes()?.0);
    pool.token_b_decimals = tokens[5].as_uint()?.0.to::<u8>();
    pool.factory_address = tokens[6].as_address()?;
    pool.reserve_0 = tokens[7].as_uint()?.0.to::<u128>();
    pool.reserve_1 = tokens[8].as_uint()?.0.to::<u128>();
    pool.fee = tokens[9].as_uint()?.0.to::<u32>();
    Some(pool)
}

fn bytes32_to_string(bytes: &[u8]) -> String {
    let mut result = String::from_utf8_lossy(bytes).into_owned();
    result.truncate(result.trim_end_matches('\0').len());
    result
}
