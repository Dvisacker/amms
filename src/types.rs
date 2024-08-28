use alloy::primitives::Address;
use db::models::NewPool;

#[derive(Debug, Clone)]
pub struct DetailedPool {
    pub address: Address,
    pub chain: String,
    pub exchange_name: String,
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

impl From<NewPool> for DetailedPool {
    fn from(pool: NewPool) -> Self {
        DetailedPool {
            chain: pool.chain.to_string(),
            exchange_name: pool.exchange_name.to_string(),
            address: pool.address.parse().unwrap_or(Address::ZERO),
            token_a: pool.token_a.parse().unwrap_or(Address::ZERO),
            token_a_symbol: pool.token_a_symbol.to_string(),
            token_a_decimals: pool.token_a_decimals as u8,
            token_b: pool.token_b.parse().unwrap_or(Address::ZERO),
            token_b_symbol: pool.token_b_symbol.to_string(),
            token_b_decimals: pool.token_b_decimals as u8,
            factory_address: pool.factory_address.parse().unwrap_or(Address::ZERO),
            reserve_0: pool.reserve_0.parse().unwrap_or(0),
            reserve_1: pool.reserve_1.parse().unwrap_or(0),
            fee: pool.fee as u32,
        }
    }
}

impl DetailedPool {
    pub fn name(&self) -> String {
        format!("{}_{}", self.token_a_symbol, self.token_b_symbol)
    }

    pub fn empty(address: Address) -> DetailedPool {
        DetailedPool {
            address,
            chain: String::new(),
            exchange_name: String::new(),
            token_a: Address::ZERO,
            token_a_symbol: String::new(),
            token_a_decimals: 0,
            token_b: Address::ZERO,
            token_b_symbol: String::new(),
            token_b_decimals: 0,
            factory_address: Address::ZERO,
            reserve_0: 0,
            reserve_1: 0,
            fee: 0,
        }
    }

    pub fn to_new_pool(&self) -> NewPool {
        NewPool {
            address: self.address.to_string(),
            chain: self.chain.clone(),
            factory_address: self.factory_address.to_string(),
            exchange_name: self.exchange_name.clone(),
            token_a: self.token_a.to_string(),
            token_a_symbol: self.token_a_symbol.clone(),
            token_a_decimals: self.token_a_decimals as i32,
            token_b: self.token_b.to_string(),
            token_b_symbol: self.token_b_symbol.clone(),
            token_b_decimals: self.token_b_decimals as i32,
            reserve_0: self.reserve_0.to_string(),
            reserve_1: self.reserve_1.to_string(),
            fee: self.fee as i32,
        }
    }
}

// struct NewPoolData {
//     address: String,
//     factory_address: String,
//     token_a: String,
//     token_b: String,
//     reserve_0: String,
//     reserve_1: String,
// }
// implement deref
// impl Deref for DetailedPool {
//     type Target = UniswapV2Pool;

//     fn deref(&self) -> &Self::Target {
//         &self.inner
//     }
// }
