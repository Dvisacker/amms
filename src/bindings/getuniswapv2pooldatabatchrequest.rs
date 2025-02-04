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

interface GetUniswapV2PoolDataBatchRequest {
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
pub mod GetUniswapV2PoolDataBatchRequest {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b50604051610ceb380380610ceb8339818101604052810190610031919061092d565b5f6100418261007060201b60201c565b90505f816040516020016100559190610b2c565b60405160208183030381529060405290506020810180590381f35b60605f825167ffffffffffffffff81111561008e5761008d610797565b5b6040519080825280602002602001820160405280156100c757816020015b6100b46106e1565b8152602001906001900390816100ac5790505b5090505f5b83518110156106a6575f8482815181106100e9576100e8610b4c565b5b60200260200101519050610102816106b060201b60201c565b1561010d575061069b565b6101156106e1565b8173ffffffffffffffffffffffffffffffffffffffff16630dfe16816040518163ffffffff1660e01b8152600401602060405180830381865afa15801561015e573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101829190610b79565b815f019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff16815250508173ffffffffffffffffffffffffffffffffffffffff1663d21220a76040518163ffffffff1660e01b8152600401602060405180830381865afa158015610201573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102259190610b79565b816060019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff168152505061026e815f01516106b060201b60201c565b1561027a57505061069b565b61028d81606001516106b060201b60201c565b1561029957505061069b565b5f80825f015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f313ce567000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506040516103489190610bf6565b5f604051808303815f8787f1925050503d805f8114610382576040519150601f19603f3d011682016040523d82523d5f602084013e610387565b606091505b509150915081156103f6575f60208251036103e657818060200190518101906103b09190610c36565b90505f8114806103c0575060ff81115b156103cf57505050505061069b565b80846040019060ff16908160ff16815250506103f0565b505050505061069b565b506103ff565b5050505061069b565b5f80846060015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f313ce567000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506040516104af9190610bf6565b5f604051808303815f8787f1925050503d805f81146104e9576040519150601f19603f3d011682016040523d82523d5f602084013e6104ee565b606091505b50915091508115610561575f602082510361054f57818060200190518101906105179190610c36565b90505f811480610527575060ff81115b15610538575050505050505061069b565b808660a0019060ff16908160ff168152505061055b565b5050505050505061069b565b5061056c565b50505050505061069b565b8573ffffffffffffffffffffffffffffffffffffffff16630902f1ac6040518163ffffffff1660e01b8152600401606060405180830381865afa9250505080156105d457506040513d601f19601f820116820180604052508101906105d19190610c9a565b60015b6105f1575f8560c00181815250505f8560e0018181525050610675565b6dffffffffffffffffffffffffffff801683118061061e57506dffffffffffffffffffffffffffff801682115b1561063c575f8860c00181815250505f8860e0018181525050610671565b826dffffffffffffffffffffffffffff168860c0018181525050816dffffffffffffffffffffffffffff168860e00181815250505b5050505b8488888151811061068957610688610b4c565b5b60200260200101819052505050505050505b8060010190506100cc565b5080915050919050565b5f808273ffffffffffffffffffffffffffffffffffffffff163b036106d857600190506106dc565b5f90505b919050565b6040518061012001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f80191681526020015f60ff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f80191681526020015f60ff1681526020015f81526020015f81526020015f73ffffffffffffffffffffffffffffffffffffffff1681525090565b5f604051905090565b5f80fd5b5f80fd5b5f80fd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6107cd82610787565b810181811067ffffffffffffffff821117156107ec576107eb610797565b5b80604052505050565b5f6107fe610772565b905061080a82826107c4565b919050565b5f67ffffffffffffffff82111561082957610828610797565b5b602082029050602081019050919050565b5f80fd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6108678261083e565b9050919050565b6108778161085d565b8114610881575f80fd5b50565b5f815190506108928161086e565b92915050565b5f6108aa6108a58461080f565b6107f5565b905080838252602082019050602084028301858111156108cd576108cc61083a565b5b835b818110156108f657806108e28882610884565b8452602084019350506020810190506108cf565b5050509392505050565b5f82601f83011261091457610913610783565b5b8151610924848260208601610898565b91505092915050565b5f602082840312156109425761094161077b565b5b5f82015167ffffffffffffffff81111561095f5761095e61077f565b5b61096b84828501610900565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b6109a68161085d565b82525050565b5f819050919050565b6109be816109ac565b82525050565b5f60ff82169050919050565b6109d9816109c4565b82525050565b5f819050919050565b6109f1816109df565b82525050565b61012082015f820151610a0c5f85018261099d565b506020820151610a1f60208501826109b5565b506040820151610a3260408501826109d0565b506060820151610a45606085018261099d565b506080820151610a5860808501826109b5565b5060a0820151610a6b60a08501826109d0565b5060c0820151610a7e60c08501826109e8565b5060e0820151610a9160e08501826109e8565b50610100820151610aa661010085018261099d565b50505050565b5f610ab783836109f7565b6101208301905092915050565b5f602082019050919050565b5f610ada82610974565b610ae4818561097e565b9350610aef8361098e565b805f5b83811015610b1f578151610b068882610aac565b9750610b1183610ac4565b925050600181019050610af2565b5085935050505092915050565b5f6020820190508181035f830152610b448184610ad0565b905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f60208284031215610b8e57610b8d61077b565b5b5f610b9b84828501610884565b91505092915050565b5f81519050919050565b5f81905092915050565b8281835e5f83830152505050565b5f610bd082610ba4565b610bda8185610bae565b9350610bea818560208601610bb8565b80840191505092915050565b5f610c018284610bc6565b915081905092915050565b610c15816109df565b8114610c1f575f80fd5b50565b5f81519050610c3081610c0c565b92915050565b5f60208284031215610c4b57610c4a61077b565b5b5f610c5884828501610c22565b91505092915050565b5f63ffffffff82169050919050565b610c7981610c61565b8114610c83575f80fd5b50565b5f81519050610c9481610c70565b92915050565b5f805f60608486031215610cb157610cb061077b565b5b5f610cbe86828701610c22565b9350506020610ccf86828701610c22565b9250506040610ce086828701610c86565b915050925092509256fe
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`@Qa\x0C\xEB8\x03\x80a\x0C\xEB\x839\x81\x81\x01`@R\x81\x01\x90a\x001\x91\x90a\t-V[_a\0A\x82a\0p` \x1B` \x1CV[\x90P_\x81`@Q` \x01a\0U\x91\x90a\x0B,V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P` \x81\x01\x80Y\x03\x81\xF3[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\x8EWa\0\x8Da\x07\x97V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\xC7W\x81` \x01[a\0\xB4a\x06\xE1V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\0\xACW\x90P[P\x90P_[\x83Q\x81\x10\x15a\x06\xA6W_\x84\x82\x81Q\x81\x10a\0\xE9Wa\0\xE8a\x0BLV[[` \x02` \x01\x01Q\x90Pa\x01\x02\x81a\x06\xB0` \x1B` \x1CV[\x15a\x01\rWPa\x06\x9BV[a\x01\x15a\x06\xE1V[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01^W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x82\x91\x90a\x0ByV[\x81_\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD2\x12 \xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x01W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02%\x91\x90a\x0ByV[\x81``\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\x02n\x81_\x01Qa\x06\xB0` \x1B` \x1CV[\x15a\x02zWPPa\x06\x9BV[a\x02\x8D\x81``\x01Qa\x06\xB0` \x1B` \x1CV[\x15a\x02\x99WPPa\x06\x9BV[_\x80\x82_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x03H\x91\x90a\x0B\xF6V[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x03\x82W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x03\x87V[``\x91P[P\x91P\x91P\x81\x15a\x03\xF6W_` \x82Q\x03a\x03\xE6W\x81\x80` \x01\x90Q\x81\x01\x90a\x03\xB0\x91\x90a\x0C6V[\x90P_\x81\x14\x80a\x03\xC0WP`\xFF\x81\x11[\x15a\x03\xCFWPPPPPa\x06\x9BV[\x80\x84`@\x01\x90`\xFF\x16\x90\x81`\xFF\x16\x81RPPa\x03\xF0V[PPPPPa\x06\x9BV[Pa\x03\xFFV[PPPPa\x06\x9BV[_\x80\x84``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x04\xAF\x91\x90a\x0B\xF6V[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x04\xE9W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x04\xEEV[``\x91P[P\x91P\x91P\x81\x15a\x05aW_` \x82Q\x03a\x05OW\x81\x80` \x01\x90Q\x81\x01\x90a\x05\x17\x91\x90a\x0C6V[\x90P_\x81\x14\x80a\x05'WP`\xFF\x81\x11[\x15a\x058WPPPPPPPa\x06\x9BV[\x80\x86`\xA0\x01\x90`\xFF\x16\x90\x81`\xFF\x16\x81RPPa\x05[V[PPPPPPPa\x06\x9BV[Pa\x05lV[PPPPPPa\x06\x9BV[\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\t\x02\xF1\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x05\xD4WP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xD1\x91\x90a\x0C\x9AV[`\x01[a\x05\xF1W_\x85`\xC0\x01\x81\x81RPP_\x85`\xE0\x01\x81\x81RPPa\x06uV[m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x83\x11\x80a\x06\x1EWPm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x82\x11[\x15a\x06<W_\x88`\xC0\x01\x81\x81RPP_\x88`\xE0\x01\x81\x81RPPa\x06qV[\x82m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88`\xC0\x01\x81\x81RPP\x81m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88`\xE0\x01\x81\x81RPP[PPP[\x84\x88\x88\x81Q\x81\x10a\x06\x89Wa\x06\x88a\x0BLV[[` \x02` \x01\x01\x81\x90RPPPPPPP[\x80`\x01\x01\x90Pa\0\xCCV[P\x80\x91PP\x91\x90PV[_\x80\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a\x06\xD8W`\x01\x90Pa\x06\xDCV[_\x90P[\x91\x90PV[`@Q\x80a\x01 \x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x80\x19\x16\x81R` \x01_`\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x80\x19\x16\x81R` \x01_`\xFF\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x07\xCD\x82a\x07\x87V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x07\xECWa\x07\xEBa\x07\x97V[[\x80`@RPPPV[_a\x07\xFEa\x07rV[\x90Pa\x08\n\x82\x82a\x07\xC4V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x08)Wa\x08(a\x07\x97V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x80\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x08g\x82a\x08>V[\x90P\x91\x90PV[a\x08w\x81a\x08]V[\x81\x14a\x08\x81W_\x80\xFD[PV[_\x81Q\x90Pa\x08\x92\x81a\x08nV[\x92\x91PPV[_a\x08\xAAa\x08\xA5\x84a\x08\x0FV[a\x07\xF5V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x08\xCDWa\x08\xCCa\x08:V[[\x83[\x81\x81\x10\x15a\x08\xF6W\x80a\x08\xE2\x88\x82a\x08\x84V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x08\xCFV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\t\x14Wa\t\x13a\x07\x83V[[\x81Qa\t$\x84\x82` \x86\x01a\x08\x98V[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\tBWa\tAa\x07{V[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t_Wa\t^a\x07\x7FV[[a\tk\x84\x82\x85\x01a\t\0V[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a\t\xA6\x81a\x08]V[\x82RPPV[_\x81\x90P\x91\x90PV[a\t\xBE\x81a\t\xACV[\x82RPPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\t\xD9\x81a\t\xC4V[\x82RPPV[_\x81\x90P\x91\x90PV[a\t\xF1\x81a\t\xDFV[\x82RPPV[a\x01 \x82\x01_\x82\x01Qa\n\x0C_\x85\x01\x82a\t\x9DV[P` \x82\x01Qa\n\x1F` \x85\x01\x82a\t\xB5V[P`@\x82\x01Qa\n2`@\x85\x01\x82a\t\xD0V[P``\x82\x01Qa\nE``\x85\x01\x82a\t\x9DV[P`\x80\x82\x01Qa\nX`\x80\x85\x01\x82a\t\xB5V[P`\xA0\x82\x01Qa\nk`\xA0\x85\x01\x82a\t\xD0V[P`\xC0\x82\x01Qa\n~`\xC0\x85\x01\x82a\t\xE8V[P`\xE0\x82\x01Qa\n\x91`\xE0\x85\x01\x82a\t\xE8V[Pa\x01\0\x82\x01Qa\n\xA6a\x01\0\x85\x01\x82a\t\x9DV[PPPPV[_a\n\xB7\x83\x83a\t\xF7V[a\x01 \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\n\xDA\x82a\ttV[a\n\xE4\x81\x85a\t~V[\x93Pa\n\xEF\x83a\t\x8EV[\x80_[\x83\x81\x10\x15a\x0B\x1FW\x81Qa\x0B\x06\x88\x82a\n\xACV[\x97Pa\x0B\x11\x83a\n\xC4V[\x92PP`\x01\x81\x01\x90Pa\n\xF2V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x0BD\x81\x84a\n\xD0V[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\x0B\x8EWa\x0B\x8Da\x07{V[[_a\x0B\x9B\x84\x82\x85\x01a\x08\x84V[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x0B\xD0\x82a\x0B\xA4V[a\x0B\xDA\x81\x85a\x0B\xAEV[\x93Pa\x0B\xEA\x81\x85` \x86\x01a\x0B\xB8V[\x80\x84\x01\x91PP\x92\x91PPV[_a\x0C\x01\x82\x84a\x0B\xC6V[\x91P\x81\x90P\x92\x91PPV[a\x0C\x15\x81a\t\xDFV[\x81\x14a\x0C\x1FW_\x80\xFD[PV[_\x81Q\x90Pa\x0C0\x81a\x0C\x0CV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0CKWa\x0CJa\x07{V[[_a\x0CX\x84\x82\x85\x01a\x0C\"V[\x91PP\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x0Cy\x81a\x0CaV[\x81\x14a\x0C\x83W_\x80\xFD[PV[_\x81Q\x90Pa\x0C\x94\x81a\x0CpV[\x92\x91PPV[_\x80_``\x84\x86\x03\x12\x15a\x0C\xB1Wa\x0C\xB0a\x07{V[[_a\x0C\xBE\x86\x82\x87\x01a\x0C\"V[\x93PP` a\x0C\xCF\x86\x82\x87\x01a\x0C\"V[\x92PP`@a\x0C\xE0\x86\x82\x87\x01a\x0C\x86V[\x91PP\x92P\x92P\x92V\xFE",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b5060043610610029575f3560e01c80639ae919bf1461002d575b5f80fd5b61004760048036038101906100429190610908565b61005d565b6040516100549190610b07565b60405180910390f35b60605f825167ffffffffffffffff81111561007b5761007a610772565b5b6040519080825280602002602001820160405280156100b457816020015b6100a16106bc565b8152602001906001900390816100995790505b5090505f5b8351811015610681575f8482815181106100d6576100d5610b27565b5b602002602001015190506100e98161068b565b156100f45750610676565b6100fc6106bc565b8173ffffffffffffffffffffffffffffffffffffffff16630dfe16816040518163ffffffff1660e01b8152600401602060405180830381865afa158015610145573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101699190610b68565b815f019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff16815250508173ffffffffffffffffffffffffffffffffffffffff1663d21220a76040518163ffffffff1660e01b8152600401602060405180830381865afa1580156101e8573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061020c9190610b68565b816060019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff168152505061024f815f015161068b565b1561025b575050610676565b610268816060015161068b565b15610274575050610676565b5f80825f015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f313ce567000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506040516103239190610be5565b5f604051808303815f8787f1925050503d805f811461035d576040519150601f19603f3d011682016040523d82523d5f602084013e610362565b606091505b509150915081156103d1575f60208251036103c1578180602001905181019061038b9190610c25565b90505f81148061039b575060ff81115b156103aa575050505050610676565b80846040019060ff16908160ff16815250506103cb565b5050505050610676565b506103da565b50505050610676565b5f80846060015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f313ce567000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff838183161783525050505060405161048a9190610be5565b5f604051808303815f8787f1925050503d805f81146104c4576040519150601f19603f3d011682016040523d82523d5f602084013e6104c9565b606091505b5091509150811561053c575f602082510361052a57818060200190518101906104f29190610c25565b90505f811480610502575060ff81115b156105135750505050505050610676565b808660a0019060ff16908160ff1681525050610536565b50505050505050610676565b50610547565b505050505050610676565b8573ffffffffffffffffffffffffffffffffffffffff16630902f1ac6040518163ffffffff1660e01b8152600401606060405180830381865afa9250505080156105af57506040513d601f19601f820116820180604052508101906105ac9190610c89565b60015b6105cc575f8560c00181815250505f8560e0018181525050610650565b6dffffffffffffffffffffffffffff80168311806105f957506dffffffffffffffffffffffffffff801682115b15610617575f8860c00181815250505f8860e001818152505061064c565b826dffffffffffffffffffffffffffff168860c0018181525050816dffffffffffffffffffffffffffff168860e00181815250505b5050505b8488888151811061066457610663610b27565b5b60200260200101819052505050505050505b8060010190506100b9565b5080915050919050565b5f808273ffffffffffffffffffffffffffffffffffffffff163b036106b357600190506106b7565b5f90505b919050565b6040518061012001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f80191681526020015f60ff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f80191681526020015f60ff1681526020015f81526020015f81526020015f73ffffffffffffffffffffffffffffffffffffffff1681525090565b5f604051905090565b5f80fd5b5f80fd5b5f80fd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6107a882610762565b810181811067ffffffffffffffff821117156107c7576107c6610772565b5b80604052505050565b5f6107d961074d565b90506107e5828261079f565b919050565b5f67ffffffffffffffff82111561080457610803610772565b5b602082029050602081019050919050565b5f80fd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61084282610819565b9050919050565b61085281610838565b811461085c575f80fd5b50565b5f8135905061086d81610849565b92915050565b5f610885610880846107ea565b6107d0565b905080838252602082019050602084028301858111156108a8576108a7610815565b5b835b818110156108d157806108bd888261085f565b8452602084019350506020810190506108aa565b5050509392505050565b5f82601f8301126108ef576108ee61075e565b5b81356108ff848260208601610873565b91505092915050565b5f6020828403121561091d5761091c610756565b5b5f82013567ffffffffffffffff81111561093a5761093961075a565b5b610946848285016108db565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b61098181610838565b82525050565b5f819050919050565b61099981610987565b82525050565b5f60ff82169050919050565b6109b48161099f565b82525050565b5f819050919050565b6109cc816109ba565b82525050565b61012082015f8201516109e75f850182610978565b5060208201516109fa6020850182610990565b506040820151610a0d60408501826109ab565b506060820151610a206060850182610978565b506080820151610a336080850182610990565b5060a0820151610a4660a08501826109ab565b5060c0820151610a5960c08501826109c3565b5060e0820151610a6c60e08501826109c3565b50610100820151610a81610100850182610978565b50505050565b5f610a9283836109d2565b6101208301905092915050565b5f602082019050919050565b5f610ab58261094f565b610abf8185610959565b9350610aca83610969565b805f5b83811015610afa578151610ae18882610a87565b9750610aec83610a9f565b925050600181019050610acd565b5085935050505092915050565b5f6020820190508181035f830152610b1f8184610aab565b905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f81519050610b6281610849565b92915050565b5f60208284031215610b7d57610b7c610756565b5b5f610b8a84828501610b54565b91505092915050565b5f81519050919050565b5f81905092915050565b8281835e5f83830152505050565b5f610bbf82610b93565b610bc98185610b9d565b9350610bd9818560208601610ba7565b80840191505092915050565b5f610bf08284610bb5565b915081905092915050565b610c04816109ba565b8114610c0e575f80fd5b50565b5f81519050610c1f81610bfb565b92915050565b5f60208284031215610c3a57610c39610756565b5b5f610c4784828501610c11565b91505092915050565b5f63ffffffff82169050919050565b610c6881610c50565b8114610c72575f80fd5b50565b5f81519050610c8381610c5f565b92915050565b5f805f60608486031215610ca057610c9f610756565b5b5f610cad86828701610c11565b9350506020610cbe86828701610c11565b9250506040610ccf86828701610c75565b915050925092509256fea26469706673582212208df6c810877c7bddbab6367fa21854b0516227909a3fb7b62790c2e5870a91aa64736f6c634300081a0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0)W_5`\xE0\x1C\x80c\x9A\xE9\x19\xBF\x14a\0-W[_\x80\xFD[a\0G`\x04\x806\x03\x81\x01\x90a\0B\x91\x90a\t\x08V[a\0]V[`@Qa\0T\x91\x90a\x0B\x07V[`@Q\x80\x91\x03\x90\xF3[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0{Wa\0za\x07rV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\xB4W\x81` \x01[a\0\xA1a\x06\xBCV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\0\x99W\x90P[P\x90P_[\x83Q\x81\x10\x15a\x06\x81W_\x84\x82\x81Q\x81\x10a\0\xD6Wa\0\xD5a\x0B'V[[` \x02` \x01\x01Q\x90Pa\0\xE9\x81a\x06\x8BV[\x15a\0\xF4WPa\x06vV[a\0\xFCa\x06\xBCV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01EW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01i\x91\x90a\x0BhV[\x81_\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD2\x12 \xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xE8W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x0C\x91\x90a\x0BhV[\x81``\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\x02O\x81_\x01Qa\x06\x8BV[\x15a\x02[WPPa\x06vV[a\x02h\x81``\x01Qa\x06\x8BV[\x15a\x02tWPPa\x06vV[_\x80\x82_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x03#\x91\x90a\x0B\xE5V[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x03]W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x03bV[``\x91P[P\x91P\x91P\x81\x15a\x03\xD1W_` \x82Q\x03a\x03\xC1W\x81\x80` \x01\x90Q\x81\x01\x90a\x03\x8B\x91\x90a\x0C%V[\x90P_\x81\x14\x80a\x03\x9BWP`\xFF\x81\x11[\x15a\x03\xAAWPPPPPa\x06vV[\x80\x84`@\x01\x90`\xFF\x16\x90\x81`\xFF\x16\x81RPPa\x03\xCBV[PPPPPa\x06vV[Pa\x03\xDAV[PPPPa\x06vV[_\x80\x84``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x04\x8A\x91\x90a\x0B\xE5V[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x04\xC4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x04\xC9V[``\x91P[P\x91P\x91P\x81\x15a\x05<W_` \x82Q\x03a\x05*W\x81\x80` \x01\x90Q\x81\x01\x90a\x04\xF2\x91\x90a\x0C%V[\x90P_\x81\x14\x80a\x05\x02WP`\xFF\x81\x11[\x15a\x05\x13WPPPPPPPa\x06vV[\x80\x86`\xA0\x01\x90`\xFF\x16\x90\x81`\xFF\x16\x81RPPa\x056V[PPPPPPPa\x06vV[Pa\x05GV[PPPPPPa\x06vV[\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\t\x02\xF1\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x05\xAFWP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xAC\x91\x90a\x0C\x89V[`\x01[a\x05\xCCW_\x85`\xC0\x01\x81\x81RPP_\x85`\xE0\x01\x81\x81RPPa\x06PV[m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x83\x11\x80a\x05\xF9WPm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x82\x11[\x15a\x06\x17W_\x88`\xC0\x01\x81\x81RPP_\x88`\xE0\x01\x81\x81RPPa\x06LV[\x82m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88`\xC0\x01\x81\x81RPP\x81m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88`\xE0\x01\x81\x81RPP[PPP[\x84\x88\x88\x81Q\x81\x10a\x06dWa\x06ca\x0B'V[[` \x02` \x01\x01\x81\x90RPPPPPPP[\x80`\x01\x01\x90Pa\0\xB9V[P\x80\x91PP\x91\x90PV[_\x80\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a\x06\xB3W`\x01\x90Pa\x06\xB7V[_\x90P[\x91\x90PV[`@Q\x80a\x01 \x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x80\x19\x16\x81R` \x01_`\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x80\x19\x16\x81R` \x01_`\xFF\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x07\xA8\x82a\x07bV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x07\xC7Wa\x07\xC6a\x07rV[[\x80`@RPPPV[_a\x07\xD9a\x07MV[\x90Pa\x07\xE5\x82\x82a\x07\x9FV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x08\x04Wa\x08\x03a\x07rV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x80\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x08B\x82a\x08\x19V[\x90P\x91\x90PV[a\x08R\x81a\x088V[\x81\x14a\x08\\W_\x80\xFD[PV[_\x815\x90Pa\x08m\x81a\x08IV[\x92\x91PPV[_a\x08\x85a\x08\x80\x84a\x07\xEAV[a\x07\xD0V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x08\xA8Wa\x08\xA7a\x08\x15V[[\x83[\x81\x81\x10\x15a\x08\xD1W\x80a\x08\xBD\x88\x82a\x08_V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x08\xAAV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x08\xEFWa\x08\xEEa\x07^V[[\x815a\x08\xFF\x84\x82` \x86\x01a\x08sV[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\t\x1DWa\t\x1Ca\x07VV[[_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t:Wa\t9a\x07ZV[[a\tF\x84\x82\x85\x01a\x08\xDBV[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a\t\x81\x81a\x088V[\x82RPPV[_\x81\x90P\x91\x90PV[a\t\x99\x81a\t\x87V[\x82RPPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\t\xB4\x81a\t\x9FV[\x82RPPV[_\x81\x90P\x91\x90PV[a\t\xCC\x81a\t\xBAV[\x82RPPV[a\x01 \x82\x01_\x82\x01Qa\t\xE7_\x85\x01\x82a\txV[P` \x82\x01Qa\t\xFA` \x85\x01\x82a\t\x90V[P`@\x82\x01Qa\n\r`@\x85\x01\x82a\t\xABV[P``\x82\x01Qa\n ``\x85\x01\x82a\txV[P`\x80\x82\x01Qa\n3`\x80\x85\x01\x82a\t\x90V[P`\xA0\x82\x01Qa\nF`\xA0\x85\x01\x82a\t\xABV[P`\xC0\x82\x01Qa\nY`\xC0\x85\x01\x82a\t\xC3V[P`\xE0\x82\x01Qa\nl`\xE0\x85\x01\x82a\t\xC3V[Pa\x01\0\x82\x01Qa\n\x81a\x01\0\x85\x01\x82a\txV[PPPPV[_a\n\x92\x83\x83a\t\xD2V[a\x01 \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\n\xB5\x82a\tOV[a\n\xBF\x81\x85a\tYV[\x93Pa\n\xCA\x83a\tiV[\x80_[\x83\x81\x10\x15a\n\xFAW\x81Qa\n\xE1\x88\x82a\n\x87V[\x97Pa\n\xEC\x83a\n\x9FV[\x92PP`\x01\x81\x01\x90Pa\n\xCDV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x0B\x1F\x81\x84a\n\xABV[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81Q\x90Pa\x0Bb\x81a\x08IV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0B}Wa\x0B|a\x07VV[[_a\x0B\x8A\x84\x82\x85\x01a\x0BTV[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x0B\xBF\x82a\x0B\x93V[a\x0B\xC9\x81\x85a\x0B\x9DV[\x93Pa\x0B\xD9\x81\x85` \x86\x01a\x0B\xA7V[\x80\x84\x01\x91PP\x92\x91PPV[_a\x0B\xF0\x82\x84a\x0B\xB5V[\x91P\x81\x90P\x92\x91PPV[a\x0C\x04\x81a\t\xBAV[\x81\x14a\x0C\x0EW_\x80\xFD[PV[_\x81Q\x90Pa\x0C\x1F\x81a\x0B\xFBV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0C:Wa\x0C9a\x07VV[[_a\x0CG\x84\x82\x85\x01a\x0C\x11V[\x91PP\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x0Ch\x81a\x0CPV[\x81\x14a\x0CrW_\x80\xFD[PV[_\x81Q\x90Pa\x0C\x83\x81a\x0C_V[\x92\x91PPV[_\x80_``\x84\x86\x03\x12\x15a\x0C\xA0Wa\x0C\x9Fa\x07VV[[_a\x0C\xAD\x86\x82\x87\x01a\x0C\x11V[\x93PP` a\x0C\xBE\x86\x82\x87\x01a\x0C\x11V[\x92PP`@a\x0C\xCF\x86\x82\x87\x01a\x0CuV[\x91PP\x92P\x92P\x92V\xFE\xA2dipfsX\"\x12 \x8D\xF6\xC8\x10\x87|{\xDD\xBA\xB66\x7F\xA2\x18T\xB0Qb'\x90\x9A?\xB7\xB6'\x90\xC2\xE5\x87\n\x91\xAAdsolcC\0\x08\x1A\x003",
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
    ///Container for all the [`GetUniswapV2PoolDataBatchRequest`](self) function calls.
    pub enum GetUniswapV2PoolDataBatchRequestCalls {
        #[allow(missing_docs)]
        getPoolData(getPoolDataCall),
    }
    #[automatically_derived]
    impl GetUniswapV2PoolDataBatchRequestCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[154u8, 233u8, 25u8, 191u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for GetUniswapV2PoolDataBatchRequestCalls {
        const NAME: &'static str = "GetUniswapV2PoolDataBatchRequestCalls";
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
            ) -> alloy_sol_types::Result<GetUniswapV2PoolDataBatchRequestCalls>] = &[
                {
                    fn getPoolData(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<GetUniswapV2PoolDataBatchRequestCalls> {
                        <getPoolDataCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(GetUniswapV2PoolDataBatchRequestCalls::getPoolData)
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
    /**Creates a new wrapper around an on-chain [`GetUniswapV2PoolDataBatchRequest`](self) contract instance.

See the [wrapper's documentation](`GetUniswapV2PoolDataBatchRequestInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> GetUniswapV2PoolDataBatchRequestInstance<T, P, N> {
        GetUniswapV2PoolDataBatchRequestInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<
            GetUniswapV2PoolDataBatchRequestInstance<T, P, N>,
        >,
    > {
        GetUniswapV2PoolDataBatchRequestInstance::<T, P, N>::deploy(provider, pools)
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
        GetUniswapV2PoolDataBatchRequestInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, pools)
    }
    /**A [`GetUniswapV2PoolDataBatchRequest`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`GetUniswapV2PoolDataBatchRequest`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct GetUniswapV2PoolDataBatchRequestInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug
    for GetUniswapV2PoolDataBatchRequestInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("GetUniswapV2PoolDataBatchRequestInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > GetUniswapV2PoolDataBatchRequestInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`GetUniswapV2PoolDataBatchRequest`](self) contract instance.

See the [wrapper's documentation](`GetUniswapV2PoolDataBatchRequestInstance`) for more details.*/
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
        ) -> alloy_contract::Result<GetUniswapV2PoolDataBatchRequestInstance<T, P, N>> {
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
    impl<
        T,
        P: ::core::clone::Clone,
        N,
    > GetUniswapV2PoolDataBatchRequestInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(
            self,
        ) -> GetUniswapV2PoolDataBatchRequestInstance<T, P, N> {
            GetUniswapV2PoolDataBatchRequestInstance {
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
    > GetUniswapV2PoolDataBatchRequestInstance<T, P, N> {
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
    > GetUniswapV2PoolDataBatchRequestInstance<T, P, N> {
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
