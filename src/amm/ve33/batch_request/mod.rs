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
    bindings::{self, getve33poolsbatchrequest::GetVe33PoolsBatchRequest},
    errors::AMMError,
};

// contract to get all pools from a ve33 factory.
// sol! {
//     #[allow(missing_docs)]
//     #[sol(rpc)]
//     IGetVe33PoolsBatchRequest,
//     "src/amm/ve33/batch_request/GetVe33PoolsBatchRequest.json"
// }

pub async fn get_pools_batch_request<T, N, P>(
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
