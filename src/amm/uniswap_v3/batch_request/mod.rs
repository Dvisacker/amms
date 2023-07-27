use std::{collections::HashMap, sync::Arc, vec};

use ethers::{
    abi::{Hash, ParamType, Token},
    providers::Middleware,
    types::{Bytes, H160, I256, U256, U64},
};

use crate::{
    amm::{AutomatedMarketMaker, AMM},
    errors::AMMError,
};

use super::UniswapV3Pool;

use ethers::prelude::abigen;

abigen!(
    IGetUniswapV3PoolDataBatchRequest,
    "src/amm/uniswap_v3/batch_request/GetUniswapV3PoolDataBatchRequestABI.json";
    IGetUniswapV3TickDataBatchRequest,
    "src/amm/uniswap_v3/batch_request/GetUniswapV3TickDataBatchRequestABI.json";
    ISyncUniswapV3PoolBatchRequest,
    "src/amm/uniswap_v3/batch_request/SyncUniswapV3PoolBatchRequestABI.json";

);

impl From<Vec<Token>> for UniswapV3Pool {
    fn from(tokens: Vec<Token>) -> Self {
        UniswapV3Pool {
            address: H160::zero(),
            token_a: tokens[0].to_owned().into_address().unwrap(),
            token_a_decimals: tokens[1].to_owned().into_uint().unwrap().as_u32() as u8,
            token_b: tokens[2].to_owned().into_address().unwrap(),
            token_b_decimals: tokens[3].to_owned().into_uint().unwrap().as_u32() as u8,
            liquidity: tokens[4].to_owned().into_uint().unwrap().as_u128(),
            sqrt_price: tokens[5].to_owned().into_uint().unwrap(),
            tick_bitmap: HashMap::new(),
            ticks: HashMap::new(),
            tick: I256::from_raw(tokens[6].to_owned().into_int().unwrap()).as_i32(),
            tick_spacing: I256::from_raw(tokens[7].to_owned().into_int().unwrap()).as_i32(),
            fee: tokens[8].to_owned().into_uint().unwrap().as_u64() as u32,
        }
    }
}

pub async fn get_v3_pool_data_batch_request<M: Middleware>(
    pool: &mut UniswapV3Pool,
    block_number: Option<u64>,
    middleware: Arc<M>,
) -> Result<(), AMMError<M>> {
    let constructor_args = Token::Tuple(vec![Token::Array(vec![Token::Address(pool.address)])]);

    let deployer =
        IGetUniswapV3PoolDataBatchRequest::deploy(middleware.clone(), constructor_args).unwrap();

    let return_data: Bytes = if let Some(block_number) = block_number {
        deployer.block(block_number).call_raw().await?
    } else {
        deployer.call_raw().await?
    };

    let return_data_tokens = ethers::abi::decode(
        &[ParamType::Array(Box::new(ParamType::Tuple(vec![
            ParamType::Address,   // token a
            ParamType::Uint(8),   // token a decimals
            ParamType::Address,   // token b
            ParamType::Uint(8),   // token b decimals
            ParamType::Uint(128), // liquidity
            ParamType::Uint(160), // sqrtPrice
            ParamType::Int(24),   // tick
            ParamType::Int(24),   // tickSpacing
            ParamType::Uint(24),  // fee
            ParamType::Int(128),  // liquidityNet
        ])))],
        &return_data,
    )?;

    //Update pool data
    for tokens in return_data_tokens {
        if let Some(tokens_arr) = tokens.into_array() {
            for tup in tokens_arr {
                if let Some(pool_data) = tup.into_tuple() {
                    //If the pool token A is not zero, signaling that the pool data was populated
                    if !pool_data[0].to_owned().into_address().unwrap().is_zero() {
                        let mut pool_data = UniswapV3Pool::from(pool_data);
                        pool_data.address = pool.address;
                        pool_data.tick_bitmap = pool.tick_bitmap.clone();
                        pool_data.ticks = pool.ticks.clone();
                        *pool = pool_data;
                    }
                }
            }
        }
    }
    Ok(())
}

pub struct UniswapV3TickData {
    pub initialized: bool,
    pub tick: i32,
    pub liquidity_net: i128,
}

