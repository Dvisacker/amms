///Module containing a contract's types and functions.
/**

```solidity
library PoolHelpers {
    struct UniswapV3PoolPriceData { uint128 liquidity; uint160 sqrtPrice; int24 tick; int128 liquidityNet; }
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
struct UniswapV3PoolPriceData { uint128 liquidity; uint160 sqrtPrice; int24 tick; int128 liquidityNet; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UniswapV3PoolPriceData {
        #[allow(missing_docs)]
        pub liquidity: u128,
        #[allow(missing_docs)]
        pub sqrtPrice: alloy::sol_types::private::primitives::aliases::U160,
        #[allow(missing_docs)]
        pub tick: alloy::sol_types::private::primitives::aliases::I24,
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
            alloy::sol_types::sol_data::Uint<128>,
            alloy::sol_types::sol_data::Uint<160>,
            alloy::sol_types::sol_data::Int<24>,
            alloy::sol_types::sol_data::Int<128>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u128,
            alloy::sol_types::private::primitives::aliases::U160,
            alloy::sol_types::private::primitives::aliases::I24,
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
        impl ::core::convert::From<UniswapV3PoolPriceData> for UnderlyingRustTuple<'_> {
            fn from(value: UniswapV3PoolPriceData) -> Self {
                (value.liquidity, value.sqrtPrice, value.tick, value.liquidityNet)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UniswapV3PoolPriceData {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    liquidity: tuple.0,
                    sqrtPrice: tuple.1,
                    tick: tuple.2,
                    liquidityNet: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for UniswapV3PoolPriceData {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for UniswapV3PoolPriceData {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
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
        impl alloy_sol_types::SolType for UniswapV3PoolPriceData {
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
        impl alloy_sol_types::SolStruct for UniswapV3PoolPriceData {
            const NAME: &'static str = "UniswapV3PoolPriceData";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "UniswapV3PoolPriceData(uint128 liquidity,uint160 sqrtPrice,int24 tick,int128 liquidityNet)",
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
                        128,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.liquidityNet)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for UniswapV3PoolPriceData {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
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
    struct UniswapV3PoolPriceData {
        uint128 liquidity;
        uint160 sqrtPrice;
        int24 tick;
        int128 liquidityNet;
    }
}

interface SyncUniswapV3PoolBatchRequest {
    constructor(address[] pools);

    function getPoolData(address[] memory pools) external view returns (PoolHelpers.UniswapV3PoolPriceData[] memory);
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
        "internalType": "struct PoolHelpers.UniswapV3PoolPriceData[]",
        "components": [
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
            "name": "liquidityNet",
            "type": "int128",
            "internalType": "int128"
          }
        ]
      }
    ],
    "stateMutability": "view"
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
pub mod SyncUniswapV3PoolBatchRequest {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b50604051610b07380380610b078339818101604052810190610031919061057b565b5f6100418261006c60201b60201c565b604051602001610051919061072c565b60405160208183030381529060405290506020810180590381f35b60605f825167ffffffffffffffff81111561008a576100896103e5565b5b6040519080825280602002602001820160405280156100c357816020015b6100b061036e565b8152602001906001900390816100a85790505b5090505f5b8351811015610333575f8482815181106100e5576100e461074c565b5b602002602001015190506100fe8161033d60201b60201c565b156101095750610328565b61011161036e565b5f808373ffffffffffffffffffffffffffffffffffffffff16633850c7bd6040518163ffffffff1660e01b815260040160e060405180830381865afa15801561015c573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610180919061086f565b5050505050915091505f8473ffffffffffffffffffffffffffffffffffffffff1663f30dba93836040518263ffffffff1660e01b81526004016101c3919061091b565b61010060405180830381865afa1580156101df573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102039190610a2a565b5050505050509150508473ffffffffffffffffffffffffffffffffffffffff16631a6865026040518163ffffffff1660e01b8152600401602060405180830381865afa158015610255573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102799190610adb565b845f01906fffffffffffffffffffffffffffffffff1690816fffffffffffffffffffffffffffffffff168152505082846020019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff168152505081846040019060020b908160020b81525050808460600190600f0b9081600f0b81525050838787815181106103175761031661074c565b5b602002602001018190525050505050505b8060010190506100c8565b5080915050919050565b5f808273ffffffffffffffffffffffffffffffffffffffff163b036103655760019050610369565b5f90505b919050565b60405180608001604052805f6fffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f60020b81526020015f600f0b81525090565b5f604051905090565b5f80fd5b5f80fd5b5f80fd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b61041b826103d5565b810181811067ffffffffffffffff8211171561043a576104396103e5565b5b80604052505050565b5f61044c6103c0565b90506104588282610412565b919050565b5f67ffffffffffffffff821115610477576104766103e5565b5b602082029050602081019050919050565b5f80fd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6104b58261048c565b9050919050565b6104c5816104ab565b81146104cf575f80fd5b50565b5f815190506104e0816104bc565b92915050565b5f6104f86104f38461045d565b610443565b9050808382526020820190506020840283018581111561051b5761051a610488565b5b835b81811015610544578061053088826104d2565b84526020840193505060208101905061051d565b5050509392505050565b5f82601f830112610562576105616103d1565b5b81516105728482602086016104e6565b91505092915050565b5f602082840312156105905761058f6103c9565b5b5f82015167ffffffffffffffff8111156105ad576105ac6103cd565b5b6105b98482850161054e565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f6fffffffffffffffffffffffffffffffff82169050919050565b61060f816105eb565b82525050565b61061e8161048c565b82525050565b5f8160020b9050919050565b61063981610624565b82525050565b5f81600f0b9050919050565b6106548161063f565b82525050565b608082015f82015161066e5f850182610606565b5060208201516106816020850182610615565b5060408201516106946040850182610630565b5060608201516106a7606085018261064b565b50505050565b5f6106b8838361065a565b60808301905092915050565b5f602082019050919050565b5f6106da826105c2565b6106e481856105cc565b93506106ef836105dc565b805f5b8381101561071f57815161070688826106ad565b9750610711836106c4565b9250506001810190506106f2565b5085935050505092915050565b5f6020820190508181035f83015261074481846106d0565b905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b6107828161048c565b811461078c575f80fd5b50565b5f8151905061079d81610779565b92915050565b6107ac81610624565b81146107b6575f80fd5b50565b5f815190506107c7816107a3565b92915050565b5f61ffff82169050919050565b6107e3816107cd565b81146107ed575f80fd5b50565b5f815190506107fe816107da565b92915050565b5f60ff82169050919050565b61081981610804565b8114610823575f80fd5b50565b5f8151905061083481610810565b92915050565b5f8115159050919050565b61084e8161083a565b8114610858575f80fd5b50565b5f8151905061086981610845565b92915050565b5f805f805f805f60e0888a03121561088a576108896103c9565b5b5f6108978a828b0161078f565b97505060206108a88a828b016107b9565b96505060406108b98a828b016107f0565b95505060606108ca8a828b016107f0565b94505060806108db8a828b016107f0565b93505060a06108ec8a828b01610826565b92505060c06108fd8a828b0161085b565b91505092959891949750929550565b61091581610624565b82525050565b5f60208201905061092e5f83018461090c565b92915050565b61093d816105eb565b8114610947575f80fd5b50565b5f8151905061095881610934565b92915050565b6109678161063f565b8114610971575f80fd5b50565b5f815190506109828161095e565b92915050565b5f819050919050565b61099a81610988565b81146109a4575f80fd5b50565b5f815190506109b581610991565b92915050565b5f8160060b9050919050565b6109d0816109bb565b81146109da575f80fd5b50565b5f815190506109eb816109c7565b92915050565b5f63ffffffff82169050919050565b610a09816109f1565b8114610a13575f80fd5b50565b5f81519050610a2481610a00565b92915050565b5f805f805f805f80610100898b031215610a4757610a466103c9565b5b5f610a548b828c0161094a565b9850506020610a658b828c01610974565b9750506040610a768b828c016109a7565b9650506060610a878b828c016109a7565b9550506080610a988b828c016109dd565b94505060a0610aa98b828c0161078f565b93505060c0610aba8b828c01610a16565b92505060e0610acb8b828c0161085b565b9150509295985092959890939650565b5f60208284031215610af057610aef6103c9565b5b5f610afd8482850161094a565b9150509291505056fe
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`@Qa\x0B\x078\x03\x80a\x0B\x07\x839\x81\x81\x01`@R\x81\x01\x90a\x001\x91\x90a\x05{V[_a\0A\x82a\0l` \x1B` \x1CV[`@Q` \x01a\0Q\x91\x90a\x07,V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P` \x81\x01\x80Y\x03\x81\xF3[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\x8AWa\0\x89a\x03\xE5V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\xC3W\x81` \x01[a\0\xB0a\x03nV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\0\xA8W\x90P[P\x90P_[\x83Q\x81\x10\x15a\x033W_\x84\x82\x81Q\x81\x10a\0\xE5Wa\0\xE4a\x07LV[[` \x02` \x01\x01Q\x90Pa\0\xFE\x81a\x03=` \x1B` \x1CV[\x15a\x01\tWPa\x03(V[a\x01\x11a\x03nV[_\x80\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c8P\xC7\xBD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\\W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x80\x91\x90a\x08oV[PPPPP\x91P\x91P_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF3\r\xBA\x93\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\xC3\x91\x90a\t\x1BV[a\x01\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xDFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x03\x91\x90a\n*V[PPPPPP\x91PP\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x1Ahe\x02`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02UW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02y\x91\x90a\n\xDBV[\x84_\x01\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x82\x84` \x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81\x84`@\x01\x90`\x02\x0B\x90\x81`\x02\x0B\x81RPP\x80\x84``\x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP\x83\x87\x87\x81Q\x81\x10a\x03\x17Wa\x03\x16a\x07LV[[` \x02` \x01\x01\x81\x90RPPPPPP[\x80`\x01\x01\x90Pa\0\xC8V[P\x80\x91PP\x91\x90PV[_\x80\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a\x03eW`\x01\x90Pa\x03iV[_\x90P[\x91\x90PV[`@Q\x80`\x80\x01`@R\x80_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_`\x02\x0B\x81R` \x01_`\x0F\x0B\x81RP\x90V[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x04\x1B\x82a\x03\xD5V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x04:Wa\x049a\x03\xE5V[[\x80`@RPPPV[_a\x04La\x03\xC0V[\x90Pa\x04X\x82\x82a\x04\x12V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x04wWa\x04va\x03\xE5V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x80\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x04\xB5\x82a\x04\x8CV[\x90P\x91\x90PV[a\x04\xC5\x81a\x04\xABV[\x81\x14a\x04\xCFW_\x80\xFD[PV[_\x81Q\x90Pa\x04\xE0\x81a\x04\xBCV[\x92\x91PPV[_a\x04\xF8a\x04\xF3\x84a\x04]V[a\x04CV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x05\x1BWa\x05\x1Aa\x04\x88V[[\x83[\x81\x81\x10\x15a\x05DW\x80a\x050\x88\x82a\x04\xD2V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x05\x1DV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x05bWa\x05aa\x03\xD1V[[\x81Qa\x05r\x84\x82` \x86\x01a\x04\xE6V[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x05\x90Wa\x05\x8Fa\x03\xC9V[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\xADWa\x05\xACa\x03\xCDV[[a\x05\xB9\x84\x82\x85\x01a\x05NV[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x06\x0F\x81a\x05\xEBV[\x82RPPV[a\x06\x1E\x81a\x04\x8CV[\x82RPPV[_\x81`\x02\x0B\x90P\x91\x90PV[a\x069\x81a\x06$V[\x82RPPV[_\x81`\x0F\x0B\x90P\x91\x90PV[a\x06T\x81a\x06?V[\x82RPPV[`\x80\x82\x01_\x82\x01Qa\x06n_\x85\x01\x82a\x06\x06V[P` \x82\x01Qa\x06\x81` \x85\x01\x82a\x06\x15V[P`@\x82\x01Qa\x06\x94`@\x85\x01\x82a\x060V[P``\x82\x01Qa\x06\xA7``\x85\x01\x82a\x06KV[PPPPV[_a\x06\xB8\x83\x83a\x06ZV[`\x80\x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x06\xDA\x82a\x05\xC2V[a\x06\xE4\x81\x85a\x05\xCCV[\x93Pa\x06\xEF\x83a\x05\xDCV[\x80_[\x83\x81\x10\x15a\x07\x1FW\x81Qa\x07\x06\x88\x82a\x06\xADV[\x97Pa\x07\x11\x83a\x06\xC4V[\x92PP`\x01\x81\x01\x90Pa\x06\xF2V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x07D\x81\x84a\x06\xD0V[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[a\x07\x82\x81a\x04\x8CV[\x81\x14a\x07\x8CW_\x80\xFD[PV[_\x81Q\x90Pa\x07\x9D\x81a\x07yV[\x92\x91PPV[a\x07\xAC\x81a\x06$V[\x81\x14a\x07\xB6W_\x80\xFD[PV[_\x81Q\x90Pa\x07\xC7\x81a\x07\xA3V[\x92\x91PPV[_a\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x07\xE3\x81a\x07\xCDV[\x81\x14a\x07\xEDW_\x80\xFD[PV[_\x81Q\x90Pa\x07\xFE\x81a\x07\xDAV[\x92\x91PPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\x08\x19\x81a\x08\x04V[\x81\x14a\x08#W_\x80\xFD[PV[_\x81Q\x90Pa\x084\x81a\x08\x10V[\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a\x08N\x81a\x08:V[\x81\x14a\x08XW_\x80\xFD[PV[_\x81Q\x90Pa\x08i\x81a\x08EV[\x92\x91PPV[_\x80_\x80_\x80_`\xE0\x88\x8A\x03\x12\x15a\x08\x8AWa\x08\x89a\x03\xC9V[[_a\x08\x97\x8A\x82\x8B\x01a\x07\x8FV[\x97PP` a\x08\xA8\x8A\x82\x8B\x01a\x07\xB9V[\x96PP`@a\x08\xB9\x8A\x82\x8B\x01a\x07\xF0V[\x95PP``a\x08\xCA\x8A\x82\x8B\x01a\x07\xF0V[\x94PP`\x80a\x08\xDB\x8A\x82\x8B\x01a\x07\xF0V[\x93PP`\xA0a\x08\xEC\x8A\x82\x8B\x01a\x08&V[\x92PP`\xC0a\x08\xFD\x8A\x82\x8B\x01a\x08[V[\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[a\t\x15\x81a\x06$V[\x82RPPV[_` \x82\x01\x90Pa\t._\x83\x01\x84a\t\x0CV[\x92\x91PPV[a\t=\x81a\x05\xEBV[\x81\x14a\tGW_\x80\xFD[PV[_\x81Q\x90Pa\tX\x81a\t4V[\x92\x91PPV[a\tg\x81a\x06?V[\x81\x14a\tqW_\x80\xFD[PV[_\x81Q\x90Pa\t\x82\x81a\t^V[\x92\x91PPV[_\x81\x90P\x91\x90PV[a\t\x9A\x81a\t\x88V[\x81\x14a\t\xA4W_\x80\xFD[PV[_\x81Q\x90Pa\t\xB5\x81a\t\x91V[\x92\x91PPV[_\x81`\x06\x0B\x90P\x91\x90PV[a\t\xD0\x81a\t\xBBV[\x81\x14a\t\xDAW_\x80\xFD[PV[_\x81Q\x90Pa\t\xEB\x81a\t\xC7V[\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\n\t\x81a\t\xF1V[\x81\x14a\n\x13W_\x80\xFD[PV[_\x81Q\x90Pa\n$\x81a\n\0V[\x92\x91PPV[_\x80_\x80_\x80_\x80a\x01\0\x89\x8B\x03\x12\x15a\nGWa\nFa\x03\xC9V[[_a\nT\x8B\x82\x8C\x01a\tJV[\x98PP` a\ne\x8B\x82\x8C\x01a\ttV[\x97PP`@a\nv\x8B\x82\x8C\x01a\t\xA7V[\x96PP``a\n\x87\x8B\x82\x8C\x01a\t\xA7V[\x95PP`\x80a\n\x98\x8B\x82\x8C\x01a\t\xDDV[\x94PP`\xA0a\n\xA9\x8B\x82\x8C\x01a\x07\x8FV[\x93PP`\xC0a\n\xBA\x8B\x82\x8C\x01a\n\x16V[\x92PP`\xE0a\n\xCB\x8B\x82\x8C\x01a\x08[V[\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[_` \x82\x84\x03\x12\x15a\n\xF0Wa\n\xEFa\x03\xC9V[[_a\n\xFD\x84\x82\x85\x01a\tJV[\x91PP\x92\x91PPV\xFE",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b5060043610610029575f3560e01c80639ae919bf1461002d575b5f80fd5b61004760048036038101906100429190610566565b61005d565b6040516100549190610717565b60405180910390f35b60605f825167ffffffffffffffff81111561007b5761007a6103d0565b5b6040519080825280602002602001820160405280156100b457816020015b6100a1610359565b8152602001906001900390816100995790505b5090505f5b835181101561031e575f8482815181106100d6576100d5610737565b5b602002602001015190506100e981610328565b156100f45750610313565b6100fc610359565b5f808373ffffffffffffffffffffffffffffffffffffffff16633850c7bd6040518163ffffffff1660e01b815260040160e060405180830381865afa158015610147573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061016b919061085a565b5050505050915091505f8473ffffffffffffffffffffffffffffffffffffffff1663f30dba93836040518263ffffffff1660e01b81526004016101ae9190610906565b61010060405180830381865afa1580156101ca573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101ee9190610a15565b5050505050509150508473ffffffffffffffffffffffffffffffffffffffff16631a6865026040518163ffffffff1660e01b8152600401602060405180830381865afa158015610240573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102649190610ac6565b845f01906fffffffffffffffffffffffffffffffff1690816fffffffffffffffffffffffffffffffff168152505082846020019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff168152505081846040019060020b908160020b81525050808460600190600f0b9081600f0b815250508387878151811061030257610301610737565b5b602002602001018190525050505050505b8060010190506100b9565b5080915050919050565b5f808273ffffffffffffffffffffffffffffffffffffffff163b036103505760019050610354565b5f90505b919050565b60405180608001604052805f6fffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f60020b81526020015f600f0b81525090565b5f604051905090565b5f80fd5b5f80fd5b5f80fd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b610406826103c0565b810181811067ffffffffffffffff82111715610425576104246103d0565b5b80604052505050565b5f6104376103ab565b905061044382826103fd565b919050565b5f67ffffffffffffffff821115610462576104616103d0565b5b602082029050602081019050919050565b5f80fd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6104a082610477565b9050919050565b6104b081610496565b81146104ba575f80fd5b50565b5f813590506104cb816104a7565b92915050565b5f6104e36104de84610448565b61042e565b9050808382526020820190506020840283018581111561050657610505610473565b5b835b8181101561052f578061051b88826104bd565b845260208401935050602081019050610508565b5050509392505050565b5f82601f83011261054d5761054c6103bc565b5b813561055d8482602086016104d1565b91505092915050565b5f6020828403121561057b5761057a6103b4565b5b5f82013567ffffffffffffffff811115610598576105976103b8565b5b6105a484828501610539565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f6fffffffffffffffffffffffffffffffff82169050919050565b6105fa816105d6565b82525050565b61060981610477565b82525050565b5f8160020b9050919050565b6106248161060f565b82525050565b5f81600f0b9050919050565b61063f8161062a565b82525050565b608082015f8201516106595f8501826105f1565b50602082015161066c6020850182610600565b50604082015161067f604085018261061b565b5060608201516106926060850182610636565b50505050565b5f6106a38383610645565b60808301905092915050565b5f602082019050919050565b5f6106c5826105ad565b6106cf81856105b7565b93506106da836105c7565b805f5b8381101561070a5781516106f18882610698565b97506106fc836106af565b9250506001810190506106dd565b5085935050505092915050565b5f6020820190508181035f83015261072f81846106bb565b905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b61076d81610477565b8114610777575f80fd5b50565b5f8151905061078881610764565b92915050565b6107978161060f565b81146107a1575f80fd5b50565b5f815190506107b28161078e565b92915050565b5f61ffff82169050919050565b6107ce816107b8565b81146107d8575f80fd5b50565b5f815190506107e9816107c5565b92915050565b5f60ff82169050919050565b610804816107ef565b811461080e575f80fd5b50565b5f8151905061081f816107fb565b92915050565b5f8115159050919050565b61083981610825565b8114610843575f80fd5b50565b5f8151905061085481610830565b92915050565b5f805f805f805f60e0888a031215610875576108746103b4565b5b5f6108828a828b0161077a565b97505060206108938a828b016107a4565b96505060406108a48a828b016107db565b95505060606108b58a828b016107db565b94505060806108c68a828b016107db565b93505060a06108d78a828b01610811565b92505060c06108e88a828b01610846565b91505092959891949750929550565b6109008161060f565b82525050565b5f6020820190506109195f8301846108f7565b92915050565b610928816105d6565b8114610932575f80fd5b50565b5f815190506109438161091f565b92915050565b6109528161062a565b811461095c575f80fd5b50565b5f8151905061096d81610949565b92915050565b5f819050919050565b61098581610973565b811461098f575f80fd5b50565b5f815190506109a08161097c565b92915050565b5f8160060b9050919050565b6109bb816109a6565b81146109c5575f80fd5b50565b5f815190506109d6816109b2565b92915050565b5f63ffffffff82169050919050565b6109f4816109dc565b81146109fe575f80fd5b50565b5f81519050610a0f816109eb565b92915050565b5f805f805f805f80610100898b031215610a3257610a316103b4565b5b5f610a3f8b828c01610935565b9850506020610a508b828c0161095f565b9750506040610a618b828c01610992565b9650506060610a728b828c01610992565b9550506080610a838b828c016109c8565b94505060a0610a948b828c0161077a565b93505060c0610aa58b828c01610a01565b92505060e0610ab68b828c01610846565b9150509295985092959890939650565b5f60208284031215610adb57610ada6103b4565b5b5f610ae884828501610935565b9150509291505056fea26469706673582212209a39b7677f8e7393c3d3ba79b53c52b6d4abfca60e8d566ac2fe8b6d3882e51e64736f6c634300081a0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0)W_5`\xE0\x1C\x80c\x9A\xE9\x19\xBF\x14a\0-W[_\x80\xFD[a\0G`\x04\x806\x03\x81\x01\x90a\0B\x91\x90a\x05fV[a\0]V[`@Qa\0T\x91\x90a\x07\x17V[`@Q\x80\x91\x03\x90\xF3[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0{Wa\0za\x03\xD0V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\xB4W\x81` \x01[a\0\xA1a\x03YV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\0\x99W\x90P[P\x90P_[\x83Q\x81\x10\x15a\x03\x1EW_\x84\x82\x81Q\x81\x10a\0\xD6Wa\0\xD5a\x077V[[` \x02` \x01\x01Q\x90Pa\0\xE9\x81a\x03(V[\x15a\0\xF4WPa\x03\x13V[a\0\xFCa\x03YV[_\x80\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c8P\xC7\xBD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01GW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01k\x91\x90a\x08ZV[PPPPP\x91P\x91P_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF3\r\xBA\x93\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\xAE\x91\x90a\t\x06V[a\x01\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xCAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xEE\x91\x90a\n\x15V[PPPPPP\x91PP\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x1Ahe\x02`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02@W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02d\x91\x90a\n\xC6V[\x84_\x01\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x82\x84` \x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81\x84`@\x01\x90`\x02\x0B\x90\x81`\x02\x0B\x81RPP\x80\x84``\x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP\x83\x87\x87\x81Q\x81\x10a\x03\x02Wa\x03\x01a\x077V[[` \x02` \x01\x01\x81\x90RPPPPPP[\x80`\x01\x01\x90Pa\0\xB9V[P\x80\x91PP\x91\x90PV[_\x80\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a\x03PW`\x01\x90Pa\x03TV[_\x90P[\x91\x90PV[`@Q\x80`\x80\x01`@R\x80_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_`\x02\x0B\x81R` \x01_`\x0F\x0B\x81RP\x90V[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x04\x06\x82a\x03\xC0V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x04%Wa\x04$a\x03\xD0V[[\x80`@RPPPV[_a\x047a\x03\xABV[\x90Pa\x04C\x82\x82a\x03\xFDV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x04bWa\x04aa\x03\xD0V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x80\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x04\xA0\x82a\x04wV[\x90P\x91\x90PV[a\x04\xB0\x81a\x04\x96V[\x81\x14a\x04\xBAW_\x80\xFD[PV[_\x815\x90Pa\x04\xCB\x81a\x04\xA7V[\x92\x91PPV[_a\x04\xE3a\x04\xDE\x84a\x04HV[a\x04.V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x05\x06Wa\x05\x05a\x04sV[[\x83[\x81\x81\x10\x15a\x05/W\x80a\x05\x1B\x88\x82a\x04\xBDV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x05\x08V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x05MWa\x05La\x03\xBCV[[\x815a\x05]\x84\x82` \x86\x01a\x04\xD1V[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x05{Wa\x05za\x03\xB4V[[_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\x98Wa\x05\x97a\x03\xB8V[[a\x05\xA4\x84\x82\x85\x01a\x059V[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x05\xFA\x81a\x05\xD6V[\x82RPPV[a\x06\t\x81a\x04wV[\x82RPPV[_\x81`\x02\x0B\x90P\x91\x90PV[a\x06$\x81a\x06\x0FV[\x82RPPV[_\x81`\x0F\x0B\x90P\x91\x90PV[a\x06?\x81a\x06*V[\x82RPPV[`\x80\x82\x01_\x82\x01Qa\x06Y_\x85\x01\x82a\x05\xF1V[P` \x82\x01Qa\x06l` \x85\x01\x82a\x06\0V[P`@\x82\x01Qa\x06\x7F`@\x85\x01\x82a\x06\x1BV[P``\x82\x01Qa\x06\x92``\x85\x01\x82a\x066V[PPPPV[_a\x06\xA3\x83\x83a\x06EV[`\x80\x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x06\xC5\x82a\x05\xADV[a\x06\xCF\x81\x85a\x05\xB7V[\x93Pa\x06\xDA\x83a\x05\xC7V[\x80_[\x83\x81\x10\x15a\x07\nW\x81Qa\x06\xF1\x88\x82a\x06\x98V[\x97Pa\x06\xFC\x83a\x06\xAFV[\x92PP`\x01\x81\x01\x90Pa\x06\xDDV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x07/\x81\x84a\x06\xBBV[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[a\x07m\x81a\x04wV[\x81\x14a\x07wW_\x80\xFD[PV[_\x81Q\x90Pa\x07\x88\x81a\x07dV[\x92\x91PPV[a\x07\x97\x81a\x06\x0FV[\x81\x14a\x07\xA1W_\x80\xFD[PV[_\x81Q\x90Pa\x07\xB2\x81a\x07\x8EV[\x92\x91PPV[_a\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x07\xCE\x81a\x07\xB8V[\x81\x14a\x07\xD8W_\x80\xFD[PV[_\x81Q\x90Pa\x07\xE9\x81a\x07\xC5V[\x92\x91PPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\x08\x04\x81a\x07\xEFV[\x81\x14a\x08\x0EW_\x80\xFD[PV[_\x81Q\x90Pa\x08\x1F\x81a\x07\xFBV[\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a\x089\x81a\x08%V[\x81\x14a\x08CW_\x80\xFD[PV[_\x81Q\x90Pa\x08T\x81a\x080V[\x92\x91PPV[_\x80_\x80_\x80_`\xE0\x88\x8A\x03\x12\x15a\x08uWa\x08ta\x03\xB4V[[_a\x08\x82\x8A\x82\x8B\x01a\x07zV[\x97PP` a\x08\x93\x8A\x82\x8B\x01a\x07\xA4V[\x96PP`@a\x08\xA4\x8A\x82\x8B\x01a\x07\xDBV[\x95PP``a\x08\xB5\x8A\x82\x8B\x01a\x07\xDBV[\x94PP`\x80a\x08\xC6\x8A\x82\x8B\x01a\x07\xDBV[\x93PP`\xA0a\x08\xD7\x8A\x82\x8B\x01a\x08\x11V[\x92PP`\xC0a\x08\xE8\x8A\x82\x8B\x01a\x08FV[\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[a\t\0\x81a\x06\x0FV[\x82RPPV[_` \x82\x01\x90Pa\t\x19_\x83\x01\x84a\x08\xF7V[\x92\x91PPV[a\t(\x81a\x05\xD6V[\x81\x14a\t2W_\x80\xFD[PV[_\x81Q\x90Pa\tC\x81a\t\x1FV[\x92\x91PPV[a\tR\x81a\x06*V[\x81\x14a\t\\W_\x80\xFD[PV[_\x81Q\x90Pa\tm\x81a\tIV[\x92\x91PPV[_\x81\x90P\x91\x90PV[a\t\x85\x81a\tsV[\x81\x14a\t\x8FW_\x80\xFD[PV[_\x81Q\x90Pa\t\xA0\x81a\t|V[\x92\x91PPV[_\x81`\x06\x0B\x90P\x91\x90PV[a\t\xBB\x81a\t\xA6V[\x81\x14a\t\xC5W_\x80\xFD[PV[_\x81Q\x90Pa\t\xD6\x81a\t\xB2V[\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\t\xF4\x81a\t\xDCV[\x81\x14a\t\xFEW_\x80\xFD[PV[_\x81Q\x90Pa\n\x0F\x81a\t\xEBV[\x92\x91PPV[_\x80_\x80_\x80_\x80a\x01\0\x89\x8B\x03\x12\x15a\n2Wa\n1a\x03\xB4V[[_a\n?\x8B\x82\x8C\x01a\t5V[\x98PP` a\nP\x8B\x82\x8C\x01a\t_V[\x97PP`@a\na\x8B\x82\x8C\x01a\t\x92V[\x96PP``a\nr\x8B\x82\x8C\x01a\t\x92V[\x95PP`\x80a\n\x83\x8B\x82\x8C\x01a\t\xC8V[\x94PP`\xA0a\n\x94\x8B\x82\x8C\x01a\x07zV[\x93PP`\xC0a\n\xA5\x8B\x82\x8C\x01a\n\x01V[\x92PP`\xE0a\n\xB6\x8B\x82\x8C\x01a\x08FV[\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[_` \x82\x84\x03\x12\x15a\n\xDBWa\n\xDAa\x03\xB4V[[_a\n\xE8\x84\x82\x85\x01a\t5V[\x91PP\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x9A9\xB7g\x7F\x8Es\x93\xC3\xD3\xBAy\xB5<R\xB6\xD4\xAB\xFC\xA6\x0E\x8DVj\xC2\xFE\x8Bm8\x82\xE5\x1EdsolcC\0\x08\x1A\x003",
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
function getPoolData(address[] memory pools) external view returns (PoolHelpers.UniswapV3PoolPriceData[] memory);
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
            <PoolHelpers::UniswapV3PoolPriceData as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Array<PoolHelpers::UniswapV3PoolPriceData>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <PoolHelpers::UniswapV3PoolPriceData as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Array<PoolHelpers::UniswapV3PoolPriceData>,
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
    ///Container for all the [`SyncUniswapV3PoolBatchRequest`](self) function calls.
    pub enum SyncUniswapV3PoolBatchRequestCalls {
        #[allow(missing_docs)]
        getPoolData(getPoolDataCall),
    }
    #[automatically_derived]
    impl SyncUniswapV3PoolBatchRequestCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[154u8, 233u8, 25u8, 191u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SyncUniswapV3PoolBatchRequestCalls {
        const NAME: &'static str = "SyncUniswapV3PoolBatchRequestCalls";
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
            ) -> alloy_sol_types::Result<SyncUniswapV3PoolBatchRequestCalls>] = &[
                {
                    fn getPoolData(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyncUniswapV3PoolBatchRequestCalls> {
                        <getPoolDataCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyncUniswapV3PoolBatchRequestCalls::getPoolData)
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
    /**Creates a new wrapper around an on-chain [`SyncUniswapV3PoolBatchRequest`](self) contract instance.

See the [wrapper's documentation](`SyncUniswapV3PoolBatchRequestInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> SyncUniswapV3PoolBatchRequestInstance<T, P, N> {
        SyncUniswapV3PoolBatchRequestInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<SyncUniswapV3PoolBatchRequestInstance<T, P, N>>,
    > {
        SyncUniswapV3PoolBatchRequestInstance::<T, P, N>::deploy(provider, pools)
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
        SyncUniswapV3PoolBatchRequestInstance::<T, P, N>::deploy_builder(provider, pools)
    }
    /**A [`SyncUniswapV3PoolBatchRequest`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`SyncUniswapV3PoolBatchRequest`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct SyncUniswapV3PoolBatchRequestInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for SyncUniswapV3PoolBatchRequestInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("SyncUniswapV3PoolBatchRequestInstance")
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
    > SyncUniswapV3PoolBatchRequestInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`SyncUniswapV3PoolBatchRequest`](self) contract instance.

See the [wrapper's documentation](`SyncUniswapV3PoolBatchRequestInstance`) for more details.*/
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
        ) -> alloy_contract::Result<SyncUniswapV3PoolBatchRequestInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> SyncUniswapV3PoolBatchRequestInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(
            self,
        ) -> SyncUniswapV3PoolBatchRequestInstance<T, P, N> {
            SyncUniswapV3PoolBatchRequestInstance {
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
    > SyncUniswapV3PoolBatchRequestInstance<T, P, N> {
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
    > SyncUniswapV3PoolBatchRequestInstance<T, P, N> {
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
