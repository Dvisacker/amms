///Module containing a contract's types and functions.
/**

```solidity
library PoolHelpers {
    struct PopulatedTick { int24 tick; int128 liquidityNet; uint128 liquidityGross; uint256 feeGrowthOutside0X128; uint256 feeGrowthOutside1X128; }
    struct PopulatedTicks { PopulatedTick[] ticks; }
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
struct PopulatedTick { int24 tick; int128 liquidityNet; uint128 liquidityGross; uint256 feeGrowthOutside0X128; uint256 feeGrowthOutside1X128; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PopulatedTick {
        #[allow(missing_docs)]
        pub tick: alloy::sol_types::private::primitives::aliases::I24,
        #[allow(missing_docs)]
        pub liquidityNet: i128,
        #[allow(missing_docs)]
        pub liquidityGross: u128,
        #[allow(missing_docs)]
        pub feeGrowthOutside0X128: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub feeGrowthOutside1X128: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Int<24>,
            alloy::sol_types::sol_data::Int<128>,
            alloy::sol_types::sol_data::Uint<128>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::I24,
            i128,
            u128,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<PopulatedTick> for UnderlyingRustTuple<'_> {
            fn from(value: PopulatedTick) -> Self {
                (
                    value.tick,
                    value.liquidityNet,
                    value.liquidityGross,
                    value.feeGrowthOutside0X128,
                    value.feeGrowthOutside1X128,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PopulatedTick {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    tick: tuple.0,
                    liquidityNet: tuple.1,
                    liquidityGross: tuple.2,
                    feeGrowthOutside0X128: tuple.3,
                    feeGrowthOutside1X128: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for PopulatedTick {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for PopulatedTick {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tick),
                    <alloy::sol_types::sol_data::Int<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidityNet),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidityGross),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.feeGrowthOutside0X128,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.feeGrowthOutside1X128),
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
        impl alloy_sol_types::SolType for PopulatedTick {
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
        impl alloy_sol_types::SolStruct for PopulatedTick {
            const NAME: &'static str = "PopulatedTick";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "PopulatedTick(int24 tick,int128 liquidityNet,uint128 liquidityGross,uint256 feeGrowthOutside0X128,uint256 feeGrowthOutside1X128)",
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
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.tick)
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        128,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.liquidityNet)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.liquidityGross,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.feeGrowthOutside0X128,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.feeGrowthOutside1X128,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for PopulatedTick {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.tick)
                    + <alloy::sol_types::sol_data::Int<
                        128,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.liquidityNet,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.liquidityGross,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.feeGrowthOutside0X128,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.feeGrowthOutside1X128,
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
                <alloy::sol_types::sol_data::Int<
                    24,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tick,
                    out,
                );
                <alloy::sol_types::sol_data::Int<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.liquidityNet,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.liquidityGross,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.feeGrowthOutside0X128,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.feeGrowthOutside1X128,
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
    /**```solidity
struct PopulatedTicks { PopulatedTick[] ticks; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PopulatedTicks {
        #[allow(missing_docs)]
        pub ticks: alloy::sol_types::private::Vec<
            <PopulatedTick as alloy::sol_types::SolType>::RustType,
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
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Array<PopulatedTick>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<
                <PopulatedTick as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<PopulatedTicks> for UnderlyingRustTuple<'_> {
            fn from(value: PopulatedTicks) -> Self {
                (value.ticks,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PopulatedTicks {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { ticks: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for PopulatedTicks {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for PopulatedTicks {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        PopulatedTick,
                    > as alloy_sol_types::SolType>::tokenize(&self.ticks),
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
        impl alloy_sol_types::SolType for PopulatedTicks {
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
        impl alloy_sol_types::SolStruct for PopulatedTicks {
            const NAME: &'static str = "PopulatedTicks";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "PopulatedTicks(PopulatedTick[] ticks)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <PopulatedTick as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <PopulatedTick as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                <alloy::sol_types::sol_data::Array<
                    PopulatedTick,
                > as alloy_sol_types::SolType>::eip712_data_word(&self.ticks)
                    .0
                    .to_vec()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for PopulatedTicks {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        PopulatedTick,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.ticks)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Array<
                    PopulatedTick,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.ticks,
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
    struct PopulatedTick {
        int24 tick;
        int128 liquidityNet;
        uint128 liquidityGross;
        uint256 feeGrowthOutside0X128;
        uint256 feeGrowthOutside1X128;
    }
    struct PopulatedTicks {
        PopulatedTick[] ticks;
    }
}

interface GetCLPoolTicksInRange {
    type DEX is uint8;
    type V3PoolCallee is address;

    constructor(DEX dex, V3PoolCallee pool, int24 tickLower, int24 tickUpper) payable;

    function getPopulatedTicksInRange(DEX dex, V3PoolCallee pool, int24 tickLower, int24 tickUpper) external payable returns (PoolHelpers.PopulatedTicks memory populatedTicks);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "dex",
        "type": "uint8",
        "internalType": "enum DEX"
      },
      {
        "name": "pool",
        "type": "address",
        "internalType": "V3PoolCallee"
      },
      {
        "name": "tickLower",
        "type": "int24",
        "internalType": "int24"
      },
      {
        "name": "tickUpper",
        "type": "int24",
        "internalType": "int24"
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "getPopulatedTicksInRange",
    "inputs": [
      {
        "name": "dex",
        "type": "uint8",
        "internalType": "enum DEX"
      },
      {
        "name": "pool",
        "type": "address",
        "internalType": "V3PoolCallee"
      },
      {
        "name": "tickLower",
        "type": "int24",
        "internalType": "int24"
      },
      {
        "name": "tickUpper",
        "type": "int24",
        "internalType": "int24"
      }
    ],
    "outputs": [
      {
        "name": "populatedTicks",
        "type": "tuple",
        "internalType": "struct PoolHelpers.PopulatedTicks",
        "components": [
          {
            "name": "ticks",
            "type": "tuple[]",
            "internalType": "struct PoolHelpers.PopulatedTick[]",
            "components": [
              {
                "name": "tick",
                "type": "int24",
                "internalType": "int24"
              },
              {
                "name": "liquidityNet",
                "type": "int128",
                "internalType": "int128"
              },
              {
                "name": "liquidityGross",
                "type": "uint128",
                "internalType": "uint128"
              },
              {
                "name": "feeGrowthOutside0X128",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "feeGrowthOutside1X128",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          }
        ]
      }
    ],
    "stateMutability": "payable"
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
pub mod GetCLPoolTicksInRange {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052604051610e24380380610e24833981810160405281019061002591906107b8565b5f6100388585858561006560201b60201c565b90505f8160405160200161004c91906109bd565b6040516020818303038152906040529050805160208201fd5b61006d610639565b8160020b8360020b131561007f575f80fd5b5f8473ffffffffffffffffffffffffffffffffffffffff1663d0c93a7c6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156100c9573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906100ed91906109dd565b90505f806101028686856101df60201b60201c565b915091505f8061011989858561022360201b60201c565b915091508067ffffffffffffffff81111561013757610136610a08565b5b60405190808252806020026020018201604052801561017057816020015b61015d61064c565b8152602001906001900390816101555790505b50865f01819052505f808590505b8460010b8160010b136101d0576101c38c8c838a888b870361ffff16815181106101ab576101aa610a35565b5b60200260200101518d5f01518861033060201b60201c565b915080600101905061017e565b50505050505050949350505050565b5f805f6101f286856103a360201b60201c565b905060088160020b901d925061020e85856103a360201b60201c565b905060088160020b901d915050935093915050565b60605f600184846102349190610a9b565b61023e9190610af4565b61ffff1667ffffffffffffffff81111561025b5761025a610a08565b5b6040519080825280602002602001820160405280156102895781602001602082028036833780820191505090505b5091505f8490505b8360010b8160010b13610327575f6102c8828873ffffffffffffffffffffffffffffffffffffffff166103b560201b90919060201c565b9050808487846102d89190610a9b565b61ffff16815181106102ed576102ec610a35565b5b602002602001018181525050610308816103e360201b60201c565b836103139190610b4d565b9250508061032090610b80565b9050610291565b50935093915050565b5f805b610100811015610394575f816001901b861614610389575f818860081b01870290506103878a8a8388888060010199508151811061037457610373610a35565b5b602002602001015161042c60201b60201c565b505b806001019050610333565b50819050979650505050505050565b5f808284071282840503905092915050565b5f808260010b90506103d884635339c29660e01b835f60206105e460201b60201c565b5f5191505092915050565b5f8019808314600382048460011c1684039350600582048460021c16600583048516019350601182048460041c850116935060ff8204840260f81c8160081b1792505050919050565b81815f019060020b908160020b815250506002808111156104505761044f610ba9565b5b84600281111561046357610462610ba9565b5b0361054b578273ffffffffffffffffffffffffffffffffffffffff1663f30dba93836040518263ffffffff1660e01b81526004016104a19190610be5565b61014060405180830381865afa1580156104bd573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906104e19190610d4a565b909192939495965090919293509091925090915090505084604001856020018660600187608001848152508481525084600f0b600f0b815250846fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff16815250505050506105de565b5f610575838573ffffffffffffffffffffffffffffffffffffffff1661060260201b90919060201c565b905080602001518260200190600f0b9081600f0b81525050805f015182604001906fffffffffffffffffffffffffffffffff1690816fffffffffffffffffffffffffffffffff168152505080604001518260600181815250508060600151826080018181525050505b50505050565b835f5282600452808260245f885afa6105fb575f80fd5b5050505050565b61060a61068e565b5f808360020b91508290506106318563f30dba9360e01b84846101006105e460201b60201c565b505092915050565b6040518060200160405280606081525090565b6040518060a001604052805f60020b81526020015f600f0b81526020015f6fffffffffffffffffffffffffffffffff1681526020015f81526020015f81525090565b6040518061010001604052805f6fffffffffffffffffffffffffffffffff1681526020015f600f0b81526020015f81526020015f81526020015f60060b81526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f63ffffffff1681526020015f151581525090565b5f80fd5b60038110610711575f80fd5b50565b5f8151905061072281610705565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61075182610728565b9050919050565b61076181610747565b811461076b575f80fd5b50565b5f8151905061077c81610758565b92915050565b5f8160020b9050919050565b61079781610782565b81146107a1575f80fd5b50565b5f815190506107b28161078e565b92915050565b5f805f80608085870312156107d0576107cf610701565b5b5f6107dd87828801610714565b94505060206107ee8782880161076e565b93505060406107ff878288016107a4565b9250506060610810878288016107a4565b91505092959194509250565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b61084e81610782565b82525050565b5f81600f0b9050919050565b61086981610854565b82525050565b5f6fffffffffffffffffffffffffffffffff82169050919050565b6108938161086f565b82525050565b5f819050919050565b6108ab81610899565b82525050565b60a082015f8201516108c55f850182610845565b5060208201516108d86020850182610860565b5060408201516108eb604085018261088a565b5060608201516108fe60608501826108a2565b50608082015161091160808501826108a2565b50505050565b5f61092283836108b1565b60a08301905092915050565b5f602082019050919050565b5f6109448261081c565b61094e8185610826565b935061095983610836565b805f5b838110156109895781516109708882610917565b975061097b8361092e565b92505060018101905061095c565b5085935050505092915050565b5f602083015f8301518482035f8601526109b0828261093a565b9150508091505092915050565b5f6020820190508181035f8301526109d58184610996565b905092915050565b5f602082840312156109f2576109f1610701565b5b5f6109ff848285016107a4565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f8160010b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f610aa582610a62565b9150610ab083610a62565b92508282039050617fff81137fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff800082121715610aee57610aed610a6e565b5b92915050565b5f610afe82610a62565b9150610b0983610a62565b925082820190507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80008112617fff82131715610b4757610b46610a6e565b5b92915050565b5f610b5782610899565b9150610b6283610899565b9250828201905080821115610b7a57610b79610a6e565b5b92915050565b5f610b8a82610a62565b9150617fff8203610b9e57610b9d610a6e565b5b600182019050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b610bdf81610782565b82525050565b5f602082019050610bf85f830184610bd6565b92915050565b610c078161086f565b8114610c11575f80fd5b50565b5f81519050610c2281610bfe565b92915050565b610c3181610854565b8114610c3b575f80fd5b50565b5f81519050610c4c81610c28565b92915050565b610c5b81610899565b8114610c65575f80fd5b50565b5f81519050610c7681610c52565b92915050565b5f8160060b9050919050565b610c9181610c7c565b8114610c9b575f80fd5b50565b5f81519050610cac81610c88565b92915050565b610cbb81610728565b8114610cc5575f80fd5b50565b5f81519050610cd681610cb2565b92915050565b5f63ffffffff82169050919050565b610cf481610cdc565b8114610cfe575f80fd5b50565b5f81519050610d0f81610ceb565b92915050565b5f8115159050919050565b610d2981610d15565b8114610d33575f80fd5b50565b5f81519050610d4481610d20565b92915050565b5f805f805f805f805f806101408b8d031215610d6957610d68610701565b5b5f610d768d828e01610c14565b9a50506020610d878d828e01610c3e565b9950506040610d988d828e01610c3e565b9850506060610da98d828e01610c68565b9750506080610dba8d828e01610c68565b96505060a0610dcb8d828e01610c68565b95505060c0610ddc8d828e01610c9e565b94505060e0610ded8d828e01610cc8565b935050610100610dff8d828e01610d01565b925050610120610e118d828e01610d36565b9150509295989b9194979a509295985056fe
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`@Qa\x0E$8\x03\x80a\x0E$\x839\x81\x81\x01`@R\x81\x01\x90a\0%\x91\x90a\x07\xB8V[_a\08\x85\x85\x85\x85a\0e` \x1B` \x1CV[\x90P_\x81`@Q` \x01a\0L\x91\x90a\t\xBDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80Q` \x82\x01\xFD[a\0ma\x069V[\x81`\x02\x0B\x83`\x02\x0B\x13\x15a\0\x7FW_\x80\xFD[_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xC9W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xED\x91\x90a\t\xDDV[\x90P_\x80a\x01\x02\x86\x86\x85a\x01\xDF` \x1B` \x1CV[\x91P\x91P_\x80a\x01\x19\x89\x85\x85a\x02#` \x1B` \x1CV[\x91P\x91P\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x017Wa\x016a\n\x08V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01pW\x81` \x01[a\x01]a\x06LV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x01UW\x90P[P\x86_\x01\x81\x90RP_\x80\x85\x90P[\x84`\x01\x0B\x81`\x01\x0B\x13a\x01\xD0Wa\x01\xC3\x8C\x8C\x83\x8A\x88\x8B\x87\x03a\xFF\xFF\x16\x81Q\x81\x10a\x01\xABWa\x01\xAAa\n5V[[` \x02` \x01\x01Q\x8D_\x01Q\x88a\x030` \x1B` \x1CV[\x91P\x80`\x01\x01\x90Pa\x01~V[PPPPPPP\x94\x93PPPPV[_\x80_a\x01\xF2\x86\x85a\x03\xA3` \x1B` \x1CV[\x90P`\x08\x81`\x02\x0B\x90\x1D\x92Pa\x02\x0E\x85\x85a\x03\xA3` \x1B` \x1CV[\x90P`\x08\x81`\x02\x0B\x90\x1D\x91PP\x93P\x93\x91PPV[``_`\x01\x84\x84a\x024\x91\x90a\n\x9BV[a\x02>\x91\x90a\n\xF4V[a\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02[Wa\x02Za\n\x08V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02\x89W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x91P_\x84\x90P[\x83`\x01\x0B\x81`\x01\x0B\x13a\x03'W_a\x02\xC8\x82\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03\xB5` \x1B\x90\x91\x90` \x1CV[\x90P\x80\x84\x87\x84a\x02\xD8\x91\x90a\n\x9BV[a\xFF\xFF\x16\x81Q\x81\x10a\x02\xEDWa\x02\xECa\n5V[[` \x02` \x01\x01\x81\x81RPPa\x03\x08\x81a\x03\xE3` \x1B` \x1CV[\x83a\x03\x13\x91\x90a\x0BMV[\x92PP\x80a\x03 \x90a\x0B\x80V[\x90Pa\x02\x91V[P\x93P\x93\x91PPV[_\x80[a\x01\0\x81\x10\x15a\x03\x94W_\x81`\x01\x90\x1B\x86\x16\x14a\x03\x89W_\x81\x88`\x08\x1B\x01\x87\x02\x90Pa\x03\x87\x8A\x8A\x83\x88\x88\x80`\x01\x01\x99P\x81Q\x81\x10a\x03tWa\x03sa\n5V[[` \x02` \x01\x01Qa\x04,` \x1B` \x1CV[P[\x80`\x01\x01\x90Pa\x033V[P\x81\x90P\x97\x96PPPPPPPV[_\x80\x82\x84\x07\x12\x82\x84\x05\x03\x90P\x92\x91PPV[_\x80\x82`\x01\x0B\x90Pa\x03\xD8\x84cS9\xC2\x96`\xE0\x1B\x83_` a\x05\xE4` \x1B` \x1CV[_Q\x91PP\x92\x91PPV[_\x80\x19\x80\x83\x14`\x03\x82\x04\x84`\x01\x1C\x16\x84\x03\x93P`\x05\x82\x04\x84`\x02\x1C\x16`\x05\x83\x04\x85\x16\x01\x93P`\x11\x82\x04\x84`\x04\x1C\x85\x01\x16\x93P`\xFF\x82\x04\x84\x02`\xF8\x1C\x81`\x08\x1B\x17\x92PPP\x91\x90PV[\x81\x81_\x01\x90`\x02\x0B\x90\x81`\x02\x0B\x81RPP`\x02\x80\x81\x11\x15a\x04PWa\x04Oa\x0B\xA9V[[\x84`\x02\x81\x11\x15a\x04cWa\x04ba\x0B\xA9V[[\x03a\x05KW\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF3\r\xBA\x93\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xA1\x91\x90a\x0B\xE5V[a\x01@`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xBDW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xE1\x91\x90a\rJV[\x90\x91\x92\x93\x94\x95\x96P\x90\x91\x92\x93P\x90\x91\x92P\x90\x91P\x90PP\x84`@\x01\x85` \x01\x86``\x01\x87`\x80\x01\x84\x81RP\x84\x81RP\x84`\x0F\x0B`\x0F\x0B\x81RP\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPPPPa\x05\xDEV[_a\x05u\x83\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x06\x02` \x1B\x90\x91\x90` \x1CV[\x90P\x80` \x01Q\x82` \x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP\x80_\x01Q\x82`@\x01\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x80`@\x01Q\x82``\x01\x81\x81RPP\x80``\x01Q\x82`\x80\x01\x81\x81RPPP[PPPPV[\x83_R\x82`\x04R\x80\x82`$_\x88Z\xFAa\x05\xFBW_\x80\xFD[PPPPPV[a\x06\na\x06\x8EV[_\x80\x83`\x02\x0B\x91P\x82\x90Pa\x061\x85c\xF3\r\xBA\x93`\xE0\x1B\x84\x84a\x01\0a\x05\xE4` \x1B` \x1CV[PP\x92\x91PPV[`@Q\x80` \x01`@R\x80``\x81RP\x90V[`@Q\x80`\xA0\x01`@R\x80_`\x02\x0B\x81R` \x01_`\x0F\x0B\x81R` \x01_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01\0\x01`@R\x80_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_`\x0F\x0B\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x06\x0B\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x15\x15\x81RP\x90V[_\x80\xFD[`\x03\x81\x10a\x07\x11W_\x80\xFD[PV[_\x81Q\x90Pa\x07\"\x81a\x07\x05V[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x07Q\x82a\x07(V[\x90P\x91\x90PV[a\x07a\x81a\x07GV[\x81\x14a\x07kW_\x80\xFD[PV[_\x81Q\x90Pa\x07|\x81a\x07XV[\x92\x91PPV[_\x81`\x02\x0B\x90P\x91\x90PV[a\x07\x97\x81a\x07\x82V[\x81\x14a\x07\xA1W_\x80\xFD[PV[_\x81Q\x90Pa\x07\xB2\x81a\x07\x8EV[\x92\x91PPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a\x07\xD0Wa\x07\xCFa\x07\x01V[[_a\x07\xDD\x87\x82\x88\x01a\x07\x14V[\x94PP` a\x07\xEE\x87\x82\x88\x01a\x07nV[\x93PP`@a\x07\xFF\x87\x82\x88\x01a\x07\xA4V[\x92PP``a\x08\x10\x87\x82\x88\x01a\x07\xA4V[\x91PP\x92\x95\x91\x94P\x92PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x08N\x81a\x07\x82V[\x82RPPV[_\x81`\x0F\x0B\x90P\x91\x90PV[a\x08i\x81a\x08TV[\x82RPPV[_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x08\x93\x81a\x08oV[\x82RPPV[_\x81\x90P\x91\x90PV[a\x08\xAB\x81a\x08\x99V[\x82RPPV[`\xA0\x82\x01_\x82\x01Qa\x08\xC5_\x85\x01\x82a\x08EV[P` \x82\x01Qa\x08\xD8` \x85\x01\x82a\x08`V[P`@\x82\x01Qa\x08\xEB`@\x85\x01\x82a\x08\x8AV[P``\x82\x01Qa\x08\xFE``\x85\x01\x82a\x08\xA2V[P`\x80\x82\x01Qa\t\x11`\x80\x85\x01\x82a\x08\xA2V[PPPPV[_a\t\"\x83\x83a\x08\xB1V[`\xA0\x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\tD\x82a\x08\x1CV[a\tN\x81\x85a\x08&V[\x93Pa\tY\x83a\x086V[\x80_[\x83\x81\x10\x15a\t\x89W\x81Qa\tp\x88\x82a\t\x17V[\x97Pa\t{\x83a\t.V[\x92PP`\x01\x81\x01\x90Pa\t\\V[P\x85\x93PPPP\x92\x91PPV[_` \x83\x01_\x83\x01Q\x84\x82\x03_\x86\x01Ra\t\xB0\x82\x82a\t:V[\x91PP\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\t\xD5\x81\x84a\t\x96V[\x90P\x92\x91PPV[_` \x82\x84\x03\x12\x15a\t\xF2Wa\t\xF1a\x07\x01V[[_a\t\xFF\x84\x82\x85\x01a\x07\xA4V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81`\x01\x0B\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\n\xA5\x82a\nbV[\x91Pa\n\xB0\x83a\nbV[\x92P\x82\x82\x03\x90Pa\x7F\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\x82\x12\x17\x15a\n\xEEWa\n\xEDa\nnV[[\x92\x91PPV[_a\n\xFE\x82a\nbV[\x91Pa\x0B\t\x83a\nbV[\x92P\x82\x82\x01\x90P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\x81\x12a\x7F\xFF\x82\x13\x17\x15a\x0BGWa\x0BFa\nnV[[\x92\x91PPV[_a\x0BW\x82a\x08\x99V[\x91Pa\x0Bb\x83a\x08\x99V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x0BzWa\x0Bya\nnV[[\x92\x91PPV[_a\x0B\x8A\x82a\nbV[\x91Pa\x7F\xFF\x82\x03a\x0B\x9EWa\x0B\x9Da\nnV[[`\x01\x82\x01\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[a\x0B\xDF\x81a\x07\x82V[\x82RPPV[_` \x82\x01\x90Pa\x0B\xF8_\x83\x01\x84a\x0B\xD6V[\x92\x91PPV[a\x0C\x07\x81a\x08oV[\x81\x14a\x0C\x11W_\x80\xFD[PV[_\x81Q\x90Pa\x0C\"\x81a\x0B\xFEV[\x92\x91PPV[a\x0C1\x81a\x08TV[\x81\x14a\x0C;W_\x80\xFD[PV[_\x81Q\x90Pa\x0CL\x81a\x0C(V[\x92\x91PPV[a\x0C[\x81a\x08\x99V[\x81\x14a\x0CeW_\x80\xFD[PV[_\x81Q\x90Pa\x0Cv\x81a\x0CRV[\x92\x91PPV[_\x81`\x06\x0B\x90P\x91\x90PV[a\x0C\x91\x81a\x0C|V[\x81\x14a\x0C\x9BW_\x80\xFD[PV[_\x81Q\x90Pa\x0C\xAC\x81a\x0C\x88V[\x92\x91PPV[a\x0C\xBB\x81a\x07(V[\x81\x14a\x0C\xC5W_\x80\xFD[PV[_\x81Q\x90Pa\x0C\xD6\x81a\x0C\xB2V[\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x0C\xF4\x81a\x0C\xDCV[\x81\x14a\x0C\xFEW_\x80\xFD[PV[_\x81Q\x90Pa\r\x0F\x81a\x0C\xEBV[\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a\r)\x81a\r\x15V[\x81\x14a\r3W_\x80\xFD[PV[_\x81Q\x90Pa\rD\x81a\r V[\x92\x91PPV[_\x80_\x80_\x80_\x80_\x80a\x01@\x8B\x8D\x03\x12\x15a\riWa\rha\x07\x01V[[_a\rv\x8D\x82\x8E\x01a\x0C\x14V[\x9APP` a\r\x87\x8D\x82\x8E\x01a\x0C>V[\x99PP`@a\r\x98\x8D\x82\x8E\x01a\x0C>V[\x98PP``a\r\xA9\x8D\x82\x8E\x01a\x0ChV[\x97PP`\x80a\r\xBA\x8D\x82\x8E\x01a\x0ChV[\x96PP`\xA0a\r\xCB\x8D\x82\x8E\x01a\x0ChV[\x95PP`\xC0a\r\xDC\x8D\x82\x8E\x01a\x0C\x9EV[\x94PP`\xE0a\r\xED\x8D\x82\x8E\x01a\x0C\xC8V[\x93PPa\x01\0a\r\xFF\x8D\x82\x8E\x01a\r\x01V[\x92PPa\x01 a\x0E\x11\x8D\x82\x8E\x01a\r6V[\x91PP\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV\xFE",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361061001d575f3560e01c8063fa5b11d414610021575b5f80fd5b61003b6004803603810190610036919061076e565b610051565b6040516100489190610973565b60405180910390f35b6100596105ef565b8160020b8360020b131561006b575f80fd5b5f8473ffffffffffffffffffffffffffffffffffffffff1663d0c93a7c6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156100b5573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906100d991906109a7565b90505f806100e88686856101b9565b915091505f806100f98985856101f1565b915091508067ffffffffffffffff811115610117576101166109d2565b5b60405190808252806020026020018201604052801561015057816020015b61013d610602565b8152602001906001900390816101355790505b50865f01819052505f808590505b8460010b8160010b136101aa5761019d8c8c838a888b870361ffff168151811061018b5761018a6109ff565b5b60200260200101518d5f0151886102f8565b915080600101905061015e565b50505050505050949350505050565b5f805f6101c68685610365565b905060088160020b901d92506101dc8585610365565b905060088160020b901d915050935093915050565b60605f600184846102029190610a65565b61020c9190610abe565b61ffff1667ffffffffffffffff811115610229576102286109d2565b5b6040519080825280602002602001820160405280156102575781602001602082028036833780820191505090505b5091505f8490505b8360010b8160010b136102ef575f610296828873ffffffffffffffffffffffffffffffffffffffff1661037790919063ffffffff16565b9050808487846102a69190610a65565b61ffff16815181106102bb576102ba6109ff565b5b6020026020010181815250506102d08161039f565b836102db9190610b17565b925050806102e890610b4a565b905061025f565b50935093915050565b5f805b610100811015610356575f816001901b86161461034b575f818860081b01870290506103498a8a8388888060010199508151811061033c5761033b6109ff565b5b60200260200101516103e8565b505b8060010190506102fb565b50819050979650505050505050565b5f808284071282840503905092915050565b5f808260010b905061039484635339c29660e01b835f60206105a0565b5f5191505092915050565b5f8019808314600382048460011c1684039350600582048460021c16600583048516019350601182048460041c850116935060ff8204840260f81c8160081b1792505050919050565b81815f019060020b908160020b8152505060028081111561040c5761040b610b73565b5b84600281111561041f5761041e610b73565b5b03610507578273ffffffffffffffffffffffffffffffffffffffff1663f30dba93836040518263ffffffff1660e01b815260040161045d9190610baf565b61014060405180830381865afa158015610479573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061049d9190610d14565b909192939495965090919293509091925090915090505084604001856020018660600187608001848152508481525084600f0b600f0b815250846fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff168152505050505061059a565b5f610531838573ffffffffffffffffffffffffffffffffffffffff166105be90919063ffffffff16565b905080602001518260200190600f0b9081600f0b81525050805f015182604001906fffffffffffffffffffffffffffffffff1690816fffffffffffffffffffffffffffffffff168152505080604001518260600181815250508060600151826080018181525050505b50505050565b835f5282600452808260245f885afa6105b7575f80fd5b5050505050565b6105c6610644565b5f808360020b91508290506105e78563f30dba9360e01b84846101006105a0565b505092915050565b6040518060200160405280606081525090565b6040518060a001604052805f60020b81526020015f600f0b81526020015f6fffffffffffffffffffffffffffffffff1681526020015f81526020015f81525090565b6040518061010001604052805f6fffffffffffffffffffffffffffffffff1681526020015f600f0b81526020015f81526020015f81526020015f60060b81526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f63ffffffff1681526020015f151581525090565b5f80fd5b600381106106c7575f80fd5b50565b5f813590506106d8816106bb565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610707826106de565b9050919050565b610717816106fd565b8114610721575f80fd5b50565b5f813590506107328161070e565b92915050565b5f8160020b9050919050565b61074d81610738565b8114610757575f80fd5b50565b5f8135905061076881610744565b92915050565b5f805f8060808587031215610786576107856106b7565b5b5f610793878288016106ca565b94505060206107a487828801610724565b93505060406107b58782880161075a565b92505060606107c68782880161075a565b91505092959194509250565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b61080481610738565b82525050565b5f81600f0b9050919050565b61081f8161080a565b82525050565b5f6fffffffffffffffffffffffffffffffff82169050919050565b61084981610825565b82525050565b5f819050919050565b6108618161084f565b82525050565b60a082015f82015161087b5f8501826107fb565b50602082015161088e6020850182610816565b5060408201516108a16040850182610840565b5060608201516108b46060850182610858565b5060808201516108c76080850182610858565b50505050565b5f6108d88383610867565b60a08301905092915050565b5f602082019050919050565b5f6108fa826107d2565b61090481856107dc565b935061090f836107ec565b805f5b8381101561093f57815161092688826108cd565b9750610931836108e4565b925050600181019050610912565b5085935050505092915050565b5f602083015f8301518482035f86015261096682826108f0565b9150508091505092915050565b5f6020820190508181035f83015261098b818461094c565b905092915050565b5f815190506109a181610744565b92915050565b5f602082840312156109bc576109bb6106b7565b5b5f6109c984828501610993565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f8160010b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f610a6f82610a2c565b9150610a7a83610a2c565b92508282039050617fff81137fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff800082121715610ab857610ab7610a38565b5b92915050565b5f610ac882610a2c565b9150610ad383610a2c565b925082820190507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80008112617fff82131715610b1157610b10610a38565b5b92915050565b5f610b218261084f565b9150610b2c8361084f565b9250828201905080821115610b4457610b43610a38565b5b92915050565b5f610b5482610a2c565b9150617fff8203610b6857610b67610a38565b5b600182019050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b610ba981610738565b82525050565b5f602082019050610bc25f830184610ba0565b92915050565b610bd181610825565b8114610bdb575f80fd5b50565b5f81519050610bec81610bc8565b92915050565b610bfb8161080a565b8114610c05575f80fd5b50565b5f81519050610c1681610bf2565b92915050565b610c258161084f565b8114610c2f575f80fd5b50565b5f81519050610c4081610c1c565b92915050565b5f8160060b9050919050565b610c5b81610c46565b8114610c65575f80fd5b50565b5f81519050610c7681610c52565b92915050565b610c85816106de565b8114610c8f575f80fd5b50565b5f81519050610ca081610c7c565b92915050565b5f63ffffffff82169050919050565b610cbe81610ca6565b8114610cc8575f80fd5b50565b5f81519050610cd981610cb5565b92915050565b5f8115159050919050565b610cf381610cdf565b8114610cfd575f80fd5b50565b5f81519050610d0e81610cea565b92915050565b5f805f805f805f805f806101408b8d031215610d3357610d326106b7565b5b5f610d408d828e01610bde565b9a50506020610d518d828e01610c08565b9950506040610d628d828e01610c08565b9850506060610d738d828e01610c32565b9750506080610d848d828e01610c32565b96505060a0610d958d828e01610c32565b95505060c0610da68d828e01610c68565b94505060e0610db78d828e01610c92565b935050610100610dc98d828e01610ccb565b925050610120610ddb8d828e01610d00565b9150509295989b9194979a509295985056fea2646970667358221220a22f05cac23454f1849007fee8e83e2b232001ae2ca9a5eec5ea53dd00e38efa64736f6c634300081a0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\0\x1DW_5`\xE0\x1C\x80c\xFA[\x11\xD4\x14a\0!W[_\x80\xFD[a\0;`\x04\x806\x03\x81\x01\x90a\x006\x91\x90a\x07nV[a\0QV[`@Qa\0H\x91\x90a\tsV[`@Q\x80\x91\x03\x90\xF3[a\0Ya\x05\xEFV[\x81`\x02\x0B\x83`\x02\x0B\x13\x15a\0kW_\x80\xFD[_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xB5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xD9\x91\x90a\t\xA7V[\x90P_\x80a\0\xE8\x86\x86\x85a\x01\xB9V[\x91P\x91P_\x80a\0\xF9\x89\x85\x85a\x01\xF1V[\x91P\x91P\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\x17Wa\x01\x16a\t\xD2V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01PW\x81` \x01[a\x01=a\x06\x02V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x015W\x90P[P\x86_\x01\x81\x90RP_\x80\x85\x90P[\x84`\x01\x0B\x81`\x01\x0B\x13a\x01\xAAWa\x01\x9D\x8C\x8C\x83\x8A\x88\x8B\x87\x03a\xFF\xFF\x16\x81Q\x81\x10a\x01\x8BWa\x01\x8Aa\t\xFFV[[` \x02` \x01\x01Q\x8D_\x01Q\x88a\x02\xF8V[\x91P\x80`\x01\x01\x90Pa\x01^V[PPPPPPP\x94\x93PPPPV[_\x80_a\x01\xC6\x86\x85a\x03eV[\x90P`\x08\x81`\x02\x0B\x90\x1D\x92Pa\x01\xDC\x85\x85a\x03eV[\x90P`\x08\x81`\x02\x0B\x90\x1D\x91PP\x93P\x93\x91PPV[``_`\x01\x84\x84a\x02\x02\x91\x90a\neV[a\x02\x0C\x91\x90a\n\xBEV[a\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02)Wa\x02(a\t\xD2V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02WW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x91P_\x84\x90P[\x83`\x01\x0B\x81`\x01\x0B\x13a\x02\xEFW_a\x02\x96\x82\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03w\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80\x84\x87\x84a\x02\xA6\x91\x90a\neV[a\xFF\xFF\x16\x81Q\x81\x10a\x02\xBBWa\x02\xBAa\t\xFFV[[` \x02` \x01\x01\x81\x81RPPa\x02\xD0\x81a\x03\x9FV[\x83a\x02\xDB\x91\x90a\x0B\x17V[\x92PP\x80a\x02\xE8\x90a\x0BJV[\x90Pa\x02_V[P\x93P\x93\x91PPV[_\x80[a\x01\0\x81\x10\x15a\x03VW_\x81`\x01\x90\x1B\x86\x16\x14a\x03KW_\x81\x88`\x08\x1B\x01\x87\x02\x90Pa\x03I\x8A\x8A\x83\x88\x88\x80`\x01\x01\x99P\x81Q\x81\x10a\x03<Wa\x03;a\t\xFFV[[` \x02` \x01\x01Qa\x03\xE8V[P[\x80`\x01\x01\x90Pa\x02\xFBV[P\x81\x90P\x97\x96PPPPPPPV[_\x80\x82\x84\x07\x12\x82\x84\x05\x03\x90P\x92\x91PPV[_\x80\x82`\x01\x0B\x90Pa\x03\x94\x84cS9\xC2\x96`\xE0\x1B\x83_` a\x05\xA0V[_Q\x91PP\x92\x91PPV[_\x80\x19\x80\x83\x14`\x03\x82\x04\x84`\x01\x1C\x16\x84\x03\x93P`\x05\x82\x04\x84`\x02\x1C\x16`\x05\x83\x04\x85\x16\x01\x93P`\x11\x82\x04\x84`\x04\x1C\x85\x01\x16\x93P`\xFF\x82\x04\x84\x02`\xF8\x1C\x81`\x08\x1B\x17\x92PPP\x91\x90PV[\x81\x81_\x01\x90`\x02\x0B\x90\x81`\x02\x0B\x81RPP`\x02\x80\x81\x11\x15a\x04\x0CWa\x04\x0Ba\x0BsV[[\x84`\x02\x81\x11\x15a\x04\x1FWa\x04\x1Ea\x0BsV[[\x03a\x05\x07W\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF3\r\xBA\x93\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04]\x91\x90a\x0B\xAFV[a\x01@`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04yW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x9D\x91\x90a\r\x14V[\x90\x91\x92\x93\x94\x95\x96P\x90\x91\x92\x93P\x90\x91\x92P\x90\x91P\x90PP\x84`@\x01\x85` \x01\x86``\x01\x87`\x80\x01\x84\x81RP\x84\x81RP\x84`\x0F\x0B`\x0F\x0B\x81RP\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPPPPa\x05\x9AV[_a\x051\x83\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x05\xBE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80` \x01Q\x82` \x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP\x80_\x01Q\x82`@\x01\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x80`@\x01Q\x82``\x01\x81\x81RPP\x80``\x01Q\x82`\x80\x01\x81\x81RPPP[PPPPV[\x83_R\x82`\x04R\x80\x82`$_\x88Z\xFAa\x05\xB7W_\x80\xFD[PPPPPV[a\x05\xC6a\x06DV[_\x80\x83`\x02\x0B\x91P\x82\x90Pa\x05\xE7\x85c\xF3\r\xBA\x93`\xE0\x1B\x84\x84a\x01\0a\x05\xA0V[PP\x92\x91PPV[`@Q\x80` \x01`@R\x80``\x81RP\x90V[`@Q\x80`\xA0\x01`@R\x80_`\x02\x0B\x81R` \x01_`\x0F\x0B\x81R` \x01_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01\0\x01`@R\x80_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_`\x0F\x0B\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x06\x0B\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x15\x15\x81RP\x90V[_\x80\xFD[`\x03\x81\x10a\x06\xC7W_\x80\xFD[PV[_\x815\x90Pa\x06\xD8\x81a\x06\xBBV[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x07\x07\x82a\x06\xDEV[\x90P\x91\x90PV[a\x07\x17\x81a\x06\xFDV[\x81\x14a\x07!W_\x80\xFD[PV[_\x815\x90Pa\x072\x81a\x07\x0EV[\x92\x91PPV[_\x81`\x02\x0B\x90P\x91\x90PV[a\x07M\x81a\x078V[\x81\x14a\x07WW_\x80\xFD[PV[_\x815\x90Pa\x07h\x81a\x07DV[\x92\x91PPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a\x07\x86Wa\x07\x85a\x06\xB7V[[_a\x07\x93\x87\x82\x88\x01a\x06\xCAV[\x94PP` a\x07\xA4\x87\x82\x88\x01a\x07$V[\x93PP`@a\x07\xB5\x87\x82\x88\x01a\x07ZV[\x92PP``a\x07\xC6\x87\x82\x88\x01a\x07ZV[\x91PP\x92\x95\x91\x94P\x92PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x08\x04\x81a\x078V[\x82RPPV[_\x81`\x0F\x0B\x90P\x91\x90PV[a\x08\x1F\x81a\x08\nV[\x82RPPV[_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x08I\x81a\x08%V[\x82RPPV[_\x81\x90P\x91\x90PV[a\x08a\x81a\x08OV[\x82RPPV[`\xA0\x82\x01_\x82\x01Qa\x08{_\x85\x01\x82a\x07\xFBV[P` \x82\x01Qa\x08\x8E` \x85\x01\x82a\x08\x16V[P`@\x82\x01Qa\x08\xA1`@\x85\x01\x82a\x08@V[P``\x82\x01Qa\x08\xB4``\x85\x01\x82a\x08XV[P`\x80\x82\x01Qa\x08\xC7`\x80\x85\x01\x82a\x08XV[PPPPV[_a\x08\xD8\x83\x83a\x08gV[`\xA0\x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x08\xFA\x82a\x07\xD2V[a\t\x04\x81\x85a\x07\xDCV[\x93Pa\t\x0F\x83a\x07\xECV[\x80_[\x83\x81\x10\x15a\t?W\x81Qa\t&\x88\x82a\x08\xCDV[\x97Pa\t1\x83a\x08\xE4V[\x92PP`\x01\x81\x01\x90Pa\t\x12V[P\x85\x93PPPP\x92\x91PPV[_` \x83\x01_\x83\x01Q\x84\x82\x03_\x86\x01Ra\tf\x82\x82a\x08\xF0V[\x91PP\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\t\x8B\x81\x84a\tLV[\x90P\x92\x91PPV[_\x81Q\x90Pa\t\xA1\x81a\x07DV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\t\xBCWa\t\xBBa\x06\xB7V[[_a\t\xC9\x84\x82\x85\x01a\t\x93V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81`\x01\x0B\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\no\x82a\n,V[\x91Pa\nz\x83a\n,V[\x92P\x82\x82\x03\x90Pa\x7F\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\x82\x12\x17\x15a\n\xB8Wa\n\xB7a\n8V[[\x92\x91PPV[_a\n\xC8\x82a\n,V[\x91Pa\n\xD3\x83a\n,V[\x92P\x82\x82\x01\x90P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\x81\x12a\x7F\xFF\x82\x13\x17\x15a\x0B\x11Wa\x0B\x10a\n8V[[\x92\x91PPV[_a\x0B!\x82a\x08OV[\x91Pa\x0B,\x83a\x08OV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x0BDWa\x0BCa\n8V[[\x92\x91PPV[_a\x0BT\x82a\n,V[\x91Pa\x7F\xFF\x82\x03a\x0BhWa\x0Bga\n8V[[`\x01\x82\x01\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[a\x0B\xA9\x81a\x078V[\x82RPPV[_` \x82\x01\x90Pa\x0B\xC2_\x83\x01\x84a\x0B\xA0V[\x92\x91PPV[a\x0B\xD1\x81a\x08%V[\x81\x14a\x0B\xDBW_\x80\xFD[PV[_\x81Q\x90Pa\x0B\xEC\x81a\x0B\xC8V[\x92\x91PPV[a\x0B\xFB\x81a\x08\nV[\x81\x14a\x0C\x05W_\x80\xFD[PV[_\x81Q\x90Pa\x0C\x16\x81a\x0B\xF2V[\x92\x91PPV[a\x0C%\x81a\x08OV[\x81\x14a\x0C/W_\x80\xFD[PV[_\x81Q\x90Pa\x0C@\x81a\x0C\x1CV[\x92\x91PPV[_\x81`\x06\x0B\x90P\x91\x90PV[a\x0C[\x81a\x0CFV[\x81\x14a\x0CeW_\x80\xFD[PV[_\x81Q\x90Pa\x0Cv\x81a\x0CRV[\x92\x91PPV[a\x0C\x85\x81a\x06\xDEV[\x81\x14a\x0C\x8FW_\x80\xFD[PV[_\x81Q\x90Pa\x0C\xA0\x81a\x0C|V[\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x0C\xBE\x81a\x0C\xA6V[\x81\x14a\x0C\xC8W_\x80\xFD[PV[_\x81Q\x90Pa\x0C\xD9\x81a\x0C\xB5V[\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a\x0C\xF3\x81a\x0C\xDFV[\x81\x14a\x0C\xFDW_\x80\xFD[PV[_\x81Q\x90Pa\r\x0E\x81a\x0C\xEAV[\x92\x91PPV[_\x80_\x80_\x80_\x80_\x80a\x01@\x8B\x8D\x03\x12\x15a\r3Wa\r2a\x06\xB7V[[_a\r@\x8D\x82\x8E\x01a\x0B\xDEV[\x9APP` a\rQ\x8D\x82\x8E\x01a\x0C\x08V[\x99PP`@a\rb\x8D\x82\x8E\x01a\x0C\x08V[\x98PP``a\rs\x8D\x82\x8E\x01a\x0C2V[\x97PP`\x80a\r\x84\x8D\x82\x8E\x01a\x0C2V[\x96PP`\xA0a\r\x95\x8D\x82\x8E\x01a\x0C2V[\x95PP`\xC0a\r\xA6\x8D\x82\x8E\x01a\x0ChV[\x94PP`\xE0a\r\xB7\x8D\x82\x8E\x01a\x0C\x92V[\x93PPa\x01\0a\r\xC9\x8D\x82\x8E\x01a\x0C\xCBV[\x92PPa\x01 a\r\xDB\x8D\x82\x8E\x01a\r\0V[\x91PP\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV\xFE\xA2dipfsX\"\x12 \xA2/\x05\xCA\xC24T\xF1\x84\x90\x07\xFE\xE8\xE8>+# \x01\xAE,\xA9\xA5\xEE\xC5\xEAS\xDD\0\xE3\x8E\xFAdsolcC\0\x08\x1A\x003",
    );
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEX(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<DEX> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        #[automatically_derived]
        impl DEX {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> u8 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for DEX {
            type RustType = u8;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for DEX {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct V3PoolCallee(alloy::sol_types::private::Address);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<V3PoolCallee>
        for alloy::sol_types::private::Address {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::Token<
                '_,
            > {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Address,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        self,
                    )
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::abi_encode_packed_to(
                    self,
                    out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::abi_encoded_size(
                    self,
                )
            }
        }
        #[automatically_derived]
        impl V3PoolCallee {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: alloy::sol_types::private::Address) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> alloy::sol_types::private::Address {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for V3PoolCallee {
            type RustType = alloy::sol_types::private::Address;
            type Token<'a> = <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::Token<
                'a,
            >;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::type_check(
                    token,
                )
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::detokenize(
                    token,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for V3PoolCallee {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                    rust,
                )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    out,
                )
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    rust,
                )
            }
        }
    };
    /**Constructor`.
```solidity
constructor(DEX dex, V3PoolCallee pool, int24 tickLower, int24 tickUpper) payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub dex: <DEX as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub pool: <V3PoolCallee as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub tickLower: alloy::sol_types::private::primitives::aliases::I24,
        #[allow(missing_docs)]
        pub tickUpper: alloy::sol_types::private::primitives::aliases::I24,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                DEX,
                V3PoolCallee,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Int<24>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <DEX as alloy::sol_types::SolType>::RustType,
                <V3PoolCallee as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::I24,
                alloy::sol_types::private::primitives::aliases::I24,
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
                    (value.dex, value.pool, value.tickLower, value.tickUpper)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        dex: tuple.0,
                        pool: tuple.1,
                        tickLower: tuple.2,
                        tickUpper: tuple.3,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                DEX,
                V3PoolCallee,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Int<24>,
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
                    <DEX as alloy_sol_types::SolType>::tokenize(&self.dex),
                    <V3PoolCallee as alloy_sol_types::SolType>::tokenize(&self.pool),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickLower),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickUpper),
                )
            }
        }
    };
    /**Function with signature `getPopulatedTicksInRange(uint8,address,int24,int24)` and selector `0xfa5b11d4`.
```solidity
function getPopulatedTicksInRange(DEX dex, V3PoolCallee pool, int24 tickLower, int24 tickUpper) external payable returns (PoolHelpers.PopulatedTicks memory populatedTicks);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPopulatedTicksInRangeCall {
        #[allow(missing_docs)]
        pub dex: <DEX as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub pool: <V3PoolCallee as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub tickLower: alloy::sol_types::private::primitives::aliases::I24,
        #[allow(missing_docs)]
        pub tickUpper: alloy::sol_types::private::primitives::aliases::I24,
    }
    ///Container type for the return parameters of the [`getPopulatedTicksInRange(uint8,address,int24,int24)`](getPopulatedTicksInRangeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPopulatedTicksInRangeReturn {
        #[allow(missing_docs)]
        pub populatedTicks: <PoolHelpers::PopulatedTicks as alloy::sol_types::SolType>::RustType,
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
                DEX,
                V3PoolCallee,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Int<24>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <DEX as alloy::sol_types::SolType>::RustType,
                <V3PoolCallee as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::I24,
                alloy::sol_types::private::primitives::aliases::I24,
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
            impl ::core::convert::From<getPopulatedTicksInRangeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPopulatedTicksInRangeCall) -> Self {
                    (value.dex, value.pool, value.tickLower, value.tickUpper)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPopulatedTicksInRangeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        dex: tuple.0,
                        pool: tuple.1,
                        tickLower: tuple.2,
                        tickUpper: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (PoolHelpers::PopulatedTicks,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <PoolHelpers::PopulatedTicks as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getPopulatedTicksInRangeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPopulatedTicksInRangeReturn) -> Self {
                    (value.populatedTicks,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPopulatedTicksInRangeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { populatedTicks: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPopulatedTicksInRangeCall {
            type Parameters<'a> = (
                DEX,
                V3PoolCallee,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Int<24>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPopulatedTicksInRangeReturn;
            type ReturnTuple<'a> = (PoolHelpers::PopulatedTicks,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPopulatedTicksInRange(uint8,address,int24,int24)";
            const SELECTOR: [u8; 4] = [250u8, 91u8, 17u8, 212u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <DEX as alloy_sol_types::SolType>::tokenize(&self.dex),
                    <V3PoolCallee as alloy_sol_types::SolType>::tokenize(&self.pool),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickLower),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickUpper),
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
    ///Container for all the [`GetCLPoolTicksInRange`](self) function calls.
    pub enum GetCLPoolTicksInRangeCalls {
        #[allow(missing_docs)]
        getPopulatedTicksInRange(getPopulatedTicksInRangeCall),
    }
    #[automatically_derived]
    impl GetCLPoolTicksInRangeCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[250u8, 91u8, 17u8, 212u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for GetCLPoolTicksInRangeCalls {
        const NAME: &'static str = "GetCLPoolTicksInRangeCalls";
        const MIN_DATA_LENGTH: usize = 128usize;
        const COUNT: usize = 1usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::getPopulatedTicksInRange(_) => {
                    <getPopulatedTicksInRangeCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<GetCLPoolTicksInRangeCalls>] = &[
                {
                    fn getPopulatedTicksInRange(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<GetCLPoolTicksInRangeCalls> {
                        <getPopulatedTicksInRangeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(GetCLPoolTicksInRangeCalls::getPopulatedTicksInRange)
                    }
                    getPopulatedTicksInRange
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
                Self::getPopulatedTicksInRange(inner) => {
                    <getPopulatedTicksInRangeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::getPopulatedTicksInRange(inner) => {
                    <getPopulatedTicksInRangeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`GetCLPoolTicksInRange`](self) contract instance.

See the [wrapper's documentation](`GetCLPoolTicksInRangeInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> GetCLPoolTicksInRangeInstance<T, P, N> {
        GetCLPoolTicksInRangeInstance::<T, P, N>::new(address, provider)
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
        dex: <DEX as alloy::sol_types::SolType>::RustType,
        pool: <V3PoolCallee as alloy::sol_types::SolType>::RustType,
        tickLower: alloy::sol_types::private::primitives::aliases::I24,
        tickUpper: alloy::sol_types::private::primitives::aliases::I24,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<GetCLPoolTicksInRangeInstance<T, P, N>>,
    > {
        GetCLPoolTicksInRangeInstance::<
            T,
            P,
            N,
        >::deploy(provider, dex, pool, tickLower, tickUpper)
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
        dex: <DEX as alloy::sol_types::SolType>::RustType,
        pool: <V3PoolCallee as alloy::sol_types::SolType>::RustType,
        tickLower: alloy::sol_types::private::primitives::aliases::I24,
        tickUpper: alloy::sol_types::private::primitives::aliases::I24,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        GetCLPoolTicksInRangeInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, dex, pool, tickLower, tickUpper)
    }
    /**A [`GetCLPoolTicksInRange`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`GetCLPoolTicksInRange`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct GetCLPoolTicksInRangeInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for GetCLPoolTicksInRangeInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("GetCLPoolTicksInRangeInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > GetCLPoolTicksInRangeInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`GetCLPoolTicksInRange`](self) contract instance.

See the [wrapper's documentation](`GetCLPoolTicksInRangeInstance`) for more details.*/
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
            dex: <DEX as alloy::sol_types::SolType>::RustType,
            pool: <V3PoolCallee as alloy::sol_types::SolType>::RustType,
            tickLower: alloy::sol_types::private::primitives::aliases::I24,
            tickUpper: alloy::sol_types::private::primitives::aliases::I24,
        ) -> alloy_contract::Result<GetCLPoolTicksInRangeInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                dex,
                pool,
                tickLower,
                tickUpper,
            );
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
            dex: <DEX as alloy::sol_types::SolType>::RustType,
            pool: <V3PoolCallee as alloy::sol_types::SolType>::RustType,
            tickLower: alloy::sol_types::private::primitives::aliases::I24,
            tickUpper: alloy::sol_types::private::primitives::aliases::I24,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            dex,
                            pool,
                            tickLower,
                            tickUpper,
                        },
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
    impl<T, P: ::core::clone::Clone, N> GetCLPoolTicksInRangeInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> GetCLPoolTicksInRangeInstance<T, P, N> {
            GetCLPoolTicksInRangeInstance {
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
    > GetCLPoolTicksInRangeInstance<T, P, N> {
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
        ///Creates a new call builder for the [`getPopulatedTicksInRange`] function.
        pub fn getPopulatedTicksInRange(
            &self,
            dex: <DEX as alloy::sol_types::SolType>::RustType,
            pool: <V3PoolCallee as alloy::sol_types::SolType>::RustType,
            tickLower: alloy::sol_types::private::primitives::aliases::I24,
            tickUpper: alloy::sol_types::private::primitives::aliases::I24,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPopulatedTicksInRangeCall, N> {
            self.call_builder(
                &getPopulatedTicksInRangeCall {
                    dex,
                    pool,
                    tickLower,
                    tickUpper,
                },
            )
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > GetCLPoolTicksInRangeInstance<T, P, N> {
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
