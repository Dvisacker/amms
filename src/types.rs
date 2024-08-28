use alloy::primitives::Address;

#[derive(Debug, Clone)]
pub struct DetailedPool {
    pub address: Address,
    pub token_a: Address,
    pub token_a_symbol: String,
    pub token_a_decimals: u8,
    pub token_b: Address,
    pub token_b_symbol: String,
    pub token_b_decimals: u8,
    pub factory_address: Address,
    pub reserve_0: u128,
    pub reserve_1: u128,
    pub fee: u32,
}

impl DetailedPool {
    pub fn name(&self) -> String {
        format!("{}_{}", self.token_a_symbol, self.token_b_symbol)
    }
}

// implement deref
// impl Deref for DetailedPool {
//     type Target = UniswapV2Pool;

//     fn deref(&self) -> &Self::Target {
//         &self.inner
//     }
// }
