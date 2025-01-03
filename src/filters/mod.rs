use crate::amm::AMM;

pub mod address;
pub mod value;

pub fn filter_empty_amms(amms: Vec<AMM>) -> Vec<AMM> {
    let mut cleaned_amms = vec![];

    for amm in amms.into_iter() {
        match amm {
            AMM::UniswapV2Pool(ref uniswap_v2_pool) => {
                if !uniswap_v2_pool.token_a.is_zero() && !uniswap_v2_pool.token_b.is_zero() {
                    cleaned_amms.push(amm)
                }
            }
            AMM::UniswapV3Pool(ref uniswap_v3_pool) => {
                if !uniswap_v3_pool.token_a.is_zero() && !uniswap_v3_pool.token_b.is_zero() {
                    cleaned_amms.push(amm)
                }
            }
            AMM::ERC4626Vault(ref erc4626_vault) => {
                if !erc4626_vault.vault_token.is_zero() && !erc4626_vault.asset_token.is_zero() {
                    cleaned_amms.push(amm)
                }
            }
            AMM::CamelotV3Pool(ref camelot_v3_pool) => {
                if !camelot_v3_pool.token_a.is_zero() && !camelot_v3_pool.token_b.is_zero() {
                    cleaned_amms.push(amm)
                }
            }
            AMM::CurvePool(ref curve_pool) => {
                if !curve_pool.coins.is_empty() && !curve_pool.balances.is_empty() {
                    cleaned_amms.push(amm)
                }
            }
            AMM::Ve33Pool(ref ve33_pool) => {
                if !ve33_pool.token_a.is_zero() && !ve33_pool.token_b.is_zero() {
                    cleaned_amms.push(amm)
                }
            }
        }
    }

    cleaned_amms
}
