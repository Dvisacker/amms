///Module containing a contract's types and functions.
/**

```solidity
library PoolHelpers {
    struct UniswapV3PoolData { address tokenA; bytes32 tokenASymbol; uint8 tokenADecimals; address tokenB; bytes32 tokenBSymbol; uint8 tokenBDecimals; address factory; uint128 liquidity; uint160 sqrtPrice; int24 tick; int24 tickSpacing; uint24 fee; int128 liquidityNet; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod PoolHelpers {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct UniswapV3PoolData { address tokenA; bytes32 tokenASymbol; uint8 tokenADecimals; address tokenB; bytes32 tokenBSymbol; uint8 tokenBDecimals; address factory; uint128 liquidity; uint160 sqrtPrice; int24 tick; int24 tickSpacing; uint24 fee; int128 liquidityNet; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UniswapV3PoolData {
        #[allow(missing_docs)]
        pub tokenA: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tokenASymbol: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub tokenADecimals: u8,
        #[allow(missing_docs)]
        pub tokenB: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tokenBSymbol: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub tokenBDecimals: u8,
        #[allow(missing_docs)]
        pub factory: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub liquidity: u128,
        #[allow(missing_docs)]
        pub sqrtPrice: alloy::sol_types::private::primitives::aliases::U160,
        #[allow(missing_docs)]
        pub tick: alloy::sol_types::private::primitives::aliases::I24,
        #[allow(missing_docs)]
        pub tickSpacing: alloy::sol_types::private::primitives::aliases::I24,
        #[allow(missing_docs)]
        pub fee: alloy::sol_types::private::primitives::aliases::U24,
        #[allow(missing_docs)]
        pub liquidityNet: i128,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<8>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<8>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<128>,
            alloy::sol_types::sol_data::Uint<160>,
            alloy::sol_types::sol_data::Int<24>,
            alloy::sol_types::sol_data::Int<24>,
            alloy::sol_types::sol_data::Uint<24>,
            alloy::sol_types::sol_data::Int<128>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::FixedBytes<32>,
            u8,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::FixedBytes<32>,
            u8,
            alloy::sol_types::private::Address,
            u128,
            alloy::sol_types::private::primitives::aliases::U160,
            alloy::sol_types::private::primitives::aliases::I24,
            alloy::sol_types::private::primitives::aliases::I24,
            alloy::sol_types::private::primitives::aliases::U24,
            i128,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UniswapV3PoolData> for UnderlyingRustTuple<'_> {
            fn from(value: UniswapV3PoolData) -> Self {
                (
                    value.tokenA,
                    value.tokenASymbol,
                    value.tokenADecimals,
                    value.tokenB,
                    value.tokenBSymbol,
                    value.tokenBDecimals,
                    value.factory,
                    value.liquidity,
                    value.sqrtPrice,
                    value.tick,
                    value.tickSpacing,
                    value.fee,
                    value.liquidityNet,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UniswapV3PoolData {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    tokenA: tuple.0,
                    tokenASymbol: tuple.1,
                    tokenADecimals: tuple.2,
                    tokenB: tuple.3,
                    tokenBSymbol: tuple.4,
                    tokenBDecimals: tuple.5,
                    factory: tuple.6,
                    liquidity: tuple.7,
                    sqrtPrice: tuple.8,
                    tick: tuple.9,
                    tickSpacing: tuple.10,
                    fee: tuple.11,
                    liquidityNet: tuple.12,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for UniswapV3PoolData {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for UniswapV3PoolData {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenA,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokenASymbol),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokenADecimals),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenB,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokenBSymbol),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokenBDecimals),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.factory,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidity),
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::tokenize(&self.sqrtPrice),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tick),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickSpacing),
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.fee),
                    <alloy::sol_types::sol_data::Int<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidityNet),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for UniswapV3PoolData {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for UniswapV3PoolData {
            const NAME: &'static str = "UniswapV3PoolData";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "UniswapV3PoolData(address tokenA,bytes32 tokenASymbol,uint8 tokenADecimals,address tokenB,bytes32 tokenBSymbol,uint8 tokenBDecimals,address factory,uint128 liquidity,uint160 sqrtPrice,int24 tick,int24 tickSpacing,uint24 fee,int128 liquidityNet)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.tokenA,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.tokenASymbol)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.tokenADecimals,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.tokenB,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.tokenBSymbol)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.tokenBDecimals,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.factory,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.liquidity)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.sqrtPrice)
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.tick)
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.tickSpacing)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.fee)
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        128,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.liquidityNet)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for UniswapV3PoolData {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tokenA,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tokenASymbol,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tokenADecimals,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tokenB,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tokenBSymbol,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tokenBDecimals,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.factory,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.liquidity,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sqrtPrice,
                    )
                    + <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.tick)
                    + <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tickSpacing,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.fee)
                    + <alloy::sol_types::sol_data::Int<
                        128,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.liquidityNet,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tokenA,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tokenASymbol,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tokenADecimals,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tokenB,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tokenBSymbol,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tokenBDecimals,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.factory,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.liquidity,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    160,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sqrtPrice,
                    out,
                );
                <alloy::sol_types::sol_data::Int<
                    24,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tick,
                    out,
                );
                <alloy::sol_types::sol_data::Int<
                    24,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tickSpacing,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    24,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.fee, out);
                <alloy::sol_types::sol_data::Int<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.liquidityNet,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`PoolHelpers`](self) contract instance.

See the [wrapper's documentation](`PoolHelpersInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> PoolHelpersInstance<T, P, N> {
        PoolHelpersInstance::<T, P, N>::new(address, provider)
    }
    /**A [`PoolHelpers`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`PoolHelpers`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct PoolHelpersInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for PoolHelpersInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("PoolHelpersInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > PoolHelpersInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`PoolHelpers`](self) contract instance.

See the [wrapper's documentation](`PoolHelpersInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> PoolHelpersInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> PoolHelpersInstance<T, P, N> {
            PoolHelpersInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > PoolHelpersInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > PoolHelpersInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library PoolHelpers {
    struct UniswapV3PoolData {
        address tokenA;
        bytes32 tokenASymbol;
        uint8 tokenADecimals;
        address tokenB;
        bytes32 tokenBSymbol;
        uint8 tokenBDecimals;
        address factory;
        uint128 liquidity;
        uint160 sqrtPrice;
        int24 tick;
        int24 tickSpacing;
        uint24 fee;
        int128 liquidityNet;
    }
}

interface GetUniV3PoolData {
    constructor(address[] pools);

    function getPoolData(address[] memory pools) external returns (PoolHelpers.UniswapV3PoolData[] memory);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "pools",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getPoolData",
    "inputs": [
      {
        "name": "pools",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct PoolHelpers.UniswapV3PoolData[]",
        "components": [
          {
            "name": "tokenA",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "tokenASymbol",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "tokenADecimals",
            "type": "uint8",
            "internalType": "uint8"
          },
          {
            "name": "tokenB",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "tokenBSymbol",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "tokenBDecimals",
            "type": "uint8",
            "internalType": "uint8"
          },
          {
            "name": "factory",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "liquidity",
            "type": "uint128",
            "internalType": "uint128"
          },
          {
            "name": "sqrtPrice",
            "type": "uint160",
            "internalType": "uint160"
          },
          {
            "name": "tick",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "tickSpacing",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "fee",
            "type": "uint24",
            "internalType": "uint24"
          },
          {
            "name": "liquidityNet",
            "type": "int128",
            "internalType": "int128"
          }
        ]
      }
    ],
    "stateMutability": "nonpayable"
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod GetUniV3PoolData {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b506040516118fa3803806118fa83398181016040528101906100319190610fe8565b5f6100418261007060201b60201c565b90505f8160405160200161005591906112af565b60405160208183030381529060405290506020810180590381f35b60605f825167ffffffffffffffff81111561008e5761008d610e52565b5b6040519080825280602002602001820160405280156100c757816020015b6100b4610d4e565b8152602001906001900390816100ac5790505b5090505f5b8351811015610ced575f8482815181106100e9576100e86112cf565b5b6020026020010151905061010281610cf760201b60201c565b1561010d5750610ce2565b610115610d4e565b8173ffffffffffffffffffffffffffffffffffffffff16630dfe16816040518163ffffffff1660e01b8152600401602060405180830381865afa15801561015e573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061018291906112fc565b815f019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff16815250508173ffffffffffffffffffffffffffffffffffffffff1663d21220a76040518163ffffffff1660e01b8152600401602060405180830381865afa158015610201573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061022591906112fc565b816060019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff168152505061026e815f0151610cf760201b60201c565b1561027a575050610ce2565b61028d8160600151610cf760201b60201c565b15610299575050610ce2565b5f80825f015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f313ce567000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506040516103489190611379565b5f604051808303815f8787f1925050503d805f8114610382576040519150601f19603f3d011682016040523d82523d5f602084013e610387565b606091505b509150915081156103f6575f60208251036103e657818060200190518101906103b091906113c2565b90505f8114806103c0575060ff81115b156103cf575050505050610ce2565b80846040019060ff16908160ff16815250506103f0565b5050505050610ce2565b506103ff565b50505050610ce2565b5f80846060015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f313ce567000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506040516104af9190611379565b5f604051808303815f8787f1925050503d805f81146104e9576040519150601f19603f3d011682016040523d82523d5f602084013e6104ee565b606091505b50915091508115610561575f602082510361054f578180602001905181019061051791906113c2565b90505f811480610527575060ff81115b156105385750505050505050610ce2565b808660a0019060ff16908160ff168152505061055b565b50505050505050610ce2565b5061056c565b505050505050610ce2565b5f80865f015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f95d89b41000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff838183161783525050505060405161061b9190611379565b5f604051808303815f8787f1925050503d805f8114610655576040519150601f19603f3d011682016040523d82523d5f602084013e61065a565b606091505b509150915081156106c4575f602082510361068a57818060200190518101906106839190611417565b90506106b4565b5f8280602001905181019061069f91906114e4565b90506106b081610d2860201b60201c565b9150505b80886020018181525050506106d1565b5f801b8760200181815250505b5f80886060015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f95d89b41000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506040516107819190611379565b5f604051808303815f8787f1925050503d805f81146107bb576040519150601f19603f3d011682016040523d82523d5f602084013e6107c0565b606091505b5091509150811561082a575f60208251036107f057818060200190518101906107e99190611417565b905061081a565b5f8280602001905181019061080591906114e4565b905061081681610d2860201b60201c565b9150505b808a608001818152505050610837565b5f801b8960800181815250505b8973ffffffffffffffffffffffffffffffffffffffff1663c45a01556040518163ffffffff1660e01b8152600401602060405180830381865afa92505050801561089f57506040513d601f19601f8201168201806040525081019061089c91906112fc565b60015b6108e0575f8960c0019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff168152505061091a565b808a60c0019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff1681525050505b8973ffffffffffffffffffffffffffffffffffffffff1663ddca3f436040518163ffffffff1660e01b8152600401602060405180830381865afa92505050801561098257506040513d601f19601f8201168201806040525081019061097f9190611555565b60015b6109a457610bb889610160019062ffffff16908162ffffff16815250506109bd565b808a610160019062ffffff16908162ffffff1681525050505b5f808b73ffffffffffffffffffffffffffffffffffffffff16633850c7bd6040518163ffffffff1660e01b815260040160e060405180830381865afa158015610a08573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a2c919061166a565b5050505050915091505f8c73ffffffffffffffffffffffffffffffffffffffff1663f30dba93836040518263ffffffff1660e01b8152600401610a6f9190611716565b61010060405180830381865afa158015610a8b573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610aaf91906117f2565b5050505050509150508c73ffffffffffffffffffffffffffffffffffffffff16631a6865026040518163ffffffff1660e01b8152600401602060405180830381865afa158015610b01573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b2591906118a3565b8c60e001906fffffffffffffffffffffffffffffffff1690816fffffffffffffffffffffffffffffffff16815250508c73ffffffffffffffffffffffffffffffffffffffff1663d0c93a7c6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610b9d573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610bc191906118ce565b8c610140019060020b908160020b815250508c73ffffffffffffffffffffffffffffffffffffffff1663ddca3f436040518163ffffffff1660e01b8152600401602060405180830381865afa158015610c1c573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c409190611555565b8c610160019062ffffff16908162ffffff1681525050828c610100019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff1681525050818c610120019060020b908160020b81525050808c6101800190600f0b9081600f0b815250508b8f8f81518110610cc957610cc86112cf565b5b6020026020010181905250505050505050505050505050505b8060010190506100cc565b5080915050919050565b5f808273ffffffffffffffffffffffffffffffffffffffff163b03610d1f5760019050610d23565b5f90505b919050565b5f808290505f815103610d40575f801b915050610d49565b60208301519150505b919050565b604051806101a001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f80191681526020015f60ff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f80191681526020015f60ff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f6fffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f60020b81526020015f60020b81526020015f62ffffff1681526020015f600f0b81525090565b5f604051905090565b5f80fd5b5f80fd5b5f80fd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b610e8882610e42565b810181811067ffffffffffffffff82111715610ea757610ea6610e52565b5b80604052505050565b5f610eb9610e2d565b9050610ec58282610e7f565b919050565b5f67ffffffffffffffff821115610ee457610ee3610e52565b5b602082029050602081019050919050565b5f80fd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610f2282610ef9565b9050919050565b610f3281610f18565b8114610f3c575f80fd5b50565b5f81519050610f4d81610f29565b92915050565b5f610f65610f6084610eca565b610eb0565b90508083825260208201905060208402830185811115610f8857610f87610ef5565b5b835b81811015610fb15780610f9d8882610f3f565b845260208401935050602081019050610f8a565b5050509392505050565b5f82601f830112610fcf57610fce610e3e565b5b8151610fdf848260208601610f53565b91505092915050565b5f60208284031215610ffd57610ffc610e36565b5b5f82015167ffffffffffffffff81111561101a57611019610e3a565b5b61102684828501610fbb565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b61106181610f18565b82525050565b5f819050919050565b61107981611067565b82525050565b5f60ff82169050919050565b6110948161107f565b82525050565b5f6fffffffffffffffffffffffffffffffff82169050919050565b6110be8161109a565b82525050565b6110cd81610ef9565b82525050565b5f8160020b9050919050565b6110e8816110d3565b82525050565b5f62ffffff82169050919050565b611105816110ee565b82525050565b5f81600f0b9050919050565b6111208161110b565b82525050565b6101a082015f82015161113b5f850182611058565b50602082015161114e6020850182611070565b506040820151611161604085018261108b565b5060608201516111746060850182611058565b5060808201516111876080850182611070565b5060a082015161119a60a085018261108b565b5060c08201516111ad60c0850182611058565b5060e08201516111c060e08501826110b5565b506101008201516111d56101008501826110c4565b506101208201516111ea6101208501826110df565b506101408201516111ff6101408501826110df565b506101608201516112146101608501826110fc565b50610180820151611229610180850182611117565b50505050565b5f61123a8383611126565b6101a08301905092915050565b5f602082019050919050565b5f61125d8261102f565b6112678185611039565b935061127283611049565b805f5b838110156112a2578151611289888261122f565b975061129483611247565b925050600181019050611275565b5085935050505092915050565b5f6020820190508181035f8301526112c78184611253565b905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f6020828403121561131157611310610e36565b5b5f61131e84828501610f3f565b91505092915050565b5f81519050919050565b5f81905092915050565b8281835e5f83830152505050565b5f61135382611327565b61135d8185611331565b935061136d81856020860161133b565b80840191505092915050565b5f6113848284611349565b915081905092915050565b5f819050919050565b6113a18161138f565b81146113ab575f80fd5b50565b5f815190506113bc81611398565b92915050565b5f602082840312156113d7576113d6610e36565b5b5f6113e4848285016113ae565b91505092915050565b6113f681611067565b8114611400575f80fd5b50565b5f81519050611411816113ed565b92915050565b5f6020828403121561142c5761142b610e36565b5b5f61143984828501611403565b91505092915050565b5f80fd5b5f67ffffffffffffffff8211156114605761145f610e52565b5b61146982610e42565b9050602081019050919050565b5f61148861148384611446565b610eb0565b9050828152602081018484840111156114a4576114a3611442565b5b6114af84828561133b565b509392505050565b5f82601f8301126114cb576114ca610e3e565b5b81516114db848260208601611476565b91505092915050565b5f602082840312156114f9576114f8610e36565b5b5f82015167ffffffffffffffff81111561151657611515610e3a565b5b611522848285016114b7565b91505092915050565b611534816110ee565b811461153e575f80fd5b50565b5f8151905061154f8161152b565b92915050565b5f6020828403121561156a57611569610e36565b5b5f61157784828501611541565b91505092915050565b61158981610ef9565b8114611593575f80fd5b50565b5f815190506115a481611580565b92915050565b6115b3816110d3565b81146115bd575f80fd5b50565b5f815190506115ce816115aa565b92915050565b5f61ffff82169050919050565b6115ea816115d4565b81146115f4575f80fd5b50565b5f81519050611605816115e1565b92915050565b6116148161107f565b811461161e575f80fd5b50565b5f8151905061162f8161160b565b92915050565b5f8115159050919050565b61164981611635565b8114611653575f80fd5b50565b5f8151905061166481611640565b92915050565b5f805f805f805f60e0888a03121561168557611684610e36565b5b5f6116928a828b01611596565b97505060206116a38a828b016115c0565b96505060406116b48a828b016115f7565b95505060606116c58a828b016115f7565b94505060806116d68a828b016115f7565b93505060a06116e78a828b01611621565b92505060c06116f88a828b01611656565b91505092959891949750929550565b611710816110d3565b82525050565b5f6020820190506117295f830184611707565b92915050565b6117388161109a565b8114611742575f80fd5b50565b5f815190506117538161172f565b92915050565b6117628161110b565b811461176c575f80fd5b50565b5f8151905061177d81611759565b92915050565b5f8160060b9050919050565b61179881611783565b81146117a2575f80fd5b50565b5f815190506117b38161178f565b92915050565b5f63ffffffff82169050919050565b6117d1816117b9565b81146117db575f80fd5b50565b5f815190506117ec816117c8565b92915050565b5f805f805f805f80610100898b03121561180f5761180e610e36565b5b5f61181c8b828c01611745565b985050602061182d8b828c0161176f565b975050604061183e8b828c016113ae565b965050606061184f8b828c016113ae565b95505060806118608b828c016117a5565b94505060a06118718b828c01611596565b93505060c06118828b828c016117de565b92505060e06118938b828c01611656565b9150509295985092959890939650565b5f602082840312156118b8576118b7610e36565b5b5f6118c584828501611745565b91505092915050565b5f602082840312156118e3576118e2610e36565b5b5f6118f0848285016115c0565b9150509291505056fe
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`@Qa\x18\xFA8\x03\x80a\x18\xFA\x839\x81\x81\x01`@R\x81\x01\x90a\x001\x91\x90a\x0F\xE8V[_a\0A\x82a\0p` \x1B` \x1CV[\x90P_\x81`@Q` \x01a\0U\x91\x90a\x12\xAFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P` \x81\x01\x80Y\x03\x81\xF3[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\x8EWa\0\x8Da\x0ERV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\xC7W\x81` \x01[a\0\xB4a\rNV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\0\xACW\x90P[P\x90P_[\x83Q\x81\x10\x15a\x0C\xEDW_\x84\x82\x81Q\x81\x10a\0\xE9Wa\0\xE8a\x12\xCFV[[` \x02` \x01\x01Q\x90Pa\x01\x02\x81a\x0C\xF7` \x1B` \x1CV[\x15a\x01\rWPa\x0C\xE2V[a\x01\x15a\rNV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01^W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x82\x91\x90a\x12\xFCV[\x81_\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD2\x12 \xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x01W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02%\x91\x90a\x12\xFCV[\x81``\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\x02n\x81_\x01Qa\x0C\xF7` \x1B` \x1CV[\x15a\x02zWPPa\x0C\xE2V[a\x02\x8D\x81``\x01Qa\x0C\xF7` \x1B` \x1CV[\x15a\x02\x99WPPa\x0C\xE2V[_\x80\x82_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x03H\x91\x90a\x13yV[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x03\x82W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x03\x87V[``\x91P[P\x91P\x91P\x81\x15a\x03\xF6W_` \x82Q\x03a\x03\xE6W\x81\x80` \x01\x90Q\x81\x01\x90a\x03\xB0\x91\x90a\x13\xC2V[\x90P_\x81\x14\x80a\x03\xC0WP`\xFF\x81\x11[\x15a\x03\xCFWPPPPPa\x0C\xE2V[\x80\x84`@\x01\x90`\xFF\x16\x90\x81`\xFF\x16\x81RPPa\x03\xF0V[PPPPPa\x0C\xE2V[Pa\x03\xFFV[PPPPa\x0C\xE2V[_\x80\x84``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x04\xAF\x91\x90a\x13yV[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x04\xE9W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x04\xEEV[``\x91P[P\x91P\x91P\x81\x15a\x05aW_` \x82Q\x03a\x05OW\x81\x80` \x01\x90Q\x81\x01\x90a\x05\x17\x91\x90a\x13\xC2V[\x90P_\x81\x14\x80a\x05'WP`\xFF\x81\x11[\x15a\x058WPPPPPPPa\x0C\xE2V[\x80\x86`\xA0\x01\x90`\xFF\x16\x90\x81`\xFF\x16\x81RPPa\x05[V[PPPPPPPa\x0C\xE2V[Pa\x05lV[PPPPPPa\x0C\xE2V[_\x80\x86_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F\x95\xD8\x9BA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x06\x1B\x91\x90a\x13yV[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x06UW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x06ZV[``\x91P[P\x91P\x91P\x81\x15a\x06\xC4W_` \x82Q\x03a\x06\x8AW\x81\x80` \x01\x90Q\x81\x01\x90a\x06\x83\x91\x90a\x14\x17V[\x90Pa\x06\xB4V[_\x82\x80` \x01\x90Q\x81\x01\x90a\x06\x9F\x91\x90a\x14\xE4V[\x90Pa\x06\xB0\x81a\r(` \x1B` \x1CV[\x91PP[\x80\x88` \x01\x81\x81RPPPa\x06\xD1V[_\x80\x1B\x87` \x01\x81\x81RPP[_\x80\x88``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F\x95\xD8\x9BA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x07\x81\x91\x90a\x13yV[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x07\xBBW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x07\xC0V[``\x91P[P\x91P\x91P\x81\x15a\x08*W_` \x82Q\x03a\x07\xF0W\x81\x80` \x01\x90Q\x81\x01\x90a\x07\xE9\x91\x90a\x14\x17V[\x90Pa\x08\x1AV[_\x82\x80` \x01\x90Q\x81\x01\x90a\x08\x05\x91\x90a\x14\xE4V[\x90Pa\x08\x16\x81a\r(` \x1B` \x1CV[\x91PP[\x80\x8A`\x80\x01\x81\x81RPPPa\x087V[_\x80\x1B\x89`\x80\x01\x81\x81RPP[\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC4Z\x01U`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x08\x9FWP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x9C\x91\x90a\x12\xFCV[`\x01[a\x08\xE0W_\x89`\xC0\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\t\x1AV[\x80\x8A`\xC0\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPP[\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xDD\xCA?C`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\t\x82WP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x7F\x91\x90a\x15UV[`\x01[a\t\xA4Wa\x0B\xB8\x89a\x01`\x01\x90b\xFF\xFF\xFF\x16\x90\x81b\xFF\xFF\xFF\x16\x81RPPa\t\xBDV[\x80\x8Aa\x01`\x01\x90b\xFF\xFF\xFF\x16\x90\x81b\xFF\xFF\xFF\x16\x81RPPP[_\x80\x8Bs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c8P\xC7\xBD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x08W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n,\x91\x90a\x16jV[PPPPP\x91P\x91P_\x8Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF3\r\xBA\x93\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\no\x91\x90a\x17\x16V[a\x01\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x8BW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xAF\x91\x90a\x17\xF2V[PPPPPP\x91PP\x8Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x1Ahe\x02`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x01W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B%\x91\x90a\x18\xA3V[\x8C`\xE0\x01\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x8Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x9DW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xC1\x91\x90a\x18\xCEV[\x8Ca\x01@\x01\x90`\x02\x0B\x90\x81`\x02\x0B\x81RPP\x8Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xDD\xCA?C`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x1CW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C@\x91\x90a\x15UV[\x8Ca\x01`\x01\x90b\xFF\xFF\xFF\x16\x90\x81b\xFF\xFF\xFF\x16\x81RPP\x82\x8Ca\x01\0\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81\x8Ca\x01 \x01\x90`\x02\x0B\x90\x81`\x02\x0B\x81RPP\x80\x8Ca\x01\x80\x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP\x8B\x8F\x8F\x81Q\x81\x10a\x0C\xC9Wa\x0C\xC8a\x12\xCFV[[` \x02` \x01\x01\x81\x90RPPPPPPPPPPPPPP[\x80`\x01\x01\x90Pa\0\xCCV[P\x80\x91PP\x91\x90PV[_\x80\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a\r\x1FW`\x01\x90Pa\r#V[_\x90P[\x91\x90PV[_\x80\x82\x90P_\x81Q\x03a\r@W_\x80\x1B\x91PPa\rIV[` \x83\x01Q\x91PP[\x91\x90PV[`@Q\x80a\x01\xA0\x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x80\x19\x16\x81R` \x01_`\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x80\x19\x16\x81R` \x01_`\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_`\x02\x0B\x81R` \x01_`\x02\x0B\x81R` \x01_b\xFF\xFF\xFF\x16\x81R` \x01_`\x0F\x0B\x81RP\x90V[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x0E\x88\x82a\x0EBV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0E\xA7Wa\x0E\xA6a\x0ERV[[\x80`@RPPPV[_a\x0E\xB9a\x0E-V[\x90Pa\x0E\xC5\x82\x82a\x0E\x7FV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0E\xE4Wa\x0E\xE3a\x0ERV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x80\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x0F\"\x82a\x0E\xF9V[\x90P\x91\x90PV[a\x0F2\x81a\x0F\x18V[\x81\x14a\x0F<W_\x80\xFD[PV[_\x81Q\x90Pa\x0FM\x81a\x0F)V[\x92\x91PPV[_a\x0Fea\x0F`\x84a\x0E\xCAV[a\x0E\xB0V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x0F\x88Wa\x0F\x87a\x0E\xF5V[[\x83[\x81\x81\x10\x15a\x0F\xB1W\x80a\x0F\x9D\x88\x82a\x0F?V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x0F\x8AV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x0F\xCFWa\x0F\xCEa\x0E>V[[\x81Qa\x0F\xDF\x84\x82` \x86\x01a\x0FSV[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0F\xFDWa\x0F\xFCa\x0E6V[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\x1AWa\x10\x19a\x0E:V[[a\x10&\x84\x82\x85\x01a\x0F\xBBV[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x10a\x81a\x0F\x18V[\x82RPPV[_\x81\x90P\x91\x90PV[a\x10y\x81a\x10gV[\x82RPPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\x10\x94\x81a\x10\x7FV[\x82RPPV[_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x10\xBE\x81a\x10\x9AV[\x82RPPV[a\x10\xCD\x81a\x0E\xF9V[\x82RPPV[_\x81`\x02\x0B\x90P\x91\x90PV[a\x10\xE8\x81a\x10\xD3V[\x82RPPV[_b\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x11\x05\x81a\x10\xEEV[\x82RPPV[_\x81`\x0F\x0B\x90P\x91\x90PV[a\x11 \x81a\x11\x0BV[\x82RPPV[a\x01\xA0\x82\x01_\x82\x01Qa\x11;_\x85\x01\x82a\x10XV[P` \x82\x01Qa\x11N` \x85\x01\x82a\x10pV[P`@\x82\x01Qa\x11a`@\x85\x01\x82a\x10\x8BV[P``\x82\x01Qa\x11t``\x85\x01\x82a\x10XV[P`\x80\x82\x01Qa\x11\x87`\x80\x85\x01\x82a\x10pV[P`\xA0\x82\x01Qa\x11\x9A`\xA0\x85\x01\x82a\x10\x8BV[P`\xC0\x82\x01Qa\x11\xAD`\xC0\x85\x01\x82a\x10XV[P`\xE0\x82\x01Qa\x11\xC0`\xE0\x85\x01\x82a\x10\xB5V[Pa\x01\0\x82\x01Qa\x11\xD5a\x01\0\x85\x01\x82a\x10\xC4V[Pa\x01 \x82\x01Qa\x11\xEAa\x01 \x85\x01\x82a\x10\xDFV[Pa\x01@\x82\x01Qa\x11\xFFa\x01@\x85\x01\x82a\x10\xDFV[Pa\x01`\x82\x01Qa\x12\x14a\x01`\x85\x01\x82a\x10\xFCV[Pa\x01\x80\x82\x01Qa\x12)a\x01\x80\x85\x01\x82a\x11\x17V[PPPPV[_a\x12:\x83\x83a\x11&V[a\x01\xA0\x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x12]\x82a\x10/V[a\x12g\x81\x85a\x109V[\x93Pa\x12r\x83a\x10IV[\x80_[\x83\x81\x10\x15a\x12\xA2W\x81Qa\x12\x89\x88\x82a\x12/V[\x97Pa\x12\x94\x83a\x12GV[\x92PP`\x01\x81\x01\x90Pa\x12uV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x12\xC7\x81\x84a\x12SV[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\x13\x11Wa\x13\x10a\x0E6V[[_a\x13\x1E\x84\x82\x85\x01a\x0F?V[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x13S\x82a\x13'V[a\x13]\x81\x85a\x131V[\x93Pa\x13m\x81\x85` \x86\x01a\x13;V[\x80\x84\x01\x91PP\x92\x91PPV[_a\x13\x84\x82\x84a\x13IV[\x91P\x81\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x13\xA1\x81a\x13\x8FV[\x81\x14a\x13\xABW_\x80\xFD[PV[_\x81Q\x90Pa\x13\xBC\x81a\x13\x98V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x13\xD7Wa\x13\xD6a\x0E6V[[_a\x13\xE4\x84\x82\x85\x01a\x13\xAEV[\x91PP\x92\x91PPV[a\x13\xF6\x81a\x10gV[\x81\x14a\x14\0W_\x80\xFD[PV[_\x81Q\x90Pa\x14\x11\x81a\x13\xEDV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x14,Wa\x14+a\x0E6V[[_a\x149\x84\x82\x85\x01a\x14\x03V[\x91PP\x92\x91PPV[_\x80\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x14`Wa\x14_a\x0ERV[[a\x14i\x82a\x0EBV[\x90P` \x81\x01\x90P\x91\x90PV[_a\x14\x88a\x14\x83\x84a\x14FV[a\x0E\xB0V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x14\xA4Wa\x14\xA3a\x14BV[[a\x14\xAF\x84\x82\x85a\x13;V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x14\xCBWa\x14\xCAa\x0E>V[[\x81Qa\x14\xDB\x84\x82` \x86\x01a\x14vV[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x14\xF9Wa\x14\xF8a\x0E6V[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\x16Wa\x15\x15a\x0E:V[[a\x15\"\x84\x82\x85\x01a\x14\xB7V[\x91PP\x92\x91PPV[a\x154\x81a\x10\xEEV[\x81\x14a\x15>W_\x80\xFD[PV[_\x81Q\x90Pa\x15O\x81a\x15+V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x15jWa\x15ia\x0E6V[[_a\x15w\x84\x82\x85\x01a\x15AV[\x91PP\x92\x91PPV[a\x15\x89\x81a\x0E\xF9V[\x81\x14a\x15\x93W_\x80\xFD[PV[_\x81Q\x90Pa\x15\xA4\x81a\x15\x80V[\x92\x91PPV[a\x15\xB3\x81a\x10\xD3V[\x81\x14a\x15\xBDW_\x80\xFD[PV[_\x81Q\x90Pa\x15\xCE\x81a\x15\xAAV[\x92\x91PPV[_a\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x15\xEA\x81a\x15\xD4V[\x81\x14a\x15\xF4W_\x80\xFD[PV[_\x81Q\x90Pa\x16\x05\x81a\x15\xE1V[\x92\x91PPV[a\x16\x14\x81a\x10\x7FV[\x81\x14a\x16\x1EW_\x80\xFD[PV[_\x81Q\x90Pa\x16/\x81a\x16\x0BV[\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a\x16I\x81a\x165V[\x81\x14a\x16SW_\x80\xFD[PV[_\x81Q\x90Pa\x16d\x81a\x16@V[\x92\x91PPV[_\x80_\x80_\x80_`\xE0\x88\x8A\x03\x12\x15a\x16\x85Wa\x16\x84a\x0E6V[[_a\x16\x92\x8A\x82\x8B\x01a\x15\x96V[\x97PP` a\x16\xA3\x8A\x82\x8B\x01a\x15\xC0V[\x96PP`@a\x16\xB4\x8A\x82\x8B\x01a\x15\xF7V[\x95PP``a\x16\xC5\x8A\x82\x8B\x01a\x15\xF7V[\x94PP`\x80a\x16\xD6\x8A\x82\x8B\x01a\x15\xF7V[\x93PP`\xA0a\x16\xE7\x8A\x82\x8B\x01a\x16!V[\x92PP`\xC0a\x16\xF8\x8A\x82\x8B\x01a\x16VV[\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[a\x17\x10\x81a\x10\xD3V[\x82RPPV[_` \x82\x01\x90Pa\x17)_\x83\x01\x84a\x17\x07V[\x92\x91PPV[a\x178\x81a\x10\x9AV[\x81\x14a\x17BW_\x80\xFD[PV[_\x81Q\x90Pa\x17S\x81a\x17/V[\x92\x91PPV[a\x17b\x81a\x11\x0BV[\x81\x14a\x17lW_\x80\xFD[PV[_\x81Q\x90Pa\x17}\x81a\x17YV[\x92\x91PPV[_\x81`\x06\x0B\x90P\x91\x90PV[a\x17\x98\x81a\x17\x83V[\x81\x14a\x17\xA2W_\x80\xFD[PV[_\x81Q\x90Pa\x17\xB3\x81a\x17\x8FV[\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x17\xD1\x81a\x17\xB9V[\x81\x14a\x17\xDBW_\x80\xFD[PV[_\x81Q\x90Pa\x17\xEC\x81a\x17\xC8V[\x92\x91PPV[_\x80_\x80_\x80_\x80a\x01\0\x89\x8B\x03\x12\x15a\x18\x0FWa\x18\x0Ea\x0E6V[[_a\x18\x1C\x8B\x82\x8C\x01a\x17EV[\x98PP` a\x18-\x8B\x82\x8C\x01a\x17oV[\x97PP`@a\x18>\x8B\x82\x8C\x01a\x13\xAEV[\x96PP``a\x18O\x8B\x82\x8C\x01a\x13\xAEV[\x95PP`\x80a\x18`\x8B\x82\x8C\x01a\x17\xA5V[\x94PP`\xA0a\x18q\x8B\x82\x8C\x01a\x15\x96V[\x93PP`\xC0a\x18\x82\x8B\x82\x8C\x01a\x17\xDEV[\x92PP`\xE0a\x18\x93\x8B\x82\x8C\x01a\x16VV[\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[_` \x82\x84\x03\x12\x15a\x18\xB8Wa\x18\xB7a\x0E6V[[_a\x18\xC5\x84\x82\x85\x01a\x17EV[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x18\xE3Wa\x18\xE2a\x0E6V[[_a\x18\xF0\x84\x82\x85\x01a\x15\xC0V[\x91PP\x92\x91PPV\xFE",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b5060043610610029575f3560e01c80639ae919bf1461002d575b5f80fd5b61004760048036038101906100429190610fb7565b61005d565b604051610054919061127e565b60405180910390f35b60605f825167ffffffffffffffff81111561007b5761007a610e21565b5b6040519080825280602002602001820160405280156100b457816020015b6100a1610d1d565b8152602001906001900390816100995790505b5090505f5b8351811015610cbc575f8482815181106100d6576100d561129e565b5b602002602001015190506100e981610cc6565b156100f45750610cb1565b6100fc610d1d565b8173ffffffffffffffffffffffffffffffffffffffff16630dfe16816040518163ffffffff1660e01b8152600401602060405180830381865afa158015610145573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061016991906112df565b815f019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff16815250508173ffffffffffffffffffffffffffffffffffffffff1663d21220a76040518163ffffffff1660e01b8152600401602060405180830381865afa1580156101e8573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061020c91906112df565b816060019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff168152505061024f815f0151610cc6565b1561025b575050610cb1565b6102688160600151610cc6565b15610274575050610cb1565b5f80825f015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f313ce567000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff8381831617835250505050604051610323919061135c565b5f604051808303815f8787f1925050503d805f811461035d576040519150601f19603f3d011682016040523d82523d5f602084013e610362565b606091505b509150915081156103d1575f60208251036103c1578180602001905181019061038b91906113a5565b90505f81148061039b575060ff81115b156103aa575050505050610cb1565b80846040019060ff16908160ff16815250506103cb565b5050505050610cb1565b506103da565b50505050610cb1565b5f80846060015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f313ce567000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff838183161783525050505060405161048a919061135c565b5f604051808303815f8787f1925050503d805f81146104c4576040519150601f19603f3d011682016040523d82523d5f602084013e6104c9565b606091505b5091509150811561053c575f602082510361052a57818060200190518101906104f291906113a5565b90505f811480610502575060ff81115b156105135750505050505050610cb1565b808660a0019060ff16908160ff1681525050610536565b50505050505050610cb1565b50610547565b505050505050610cb1565b5f80865f015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f95d89b41000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506040516105f6919061135c565b5f604051808303815f8787f1925050503d805f8114610630576040519150601f19603f3d011682016040523d82523d5f602084013e610635565b606091505b50915091508115610699575f6020825103610665578180602001905181019061065e91906113fa565b9050610689565b5f8280602001905181019061067a91906114c7565b905061068581610cf7565b9150505b80886020018181525050506106a6565b5f801b8760200181815250505b5f80886060015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f95d89b41000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff8381831617835250505050604051610756919061135c565b5f604051808303815f8787f1925050503d805f8114610790576040519150601f19603f3d011682016040523d82523d5f602084013e610795565b606091505b509150915081156107f9575f60208251036107c557818060200190518101906107be91906113fa565b90506107e9565b5f828060200190518101906107da91906114c7565b90506107e581610cf7565b9150505b808a608001818152505050610806565b5f801b8960800181815250505b8973ffffffffffffffffffffffffffffffffffffffff1663c45a01556040518163ffffffff1660e01b8152600401602060405180830381865afa92505050801561086e57506040513d601f19601f8201168201806040525081019061086b91906112df565b60015b6108af575f8960c0019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff16815250506108e9565b808a60c0019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff1681525050505b8973ffffffffffffffffffffffffffffffffffffffff1663ddca3f436040518163ffffffff1660e01b8152600401602060405180830381865afa92505050801561095157506040513d601f19601f8201168201806040525081019061094e9190611538565b60015b61097357610bb889610160019062ffffff16908162ffffff168152505061098c565b808a610160019062ffffff16908162ffffff1681525050505b5f808b73ffffffffffffffffffffffffffffffffffffffff16633850c7bd6040518163ffffffff1660e01b815260040160e060405180830381865afa1580156109d7573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109fb919061164d565b5050505050915091505f8c73ffffffffffffffffffffffffffffffffffffffff1663f30dba93836040518263ffffffff1660e01b8152600401610a3e91906116f9565b61010060405180830381865afa158015610a5a573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a7e91906117d5565b5050505050509150508c73ffffffffffffffffffffffffffffffffffffffff16631a6865026040518163ffffffff1660e01b8152600401602060405180830381865afa158015610ad0573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610af49190611886565b8c60e001906fffffffffffffffffffffffffffffffff1690816fffffffffffffffffffffffffffffffff16815250508c73ffffffffffffffffffffffffffffffffffffffff1663d0c93a7c6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610b6c573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b9091906118b1565b8c610140019060020b908160020b815250508c73ffffffffffffffffffffffffffffffffffffffff1663ddca3f436040518163ffffffff1660e01b8152600401602060405180830381865afa158015610beb573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c0f9190611538565b8c610160019062ffffff16908162ffffff1681525050828c610100019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff1681525050818c610120019060020b908160020b81525050808c6101800190600f0b9081600f0b815250508b8f8f81518110610c9857610c9761129e565b5b6020026020010181905250505050505050505050505050505b8060010190506100b9565b5080915050919050565b5f808273ffffffffffffffffffffffffffffffffffffffff163b03610cee5760019050610cf2565b5f90505b919050565b5f808290505f815103610d0f575f801b915050610d18565b60208301519150505b919050565b604051806101a001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f80191681526020015f60ff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f80191681526020015f60ff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f6fffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f60020b81526020015f60020b81526020015f62ffffff1681526020015f600f0b81525090565b5f604051905090565b5f80fd5b5f80fd5b5f80fd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b610e5782610e11565b810181811067ffffffffffffffff82111715610e7657610e75610e21565b5b80604052505050565b5f610e88610dfc565b9050610e948282610e4e565b919050565b5f67ffffffffffffffff821115610eb357610eb2610e21565b5b602082029050602081019050919050565b5f80fd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610ef182610ec8565b9050919050565b610f0181610ee7565b8114610f0b575f80fd5b50565b5f81359050610f1c81610ef8565b92915050565b5f610f34610f2f84610e99565b610e7f565b90508083825260208201905060208402830185811115610f5757610f56610ec4565b5b835b81811015610f805780610f6c8882610f0e565b845260208401935050602081019050610f59565b5050509392505050565b5f82601f830112610f9e57610f9d610e0d565b5b8135610fae848260208601610f22565b91505092915050565b5f60208284031215610fcc57610fcb610e05565b5b5f82013567ffffffffffffffff811115610fe957610fe8610e09565b5b610ff584828501610f8a565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b61103081610ee7565b82525050565b5f819050919050565b61104881611036565b82525050565b5f60ff82169050919050565b6110638161104e565b82525050565b5f6fffffffffffffffffffffffffffffffff82169050919050565b61108d81611069565b82525050565b61109c81610ec8565b82525050565b5f8160020b9050919050565b6110b7816110a2565b82525050565b5f62ffffff82169050919050565b6110d4816110bd565b82525050565b5f81600f0b9050919050565b6110ef816110da565b82525050565b6101a082015f82015161110a5f850182611027565b50602082015161111d602085018261103f565b506040820151611130604085018261105a565b5060608201516111436060850182611027565b506080820151611156608085018261103f565b5060a082015161116960a085018261105a565b5060c082015161117c60c0850182611027565b5060e082015161118f60e0850182611084565b506101008201516111a4610100850182611093565b506101208201516111b96101208501826110ae565b506101408201516111ce6101408501826110ae565b506101608201516111e36101608501826110cb565b506101808201516111f86101808501826110e6565b50505050565b5f61120983836110f5565b6101a08301905092915050565b5f602082019050919050565b5f61122c82610ffe565b6112368185611008565b935061124183611018565b805f5b8381101561127157815161125888826111fe565b975061126383611216565b925050600181019050611244565b5085935050505092915050565b5f6020820190508181035f8301526112968184611222565b905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f815190506112d981610ef8565b92915050565b5f602082840312156112f4576112f3610e05565b5b5f611301848285016112cb565b91505092915050565b5f81519050919050565b5f81905092915050565b8281835e5f83830152505050565b5f6113368261130a565b6113408185611314565b935061135081856020860161131e565b80840191505092915050565b5f611367828461132c565b915081905092915050565b5f819050919050565b61138481611372565b811461138e575f80fd5b50565b5f8151905061139f8161137b565b92915050565b5f602082840312156113ba576113b9610e05565b5b5f6113c784828501611391565b91505092915050565b6113d981611036565b81146113e3575f80fd5b50565b5f815190506113f4816113d0565b92915050565b5f6020828403121561140f5761140e610e05565b5b5f61141c848285016113e6565b91505092915050565b5f80fd5b5f67ffffffffffffffff82111561144357611442610e21565b5b61144c82610e11565b9050602081019050919050565b5f61146b61146684611429565b610e7f565b90508281526020810184848401111561148757611486611425565b5b61149284828561131e565b509392505050565b5f82601f8301126114ae576114ad610e0d565b5b81516114be848260208601611459565b91505092915050565b5f602082840312156114dc576114db610e05565b5b5f82015167ffffffffffffffff8111156114f9576114f8610e09565b5b6115058482850161149a565b91505092915050565b611517816110bd565b8114611521575f80fd5b50565b5f815190506115328161150e565b92915050565b5f6020828403121561154d5761154c610e05565b5b5f61155a84828501611524565b91505092915050565b61156c81610ec8565b8114611576575f80fd5b50565b5f8151905061158781611563565b92915050565b611596816110a2565b81146115a0575f80fd5b50565b5f815190506115b18161158d565b92915050565b5f61ffff82169050919050565b6115cd816115b7565b81146115d7575f80fd5b50565b5f815190506115e8816115c4565b92915050565b6115f78161104e565b8114611601575f80fd5b50565b5f81519050611612816115ee565b92915050565b5f8115159050919050565b61162c81611618565b8114611636575f80fd5b50565b5f8151905061164781611623565b92915050565b5f805f805f805f60e0888a03121561166857611667610e05565b5b5f6116758a828b01611579565b97505060206116868a828b016115a3565b96505060406116978a828b016115da565b95505060606116a88a828b016115da565b94505060806116b98a828b016115da565b93505060a06116ca8a828b01611604565b92505060c06116db8a828b01611639565b91505092959891949750929550565b6116f3816110a2565b82525050565b5f60208201905061170c5f8301846116ea565b92915050565b61171b81611069565b8114611725575f80fd5b50565b5f8151905061173681611712565b92915050565b611745816110da565b811461174f575f80fd5b50565b5f815190506117608161173c565b92915050565b5f8160060b9050919050565b61177b81611766565b8114611785575f80fd5b50565b5f8151905061179681611772565b92915050565b5f63ffffffff82169050919050565b6117b48161179c565b81146117be575f80fd5b50565b5f815190506117cf816117ab565b92915050565b5f805f805f805f80610100898b0312156117f2576117f1610e05565b5b5f6117ff8b828c01611728565b98505060206118108b828c01611752565b97505060406118218b828c01611391565b96505060606118328b828c01611391565b95505060806118438b828c01611788565b94505060a06118548b828c01611579565b93505060c06118658b828c016117c1565b92505060e06118768b828c01611639565b9150509295985092959890939650565b5f6020828403121561189b5761189a610e05565b5b5f6118a884828501611728565b91505092915050565b5f602082840312156118c6576118c5610e05565b5b5f6118d3848285016115a3565b9150509291505056fea264697066735822122023434c69221bef90686023e5633c22ca225793e9d4a9336bf077eea66b8fd02864736f6c634300081a0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0)W_5`\xE0\x1C\x80c\x9A\xE9\x19\xBF\x14a\0-W[_\x80\xFD[a\0G`\x04\x806\x03\x81\x01\x90a\0B\x91\x90a\x0F\xB7V[a\0]V[`@Qa\0T\x91\x90a\x12~V[`@Q\x80\x91\x03\x90\xF3[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0{Wa\0za\x0E!V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\xB4W\x81` \x01[a\0\xA1a\r\x1DV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\0\x99W\x90P[P\x90P_[\x83Q\x81\x10\x15a\x0C\xBCW_\x84\x82\x81Q\x81\x10a\0\xD6Wa\0\xD5a\x12\x9EV[[` \x02` \x01\x01Q\x90Pa\0\xE9\x81a\x0C\xC6V[\x15a\0\xF4WPa\x0C\xB1V[a\0\xFCa\r\x1DV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01EW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01i\x91\x90a\x12\xDFV[\x81_\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD2\x12 \xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xE8W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x0C\x91\x90a\x12\xDFV[\x81``\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\x02O\x81_\x01Qa\x0C\xC6V[\x15a\x02[WPPa\x0C\xB1V[a\x02h\x81``\x01Qa\x0C\xC6V[\x15a\x02tWPPa\x0C\xB1V[_\x80\x82_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x03#\x91\x90a\x13\\V[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x03]W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x03bV[``\x91P[P\x91P\x91P\x81\x15a\x03\xD1W_` \x82Q\x03a\x03\xC1W\x81\x80` \x01\x90Q\x81\x01\x90a\x03\x8B\x91\x90a\x13\xA5V[\x90P_\x81\x14\x80a\x03\x9BWP`\xFF\x81\x11[\x15a\x03\xAAWPPPPPa\x0C\xB1V[\x80\x84`@\x01\x90`\xFF\x16\x90\x81`\xFF\x16\x81RPPa\x03\xCBV[PPPPPa\x0C\xB1V[Pa\x03\xDAV[PPPPa\x0C\xB1V[_\x80\x84``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x04\x8A\x91\x90a\x13\\V[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x04\xC4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x04\xC9V[``\x91P[P\x91P\x91P\x81\x15a\x05<W_` \x82Q\x03a\x05*W\x81\x80` \x01\x90Q\x81\x01\x90a\x04\xF2\x91\x90a\x13\xA5V[\x90P_\x81\x14\x80a\x05\x02WP`\xFF\x81\x11[\x15a\x05\x13WPPPPPPPa\x0C\xB1V[\x80\x86`\xA0\x01\x90`\xFF\x16\x90\x81`\xFF\x16\x81RPPa\x056V[PPPPPPPa\x0C\xB1V[Pa\x05GV[PPPPPPa\x0C\xB1V[_\x80\x86_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F\x95\xD8\x9BA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x05\xF6\x91\x90a\x13\\V[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x060W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x065V[``\x91P[P\x91P\x91P\x81\x15a\x06\x99W_` \x82Q\x03a\x06eW\x81\x80` \x01\x90Q\x81\x01\x90a\x06^\x91\x90a\x13\xFAV[\x90Pa\x06\x89V[_\x82\x80` \x01\x90Q\x81\x01\x90a\x06z\x91\x90a\x14\xC7V[\x90Pa\x06\x85\x81a\x0C\xF7V[\x91PP[\x80\x88` \x01\x81\x81RPPPa\x06\xA6V[_\x80\x1B\x87` \x01\x81\x81RPP[_\x80\x88``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F\x95\xD8\x9BA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x07V\x91\x90a\x13\\V[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x07\x90W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x07\x95V[``\x91P[P\x91P\x91P\x81\x15a\x07\xF9W_` \x82Q\x03a\x07\xC5W\x81\x80` \x01\x90Q\x81\x01\x90a\x07\xBE\x91\x90a\x13\xFAV[\x90Pa\x07\xE9V[_\x82\x80` \x01\x90Q\x81\x01\x90a\x07\xDA\x91\x90a\x14\xC7V[\x90Pa\x07\xE5\x81a\x0C\xF7V[\x91PP[\x80\x8A`\x80\x01\x81\x81RPPPa\x08\x06V[_\x80\x1B\x89`\x80\x01\x81\x81RPP[\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC4Z\x01U`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x08nWP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08k\x91\x90a\x12\xDFV[`\x01[a\x08\xAFW_\x89`\xC0\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\x08\xE9V[\x80\x8A`\xC0\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPP[\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xDD\xCA?C`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\tQWP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tN\x91\x90a\x158V[`\x01[a\tsWa\x0B\xB8\x89a\x01`\x01\x90b\xFF\xFF\xFF\x16\x90\x81b\xFF\xFF\xFF\x16\x81RPPa\t\x8CV[\x80\x8Aa\x01`\x01\x90b\xFF\xFF\xFF\x16\x90\x81b\xFF\xFF\xFF\x16\x81RPPP[_\x80\x8Bs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c8P\xC7\xBD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xD7W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xFB\x91\x90a\x16MV[PPPPP\x91P\x91P_\x8Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF3\r\xBA\x93\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n>\x91\x90a\x16\xF9V[a\x01\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nZW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n~\x91\x90a\x17\xD5V[PPPPPP\x91PP\x8Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x1Ahe\x02`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xD0W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xF4\x91\x90a\x18\x86V[\x8C`\xE0\x01\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x8Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BlW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x90\x91\x90a\x18\xB1V[\x8Ca\x01@\x01\x90`\x02\x0B\x90\x81`\x02\x0B\x81RPP\x8Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xDD\xCA?C`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xEBW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x0F\x91\x90a\x158V[\x8Ca\x01`\x01\x90b\xFF\xFF\xFF\x16\x90\x81b\xFF\xFF\xFF\x16\x81RPP\x82\x8Ca\x01\0\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81\x8Ca\x01 \x01\x90`\x02\x0B\x90\x81`\x02\x0B\x81RPP\x80\x8Ca\x01\x80\x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP\x8B\x8F\x8F\x81Q\x81\x10a\x0C\x98Wa\x0C\x97a\x12\x9EV[[` \x02` \x01\x01\x81\x90RPPPPPPPPPPPPPP[\x80`\x01\x01\x90Pa\0\xB9V[P\x80\x91PP\x91\x90PV[_\x80\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a\x0C\xEEW`\x01\x90Pa\x0C\xF2V[_\x90P[\x91\x90PV[_\x80\x82\x90P_\x81Q\x03a\r\x0FW_\x80\x1B\x91PPa\r\x18V[` \x83\x01Q\x91PP[\x91\x90PV[`@Q\x80a\x01\xA0\x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x80\x19\x16\x81R` \x01_`\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x80\x19\x16\x81R` \x01_`\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_`\x02\x0B\x81R` \x01_`\x02\x0B\x81R` \x01_b\xFF\xFF\xFF\x16\x81R` \x01_`\x0F\x0B\x81RP\x90V[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x0EW\x82a\x0E\x11V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0EvWa\x0Eua\x0E!V[[\x80`@RPPPV[_a\x0E\x88a\r\xFCV[\x90Pa\x0E\x94\x82\x82a\x0ENV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0E\xB3Wa\x0E\xB2a\x0E!V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x80\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x0E\xF1\x82a\x0E\xC8V[\x90P\x91\x90PV[a\x0F\x01\x81a\x0E\xE7V[\x81\x14a\x0F\x0BW_\x80\xFD[PV[_\x815\x90Pa\x0F\x1C\x81a\x0E\xF8V[\x92\x91PPV[_a\x0F4a\x0F/\x84a\x0E\x99V[a\x0E\x7FV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x0FWWa\x0FVa\x0E\xC4V[[\x83[\x81\x81\x10\x15a\x0F\x80W\x80a\x0Fl\x88\x82a\x0F\x0EV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x0FYV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x0F\x9EWa\x0F\x9Da\x0E\rV[[\x815a\x0F\xAE\x84\x82` \x86\x01a\x0F\"V[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0F\xCCWa\x0F\xCBa\x0E\x05V[[_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\xE9Wa\x0F\xE8a\x0E\tV[[a\x0F\xF5\x84\x82\x85\x01a\x0F\x8AV[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x100\x81a\x0E\xE7V[\x82RPPV[_\x81\x90P\x91\x90PV[a\x10H\x81a\x106V[\x82RPPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\x10c\x81a\x10NV[\x82RPPV[_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x10\x8D\x81a\x10iV[\x82RPPV[a\x10\x9C\x81a\x0E\xC8V[\x82RPPV[_\x81`\x02\x0B\x90P\x91\x90PV[a\x10\xB7\x81a\x10\xA2V[\x82RPPV[_b\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x10\xD4\x81a\x10\xBDV[\x82RPPV[_\x81`\x0F\x0B\x90P\x91\x90PV[a\x10\xEF\x81a\x10\xDAV[\x82RPPV[a\x01\xA0\x82\x01_\x82\x01Qa\x11\n_\x85\x01\x82a\x10'V[P` \x82\x01Qa\x11\x1D` \x85\x01\x82a\x10?V[P`@\x82\x01Qa\x110`@\x85\x01\x82a\x10ZV[P``\x82\x01Qa\x11C``\x85\x01\x82a\x10'V[P`\x80\x82\x01Qa\x11V`\x80\x85\x01\x82a\x10?V[P`\xA0\x82\x01Qa\x11i`\xA0\x85\x01\x82a\x10ZV[P`\xC0\x82\x01Qa\x11|`\xC0\x85\x01\x82a\x10'V[P`\xE0\x82\x01Qa\x11\x8F`\xE0\x85\x01\x82a\x10\x84V[Pa\x01\0\x82\x01Qa\x11\xA4a\x01\0\x85\x01\x82a\x10\x93V[Pa\x01 \x82\x01Qa\x11\xB9a\x01 \x85\x01\x82a\x10\xAEV[Pa\x01@\x82\x01Qa\x11\xCEa\x01@\x85\x01\x82a\x10\xAEV[Pa\x01`\x82\x01Qa\x11\xE3a\x01`\x85\x01\x82a\x10\xCBV[Pa\x01\x80\x82\x01Qa\x11\xF8a\x01\x80\x85\x01\x82a\x10\xE6V[PPPPV[_a\x12\t\x83\x83a\x10\xF5V[a\x01\xA0\x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x12,\x82a\x0F\xFEV[a\x126\x81\x85a\x10\x08V[\x93Pa\x12A\x83a\x10\x18V[\x80_[\x83\x81\x10\x15a\x12qW\x81Qa\x12X\x88\x82a\x11\xFEV[\x97Pa\x12c\x83a\x12\x16V[\x92PP`\x01\x81\x01\x90Pa\x12DV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x12\x96\x81\x84a\x12\"V[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81Q\x90Pa\x12\xD9\x81a\x0E\xF8V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x12\xF4Wa\x12\xF3a\x0E\x05V[[_a\x13\x01\x84\x82\x85\x01a\x12\xCBV[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x136\x82a\x13\nV[a\x13@\x81\x85a\x13\x14V[\x93Pa\x13P\x81\x85` \x86\x01a\x13\x1EV[\x80\x84\x01\x91PP\x92\x91PPV[_a\x13g\x82\x84a\x13,V[\x91P\x81\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x13\x84\x81a\x13rV[\x81\x14a\x13\x8EW_\x80\xFD[PV[_\x81Q\x90Pa\x13\x9F\x81a\x13{V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x13\xBAWa\x13\xB9a\x0E\x05V[[_a\x13\xC7\x84\x82\x85\x01a\x13\x91V[\x91PP\x92\x91PPV[a\x13\xD9\x81a\x106V[\x81\x14a\x13\xE3W_\x80\xFD[PV[_\x81Q\x90Pa\x13\xF4\x81a\x13\xD0V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x14\x0FWa\x14\x0Ea\x0E\x05V[[_a\x14\x1C\x84\x82\x85\x01a\x13\xE6V[\x91PP\x92\x91PPV[_\x80\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x14CWa\x14Ba\x0E!V[[a\x14L\x82a\x0E\x11V[\x90P` \x81\x01\x90P\x91\x90PV[_a\x14ka\x14f\x84a\x14)V[a\x0E\x7FV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x14\x87Wa\x14\x86a\x14%V[[a\x14\x92\x84\x82\x85a\x13\x1EV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x14\xAEWa\x14\xADa\x0E\rV[[\x81Qa\x14\xBE\x84\x82` \x86\x01a\x14YV[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x14\xDCWa\x14\xDBa\x0E\x05V[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xF9Wa\x14\xF8a\x0E\tV[[a\x15\x05\x84\x82\x85\x01a\x14\x9AV[\x91PP\x92\x91PPV[a\x15\x17\x81a\x10\xBDV[\x81\x14a\x15!W_\x80\xFD[PV[_\x81Q\x90Pa\x152\x81a\x15\x0EV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x15MWa\x15La\x0E\x05V[[_a\x15Z\x84\x82\x85\x01a\x15$V[\x91PP\x92\x91PPV[a\x15l\x81a\x0E\xC8V[\x81\x14a\x15vW_\x80\xFD[PV[_\x81Q\x90Pa\x15\x87\x81a\x15cV[\x92\x91PPV[a\x15\x96\x81a\x10\xA2V[\x81\x14a\x15\xA0W_\x80\xFD[PV[_\x81Q\x90Pa\x15\xB1\x81a\x15\x8DV[\x92\x91PPV[_a\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x15\xCD\x81a\x15\xB7V[\x81\x14a\x15\xD7W_\x80\xFD[PV[_\x81Q\x90Pa\x15\xE8\x81a\x15\xC4V[\x92\x91PPV[a\x15\xF7\x81a\x10NV[\x81\x14a\x16\x01W_\x80\xFD[PV[_\x81Q\x90Pa\x16\x12\x81a\x15\xEEV[\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a\x16,\x81a\x16\x18V[\x81\x14a\x166W_\x80\xFD[PV[_\x81Q\x90Pa\x16G\x81a\x16#V[\x92\x91PPV[_\x80_\x80_\x80_`\xE0\x88\x8A\x03\x12\x15a\x16hWa\x16ga\x0E\x05V[[_a\x16u\x8A\x82\x8B\x01a\x15yV[\x97PP` a\x16\x86\x8A\x82\x8B\x01a\x15\xA3V[\x96PP`@a\x16\x97\x8A\x82\x8B\x01a\x15\xDAV[\x95PP``a\x16\xA8\x8A\x82\x8B\x01a\x15\xDAV[\x94PP`\x80a\x16\xB9\x8A\x82\x8B\x01a\x15\xDAV[\x93PP`\xA0a\x16\xCA\x8A\x82\x8B\x01a\x16\x04V[\x92PP`\xC0a\x16\xDB\x8A\x82\x8B\x01a\x169V[\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[a\x16\xF3\x81a\x10\xA2V[\x82RPPV[_` \x82\x01\x90Pa\x17\x0C_\x83\x01\x84a\x16\xEAV[\x92\x91PPV[a\x17\x1B\x81a\x10iV[\x81\x14a\x17%W_\x80\xFD[PV[_\x81Q\x90Pa\x176\x81a\x17\x12V[\x92\x91PPV[a\x17E\x81a\x10\xDAV[\x81\x14a\x17OW_\x80\xFD[PV[_\x81Q\x90Pa\x17`\x81a\x17<V[\x92\x91PPV[_\x81`\x06\x0B\x90P\x91\x90PV[a\x17{\x81a\x17fV[\x81\x14a\x17\x85W_\x80\xFD[PV[_\x81Q\x90Pa\x17\x96\x81a\x17rV[\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x17\xB4\x81a\x17\x9CV[\x81\x14a\x17\xBEW_\x80\xFD[PV[_\x81Q\x90Pa\x17\xCF\x81a\x17\xABV[\x92\x91PPV[_\x80_\x80_\x80_\x80a\x01\0\x89\x8B\x03\x12\x15a\x17\xF2Wa\x17\xF1a\x0E\x05V[[_a\x17\xFF\x8B\x82\x8C\x01a\x17(V[\x98PP` a\x18\x10\x8B\x82\x8C\x01a\x17RV[\x97PP`@a\x18!\x8B\x82\x8C\x01a\x13\x91V[\x96PP``a\x182\x8B\x82\x8C\x01a\x13\x91V[\x95PP`\x80a\x18C\x8B\x82\x8C\x01a\x17\x88V[\x94PP`\xA0a\x18T\x8B\x82\x8C\x01a\x15yV[\x93PP`\xC0a\x18e\x8B\x82\x8C\x01a\x17\xC1V[\x92PP`\xE0a\x18v\x8B\x82\x8C\x01a\x169V[\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[_` \x82\x84\x03\x12\x15a\x18\x9BWa\x18\x9Aa\x0E\x05V[[_a\x18\xA8\x84\x82\x85\x01a\x17(V[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x18\xC6Wa\x18\xC5a\x0E\x05V[[_a\x18\xD3\x84\x82\x85\x01a\x15\xA3V[\x91PP\x92\x91PPV\xFE\xA2dipfsX\"\x12 #CLi\"\x1B\xEF\x90h`#\xE5c<\"\xCA\"W\x93\xE9\xD4\xA93k\xF0w\xEE\xA6k\x8F\xD0(dsolcC\0\x08\x1A\x003",
    );
    /**Constructor`.
```solidity
constructor(address[] pools);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub pools: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value.pools,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { pools: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.pools),
                )
            }
        }
    };
    /**Function with signature `getPoolData(address[])` and selector `0x9ae919bf`.
```solidity
function getPoolData(address[] memory pools) external returns (PoolHelpers.UniswapV3PoolData[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPoolDataCall {
        #[allow(missing_docs)]
        pub pools: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    ///Container type for the return parameters of the [`getPoolData(address[])`](getPoolDataCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPoolDataReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            <PoolHelpers::UniswapV3PoolData as alloy::sol_types::SolType>::RustType,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getPoolDataCall> for UnderlyingRustTuple<'_> {
                fn from(value: getPoolDataCall) -> Self {
                    (value.pools,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPoolDataCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { pools: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<PoolHelpers::UniswapV3PoolData>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <PoolHelpers::UniswapV3PoolData as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getPoolDataReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getPoolDataReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPoolDataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPoolDataCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPoolDataReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<PoolHelpers::UniswapV3PoolData>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPoolData(address[])";
            const SELECTOR: [u8; 4] = [154u8, 233u8, 25u8, 191u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.pools),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`GetUniV3PoolData`](self) function calls.
    pub enum GetUniV3PoolDataCalls {
        #[allow(missing_docs)]
        getPoolData(getPoolDataCall),
    }
    #[automatically_derived]
    impl GetUniV3PoolDataCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[154u8, 233u8, 25u8, 191u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for GetUniV3PoolDataCalls {
        const NAME: &'static str = "GetUniV3PoolDataCalls";
        const MIN_DATA_LENGTH: usize = 64usize;
        const COUNT: usize = 1usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::getPoolData(_) => {
                    <getPoolDataCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<GetUniV3PoolDataCalls>] = &[
                {
                    fn getPoolData(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<GetUniV3PoolDataCalls> {
                        <getPoolDataCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(GetUniV3PoolDataCalls::getPoolData)
                    }
                    getPoolData
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::getPoolData(inner) => {
                    <getPoolDataCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::getPoolData(inner) => {
                    <getPoolDataCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`GetUniV3PoolData`](self) contract instance.

See the [wrapper's documentation](`GetUniV3PoolDataInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> GetUniV3PoolDataInstance<T, P, N> {
        GetUniV3PoolDataInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        pools: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<GetUniV3PoolDataInstance<T, P, N>>,
    > {
        GetUniV3PoolDataInstance::<T, P, N>::deploy(provider, pools)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        pools: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        GetUniV3PoolDataInstance::<T, P, N>::deploy_builder(provider, pools)
    }
    /**A [`GetUniV3PoolData`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`GetUniV3PoolData`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct GetUniV3PoolDataInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for GetUniV3PoolDataInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("GetUniV3PoolDataInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > GetUniV3PoolDataInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`GetUniV3PoolData`](self) contract instance.

See the [wrapper's documentation](`GetUniV3PoolDataInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
            pools: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::Result<GetUniV3PoolDataInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, pools);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            pools: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall { pools },
                    )[..],
                ]
                    .concat()
                    .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> GetUniV3PoolDataInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> GetUniV3PoolDataInstance<T, P, N> {
            GetUniV3PoolDataInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > GetUniV3PoolDataInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`getPoolData`] function.
        pub fn getPoolData(
            &self,
            pools: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPoolDataCall, N> {
            self.call_builder(&getPoolDataCall { pools })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > GetUniV3PoolDataInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
