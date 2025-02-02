///Module containing a contract's types and functions.
/**

```solidity
library PoolUtils {
    struct Slot { uint256 slot; uint256 data; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod PoolUtils {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct Slot { uint256 slot; uint256 data; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Slot {
        #[allow(missing_docs)]
        pub slot: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
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
        impl ::core::convert::From<Slot> for UnderlyingRustTuple<'_> {
            fn from(value: Slot) -> Self {
                (value.slot, value.data)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Slot {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    slot: tuple.0,
                    data: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Slot {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Slot {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.slot),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.data),
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
        impl alloy_sol_types::SolType for Slot {
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
        impl alloy_sol_types::SolStruct for Slot {
            const NAME: &'static str = "Slot";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Slot(uint256 slot,uint256 data)",
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
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.slot)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.data)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Slot {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.slot)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.data)
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
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.slot,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.data,
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
    /**Creates a new wrapper around an on-chain [`PoolUtils`](self) contract instance.

See the [wrapper's documentation](`PoolUtilsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> PoolUtilsInstance<T, P, N> {
        PoolUtilsInstance::<T, P, N>::new(address, provider)
    }
    /**A [`PoolUtils`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`PoolUtils`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct PoolUtilsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for PoolUtilsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("PoolUtilsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > PoolUtilsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`PoolUtils`](self) contract instance.

See the [wrapper's documentation](`PoolUtilsInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> PoolUtilsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> PoolUtilsInstance<T, P, N> {
            PoolUtilsInstance {
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
    > PoolUtilsInstance<T, P, N> {
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
    > PoolUtilsInstance<T, P, N> {
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
library PoolUtils {
    struct Slot {
        uint256 slot;
        uint256 data;
    }
}

interface GetCLPoolTickBitmap {
    type DEX is uint8;
    type V3PoolCallee is address;

    constructor(DEX dex, V3PoolCallee pool) payable;

    function getTickBitmap(DEX dex, V3PoolCallee pool) external payable returns (PoolUtils.Slot[] memory slots);
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
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "getTickBitmap",
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
      }
    ],
    "outputs": [
      {
        "name": "slots",
        "type": "tuple[]",
        "internalType": "struct PoolUtils.Slot[]",
        "components": [
          {
            "name": "slot",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "data",
            "type": "uint256",
            "internalType": "uint256"
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
pub mod GetCLPoolTickBitmap {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60806040526040516106cf3803806106cf833981810160405281019061002591906103fd565b5f610036838361006360201b60201c565b90505f8160405160200161004a9190610528565b6040516020818303038152906040529050805160208201fd5b60605f8273ffffffffffffffffffffffffffffffffffffffff1663d0c93a7c6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156100af573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906100d3919061057e565b90505f8061010b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff27618620d89e88561021860201b60201c565b915091505f61011f8761025c60201b60201c565b905060018383030161ffff1667ffffffffffffffff811115610144576101436105a9565b5b60405190808252806020026020018201604052801561017d57816020015b61016a610364565b8152602001906001900390816101625790505b5094505f8390505b8260010b8160010b1361020d575f815f528260205260405f20905060405180604001604052808281526020016101da848b73ffffffffffffffffffffffffffffffffffffffff1661030660201b90919060201c565b8152508786840361ffff16815181106101f6576101f56105d6565b5b602002602001018190525050806001019050610185565b505050505092915050565b5f805f61022b868561033460201b60201c565b905060088160020b901d9250610247858561033460201b60201c565b905060088160020b901d915050935093915050565b5f8060028111156102705761026f610603565b5b82600281111561028357610282610603565b5b036102915760069050610301565b600160028111156102a5576102a4610603565b5b8260028111156102b8576102b7610603565b5b036102c65760079050610301565b6040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016102f8906106b0565b60405180910390fd5b919050565b5f808260010b905061032984635339c29660e01b835f602061034660201b60201c565b5f5191505092915050565b5f808284071282840503905092915050565b835f5282600452808260245f885afa61035d575f80fd5b5050505050565b60405180604001604052805f81526020015f81525090565b5f80fd5b6003811061038c575f80fd5b50565b5f8151905061039d81610380565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6103cc826103a3565b9050919050565b6103dc816103c2565b81146103e6575f80fd5b50565b5f815190506103f7816103d3565b92915050565b5f80604083850312156104135761041261037c565b5b5f6104208582860161038f565b9250506020610431858286016103e9565b9150509250929050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f819050919050565b61047681610464565b82525050565b604082015f8201516104905f85018261046d565b5060208201516104a3602085018261046d565b50505050565b5f6104b4838361047c565b60408301905092915050565b5f602082019050919050565b5f6104d68261043b565b6104e08185610445565b93506104eb83610455565b805f5b8381101561051b57815161050288826104a9565b975061050d836104c0565b9250506001810190506104ee565b5085935050505092915050565b5f6020820190508181035f83015261054081846104cc565b905092915050565b5f8160020b9050919050565b61055d81610548565b8114610567575f80fd5b50565b5f8151905061057881610554565b92915050565b5f602082840312156105935761059261037c565b5b5f6105a08482850161056a565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b5f82825260208201905092915050565b7f457068656d6572616c506f6f6c5469636b4269746d61703a20696e76616c69645f8201527f206f7220756e737570706f727465642044455800000000000000000000000000602082015250565b5f61069a603383610630565b91506106a582610640565b604082019050919050565b5f6020820190508181035f8301526106c78161068e565b905091905056fe
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`@Qa\x06\xCF8\x03\x80a\x06\xCF\x839\x81\x81\x01`@R\x81\x01\x90a\0%\x91\x90a\x03\xFDV[_a\x006\x83\x83a\0c` \x1B` \x1CV[\x90P_\x81`@Q` \x01a\0J\x91\x90a\x05(V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80Q` \x82\x01\xFD[``_\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xAFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xD3\x91\x90a\x05~V[\x90P_\x80a\x01\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2v\x18b\r\x89\xE8\x85a\x02\x18` \x1B` \x1CV[\x91P\x91P_a\x01\x1F\x87a\x02\\` \x1B` \x1CV[\x90P`\x01\x83\x83\x03\x01a\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01DWa\x01Ca\x05\xA9V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01}W\x81` \x01[a\x01ja\x03dV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x01bW\x90P[P\x94P_\x83\x90P[\x82`\x01\x0B\x81`\x01\x0B\x13a\x02\rW_\x81_R\x82` R`@_ \x90P`@Q\x80`@\x01`@R\x80\x82\x81R` \x01a\x01\xDA\x84\x8Bs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03\x06` \x1B\x90\x91\x90` \x1CV[\x81RP\x87\x86\x84\x03a\xFF\xFF\x16\x81Q\x81\x10a\x01\xF6Wa\x01\xF5a\x05\xD6V[[` \x02` \x01\x01\x81\x90RPP\x80`\x01\x01\x90Pa\x01\x85V[PPPPP\x92\x91PPV[_\x80_a\x02+\x86\x85a\x034` \x1B` \x1CV[\x90P`\x08\x81`\x02\x0B\x90\x1D\x92Pa\x02G\x85\x85a\x034` \x1B` \x1CV[\x90P`\x08\x81`\x02\x0B\x90\x1D\x91PP\x93P\x93\x91PPV[_\x80`\x02\x81\x11\x15a\x02pWa\x02oa\x06\x03V[[\x82`\x02\x81\x11\x15a\x02\x83Wa\x02\x82a\x06\x03V[[\x03a\x02\x91W`\x06\x90Pa\x03\x01V[`\x01`\x02\x81\x11\x15a\x02\xA5Wa\x02\xA4a\x06\x03V[[\x82`\x02\x81\x11\x15a\x02\xB8Wa\x02\xB7a\x06\x03V[[\x03a\x02\xC6W`\x07\x90Pa\x03\x01V[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x02\xF8\x90a\x06\xB0V[`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[_\x80\x82`\x01\x0B\x90Pa\x03)\x84cS9\xC2\x96`\xE0\x1B\x83_` a\x03F` \x1B` \x1CV[_Q\x91PP\x92\x91PPV[_\x80\x82\x84\x07\x12\x82\x84\x05\x03\x90P\x92\x91PPV[\x83_R\x82`\x04R\x80\x82`$_\x88Z\xFAa\x03]W_\x80\xFD[PPPPPV[`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[_\x80\xFD[`\x03\x81\x10a\x03\x8CW_\x80\xFD[PV[_\x81Q\x90Pa\x03\x9D\x81a\x03\x80V[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x03\xCC\x82a\x03\xA3V[\x90P\x91\x90PV[a\x03\xDC\x81a\x03\xC2V[\x81\x14a\x03\xE6W_\x80\xFD[PV[_\x81Q\x90Pa\x03\xF7\x81a\x03\xD3V[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a\x04\x13Wa\x04\x12a\x03|V[[_a\x04 \x85\x82\x86\x01a\x03\x8FV[\x92PP` a\x041\x85\x82\x86\x01a\x03\xE9V[\x91PP\x92P\x92\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\x04v\x81a\x04dV[\x82RPPV[`@\x82\x01_\x82\x01Qa\x04\x90_\x85\x01\x82a\x04mV[P` \x82\x01Qa\x04\xA3` \x85\x01\x82a\x04mV[PPPPV[_a\x04\xB4\x83\x83a\x04|V[`@\x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x04\xD6\x82a\x04;V[a\x04\xE0\x81\x85a\x04EV[\x93Pa\x04\xEB\x83a\x04UV[\x80_[\x83\x81\x10\x15a\x05\x1BW\x81Qa\x05\x02\x88\x82a\x04\xA9V[\x97Pa\x05\r\x83a\x04\xC0V[\x92PP`\x01\x81\x01\x90Pa\x04\xEEV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x05@\x81\x84a\x04\xCCV[\x90P\x92\x91PPV[_\x81`\x02\x0B\x90P\x91\x90PV[a\x05]\x81a\x05HV[\x81\x14a\x05gW_\x80\xFD[PV[_\x81Q\x90Pa\x05x\x81a\x05TV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x05\x93Wa\x05\x92a\x03|V[[_a\x05\xA0\x84\x82\x85\x01a\x05jV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FEphemeralPoolTickBitmap: invalid_\x82\x01R\x7F or unsupported DEX\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x06\x9A`3\x83a\x060V[\x91Pa\x06\xA5\x82a\x06@V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x06\xC7\x81a\x06\x8EV[\x90P\x91\x90PV\xFE",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361061001d575f3560e01c8063cc4e959e14610021575b5f80fd5b61003b600480360381019061003691906103cd565b610051565b60405161004891906104f8565b60405180910390f35b60605f8273ffffffffffffffffffffffffffffffffffffffff1663d0c93a7c6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561009d573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906100c1919061054e565b90505f806100f37ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff27618620d89e8856101fa565b915091505f61010187610232565b905060018383030161ffff1667ffffffffffffffff81111561012657610125610579565b5b60405190808252806020026020018201604052801561015f57816020015b61014c610334565b8152602001906001900390816101445790505b5094505f8390505b8260010b8160010b136101ef575f815f528260205260405f20905060405180604001604052808281526020016101bc848b73ffffffffffffffffffffffffffffffffffffffff166102dc90919063ffffffff16565b8152508786840361ffff16815181106101d8576101d76105a6565b5b602002602001018190525050806001019050610167565b505050505092915050565b5f805f6102078685610304565b905060088160020b901d925061021d8585610304565b905060088160020b901d915050935093915050565b5f806002811115610246576102456105d3565b5b826002811115610259576102586105d3565b5b0361026757600690506102d7565b6001600281111561027b5761027a6105d3565b5b82600281111561028e5761028d6105d3565b5b0361029c57600790506102d7565b6040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016102ce90610680565b60405180910390fd5b919050565b5f808260010b90506102f984635339c29660e01b835f6020610316565b5f5191505092915050565b5f808284071282840503905092915050565b835f5282600452808260245f885afa61032d575f80fd5b5050505050565b60405180604001604052805f81526020015f81525090565b5f80fd5b6003811061035c575f80fd5b50565b5f8135905061036d81610350565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61039c82610373565b9050919050565b6103ac81610392565b81146103b6575f80fd5b50565b5f813590506103c7816103a3565b92915050565b5f80604083850312156103e3576103e261034c565b5b5f6103f08582860161035f565b9250506020610401858286016103b9565b9150509250929050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f819050919050565b61044681610434565b82525050565b604082015f8201516104605f85018261043d565b506020820151610473602085018261043d565b50505050565b5f610484838361044c565b60408301905092915050565b5f602082019050919050565b5f6104a68261040b565b6104b08185610415565b93506104bb83610425565b805f5b838110156104eb5781516104d28882610479565b97506104dd83610490565b9250506001810190506104be565b5085935050505092915050565b5f6020820190508181035f830152610510818461049c565b905092915050565b5f8160020b9050919050565b61052d81610518565b8114610537575f80fd5b50565b5f8151905061054881610524565b92915050565b5f602082840312156105635761056261034c565b5b5f6105708482850161053a565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b5f82825260208201905092915050565b7f457068656d6572616c506f6f6c5469636b4269746d61703a20696e76616c69645f8201527f206f7220756e737570706f727465642044455800000000000000000000000000602082015250565b5f61066a603383610600565b915061067582610610565b604082019050919050565b5f6020820190508181035f8301526106978161065e565b905091905056fea2646970667358221220511d0aae8315b2e5461776ef9f80dffad6fd0f4f89343938e5498631e2daddd564736f6c634300081a0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\0\x1DW_5`\xE0\x1C\x80c\xCCN\x95\x9E\x14a\0!W[_\x80\xFD[a\0;`\x04\x806\x03\x81\x01\x90a\x006\x91\x90a\x03\xCDV[a\0QV[`@Qa\0H\x91\x90a\x04\xF8V[`@Q\x80\x91\x03\x90\xF3[``_\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\x9DW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xC1\x91\x90a\x05NV[\x90P_\x80a\0\xF3\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2v\x18b\r\x89\xE8\x85a\x01\xFAV[\x91P\x91P_a\x01\x01\x87a\x022V[\x90P`\x01\x83\x83\x03\x01a\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01&Wa\x01%a\x05yV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01_W\x81` \x01[a\x01La\x034V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x01DW\x90P[P\x94P_\x83\x90P[\x82`\x01\x0B\x81`\x01\x0B\x13a\x01\xEFW_\x81_R\x82` R`@_ \x90P`@Q\x80`@\x01`@R\x80\x82\x81R` \x01a\x01\xBC\x84\x8Bs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x02\xDC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81RP\x87\x86\x84\x03a\xFF\xFF\x16\x81Q\x81\x10a\x01\xD8Wa\x01\xD7a\x05\xA6V[[` \x02` \x01\x01\x81\x90RPP\x80`\x01\x01\x90Pa\x01gV[PPPPP\x92\x91PPV[_\x80_a\x02\x07\x86\x85a\x03\x04V[\x90P`\x08\x81`\x02\x0B\x90\x1D\x92Pa\x02\x1D\x85\x85a\x03\x04V[\x90P`\x08\x81`\x02\x0B\x90\x1D\x91PP\x93P\x93\x91PPV[_\x80`\x02\x81\x11\x15a\x02FWa\x02Ea\x05\xD3V[[\x82`\x02\x81\x11\x15a\x02YWa\x02Xa\x05\xD3V[[\x03a\x02gW`\x06\x90Pa\x02\xD7V[`\x01`\x02\x81\x11\x15a\x02{Wa\x02za\x05\xD3V[[\x82`\x02\x81\x11\x15a\x02\x8EWa\x02\x8Da\x05\xD3V[[\x03a\x02\x9CW`\x07\x90Pa\x02\xD7V[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x02\xCE\x90a\x06\x80V[`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[_\x80\x82`\x01\x0B\x90Pa\x02\xF9\x84cS9\xC2\x96`\xE0\x1B\x83_` a\x03\x16V[_Q\x91PP\x92\x91PPV[_\x80\x82\x84\x07\x12\x82\x84\x05\x03\x90P\x92\x91PPV[\x83_R\x82`\x04R\x80\x82`$_\x88Z\xFAa\x03-W_\x80\xFD[PPPPPV[`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[_\x80\xFD[`\x03\x81\x10a\x03\\W_\x80\xFD[PV[_\x815\x90Pa\x03m\x81a\x03PV[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x03\x9C\x82a\x03sV[\x90P\x91\x90PV[a\x03\xAC\x81a\x03\x92V[\x81\x14a\x03\xB6W_\x80\xFD[PV[_\x815\x90Pa\x03\xC7\x81a\x03\xA3V[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a\x03\xE3Wa\x03\xE2a\x03LV[[_a\x03\xF0\x85\x82\x86\x01a\x03_V[\x92PP` a\x04\x01\x85\x82\x86\x01a\x03\xB9V[\x91PP\x92P\x92\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\x04F\x81a\x044V[\x82RPPV[`@\x82\x01_\x82\x01Qa\x04`_\x85\x01\x82a\x04=V[P` \x82\x01Qa\x04s` \x85\x01\x82a\x04=V[PPPPV[_a\x04\x84\x83\x83a\x04LV[`@\x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x04\xA6\x82a\x04\x0BV[a\x04\xB0\x81\x85a\x04\x15V[\x93Pa\x04\xBB\x83a\x04%V[\x80_[\x83\x81\x10\x15a\x04\xEBW\x81Qa\x04\xD2\x88\x82a\x04yV[\x97Pa\x04\xDD\x83a\x04\x90V[\x92PP`\x01\x81\x01\x90Pa\x04\xBEV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x05\x10\x81\x84a\x04\x9CV[\x90P\x92\x91PPV[_\x81`\x02\x0B\x90P\x91\x90PV[a\x05-\x81a\x05\x18V[\x81\x14a\x057W_\x80\xFD[PV[_\x81Q\x90Pa\x05H\x81a\x05$V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x05cWa\x05ba\x03LV[[_a\x05p\x84\x82\x85\x01a\x05:V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FEphemeralPoolTickBitmap: invalid_\x82\x01R\x7F or unsupported DEX\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x06j`3\x83a\x06\0V[\x91Pa\x06u\x82a\x06\x10V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x06\x97\x81a\x06^V[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 Q\x1D\n\xAE\x83\x15\xB2\xE5F\x17v\xEF\x9F\x80\xDF\xFA\xD6\xFD\x0FO\x89498\xE5I\x861\xE2\xDA\xDD\xD5dsolcC\0\x08\x1A\x003",
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
constructor(DEX dex, V3PoolCallee pool) payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub dex: <DEX as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub pool: <V3PoolCallee as alloy::sol_types::SolType>::RustType,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (DEX, V3PoolCallee);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <DEX as alloy::sol_types::SolType>::RustType,
                <V3PoolCallee as alloy::sol_types::SolType>::RustType,
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
                    (value.dex, value.pool)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        dex: tuple.0,
                        pool: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (DEX, V3PoolCallee);
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
                )
            }
        }
    };
    /**Function with signature `getTickBitmap(uint8,address)` and selector `0xcc4e959e`.
```solidity
function getTickBitmap(DEX dex, V3PoolCallee pool) external payable returns (PoolUtils.Slot[] memory slots);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTickBitmapCall {
        #[allow(missing_docs)]
        pub dex: <DEX as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub pool: <V3PoolCallee as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`getTickBitmap(uint8,address)`](getTickBitmapCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTickBitmapReturn {
        #[allow(missing_docs)]
        pub slots: alloy::sol_types::private::Vec<
            <PoolUtils::Slot as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (DEX, V3PoolCallee);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <DEX as alloy::sol_types::SolType>::RustType,
                <V3PoolCallee as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getTickBitmapCall> for UnderlyingRustTuple<'_> {
                fn from(value: getTickBitmapCall) -> Self {
                    (value.dex, value.pool)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTickBitmapCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        dex: tuple.0,
                        pool: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<PoolUtils::Slot>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <PoolUtils::Slot as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getTickBitmapReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getTickBitmapReturn) -> Self {
                    (value.slots,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTickBitmapReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { slots: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTickBitmapCall {
            type Parameters<'a> = (DEX, V3PoolCallee);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTickBitmapReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Array<PoolUtils::Slot>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTickBitmap(uint8,address)";
            const SELECTOR: [u8; 4] = [204u8, 78u8, 149u8, 158u8];
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
    ///Container for all the [`GetCLPoolTickBitmap`](self) function calls.
    pub enum GetCLPoolTickBitmapCalls {
        #[allow(missing_docs)]
        getTickBitmap(getTickBitmapCall),
    }
    #[automatically_derived]
    impl GetCLPoolTickBitmapCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[204u8, 78u8, 149u8, 158u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for GetCLPoolTickBitmapCalls {
        const NAME: &'static str = "GetCLPoolTickBitmapCalls";
        const MIN_DATA_LENGTH: usize = 64usize;
        const COUNT: usize = 1usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::getTickBitmap(_) => {
                    <getTickBitmapCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<GetCLPoolTickBitmapCalls>] = &[
                {
                    fn getTickBitmap(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<GetCLPoolTickBitmapCalls> {
                        <getTickBitmapCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(GetCLPoolTickBitmapCalls::getTickBitmap)
                    }
                    getTickBitmap
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
                Self::getTickBitmap(inner) => {
                    <getTickBitmapCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::getTickBitmap(inner) => {
                    <getTickBitmapCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`GetCLPoolTickBitmap`](self) contract instance.

See the [wrapper's documentation](`GetCLPoolTickBitmapInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> GetCLPoolTickBitmapInstance<T, P, N> {
        GetCLPoolTickBitmapInstance::<T, P, N>::new(address, provider)
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
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<GetCLPoolTickBitmapInstance<T, P, N>>,
    > {
        GetCLPoolTickBitmapInstance::<T, P, N>::deploy(provider, dex, pool)
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
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        GetCLPoolTickBitmapInstance::<T, P, N>::deploy_builder(provider, dex, pool)
    }
    /**A [`GetCLPoolTickBitmap`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`GetCLPoolTickBitmap`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct GetCLPoolTickBitmapInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for GetCLPoolTickBitmapInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("GetCLPoolTickBitmapInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > GetCLPoolTickBitmapInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`GetCLPoolTickBitmap`](self) contract instance.

See the [wrapper's documentation](`GetCLPoolTickBitmapInstance`) for more details.*/
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
        ) -> alloy_contract::Result<GetCLPoolTickBitmapInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, dex, pool);
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
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall { dex, pool },
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
    impl<T, P: ::core::clone::Clone, N> GetCLPoolTickBitmapInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> GetCLPoolTickBitmapInstance<T, P, N> {
            GetCLPoolTickBitmapInstance {
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
    > GetCLPoolTickBitmapInstance<T, P, N> {
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
        ///Creates a new call builder for the [`getTickBitmap`] function.
        pub fn getTickBitmap(
            &self,
            dex: <DEX as alloy::sol_types::SolType>::RustType,
            pool: <V3PoolCallee as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTickBitmapCall, N> {
            self.call_builder(&getTickBitmapCall { dex, pool })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > GetCLPoolTickBitmapInstance<T, P, N> {
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