pub async fn get_uniswap_v3_tick_data_batch_request<M: Middleware>(
    pool: &UniswapV3Pool,
    tick_start: i32,
    zero_for_one: bool,
    num_ticks: u16,
    block_number: Option<U64>,
    middleware: Arc<M>,
) -> Result<(Vec<UniswapV3TickData>, U64), AMMError<M>> {
    let constructor_args = Token::Tuple(vec![
        Token::Address(pool.address),
        Token::Bool(zero_for_one),
        Token::Int(I256::from(tick_start).into_raw()),
        Token::Uint(U256::from(num_ticks)),
        Token::Int(I256::from(pool.tick_spacing).into_raw()),
    ]);

    let deployer =
        IGetUniswapV3TickDataBatchRequest::deploy(middleware.clone(), constructor_args).unwrap();

    let return_data: Bytes = if block_number.is_some() {
        deployer.block(block_number.unwrap()).call_raw().await?
    } else {
        deployer.call_raw().await?
    };

    let return_data_tokens = ethers::abi::decode(
        &[
            ParamType::Array(Box::new(ParamType::Tuple(vec![
                ParamType::Bool,
                ParamType::Int(24),
                ParamType::Int(128),
            ]))),
            ParamType::Uint(32),
        ],
        &return_data,
    )?;

    //TODO: handle these errors instead of using expect
    let tick_data_array = return_data_tokens[0]
        .to_owned()
        .into_array()
        .expect("Failed to convert initialized_ticks from Vec<Token> to Vec<i128>");

    let mut tick_data = vec![];

    for tokens in tick_data_array {
        if let Some(tick_data_tuple) = tokens.into_tuple() {
            let initialized = tick_data_tuple[0]
                .to_owned()
                .into_bool()
                .expect("Could not convert token to bool");

            let initialized_tick = I256::from_raw(
                tick_data_tuple[1]
                    .to_owned()
                    .into_int()
                    .expect("Could not convert token to int"),
            )
            .as_i32();

            let liquidity_net = I256::from_raw(
                tick_data_tuple[2]
                    .to_owned()
                    .into_int()
                    .expect("Could not convert token to int"),
            )
            .as_i128();

            tick_data.push(UniswapV3TickData {
                initialized,
                tick: initialized_tick,
                liquidity_net,
            });
        }
    }

    let block_number = return_data_tokens[1]
        .to_owned()
        .into_uint()
        .expect("Failed to convert block_number from Token to U64");

    Ok((tick_data, U64::from(block_number.as_u64())))
}

pub async fn sync_v3_pool_batch_request<M: Middleware>(
    pool: &mut UniswapV3Pool,
    middleware: Arc<M>,
) -> Result<(), AMMError<M>> {
    let constructor_args = Token::Tuple(vec![Token::Address(pool.address)]);

    let deployer =
        ISyncUniswapV3PoolBatchRequest::deploy(middleware.clone(), constructor_args).unwrap();

    let return_data: Bytes = deployer.call_raw().await?;
    let return_data_tokens = ethers::abi::decode(
        &[ParamType::Tuple(vec![
            ParamType::Uint(128), // liquidity
            ParamType::Uint(160), // sqrtPrice
            ParamType::Int(24),   // tick
            ParamType::Int(128),  // liquidityNet
        ])],
        &return_data,
    )?;

    for tokens in return_data_tokens {
        if let Some(pool_data) = tokens.into_tuple() {
            //If the sqrt_price is not zero, signaling that the pool data was populated
            if !pool_data[1].to_owned().into_uint().unwrap().is_zero() {
                //Update the pool data
                pool.liquidity = pool_data[0].to_owned().into_uint().unwrap().as_u128();
                pool.sqrt_price = pool_data[1].to_owned().into_uint().unwrap();
                pool.tick = I256::from_raw(pool_data[2].to_owned().into_int().unwrap()).as_i32();
            } else {
                return Err(AMMError::SyncError(pool.address));
            }
        }
    }

    Ok(())
}

pub async fn get_amm_data_batch_request<M: Middleware>(
    amms: &mut [AMM],
    block_number: u64,
    middleware: Arc<M>,
) -> Result<(), AMMError<M>> {
    let mut target_addresses = vec![];

    for amm in amms.iter() {
        target_addresses.push(Token::Address(amm.address()));
    }

    let constructor_args = Token::Tuple(vec![Token::Array(target_addresses)]);
    let deployer =
        IGetUniswapV3PoolDataBatchRequest::deploy(middleware.clone(), constructor_args).unwrap();

    let return_data: Bytes = deployer.block(block_number).call_raw().await?;

    let return_data_tokens = ethers::abi::decode(
        &[ParamType::Array(Box::new(ParamType::Tuple(vec![
            ParamType::Address,   // token a
            ParamType::Uint(8),   // token a decimals
            ParamType::Address,   // token b
            ParamType::Uint(8),   // token b decimals
            ParamType::Uint(128), // liquidity
            ParamType::Uint(160), // sqrtPrice
            ParamType::Int(24),   // tick
            ParamType::Int(24),   // tickSpacing
            ParamType::Uint(24),  // fee
            ParamType::Int(128),  // liquidityNet
        ])))],
        &return_data,
    )?;

    let mut pool_idx = 0;

    //Update pool data
    for tokens in return_data_tokens {
        if let Some(tokens_arr) = tokens.into_array() {
            for tup in tokens_arr {
                if let Some(pool_data) = tup.into_tuple() {
                    //If the pool token A is not zero, signaling that the pool data was populated
                    if !pool_data[0].to_owned().into_address().unwrap().is_zero() {
                        //Update the pool data
                        if let AMM::UniswapV3Pool(uniswap_v3_pool) = amms.get_mut(pool_idx).unwrap()
                        {
                            let mut pool_data = UniswapV3Pool::from(pool_data);
                            pool_data.address = uniswap_v3_pool.address;
                            pool_data.tick_bitmap = uniswap_v3_pool.tick_bitmap.clone();
                            pool_data.ticks = uniswap_v3_pool.ticks.clone();
                            *uniswap_v3_pool = pool_data;
                        }
                    }
                    pool_idx += 1;
                }
            }
        }
    }
    Ok(())
}
