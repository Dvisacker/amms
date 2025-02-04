///Module containing a contract's types and functions.
/**

```solidity
library PoolHelpers {
    struct UniswapV2PoolData { address tokenA; bytes32 tokenASymbol; uint8 tokenADecimals; address tokenB; bytes32 tokenBSymbol; uint8 tokenBDecimals; uint256 reserve0; uint256 reserve1; address factory; }
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
struct UniswapV2PoolData { address tokenA; bytes32 tokenASymbol; uint8 tokenADecimals; address tokenB; bytes32 tokenBSymbol; uint8 tokenBDecimals; uint256 reserve0; uint256 reserve1; address factory; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UniswapV2PoolData {
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
        pub reserve0: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub reserve1: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub factory: alloy::sol_types::private::Address,
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::FixedBytes<32>,
            u8,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::FixedBytes<32>,
            u8,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<UniswapV2PoolData> for UnderlyingRustTuple<'_> {
            fn from(value: UniswapV2PoolData) -> Self {
                (
                    value.tokenA,
                    value.tokenASymbol,
                    value.tokenADecimals,
                    value.tokenB,
                    value.tokenBSymbol,
                    value.tokenBDecimals,
                    value.reserve0,
                    value.reserve1,
                    value.factory,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UniswapV2PoolData {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    tokenA: tuple.0,
                    tokenASymbol: tuple.1,
                    tokenADecimals: tuple.2,
                    tokenB: tuple.3,
                    tokenBSymbol: tuple.4,
                    tokenBDecimals: tuple.5,
                    reserve0: tuple.6,
                    reserve1: tuple.7,
                    factory: tuple.8,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for UniswapV2PoolData {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for UniswapV2PoolData {
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.reserve0),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.reserve1),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.factory,
                    ),
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
        impl alloy_sol_types::SolType for UniswapV2PoolData {
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
        impl alloy_sol_types::SolStruct for UniswapV2PoolData {
            const NAME: &'static str = "UniswapV2PoolData";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "UniswapV2PoolData(address tokenA,bytes32 tokenASymbol,uint8 tokenADecimals,address tokenB,bytes32 tokenBSymbol,uint8 tokenBDecimals,uint256 reserve0,uint256 reserve1,address factory)",
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.reserve0)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.reserve1)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.factory,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for UniswapV2PoolData {
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
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.reserve0,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.reserve1,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.factory,
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
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.reserve0,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.reserve1,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.factory,
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
    struct UniswapV2PoolData {
        address tokenA;
        bytes32 tokenASymbol;
        uint8 tokenADecimals;
        address tokenB;
        bytes32 tokenBSymbol;
        uint8 tokenBDecimals;
        uint256 reserve0;
        uint256 reserve1;
        address factory;
    }
}

interface GetUniV2PoolData {
    constructor(address[] pools);

    function getPoolData(address[] memory pools) external returns (PoolHelpers.UniswapV2PoolData[] memory);
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
        "internalType": "struct PoolHelpers.UniswapV2PoolData[]",
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
            "name": "reserve0",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "reserve1",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "factory",
            "type": "address",
            "internalType": "address"
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
pub mod GetUniV2PoolData {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b5060405161126638038061126683398181016040528101906100319190610d27565b5f6100418261007060201b60201c565b90505f816040516020016100559190610f26565b60405160208183030381529060405290506020810180590381f35b60605f825167ffffffffffffffff81111561008e5761008d610b91565b5b6040519080825280602002602001820160405280156100c757816020015b6100b4610adb565b8152602001906001900390816100ac5790505b5090505f5b8351811015610a7a575f8482815181106100e9576100e8610f46565b5b6020026020010151905061010281610a8460201b60201c565b1561010d5750610a6f565b610115610adb565b8173ffffffffffffffffffffffffffffffffffffffff16630dfe16816040518163ffffffff1660e01b8152600401602060405180830381865afa15801561015e573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101829190610f73565b815f019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff16815250508173ffffffffffffffffffffffffffffffffffffffff1663d21220a76040518163ffffffff1660e01b8152600401602060405180830381865afa158015610201573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102259190610f73565b816060019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff168152505061026e815f0151610a8460201b60201c565b1561027a575050610a6f565b61028d8160600151610a8460201b60201c565b15610299575050610a6f565b5f80825f015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f313ce567000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506040516103489190610ff0565b5f604051808303815f8787f1925050503d805f8114610382576040519150601f19603f3d011682016040523d82523d5f602084013e610387565b606091505b509150915081156103f6575f60208251036103e657818060200190518101906103b09190611030565b90505f8114806103c0575060ff81115b156103cf575050505050610a6f565b80846040019060ff16908160ff16815250506103f0565b5050505050610a6f565b506103ff565b50505050610a6f565b5f80846060015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f313ce567000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506040516104af9190610ff0565b5f604051808303815f8787f1925050503d805f81146104e9576040519150601f19603f3d011682016040523d82523d5f602084013e6104ee565b606091505b50915091508115610561575f602082510361054f57818060200190518101906105179190611030565b90505f811480610527575060ff81115b156105385750505050505050610a6f565b808660a0019060ff16908160ff168152505061055b565b50505050505050610a6f565b5061056c565b505050505050610a6f565b5f80865f015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f95d89b41000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff838183161783525050505060405161061b9190610ff0565b5f604051808303815f8787f1925050503d805f8114610655576040519150601f19603f3d011682016040523d82523d5f602084013e61065a565b606091505b509150915081156106c4575f602082510361068a57818060200190518101906106839190611085565b90506106b4565b5f8280602001905181019061069f9190611152565b90506106b081610ab560201b60201c565b9150505b80886020018181525050506106d1565b5f801b8760200181815250505b5f80886060015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f95d89b41000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506040516107819190610ff0565b5f604051808303815f8787f1925050503d805f81146107bb576040519150601f19603f3d011682016040523d82523d5f602084013e6107c0565b606091505b5091509150811561082a575f60208251036107f057818060200190518101906107e99190611085565b905061081a565b5f828060200190518101906108059190611152565b905061081681610ab560201b60201c565b9150505b808a608001818152505050610837565b5f801b8960800181815250505b8973ffffffffffffffffffffffffffffffffffffffff1663c45a01556040518163ffffffff1660e01b8152600401602060405180830381865afa92505050801561089f57506040513d601f19601f8201168201806040525081019061089c9190610f73565b60015b6108e1575f89610100019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff168152505061091c565b808a610100019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff1681525050505b8973ffffffffffffffffffffffffffffffffffffffff16630902f1ac6040518163ffffffff1660e01b8152600401606060405180830381865afa92505050801561098457506040513d601f19601f820116820180604052508101906109819190611215565b60015b6109a1575f8960c00181815250505f8960e0018181525050610a45565b6dffffffffffffffffffffffffffff8016836dffffffffffffffffffffffffffff1611806109ee57506dffffffffffffffffffffffffffff8016826dffffffffffffffffffffffffffff16115b15610a0c575f8c60c00181815250505f8c60e0018181525050610a41565b826dffffffffffffffffffffffffffff168c60c0018181525050816dffffffffffffffffffffffffffff168c60e00181815250505b5050505b888c8c81518110610a5957610a58610f46565b5b6020026020010181905250505050505050505050505b8060010190506100cc565b5080915050919050565b5f808273ffffffffffffffffffffffffffffffffffffffff163b03610aac5760019050610ab0565b5f90505b919050565b5f808290505f815103610acd575f801b915050610ad6565b60208301519150505b919050565b6040518061012001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f80191681526020015f60ff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f80191681526020015f60ff1681526020015f81526020015f81526020015f73ffffffffffffffffffffffffffffffffffffffff1681525090565b5f604051905090565b5f80fd5b5f80fd5b5f80fd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b610bc782610b81565b810181811067ffffffffffffffff82111715610be657610be5610b91565b5b80604052505050565b5f610bf8610b6c565b9050610c048282610bbe565b919050565b5f67ffffffffffffffff821115610c2357610c22610b91565b5b602082029050602081019050919050565b5f80fd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610c6182610c38565b9050919050565b610c7181610c57565b8114610c7b575f80fd5b50565b5f81519050610c8c81610c68565b92915050565b5f610ca4610c9f84610c09565b610bef565b90508083825260208201905060208402830185811115610cc757610cc6610c34565b5b835b81811015610cf05780610cdc8882610c7e565b845260208401935050602081019050610cc9565b5050509392505050565b5f82601f830112610d0e57610d0d610b7d565b5b8151610d1e848260208601610c92565b91505092915050565b5f60208284031215610d3c57610d3b610b75565b5b5f82015167ffffffffffffffff811115610d5957610d58610b79565b5b610d6584828501610cfa565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b610da081610c57565b82525050565b5f819050919050565b610db881610da6565b82525050565b5f60ff82169050919050565b610dd381610dbe565b82525050565b5f819050919050565b610deb81610dd9565b82525050565b61012082015f820151610e065f850182610d97565b506020820151610e196020850182610daf565b506040820151610e2c6040850182610dca565b506060820151610e3f6060850182610d97565b506080820151610e526080850182610daf565b5060a0820151610e6560a0850182610dca565b5060c0820151610e7860c0850182610de2565b5060e0820151610e8b60e0850182610de2565b50610100820151610ea0610100850182610d97565b50505050565b5f610eb18383610df1565b6101208301905092915050565b5f602082019050919050565b5f610ed482610d6e565b610ede8185610d78565b9350610ee983610d88565b805f5b83811015610f19578151610f008882610ea6565b9750610f0b83610ebe565b925050600181019050610eec565b5085935050505092915050565b5f6020820190508181035f830152610f3e8184610eca565b905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f60208284031215610f8857610f87610b75565b5b5f610f9584828501610c7e565b91505092915050565b5f81519050919050565b5f81905092915050565b8281835e5f83830152505050565b5f610fca82610f9e565b610fd48185610fa8565b9350610fe4818560208601610fb2565b80840191505092915050565b5f610ffb8284610fc0565b915081905092915050565b61100f81610dd9565b8114611019575f80fd5b50565b5f8151905061102a81611006565b92915050565b5f6020828403121561104557611044610b75565b5b5f6110528482850161101c565b91505092915050565b61106481610da6565b811461106e575f80fd5b50565b5f8151905061107f8161105b565b92915050565b5f6020828403121561109a57611099610b75565b5b5f6110a784828501611071565b91505092915050565b5f80fd5b5f67ffffffffffffffff8211156110ce576110cd610b91565b5b6110d782610b81565b9050602081019050919050565b5f6110f66110f1846110b4565b610bef565b905082815260208101848484011115611112576111116110b0565b5b61111d848285610fb2565b509392505050565b5f82601f83011261113957611138610b7d565b5b81516111498482602086016110e4565b91505092915050565b5f6020828403121561116757611166610b75565b5b5f82015167ffffffffffffffff81111561118457611183610b79565b5b61119084828501611125565b91505092915050565b5f6dffffffffffffffffffffffffffff82169050919050565b6111bb81611199565b81146111c5575f80fd5b50565b5f815190506111d6816111b2565b92915050565b5f63ffffffff82169050919050565b6111f4816111dc565b81146111fe575f80fd5b50565b5f8151905061120f816111eb565b92915050565b5f805f6060848603121561122c5761122b610b75565b5b5f611239868287016111c8565b935050602061124a868287016111c8565b925050604061125b86828701611201565b915050925092509256fe
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`@Qa\x12f8\x03\x80a\x12f\x839\x81\x81\x01`@R\x81\x01\x90a\x001\x91\x90a\r'V[_a\0A\x82a\0p` \x1B` \x1CV[\x90P_\x81`@Q` \x01a\0U\x91\x90a\x0F&V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P` \x81\x01\x80Y\x03\x81\xF3[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\x8EWa\0\x8Da\x0B\x91V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\xC7W\x81` \x01[a\0\xB4a\n\xDBV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\0\xACW\x90P[P\x90P_[\x83Q\x81\x10\x15a\nzW_\x84\x82\x81Q\x81\x10a\0\xE9Wa\0\xE8a\x0FFV[[` \x02` \x01\x01Q\x90Pa\x01\x02\x81a\n\x84` \x1B` \x1CV[\x15a\x01\rWPa\noV[a\x01\x15a\n\xDBV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01^W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x82\x91\x90a\x0FsV[\x81_\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD2\x12 \xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x01W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02%\x91\x90a\x0FsV[\x81``\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\x02n\x81_\x01Qa\n\x84` \x1B` \x1CV[\x15a\x02zWPPa\noV[a\x02\x8D\x81``\x01Qa\n\x84` \x1B` \x1CV[\x15a\x02\x99WPPa\noV[_\x80\x82_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x03H\x91\x90a\x0F\xF0V[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x03\x82W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x03\x87V[``\x91P[P\x91P\x91P\x81\x15a\x03\xF6W_` \x82Q\x03a\x03\xE6W\x81\x80` \x01\x90Q\x81\x01\x90a\x03\xB0\x91\x90a\x100V[\x90P_\x81\x14\x80a\x03\xC0WP`\xFF\x81\x11[\x15a\x03\xCFWPPPPPa\noV[\x80\x84`@\x01\x90`\xFF\x16\x90\x81`\xFF\x16\x81RPPa\x03\xF0V[PPPPPa\noV[Pa\x03\xFFV[PPPPa\noV[_\x80\x84``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x04\xAF\x91\x90a\x0F\xF0V[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x04\xE9W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x04\xEEV[``\x91P[P\x91P\x91P\x81\x15a\x05aW_` \x82Q\x03a\x05OW\x81\x80` \x01\x90Q\x81\x01\x90a\x05\x17\x91\x90a\x100V[\x90P_\x81\x14\x80a\x05'WP`\xFF\x81\x11[\x15a\x058WPPPPPPPa\noV[\x80\x86`\xA0\x01\x90`\xFF\x16\x90\x81`\xFF\x16\x81RPPa\x05[V[PPPPPPPa\noV[Pa\x05lV[PPPPPPa\noV[_\x80\x86_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F\x95\xD8\x9BA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x06\x1B\x91\x90a\x0F\xF0V[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x06UW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x06ZV[``\x91P[P\x91P\x91P\x81\x15a\x06\xC4W_` \x82Q\x03a\x06\x8AW\x81\x80` \x01\x90Q\x81\x01\x90a\x06\x83\x91\x90a\x10\x85V[\x90Pa\x06\xB4V[_\x82\x80` \x01\x90Q\x81\x01\x90a\x06\x9F\x91\x90a\x11RV[\x90Pa\x06\xB0\x81a\n\xB5` \x1B` \x1CV[\x91PP[\x80\x88` \x01\x81\x81RPPPa\x06\xD1V[_\x80\x1B\x87` \x01\x81\x81RPP[_\x80\x88``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F\x95\xD8\x9BA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x07\x81\x91\x90a\x0F\xF0V[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x07\xBBW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x07\xC0V[``\x91P[P\x91P\x91P\x81\x15a\x08*W_` \x82Q\x03a\x07\xF0W\x81\x80` \x01\x90Q\x81\x01\x90a\x07\xE9\x91\x90a\x10\x85V[\x90Pa\x08\x1AV[_\x82\x80` \x01\x90Q\x81\x01\x90a\x08\x05\x91\x90a\x11RV[\x90Pa\x08\x16\x81a\n\xB5` \x1B` \x1CV[\x91PP[\x80\x8A`\x80\x01\x81\x81RPPPa\x087V[_\x80\x1B\x89`\x80\x01\x81\x81RPP[\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC4Z\x01U`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x08\x9FWP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x9C\x91\x90a\x0FsV[`\x01[a\x08\xE1W_\x89a\x01\0\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\t\x1CV[\x80\x8Aa\x01\0\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPP[\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\t\x02\xF1\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\t\x84WP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x81\x91\x90a\x12\x15V[`\x01[a\t\xA1W_\x89`\xC0\x01\x81\x81RPP_\x89`\xE0\x01\x81\x81RPPa\nEV[m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x83m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x80a\t\xEEWPm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x82m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11[\x15a\n\x0CW_\x8C`\xC0\x01\x81\x81RPP_\x8C`\xE0\x01\x81\x81RPPa\nAV[\x82m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8C`\xC0\x01\x81\x81RPP\x81m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8C`\xE0\x01\x81\x81RPP[PPP[\x88\x8C\x8C\x81Q\x81\x10a\nYWa\nXa\x0FFV[[` \x02` \x01\x01\x81\x90RPPPPPPPPPPP[\x80`\x01\x01\x90Pa\0\xCCV[P\x80\x91PP\x91\x90PV[_\x80\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a\n\xACW`\x01\x90Pa\n\xB0V[_\x90P[\x91\x90PV[_\x80\x82\x90P_\x81Q\x03a\n\xCDW_\x80\x1B\x91PPa\n\xD6V[` \x83\x01Q\x91PP[\x91\x90PV[`@Q\x80a\x01 \x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x80\x19\x16\x81R` \x01_`\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x80\x19\x16\x81R` \x01_`\xFF\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x0B\xC7\x82a\x0B\x81V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0B\xE6Wa\x0B\xE5a\x0B\x91V[[\x80`@RPPPV[_a\x0B\xF8a\x0BlV[\x90Pa\x0C\x04\x82\x82a\x0B\xBEV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0C#Wa\x0C\"a\x0B\x91V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x80\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x0Ca\x82a\x0C8V[\x90P\x91\x90PV[a\x0Cq\x81a\x0CWV[\x81\x14a\x0C{W_\x80\xFD[PV[_\x81Q\x90Pa\x0C\x8C\x81a\x0ChV[\x92\x91PPV[_a\x0C\xA4a\x0C\x9F\x84a\x0C\tV[a\x0B\xEFV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x0C\xC7Wa\x0C\xC6a\x0C4V[[\x83[\x81\x81\x10\x15a\x0C\xF0W\x80a\x0C\xDC\x88\x82a\x0C~V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x0C\xC9V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\r\x0EWa\r\ra\x0B}V[[\x81Qa\r\x1E\x84\x82` \x86\x01a\x0C\x92V[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\r<Wa\r;a\x0BuV[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\rYWa\rXa\x0ByV[[a\re\x84\x82\x85\x01a\x0C\xFAV[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a\r\xA0\x81a\x0CWV[\x82RPPV[_\x81\x90P\x91\x90PV[a\r\xB8\x81a\r\xA6V[\x82RPPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\r\xD3\x81a\r\xBEV[\x82RPPV[_\x81\x90P\x91\x90PV[a\r\xEB\x81a\r\xD9V[\x82RPPV[a\x01 \x82\x01_\x82\x01Qa\x0E\x06_\x85\x01\x82a\r\x97V[P` \x82\x01Qa\x0E\x19` \x85\x01\x82a\r\xAFV[P`@\x82\x01Qa\x0E,`@\x85\x01\x82a\r\xCAV[P``\x82\x01Qa\x0E?``\x85\x01\x82a\r\x97V[P`\x80\x82\x01Qa\x0ER`\x80\x85\x01\x82a\r\xAFV[P`\xA0\x82\x01Qa\x0Ee`\xA0\x85\x01\x82a\r\xCAV[P`\xC0\x82\x01Qa\x0Ex`\xC0\x85\x01\x82a\r\xE2V[P`\xE0\x82\x01Qa\x0E\x8B`\xE0\x85\x01\x82a\r\xE2V[Pa\x01\0\x82\x01Qa\x0E\xA0a\x01\0\x85\x01\x82a\r\x97V[PPPPV[_a\x0E\xB1\x83\x83a\r\xF1V[a\x01 \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x0E\xD4\x82a\rnV[a\x0E\xDE\x81\x85a\rxV[\x93Pa\x0E\xE9\x83a\r\x88V[\x80_[\x83\x81\x10\x15a\x0F\x19W\x81Qa\x0F\0\x88\x82a\x0E\xA6V[\x97Pa\x0F\x0B\x83a\x0E\xBEV[\x92PP`\x01\x81\x01\x90Pa\x0E\xECV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x0F>\x81\x84a\x0E\xCAV[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\x0F\x88Wa\x0F\x87a\x0BuV[[_a\x0F\x95\x84\x82\x85\x01a\x0C~V[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x0F\xCA\x82a\x0F\x9EV[a\x0F\xD4\x81\x85a\x0F\xA8V[\x93Pa\x0F\xE4\x81\x85` \x86\x01a\x0F\xB2V[\x80\x84\x01\x91PP\x92\x91PPV[_a\x0F\xFB\x82\x84a\x0F\xC0V[\x91P\x81\x90P\x92\x91PPV[a\x10\x0F\x81a\r\xD9V[\x81\x14a\x10\x19W_\x80\xFD[PV[_\x81Q\x90Pa\x10*\x81a\x10\x06V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x10EWa\x10Da\x0BuV[[_a\x10R\x84\x82\x85\x01a\x10\x1CV[\x91PP\x92\x91PPV[a\x10d\x81a\r\xA6V[\x81\x14a\x10nW_\x80\xFD[PV[_\x81Q\x90Pa\x10\x7F\x81a\x10[V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x10\x9AWa\x10\x99a\x0BuV[[_a\x10\xA7\x84\x82\x85\x01a\x10qV[\x91PP\x92\x91PPV[_\x80\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x10\xCEWa\x10\xCDa\x0B\x91V[[a\x10\xD7\x82a\x0B\x81V[\x90P` \x81\x01\x90P\x91\x90PV[_a\x10\xF6a\x10\xF1\x84a\x10\xB4V[a\x0B\xEFV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x11\x12Wa\x11\x11a\x10\xB0V[[a\x11\x1D\x84\x82\x85a\x0F\xB2V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x119Wa\x118a\x0B}V[[\x81Qa\x11I\x84\x82` \x86\x01a\x10\xE4V[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x11gWa\x11fa\x0BuV[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\x84Wa\x11\x83a\x0ByV[[a\x11\x90\x84\x82\x85\x01a\x11%V[\x91PP\x92\x91PPV[_m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x11\xBB\x81a\x11\x99V[\x81\x14a\x11\xC5W_\x80\xFD[PV[_\x81Q\x90Pa\x11\xD6\x81a\x11\xB2V[\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x11\xF4\x81a\x11\xDCV[\x81\x14a\x11\xFEW_\x80\xFD[PV[_\x81Q\x90Pa\x12\x0F\x81a\x11\xEBV[\x92\x91PPV[_\x80_``\x84\x86\x03\x12\x15a\x12,Wa\x12+a\x0BuV[[_a\x129\x86\x82\x87\x01a\x11\xC8V[\x93PP` a\x12J\x86\x82\x87\x01a\x11\xC8V[\x92PP`@a\x12[\x86\x82\x87\x01a\x12\x01V[\x91PP\x92P\x92P\x92V\xFE",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b5060043610610029575f3560e01c80639ae919bf1461002d575b5f80fd5b61004760048036038101906100429190610cf6565b61005d565b6040516100549190610ef5565b60405180910390f35b60605f825167ffffffffffffffff81111561007b5761007a610b60565b5b6040519080825280602002602001820160405280156100b457816020015b6100a1610aaa565b8152602001906001900390816100995790505b5090505f5b8351811015610a49575f8482815181106100d6576100d5610f15565b5b602002602001015190506100e981610a53565b156100f45750610a3e565b6100fc610aaa565b8173ffffffffffffffffffffffffffffffffffffffff16630dfe16816040518163ffffffff1660e01b8152600401602060405180830381865afa158015610145573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101699190610f56565b815f019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff16815250508173ffffffffffffffffffffffffffffffffffffffff1663d21220a76040518163ffffffff1660e01b8152600401602060405180830381865afa1580156101e8573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061020c9190610f56565b816060019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff168152505061024f815f0151610a53565b1561025b575050610a3e565b6102688160600151610a53565b15610274575050610a3e565b5f80825f015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f313ce567000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506040516103239190610fd3565b5f604051808303815f8787f1925050503d805f811461035d576040519150601f19603f3d011682016040523d82523d5f602084013e610362565b606091505b509150915081156103d1575f60208251036103c1578180602001905181019061038b9190611013565b90505f81148061039b575060ff81115b156103aa575050505050610a3e565b80846040019060ff16908160ff16815250506103cb565b5050505050610a3e565b506103da565b50505050610a3e565b5f80846060015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f313ce567000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff838183161783525050505060405161048a9190610fd3565b5f604051808303815f8787f1925050503d805f81146104c4576040519150601f19603f3d011682016040523d82523d5f602084013e6104c9565b606091505b5091509150811561053c575f602082510361052a57818060200190518101906104f29190611013565b90505f811480610502575060ff81115b156105135750505050505050610a3e565b808660a0019060ff16908160ff1681525050610536565b50505050505050610a3e565b50610547565b505050505050610a3e565b5f80865f015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f95d89b41000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506040516105f69190610fd3565b5f604051808303815f8787f1925050503d805f8114610630576040519150601f19603f3d011682016040523d82523d5f602084013e610635565b606091505b50915091508115610699575f6020825103610665578180602001905181019061065e9190611068565b9050610689565b5f8280602001905181019061067a9190611135565b905061068581610a84565b9150505b80886020018181525050506106a6565b5f801b8760200181815250505b5f80886060015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f95d89b41000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506040516107569190610fd3565b5f604051808303815f8787f1925050503d805f8114610790576040519150601f19603f3d011682016040523d82523d5f602084013e610795565b606091505b509150915081156107f9575f60208251036107c557818060200190518101906107be9190611068565b90506107e9565b5f828060200190518101906107da9190611135565b90506107e581610a84565b9150505b808a608001818152505050610806565b5f801b8960800181815250505b8973ffffffffffffffffffffffffffffffffffffffff1663c45a01556040518163ffffffff1660e01b8152600401602060405180830381865afa92505050801561086e57506040513d601f19601f8201168201806040525081019061086b9190610f56565b60015b6108b0575f89610100019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff16815250506108eb565b808a610100019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff1681525050505b8973ffffffffffffffffffffffffffffffffffffffff16630902f1ac6040518163ffffffff1660e01b8152600401606060405180830381865afa92505050801561095357506040513d601f19601f8201168201806040525081019061095091906111f8565b60015b610970575f8960c00181815250505f8960e0018181525050610a14565b6dffffffffffffffffffffffffffff8016836dffffffffffffffffffffffffffff1611806109bd57506dffffffffffffffffffffffffffff8016826dffffffffffffffffffffffffffff16115b156109db575f8c60c00181815250505f8c60e0018181525050610a10565b826dffffffffffffffffffffffffffff168c60c0018181525050816dffffffffffffffffffffffffffff168c60e00181815250505b5050505b888c8c81518110610a2857610a27610f15565b5b6020026020010181905250505050505050505050505b8060010190506100b9565b5080915050919050565b5f808273ffffffffffffffffffffffffffffffffffffffff163b03610a7b5760019050610a7f565b5f90505b919050565b5f808290505f815103610a9c575f801b915050610aa5565b60208301519150505b919050565b6040518061012001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f80191681526020015f60ff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f80191681526020015f60ff1681526020015f81526020015f81526020015f73ffffffffffffffffffffffffffffffffffffffff1681525090565b5f604051905090565b5f80fd5b5f80fd5b5f80fd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b610b9682610b50565b810181811067ffffffffffffffff82111715610bb557610bb4610b60565b5b80604052505050565b5f610bc7610b3b565b9050610bd38282610b8d565b919050565b5f67ffffffffffffffff821115610bf257610bf1610b60565b5b602082029050602081019050919050565b5f80fd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610c3082610c07565b9050919050565b610c4081610c26565b8114610c4a575f80fd5b50565b5f81359050610c5b81610c37565b92915050565b5f610c73610c6e84610bd8565b610bbe565b90508083825260208201905060208402830185811115610c9657610c95610c03565b5b835b81811015610cbf5780610cab8882610c4d565b845260208401935050602081019050610c98565b5050509392505050565b5f82601f830112610cdd57610cdc610b4c565b5b8135610ced848260208601610c61565b91505092915050565b5f60208284031215610d0b57610d0a610b44565b5b5f82013567ffffffffffffffff811115610d2857610d27610b48565b5b610d3484828501610cc9565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b610d6f81610c26565b82525050565b5f819050919050565b610d8781610d75565b82525050565b5f60ff82169050919050565b610da281610d8d565b82525050565b5f819050919050565b610dba81610da8565b82525050565b61012082015f820151610dd55f850182610d66565b506020820151610de86020850182610d7e565b506040820151610dfb6040850182610d99565b506060820151610e0e6060850182610d66565b506080820151610e216080850182610d7e565b5060a0820151610e3460a0850182610d99565b5060c0820151610e4760c0850182610db1565b5060e0820151610e5a60e0850182610db1565b50610100820151610e6f610100850182610d66565b50505050565b5f610e808383610dc0565b6101208301905092915050565b5f602082019050919050565b5f610ea382610d3d565b610ead8185610d47565b9350610eb883610d57565b805f5b83811015610ee8578151610ecf8882610e75565b9750610eda83610e8d565b925050600181019050610ebb565b5085935050505092915050565b5f6020820190508181035f830152610f0d8184610e99565b905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f81519050610f5081610c37565b92915050565b5f60208284031215610f6b57610f6a610b44565b5b5f610f7884828501610f42565b91505092915050565b5f81519050919050565b5f81905092915050565b8281835e5f83830152505050565b5f610fad82610f81565b610fb78185610f8b565b9350610fc7818560208601610f95565b80840191505092915050565b5f610fde8284610fa3565b915081905092915050565b610ff281610da8565b8114610ffc575f80fd5b50565b5f8151905061100d81610fe9565b92915050565b5f6020828403121561102857611027610b44565b5b5f61103584828501610fff565b91505092915050565b61104781610d75565b8114611051575f80fd5b50565b5f815190506110628161103e565b92915050565b5f6020828403121561107d5761107c610b44565b5b5f61108a84828501611054565b91505092915050565b5f80fd5b5f67ffffffffffffffff8211156110b1576110b0610b60565b5b6110ba82610b50565b9050602081019050919050565b5f6110d96110d484611097565b610bbe565b9050828152602081018484840111156110f5576110f4611093565b5b611100848285610f95565b509392505050565b5f82601f83011261111c5761111b610b4c565b5b815161112c8482602086016110c7565b91505092915050565b5f6020828403121561114a57611149610b44565b5b5f82015167ffffffffffffffff81111561116757611166610b48565b5b61117384828501611108565b91505092915050565b5f6dffffffffffffffffffffffffffff82169050919050565b61119e8161117c565b81146111a8575f80fd5b50565b5f815190506111b981611195565b92915050565b5f63ffffffff82169050919050565b6111d7816111bf565b81146111e1575f80fd5b50565b5f815190506111f2816111ce565b92915050565b5f805f6060848603121561120f5761120e610b44565b5b5f61121c868287016111ab565b935050602061122d868287016111ab565b925050604061123e868287016111e4565b915050925092509256fea2646970667358221220c011cc731b508083ef6ae4243d6dad44185ba2a3d5826127f4b6877f3660a2a864736f6c634300081a0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0)W_5`\xE0\x1C\x80c\x9A\xE9\x19\xBF\x14a\0-W[_\x80\xFD[a\0G`\x04\x806\x03\x81\x01\x90a\0B\x91\x90a\x0C\xF6V[a\0]V[`@Qa\0T\x91\x90a\x0E\xF5V[`@Q\x80\x91\x03\x90\xF3[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0{Wa\0za\x0B`V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\xB4W\x81` \x01[a\0\xA1a\n\xAAV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\0\x99W\x90P[P\x90P_[\x83Q\x81\x10\x15a\nIW_\x84\x82\x81Q\x81\x10a\0\xD6Wa\0\xD5a\x0F\x15V[[` \x02` \x01\x01Q\x90Pa\0\xE9\x81a\nSV[\x15a\0\xF4WPa\n>V[a\0\xFCa\n\xAAV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01EW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01i\x91\x90a\x0FVV[\x81_\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD2\x12 \xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xE8W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x0C\x91\x90a\x0FVV[\x81``\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\x02O\x81_\x01Qa\nSV[\x15a\x02[WPPa\n>V[a\x02h\x81``\x01Qa\nSV[\x15a\x02tWPPa\n>V[_\x80\x82_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x03#\x91\x90a\x0F\xD3V[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x03]W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x03bV[``\x91P[P\x91P\x91P\x81\x15a\x03\xD1W_` \x82Q\x03a\x03\xC1W\x81\x80` \x01\x90Q\x81\x01\x90a\x03\x8B\x91\x90a\x10\x13V[\x90P_\x81\x14\x80a\x03\x9BWP`\xFF\x81\x11[\x15a\x03\xAAWPPPPPa\n>V[\x80\x84`@\x01\x90`\xFF\x16\x90\x81`\xFF\x16\x81RPPa\x03\xCBV[PPPPPa\n>V[Pa\x03\xDAV[PPPPa\n>V[_\x80\x84``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x04\x8A\x91\x90a\x0F\xD3V[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x04\xC4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x04\xC9V[``\x91P[P\x91P\x91P\x81\x15a\x05<W_` \x82Q\x03a\x05*W\x81\x80` \x01\x90Q\x81\x01\x90a\x04\xF2\x91\x90a\x10\x13V[\x90P_\x81\x14\x80a\x05\x02WP`\xFF\x81\x11[\x15a\x05\x13WPPPPPPPa\n>V[\x80\x86`\xA0\x01\x90`\xFF\x16\x90\x81`\xFF\x16\x81RPPa\x056V[PPPPPPPa\n>V[Pa\x05GV[PPPPPPa\n>V[_\x80\x86_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F\x95\xD8\x9BA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x05\xF6\x91\x90a\x0F\xD3V[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x060W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x065V[``\x91P[P\x91P\x91P\x81\x15a\x06\x99W_` \x82Q\x03a\x06eW\x81\x80` \x01\x90Q\x81\x01\x90a\x06^\x91\x90a\x10hV[\x90Pa\x06\x89V[_\x82\x80` \x01\x90Q\x81\x01\x90a\x06z\x91\x90a\x115V[\x90Pa\x06\x85\x81a\n\x84V[\x91PP[\x80\x88` \x01\x81\x81RPPPa\x06\xA6V[_\x80\x1B\x87` \x01\x81\x81RPP[_\x80\x88``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F\x95\xD8\x9BA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x07V\x91\x90a\x0F\xD3V[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x07\x90W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x07\x95V[``\x91P[P\x91P\x91P\x81\x15a\x07\xF9W_` \x82Q\x03a\x07\xC5W\x81\x80` \x01\x90Q\x81\x01\x90a\x07\xBE\x91\x90a\x10hV[\x90Pa\x07\xE9V[_\x82\x80` \x01\x90Q\x81\x01\x90a\x07\xDA\x91\x90a\x115V[\x90Pa\x07\xE5\x81a\n\x84V[\x91PP[\x80\x8A`\x80\x01\x81\x81RPPPa\x08\x06V[_\x80\x1B\x89`\x80\x01\x81\x81RPP[\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC4Z\x01U`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x08nWP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08k\x91\x90a\x0FVV[`\x01[a\x08\xB0W_\x89a\x01\0\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\x08\xEBV[\x80\x8Aa\x01\0\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPP[\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\t\x02\xF1\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\tSWP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tP\x91\x90a\x11\xF8V[`\x01[a\tpW_\x89`\xC0\x01\x81\x81RPP_\x89`\xE0\x01\x81\x81RPPa\n\x14V[m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x83m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x80a\t\xBDWPm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x82m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11[\x15a\t\xDBW_\x8C`\xC0\x01\x81\x81RPP_\x8C`\xE0\x01\x81\x81RPPa\n\x10V[\x82m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8C`\xC0\x01\x81\x81RPP\x81m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8C`\xE0\x01\x81\x81RPP[PPP[\x88\x8C\x8C\x81Q\x81\x10a\n(Wa\n'a\x0F\x15V[[` \x02` \x01\x01\x81\x90RPPPPPPPPPPP[\x80`\x01\x01\x90Pa\0\xB9V[P\x80\x91PP\x91\x90PV[_\x80\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a\n{W`\x01\x90Pa\n\x7FV[_\x90P[\x91\x90PV[_\x80\x82\x90P_\x81Q\x03a\n\x9CW_\x80\x1B\x91PPa\n\xA5V[` \x83\x01Q\x91PP[\x91\x90PV[`@Q\x80a\x01 \x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x80\x19\x16\x81R` \x01_`\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x80\x19\x16\x81R` \x01_`\xFF\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x0B\x96\x82a\x0BPV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0B\xB5Wa\x0B\xB4a\x0B`V[[\x80`@RPPPV[_a\x0B\xC7a\x0B;V[\x90Pa\x0B\xD3\x82\x82a\x0B\x8DV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0B\xF2Wa\x0B\xF1a\x0B`V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x80\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x0C0\x82a\x0C\x07V[\x90P\x91\x90PV[a\x0C@\x81a\x0C&V[\x81\x14a\x0CJW_\x80\xFD[PV[_\x815\x90Pa\x0C[\x81a\x0C7V[\x92\x91PPV[_a\x0Csa\x0Cn\x84a\x0B\xD8V[a\x0B\xBEV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x0C\x96Wa\x0C\x95a\x0C\x03V[[\x83[\x81\x81\x10\x15a\x0C\xBFW\x80a\x0C\xAB\x88\x82a\x0CMV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x0C\x98V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x0C\xDDWa\x0C\xDCa\x0BLV[[\x815a\x0C\xED\x84\x82` \x86\x01a\x0CaV[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\r\x0BWa\r\na\x0BDV[[_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r(Wa\r'a\x0BHV[[a\r4\x84\x82\x85\x01a\x0C\xC9V[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a\ro\x81a\x0C&V[\x82RPPV[_\x81\x90P\x91\x90PV[a\r\x87\x81a\ruV[\x82RPPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\r\xA2\x81a\r\x8DV[\x82RPPV[_\x81\x90P\x91\x90PV[a\r\xBA\x81a\r\xA8V[\x82RPPV[a\x01 \x82\x01_\x82\x01Qa\r\xD5_\x85\x01\x82a\rfV[P` \x82\x01Qa\r\xE8` \x85\x01\x82a\r~V[P`@\x82\x01Qa\r\xFB`@\x85\x01\x82a\r\x99V[P``\x82\x01Qa\x0E\x0E``\x85\x01\x82a\rfV[P`\x80\x82\x01Qa\x0E!`\x80\x85\x01\x82a\r~V[P`\xA0\x82\x01Qa\x0E4`\xA0\x85\x01\x82a\r\x99V[P`\xC0\x82\x01Qa\x0EG`\xC0\x85\x01\x82a\r\xB1V[P`\xE0\x82\x01Qa\x0EZ`\xE0\x85\x01\x82a\r\xB1V[Pa\x01\0\x82\x01Qa\x0Eoa\x01\0\x85\x01\x82a\rfV[PPPPV[_a\x0E\x80\x83\x83a\r\xC0V[a\x01 \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x0E\xA3\x82a\r=V[a\x0E\xAD\x81\x85a\rGV[\x93Pa\x0E\xB8\x83a\rWV[\x80_[\x83\x81\x10\x15a\x0E\xE8W\x81Qa\x0E\xCF\x88\x82a\x0EuV[\x97Pa\x0E\xDA\x83a\x0E\x8DV[\x92PP`\x01\x81\x01\x90Pa\x0E\xBBV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x0F\r\x81\x84a\x0E\x99V[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81Q\x90Pa\x0FP\x81a\x0C7V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0FkWa\x0Fja\x0BDV[[_a\x0Fx\x84\x82\x85\x01a\x0FBV[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x0F\xAD\x82a\x0F\x81V[a\x0F\xB7\x81\x85a\x0F\x8BV[\x93Pa\x0F\xC7\x81\x85` \x86\x01a\x0F\x95V[\x80\x84\x01\x91PP\x92\x91PPV[_a\x0F\xDE\x82\x84a\x0F\xA3V[\x91P\x81\x90P\x92\x91PPV[a\x0F\xF2\x81a\r\xA8V[\x81\x14a\x0F\xFCW_\x80\xFD[PV[_\x81Q\x90Pa\x10\r\x81a\x0F\xE9V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x10(Wa\x10'a\x0BDV[[_a\x105\x84\x82\x85\x01a\x0F\xFFV[\x91PP\x92\x91PPV[a\x10G\x81a\ruV[\x81\x14a\x10QW_\x80\xFD[PV[_\x81Q\x90Pa\x10b\x81a\x10>V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x10}Wa\x10|a\x0BDV[[_a\x10\x8A\x84\x82\x85\x01a\x10TV[\x91PP\x92\x91PPV[_\x80\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x10\xB1Wa\x10\xB0a\x0B`V[[a\x10\xBA\x82a\x0BPV[\x90P` \x81\x01\x90P\x91\x90PV[_a\x10\xD9a\x10\xD4\x84a\x10\x97V[a\x0B\xBEV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x10\xF5Wa\x10\xF4a\x10\x93V[[a\x11\0\x84\x82\x85a\x0F\x95V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x11\x1CWa\x11\x1Ba\x0BLV[[\x81Qa\x11,\x84\x82` \x86\x01a\x10\xC7V[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x11JWa\x11Ia\x0BDV[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11gWa\x11fa\x0BHV[[a\x11s\x84\x82\x85\x01a\x11\x08V[\x91PP\x92\x91PPV[_m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x11\x9E\x81a\x11|V[\x81\x14a\x11\xA8W_\x80\xFD[PV[_\x81Q\x90Pa\x11\xB9\x81a\x11\x95V[\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x11\xD7\x81a\x11\xBFV[\x81\x14a\x11\xE1W_\x80\xFD[PV[_\x81Q\x90Pa\x11\xF2\x81a\x11\xCEV[\x92\x91PPV[_\x80_``\x84\x86\x03\x12\x15a\x12\x0FWa\x12\x0Ea\x0BDV[[_a\x12\x1C\x86\x82\x87\x01a\x11\xABV[\x93PP` a\x12-\x86\x82\x87\x01a\x11\xABV[\x92PP`@a\x12>\x86\x82\x87\x01a\x11\xE4V[\x91PP\x92P\x92P\x92V\xFE\xA2dipfsX\"\x12 \xC0\x11\xCCs\x1BP\x80\x83\xEFj\xE4$=m\xADD\x18[\xA2\xA3\xD5\x82a'\xF4\xB6\x87\x7F6`\xA2\xA8dsolcC\0\x08\x1A\x003",
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
function getPoolData(address[] memory pools) external returns (PoolHelpers.UniswapV2PoolData[] memory);
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
            <PoolHelpers::UniswapV2PoolData as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Array<PoolHelpers::UniswapV2PoolData>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <PoolHelpers::UniswapV2PoolData as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Array<PoolHelpers::UniswapV2PoolData>,
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
    ///Container for all the [`GetUniV2PoolData`](self) function calls.
    pub enum GetUniV2PoolDataCalls {
        #[allow(missing_docs)]
        getPoolData(getPoolDataCall),
    }
    #[automatically_derived]
    impl GetUniV2PoolDataCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[154u8, 233u8, 25u8, 191u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for GetUniV2PoolDataCalls {
        const NAME: &'static str = "GetUniV2PoolDataCalls";
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
            ) -> alloy_sol_types::Result<GetUniV2PoolDataCalls>] = &[
                {
                    fn getPoolData(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<GetUniV2PoolDataCalls> {
                        <getPoolDataCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(GetUniV2PoolDataCalls::getPoolData)
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
    /**Creates a new wrapper around an on-chain [`GetUniV2PoolData`](self) contract instance.

See the [wrapper's documentation](`GetUniV2PoolDataInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> GetUniV2PoolDataInstance<T, P, N> {
        GetUniV2PoolDataInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<GetUniV2PoolDataInstance<T, P, N>>,
    > {
        GetUniV2PoolDataInstance::<T, P, N>::deploy(provider, pools)
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
        GetUniV2PoolDataInstance::<T, P, N>::deploy_builder(provider, pools)
    }
    /**A [`GetUniV2PoolData`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`GetUniV2PoolData`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct GetUniV2PoolDataInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for GetUniV2PoolDataInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("GetUniV2PoolDataInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > GetUniV2PoolDataInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`GetUniV2PoolData`](self) contract instance.

See the [wrapper's documentation](`GetUniV2PoolDataInstance`) for more details.*/
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
        ) -> alloy_contract::Result<GetUniV2PoolDataInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> GetUniV2PoolDataInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> GetUniV2PoolDataInstance<T, P, N> {
            GetUniV2PoolDataInstance {
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
    > GetUniV2PoolDataInstance<T, P, N> {
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
    > GetUniV2PoolDataInstance<T, P, N> {
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
