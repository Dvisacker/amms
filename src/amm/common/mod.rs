use alloy::{
    dyn_abi::{DynSolType, DynSolValue},
    network::Network,
    providers::Provider,
    sol,
    transports::Transport,
};
use std::sync::Arc;

use crate::{errors::AMMError, types::DetailedPool};

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    IGetDetailedPoolDataBatchRequest,
    "src/amm/common/batch_request/GetDetailedPoolDataBatchRequestABI.json"
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
        DynSolType::Address,   //pool address
        DynSolType::Address,   //token a
        DynSolType::String,    //token a symbol
        DynSolType::Uint(8),   //token a decimals
        DynSolType::Address,   //token b
        DynSolType::String,    //token b symbol
        DynSolType::Uint(8),   //token b decimals
        DynSolType::Address,   //factory address
        DynSolType::Uint(112), //reserve 0
        DynSolType::Uint(112), //reserve 1
        DynSolType::Uint(32),  //fee
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
                            tracing::trace!(?detailed_pool);
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
    pool.address = tokens[0].as_address()?;
    pool.token_a = tokens[1].as_address()?;
    pool.token_a_symbol = tokens[2].as_str()?.to_string();
    pool.token_a_decimals = tokens[3].as_uint()?.0.to::<u8>();
    pool.token_b = tokens[4].as_address()?;
    pool.token_b_symbol = tokens[5].as_str()?.to_string();
    pool.token_b_decimals = tokens[6].as_uint()?.0.to::<u8>();
    pool.factory_address = tokens[7].as_address()?;
    pool.reserve_0 = tokens[8].as_uint()?.0.to::<u128>();
    pool.reserve_1 = tokens[9].as_uint()?.0.to::<u128>();
    pool.fee = tokens[10].as_uint()?.0.to::<u32>();
    Some(pool)
}
