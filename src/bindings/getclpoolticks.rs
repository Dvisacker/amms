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

interface GetCLPoolTicks {
    type DEX is uint8;
    type V3PoolCallee is address;

    constructor(DEX dex, V3PoolCallee pool, int24 tickLower, int24 tickUpper) payable;

    function getPopulatedTicksInRange(DEX dex, V3PoolCallee pool, int24 tickLower, int24 tickUpper) external payable returns (PoolUtils.Slot[] memory slots);
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
pub mod GetCLPoolTicks {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052604051610bd8380380610bd8833981810160405281019061002591906107cf565b5f6100388585858561006560201b60201c565b90505f8160405160200161004c9190610920565b6040516020818303038152906040529050805160208201fd5b60608160020b8360020b1315610079575f80fd5b5f8473ffffffffffffffffffffffffffffffffffffffff1663d0c93a7c6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156100c3573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906100e79190610940565b90505f806100fc8686856101d560201b60201c565b915091505f8061011389858561021960201b60201c565b91509150600281901b67ffffffffffffffff8111156101355761013461096b565b5b60405190808252806020026020018201604052801561016e57816020015b61015b61068d565b8152602001906001900390816101535790505b5095505f808590505b8460010b8160010b136101c6576101b98c8c838a888b870361ffff16815181106101a4576101a3610998565b5b60200260200101518d8861032660201b60201c565b9150806001019050610177565b50505050505050949350505050565b5f805f6101e8868561050560201b60201c565b905060088160020b901d9250610204858561050560201b60201c565b905060088160020b901d915050935093915050565b60605f6001848461022a91906109fe565b6102349190610a57565b61ffff1667ffffffffffffffff8111156102515761025061096b565b5b60405190808252806020026020018201604052801561027f5781602001602082028036833780820191505090505b5091505f8490505b8360010b8160010b1361031d575f6102be828873ffffffffffffffffffffffffffffffffffffffff1661051760201b90919060201c565b9050808487846102ce91906109fe565b61ffff16815181106102e3576102e2610998565b5b6020026020010181815250506102fe8161054560201b60201c565b836103099190610ab0565b9250508061031690610ae3565b9050610287565b50935093915050565b5f806103378961058e60201b60201c565b90505f5b6101008110156104f5575f816001901b8716146104ea575f818960081b01880290505f610387828c73ffffffffffffffffffffffffffffffffffffffff1661063860201b90919060201c565b90505f825f528460205260405f2090505f82516020840151818160801b17925050506040518060400160405280838060010194508152602001828152508989806001019a50815181106103dd576103dc610998565b5b6020026020010181905250506040518060400160405280828060010193508152602001836040015181525088888060010199508151811061042157610420610998565b5b60200260200101819052506040518060400160405280828060010193508152602001836060015181525088888060010199508151811061046457610463610998565b5b60200260200101819052505f60e08301518060f81b915060c0840151828160d81b17925060a0850151838160381b179350608086015166ffffffffffffff168481179450505050506040518060400160405280838152602001828152508989806001019a50815181106104da576104d9610998565b5b6020026020010181905250505050505b80600101905061033b565b5082915050979650505050505050565b5f808284071282840503905092915050565b5f808260010b905061053a84635339c29660e01b835f602061066f60201b60201c565b5f5191505092915050565b5f8019808314600382048460011c1684039350600582048460021c16600583048516019350601182048460041c850116935060ff8204840260f81c8160081b1792505050919050565b5f8060028111156105a2576105a1610b0c565b5b8260028111156105b5576105b4610b0c565b5b036105c35760059050610633565b600160028111156105d7576105d6610b0c565b5b8260028111156105ea576105e9610b0c565b5b036105f85760069050610633565b6040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161062a90610bb9565b60405180910390fd5b919050565b6106406106a5565b5f808360020b91508290506106678563f30dba9360e01b848461010061066f60201b60201c565b505092915050565b835f5282600452808260245f885afa610686575f80fd5b5050505050565b60405180604001604052805f81526020015f81525090565b6040518061010001604052805f6fffffffffffffffffffffffffffffffff1681526020015f600f0b81526020015f81526020015f81526020015f60060b81526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f63ffffffff1681526020015f151581525090565b5f80fd5b60038110610728575f80fd5b50565b5f815190506107398161071c565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6107688261073f565b9050919050565b6107788161075e565b8114610782575f80fd5b50565b5f815190506107938161076f565b92915050565b5f8160020b9050919050565b6107ae81610799565b81146107b8575f80fd5b50565b5f815190506107c9816107a5565b92915050565b5f805f80608085870312156107e7576107e6610718565b5b5f6107f48782880161072b565b945050602061080587828801610785565b9350506040610816878288016107bb565b9250506060610827878288016107bb565b91505092959194509250565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f819050919050565b61086e8161085c565b82525050565b604082015f8201516108885f850182610865565b50602082015161089b6020850182610865565b50505050565b5f6108ac8383610874565b60408301905092915050565b5f602082019050919050565b5f6108ce82610833565b6108d8818561083d565b93506108e38361084d565b805f5b838110156109135781516108fa88826108a1565b9750610905836108b8565b9250506001810190506108e6565b5085935050505092915050565b5f6020820190508181035f83015261093881846108c4565b905092915050565b5f6020828403121561095557610954610718565b5b5f610962848285016107bb565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f8160010b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f610a08826109c5565b9150610a13836109c5565b92508282039050617fff81137fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff800082121715610a5157610a506109d1565b5b92915050565b5f610a61826109c5565b9150610a6c836109c5565b925082820190507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80008112617fff82131715610aaa57610aa96109d1565b5b92915050565b5f610aba8261085c565b9150610ac58361085c565b9250828201905080821115610add57610adc6109d1565b5b92915050565b5f610aed826109c5565b9150617fff8203610b0157610b006109d1565b5b600182019050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b5f82825260208201905092915050565b7f457068656d6572616c506f6f6c5469636b733a20696e76616c6964206f7220755f8201527f6e737570706f7274656420444558000000000000000000000000000000000000602082015250565b5f610ba3602e83610b39565b9150610bae82610b49565b604082019050919050565b5f6020820190508181035f830152610bd081610b97565b905091905056fe
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`@Qa\x0B\xD88\x03\x80a\x0B\xD8\x839\x81\x81\x01`@R\x81\x01\x90a\0%\x91\x90a\x07\xCFV[_a\08\x85\x85\x85\x85a\0e` \x1B` \x1CV[\x90P_\x81`@Q` \x01a\0L\x91\x90a\t V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80Q` \x82\x01\xFD[``\x81`\x02\x0B\x83`\x02\x0B\x13\x15a\0yW_\x80\xFD[_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xC3W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xE7\x91\x90a\t@V[\x90P_\x80a\0\xFC\x86\x86\x85a\x01\xD5` \x1B` \x1CV[\x91P\x91P_\x80a\x01\x13\x89\x85\x85a\x02\x19` \x1B` \x1CV[\x91P\x91P`\x02\x81\x90\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x015Wa\x014a\tkV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01nW\x81` \x01[a\x01[a\x06\x8DV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x01SW\x90P[P\x95P_\x80\x85\x90P[\x84`\x01\x0B\x81`\x01\x0B\x13a\x01\xC6Wa\x01\xB9\x8C\x8C\x83\x8A\x88\x8B\x87\x03a\xFF\xFF\x16\x81Q\x81\x10a\x01\xA4Wa\x01\xA3a\t\x98V[[` \x02` \x01\x01Q\x8D\x88a\x03&` \x1B` \x1CV[\x91P\x80`\x01\x01\x90Pa\x01wV[PPPPPPP\x94\x93PPPPV[_\x80_a\x01\xE8\x86\x85a\x05\x05` \x1B` \x1CV[\x90P`\x08\x81`\x02\x0B\x90\x1D\x92Pa\x02\x04\x85\x85a\x05\x05` \x1B` \x1CV[\x90P`\x08\x81`\x02\x0B\x90\x1D\x91PP\x93P\x93\x91PPV[``_`\x01\x84\x84a\x02*\x91\x90a\t\xFEV[a\x024\x91\x90a\nWV[a\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02QWa\x02Pa\tkV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02\x7FW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x91P_\x84\x90P[\x83`\x01\x0B\x81`\x01\x0B\x13a\x03\x1DW_a\x02\xBE\x82\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x05\x17` \x1B\x90\x91\x90` \x1CV[\x90P\x80\x84\x87\x84a\x02\xCE\x91\x90a\t\xFEV[a\xFF\xFF\x16\x81Q\x81\x10a\x02\xE3Wa\x02\xE2a\t\x98V[[` \x02` \x01\x01\x81\x81RPPa\x02\xFE\x81a\x05E` \x1B` \x1CV[\x83a\x03\t\x91\x90a\n\xB0V[\x92PP\x80a\x03\x16\x90a\n\xE3V[\x90Pa\x02\x87V[P\x93P\x93\x91PPV[_\x80a\x037\x89a\x05\x8E` \x1B` \x1CV[\x90P_[a\x01\0\x81\x10\x15a\x04\xF5W_\x81`\x01\x90\x1B\x87\x16\x14a\x04\xEAW_\x81\x89`\x08\x1B\x01\x88\x02\x90P_a\x03\x87\x82\x8Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x068` \x1B\x90\x91\x90` \x1CV[\x90P_\x82_R\x84` R`@_ \x90P_\x82Q` \x84\x01Q\x81\x81`\x80\x1B\x17\x92PPP`@Q\x80`@\x01`@R\x80\x83\x80`\x01\x01\x94P\x81R` \x01\x82\x81RP\x89\x89\x80`\x01\x01\x9AP\x81Q\x81\x10a\x03\xDDWa\x03\xDCa\t\x98V[[` \x02` \x01\x01\x81\x90RPP`@Q\x80`@\x01`@R\x80\x82\x80`\x01\x01\x93P\x81R` \x01\x83`@\x01Q\x81RP\x88\x88\x80`\x01\x01\x99P\x81Q\x81\x10a\x04!Wa\x04 a\t\x98V[[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80\x82\x80`\x01\x01\x93P\x81R` \x01\x83``\x01Q\x81RP\x88\x88\x80`\x01\x01\x99P\x81Q\x81\x10a\x04dWa\x04ca\t\x98V[[` \x02` \x01\x01\x81\x90RP_`\xE0\x83\x01Q\x80`\xF8\x1B\x91P`\xC0\x84\x01Q\x82\x81`\xD8\x1B\x17\x92P`\xA0\x85\x01Q\x83\x81`8\x1B\x17\x93P`\x80\x86\x01Qf\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x81\x17\x94PPPPP`@Q\x80`@\x01`@R\x80\x83\x81R` \x01\x82\x81RP\x89\x89\x80`\x01\x01\x9AP\x81Q\x81\x10a\x04\xDAWa\x04\xD9a\t\x98V[[` \x02` \x01\x01\x81\x90RPPPPP[\x80`\x01\x01\x90Pa\x03;V[P\x82\x91PP\x97\x96PPPPPPPV[_\x80\x82\x84\x07\x12\x82\x84\x05\x03\x90P\x92\x91PPV[_\x80\x82`\x01\x0B\x90Pa\x05:\x84cS9\xC2\x96`\xE0\x1B\x83_` a\x06o` \x1B` \x1CV[_Q\x91PP\x92\x91PPV[_\x80\x19\x80\x83\x14`\x03\x82\x04\x84`\x01\x1C\x16\x84\x03\x93P`\x05\x82\x04\x84`\x02\x1C\x16`\x05\x83\x04\x85\x16\x01\x93P`\x11\x82\x04\x84`\x04\x1C\x85\x01\x16\x93P`\xFF\x82\x04\x84\x02`\xF8\x1C\x81`\x08\x1B\x17\x92PPP\x91\x90PV[_\x80`\x02\x81\x11\x15a\x05\xA2Wa\x05\xA1a\x0B\x0CV[[\x82`\x02\x81\x11\x15a\x05\xB5Wa\x05\xB4a\x0B\x0CV[[\x03a\x05\xC3W`\x05\x90Pa\x063V[`\x01`\x02\x81\x11\x15a\x05\xD7Wa\x05\xD6a\x0B\x0CV[[\x82`\x02\x81\x11\x15a\x05\xEAWa\x05\xE9a\x0B\x0CV[[\x03a\x05\xF8W`\x06\x90Pa\x063V[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06*\x90a\x0B\xB9V[`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[a\x06@a\x06\xA5V[_\x80\x83`\x02\x0B\x91P\x82\x90Pa\x06g\x85c\xF3\r\xBA\x93`\xE0\x1B\x84\x84a\x01\0a\x06o` \x1B` \x1CV[PP\x92\x91PPV[\x83_R\x82`\x04R\x80\x82`$_\x88Z\xFAa\x06\x86W_\x80\xFD[PPPPPV[`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01\0\x01`@R\x80_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_`\x0F\x0B\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x06\x0B\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x15\x15\x81RP\x90V[_\x80\xFD[`\x03\x81\x10a\x07(W_\x80\xFD[PV[_\x81Q\x90Pa\x079\x81a\x07\x1CV[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x07h\x82a\x07?V[\x90P\x91\x90PV[a\x07x\x81a\x07^V[\x81\x14a\x07\x82W_\x80\xFD[PV[_\x81Q\x90Pa\x07\x93\x81a\x07oV[\x92\x91PPV[_\x81`\x02\x0B\x90P\x91\x90PV[a\x07\xAE\x81a\x07\x99V[\x81\x14a\x07\xB8W_\x80\xFD[PV[_\x81Q\x90Pa\x07\xC9\x81a\x07\xA5V[\x92\x91PPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a\x07\xE7Wa\x07\xE6a\x07\x18V[[_a\x07\xF4\x87\x82\x88\x01a\x07+V[\x94PP` a\x08\x05\x87\x82\x88\x01a\x07\x85V[\x93PP`@a\x08\x16\x87\x82\x88\x01a\x07\xBBV[\x92PP``a\x08'\x87\x82\x88\x01a\x07\xBBV[\x91PP\x92\x95\x91\x94P\x92PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\x08n\x81a\x08\\V[\x82RPPV[`@\x82\x01_\x82\x01Qa\x08\x88_\x85\x01\x82a\x08eV[P` \x82\x01Qa\x08\x9B` \x85\x01\x82a\x08eV[PPPPV[_a\x08\xAC\x83\x83a\x08tV[`@\x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x08\xCE\x82a\x083V[a\x08\xD8\x81\x85a\x08=V[\x93Pa\x08\xE3\x83a\x08MV[\x80_[\x83\x81\x10\x15a\t\x13W\x81Qa\x08\xFA\x88\x82a\x08\xA1V[\x97Pa\t\x05\x83a\x08\xB8V[\x92PP`\x01\x81\x01\x90Pa\x08\xE6V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\t8\x81\x84a\x08\xC4V[\x90P\x92\x91PPV[_` \x82\x84\x03\x12\x15a\tUWa\tTa\x07\x18V[[_a\tb\x84\x82\x85\x01a\x07\xBBV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81`\x01\x0B\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\n\x08\x82a\t\xC5V[\x91Pa\n\x13\x83a\t\xC5V[\x92P\x82\x82\x03\x90Pa\x7F\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\x82\x12\x17\x15a\nQWa\nPa\t\xD1V[[\x92\x91PPV[_a\na\x82a\t\xC5V[\x91Pa\nl\x83a\t\xC5V[\x92P\x82\x82\x01\x90P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\x81\x12a\x7F\xFF\x82\x13\x17\x15a\n\xAAWa\n\xA9a\t\xD1V[[\x92\x91PPV[_a\n\xBA\x82a\x08\\V[\x91Pa\n\xC5\x83a\x08\\V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\n\xDDWa\n\xDCa\t\xD1V[[\x92\x91PPV[_a\n\xED\x82a\t\xC5V[\x91Pa\x7F\xFF\x82\x03a\x0B\x01Wa\x0B\0a\t\xD1V[[`\x01\x82\x01\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FEphemeralPoolTicks: invalid or u_\x82\x01R\x7Fnsupported DEX\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x0B\xA3`.\x83a\x0B9V[\x91Pa\x0B\xAE\x82a\x0BIV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x0B\xD0\x81a\x0B\x97V[\x90P\x91\x90PV\xFE",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361061001d575f3560e01c8063fa5b11d414610021575b5f80fd5b61003b60048036038101906100369190610785565b610051565b60405161004891906108d6565b60405180910390f35b60608160020b8360020b1315610065575f80fd5b5f8473ffffffffffffffffffffffffffffffffffffffff1663d0c93a7c6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156100af573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906100d3919061090a565b90505f806100e28686856101af565b915091505f806100f38985856101e7565b91509150600281901b67ffffffffffffffff81111561011557610114610935565b5b60405190808252806020026020018201604052801561014e57816020015b61013b610643565b8152602001906001900390816101335790505b5095505f808590505b8460010b8160010b136101a0576101938c8c838a888b870361ffff168151811061018457610183610962565b5b60200260200101518d886102ee565b9150806001019050610157565b50505050505050949350505050565b5f805f6101bc86856104c7565b905060088160020b901d92506101d285856104c7565b905060088160020b901d915050935093915050565b60605f600184846101f891906109c8565b6102029190610a21565b61ffff1667ffffffffffffffff81111561021f5761021e610935565b5b60405190808252806020026020018201604052801561024d5781602001602082028036833780820191505090505b5091505f8490505b8360010b8160010b136102e5575f61028c828873ffffffffffffffffffffffffffffffffffffffff166104d990919063ffffffff16565b90508084878461029c91906109c8565b61ffff16815181106102b1576102b0610962565b5b6020026020010181815250506102c681610501565b836102d19190610a7a565b925050806102de90610aad565b9050610255565b50935093915050565b5f806102f98961054a565b90505f5b6101008110156104b7575f816001901b8716146104ac575f818960081b01880290505f610349828c73ffffffffffffffffffffffffffffffffffffffff166105f490919063ffffffff16565b90505f825f528460205260405f2090505f82516020840151818160801b17925050506040518060400160405280838060010194508152602001828152508989806001019a508151811061039f5761039e610962565b5b602002602001018190525050604051806040016040528082806001019350815260200183604001518152508888806001019950815181106103e3576103e2610962565b5b60200260200101819052506040518060400160405280828060010193508152602001836060015181525088888060010199508151811061042657610425610962565b5b60200260200101819052505f60e08301518060f81b915060c0840151828160d81b17925060a0850151838160381b179350608086015166ffffffffffffff168481179450505050506040518060400160405280838152602001828152508989806001019a508151811061049c5761049b610962565b5b6020026020010181905250505050505b8060010190506102fd565b5082915050979650505050505050565b5f808284071282840503905092915050565b5f808260010b90506104f684635339c29660e01b835f6020610625565b5f5191505092915050565b5f8019808314600382048460011c1684039350600582048460021c16600583048516019350601182048460041c850116935060ff8204840260f81c8160081b1792505050919050565b5f80600281111561055e5761055d610ad6565b5b82600281111561057157610570610ad6565b5b0361057f57600590506105ef565b6001600281111561059357610592610ad6565b5b8260028111156105a6576105a5610ad6565b5b036105b457600690506105ef565b6040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016105e690610b83565b60405180910390fd5b919050565b6105fc61065b565b5f808360020b915082905061061d8563f30dba9360e01b8484610100610625565b505092915050565b835f5282600452808260245f885afa61063c575f80fd5b5050505050565b60405180604001604052805f81526020015f81525090565b6040518061010001604052805f6fffffffffffffffffffffffffffffffff1681526020015f600f0b81526020015f81526020015f81526020015f60060b81526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f63ffffffff1681526020015f151581525090565b5f80fd5b600381106106de575f80fd5b50565b5f813590506106ef816106d2565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61071e826106f5565b9050919050565b61072e81610714565b8114610738575f80fd5b50565b5f8135905061074981610725565b92915050565b5f8160020b9050919050565b6107648161074f565b811461076e575f80fd5b50565b5f8135905061077f8161075b565b92915050565b5f805f806080858703121561079d5761079c6106ce565b5b5f6107aa878288016106e1565b94505060206107bb8782880161073b565b93505060406107cc87828801610771565b92505060606107dd87828801610771565b91505092959194509250565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f819050919050565b61082481610812565b82525050565b604082015f82015161083e5f85018261081b565b506020820151610851602085018261081b565b50505050565b5f610862838361082a565b60408301905092915050565b5f602082019050919050565b5f610884826107e9565b61088e81856107f3565b935061089983610803565b805f5b838110156108c95781516108b08882610857565b97506108bb8361086e565b92505060018101905061089c565b5085935050505092915050565b5f6020820190508181035f8301526108ee818461087a565b905092915050565b5f815190506109048161075b565b92915050565b5f6020828403121561091f5761091e6106ce565b5b5f61092c848285016108f6565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f8160010b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f6109d28261098f565b91506109dd8361098f565b92508282039050617fff81137fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff800082121715610a1b57610a1a61099b565b5b92915050565b5f610a2b8261098f565b9150610a368361098f565b925082820190507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80008112617fff82131715610a7457610a7361099b565b5b92915050565b5f610a8482610812565b9150610a8f83610812565b9250828201905080821115610aa757610aa661099b565b5b92915050565b5f610ab78261098f565b9150617fff8203610acb57610aca61099b565b5b600182019050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b5f82825260208201905092915050565b7f457068656d6572616c506f6f6c5469636b733a20696e76616c6964206f7220755f8201527f6e737570706f7274656420444558000000000000000000000000000000000000602082015250565b5f610b6d602e83610b03565b9150610b7882610b13565b604082019050919050565b5f6020820190508181035f830152610b9a81610b61565b905091905056fea2646970667358221220b6487674d13bfe6e6adabb2e5515725b1fd03c50ca73ccfd54546d2f3284203564736f6c634300081a0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\0\x1DW_5`\xE0\x1C\x80c\xFA[\x11\xD4\x14a\0!W[_\x80\xFD[a\0;`\x04\x806\x03\x81\x01\x90a\x006\x91\x90a\x07\x85V[a\0QV[`@Qa\0H\x91\x90a\x08\xD6V[`@Q\x80\x91\x03\x90\xF3[``\x81`\x02\x0B\x83`\x02\x0B\x13\x15a\0eW_\x80\xFD[_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xAFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xD3\x91\x90a\t\nV[\x90P_\x80a\0\xE2\x86\x86\x85a\x01\xAFV[\x91P\x91P_\x80a\0\xF3\x89\x85\x85a\x01\xE7V[\x91P\x91P`\x02\x81\x90\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\x15Wa\x01\x14a\t5V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01NW\x81` \x01[a\x01;a\x06CV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x013W\x90P[P\x95P_\x80\x85\x90P[\x84`\x01\x0B\x81`\x01\x0B\x13a\x01\xA0Wa\x01\x93\x8C\x8C\x83\x8A\x88\x8B\x87\x03a\xFF\xFF\x16\x81Q\x81\x10a\x01\x84Wa\x01\x83a\tbV[[` \x02` \x01\x01Q\x8D\x88a\x02\xEEV[\x91P\x80`\x01\x01\x90Pa\x01WV[PPPPPPP\x94\x93PPPPV[_\x80_a\x01\xBC\x86\x85a\x04\xC7V[\x90P`\x08\x81`\x02\x0B\x90\x1D\x92Pa\x01\xD2\x85\x85a\x04\xC7V[\x90P`\x08\x81`\x02\x0B\x90\x1D\x91PP\x93P\x93\x91PPV[``_`\x01\x84\x84a\x01\xF8\x91\x90a\t\xC8V[a\x02\x02\x91\x90a\n!V[a\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\x1FWa\x02\x1Ea\t5V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02MW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x91P_\x84\x90P[\x83`\x01\x0B\x81`\x01\x0B\x13a\x02\xE5W_a\x02\x8C\x82\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x04\xD9\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80\x84\x87\x84a\x02\x9C\x91\x90a\t\xC8V[a\xFF\xFF\x16\x81Q\x81\x10a\x02\xB1Wa\x02\xB0a\tbV[[` \x02` \x01\x01\x81\x81RPPa\x02\xC6\x81a\x05\x01V[\x83a\x02\xD1\x91\x90a\nzV[\x92PP\x80a\x02\xDE\x90a\n\xADV[\x90Pa\x02UV[P\x93P\x93\x91PPV[_\x80a\x02\xF9\x89a\x05JV[\x90P_[a\x01\0\x81\x10\x15a\x04\xB7W_\x81`\x01\x90\x1B\x87\x16\x14a\x04\xACW_\x81\x89`\x08\x1B\x01\x88\x02\x90P_a\x03I\x82\x8Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x05\xF4\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_\x82_R\x84` R`@_ \x90P_\x82Q` \x84\x01Q\x81\x81`\x80\x1B\x17\x92PPP`@Q\x80`@\x01`@R\x80\x83\x80`\x01\x01\x94P\x81R` \x01\x82\x81RP\x89\x89\x80`\x01\x01\x9AP\x81Q\x81\x10a\x03\x9FWa\x03\x9Ea\tbV[[` \x02` \x01\x01\x81\x90RPP`@Q\x80`@\x01`@R\x80\x82\x80`\x01\x01\x93P\x81R` \x01\x83`@\x01Q\x81RP\x88\x88\x80`\x01\x01\x99P\x81Q\x81\x10a\x03\xE3Wa\x03\xE2a\tbV[[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80\x82\x80`\x01\x01\x93P\x81R` \x01\x83``\x01Q\x81RP\x88\x88\x80`\x01\x01\x99P\x81Q\x81\x10a\x04&Wa\x04%a\tbV[[` \x02` \x01\x01\x81\x90RP_`\xE0\x83\x01Q\x80`\xF8\x1B\x91P`\xC0\x84\x01Q\x82\x81`\xD8\x1B\x17\x92P`\xA0\x85\x01Q\x83\x81`8\x1B\x17\x93P`\x80\x86\x01Qf\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x81\x17\x94PPPPP`@Q\x80`@\x01`@R\x80\x83\x81R` \x01\x82\x81RP\x89\x89\x80`\x01\x01\x9AP\x81Q\x81\x10a\x04\x9CWa\x04\x9Ba\tbV[[` \x02` \x01\x01\x81\x90RPPPPP[\x80`\x01\x01\x90Pa\x02\xFDV[P\x82\x91PP\x97\x96PPPPPPPV[_\x80\x82\x84\x07\x12\x82\x84\x05\x03\x90P\x92\x91PPV[_\x80\x82`\x01\x0B\x90Pa\x04\xF6\x84cS9\xC2\x96`\xE0\x1B\x83_` a\x06%V[_Q\x91PP\x92\x91PPV[_\x80\x19\x80\x83\x14`\x03\x82\x04\x84`\x01\x1C\x16\x84\x03\x93P`\x05\x82\x04\x84`\x02\x1C\x16`\x05\x83\x04\x85\x16\x01\x93P`\x11\x82\x04\x84`\x04\x1C\x85\x01\x16\x93P`\xFF\x82\x04\x84\x02`\xF8\x1C\x81`\x08\x1B\x17\x92PPP\x91\x90PV[_\x80`\x02\x81\x11\x15a\x05^Wa\x05]a\n\xD6V[[\x82`\x02\x81\x11\x15a\x05qWa\x05pa\n\xD6V[[\x03a\x05\x7FW`\x05\x90Pa\x05\xEFV[`\x01`\x02\x81\x11\x15a\x05\x93Wa\x05\x92a\n\xD6V[[\x82`\x02\x81\x11\x15a\x05\xA6Wa\x05\xA5a\n\xD6V[[\x03a\x05\xB4W`\x06\x90Pa\x05\xEFV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05\xE6\x90a\x0B\x83V[`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[a\x05\xFCa\x06[V[_\x80\x83`\x02\x0B\x91P\x82\x90Pa\x06\x1D\x85c\xF3\r\xBA\x93`\xE0\x1B\x84\x84a\x01\0a\x06%V[PP\x92\x91PPV[\x83_R\x82`\x04R\x80\x82`$_\x88Z\xFAa\x06<W_\x80\xFD[PPPPPV[`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01\0\x01`@R\x80_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_`\x0F\x0B\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x06\x0B\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x15\x15\x81RP\x90V[_\x80\xFD[`\x03\x81\x10a\x06\xDEW_\x80\xFD[PV[_\x815\x90Pa\x06\xEF\x81a\x06\xD2V[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x07\x1E\x82a\x06\xF5V[\x90P\x91\x90PV[a\x07.\x81a\x07\x14V[\x81\x14a\x078W_\x80\xFD[PV[_\x815\x90Pa\x07I\x81a\x07%V[\x92\x91PPV[_\x81`\x02\x0B\x90P\x91\x90PV[a\x07d\x81a\x07OV[\x81\x14a\x07nW_\x80\xFD[PV[_\x815\x90Pa\x07\x7F\x81a\x07[V[\x92\x91PPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a\x07\x9DWa\x07\x9Ca\x06\xCEV[[_a\x07\xAA\x87\x82\x88\x01a\x06\xE1V[\x94PP` a\x07\xBB\x87\x82\x88\x01a\x07;V[\x93PP`@a\x07\xCC\x87\x82\x88\x01a\x07qV[\x92PP``a\x07\xDD\x87\x82\x88\x01a\x07qV[\x91PP\x92\x95\x91\x94P\x92PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\x08$\x81a\x08\x12V[\x82RPPV[`@\x82\x01_\x82\x01Qa\x08>_\x85\x01\x82a\x08\x1BV[P` \x82\x01Qa\x08Q` \x85\x01\x82a\x08\x1BV[PPPPV[_a\x08b\x83\x83a\x08*V[`@\x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x08\x84\x82a\x07\xE9V[a\x08\x8E\x81\x85a\x07\xF3V[\x93Pa\x08\x99\x83a\x08\x03V[\x80_[\x83\x81\x10\x15a\x08\xC9W\x81Qa\x08\xB0\x88\x82a\x08WV[\x97Pa\x08\xBB\x83a\x08nV[\x92PP`\x01\x81\x01\x90Pa\x08\x9CV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x08\xEE\x81\x84a\x08zV[\x90P\x92\x91PPV[_\x81Q\x90Pa\t\x04\x81a\x07[V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\t\x1FWa\t\x1Ea\x06\xCEV[[_a\t,\x84\x82\x85\x01a\x08\xF6V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81`\x01\x0B\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\t\xD2\x82a\t\x8FV[\x91Pa\t\xDD\x83a\t\x8FV[\x92P\x82\x82\x03\x90Pa\x7F\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\x82\x12\x17\x15a\n\x1BWa\n\x1Aa\t\x9BV[[\x92\x91PPV[_a\n+\x82a\t\x8FV[\x91Pa\n6\x83a\t\x8FV[\x92P\x82\x82\x01\x90P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\x81\x12a\x7F\xFF\x82\x13\x17\x15a\ntWa\nsa\t\x9BV[[\x92\x91PPV[_a\n\x84\x82a\x08\x12V[\x91Pa\n\x8F\x83a\x08\x12V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\n\xA7Wa\n\xA6a\t\x9BV[[\x92\x91PPV[_a\n\xB7\x82a\t\x8FV[\x91Pa\x7F\xFF\x82\x03a\n\xCBWa\n\xCAa\t\x9BV[[`\x01\x82\x01\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FEphemeralPoolTicks: invalid or u_\x82\x01R\x7Fnsupported DEX\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x0Bm`.\x83a\x0B\x03V[\x91Pa\x0Bx\x82a\x0B\x13V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x0B\x9A\x81a\x0BaV[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 \xB6Hvt\xD1;\xFEnj\xDA\xBB.U\x15r[\x1F\xD0<P\xCAs\xCC\xFDTTm/2\x84 5dsolcC\0\x08\x1A\x003",
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
function getPopulatedTicksInRange(DEX dex, V3PoolCallee pool, int24 tickLower, int24 tickUpper) external payable returns (PoolUtils.Slot[] memory slots);
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
            impl ::core::convert::From<getPopulatedTicksInRangeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPopulatedTicksInRangeReturn) -> Self {
                    (value.slots,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPopulatedTicksInRangeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { slots: tuple.0 }
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
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Array<PoolUtils::Slot>,);
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
    ///Container for all the [`GetCLPoolTicks`](self) function calls.
    pub enum GetCLPoolTicksCalls {
        #[allow(missing_docs)]
        getPopulatedTicksInRange(getPopulatedTicksInRangeCall),
    }
    #[automatically_derived]
    impl GetCLPoolTicksCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[250u8, 91u8, 17u8, 212u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for GetCLPoolTicksCalls {
        const NAME: &'static str = "GetCLPoolTicksCalls";
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
            ) -> alloy_sol_types::Result<GetCLPoolTicksCalls>] = &[
                {
                    fn getPopulatedTicksInRange(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<GetCLPoolTicksCalls> {
                        <getPopulatedTicksInRangeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(GetCLPoolTicksCalls::getPopulatedTicksInRange)
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
    /**Creates a new wrapper around an on-chain [`GetCLPoolTicks`](self) contract instance.

See the [wrapper's documentation](`GetCLPoolTicksInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> GetCLPoolTicksInstance<T, P, N> {
        GetCLPoolTicksInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<GetCLPoolTicksInstance<T, P, N>>,
    > {
        GetCLPoolTicksInstance::<
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
        GetCLPoolTicksInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, dex, pool, tickLower, tickUpper)
    }
    /**A [`GetCLPoolTicks`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`GetCLPoolTicks`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct GetCLPoolTicksInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for GetCLPoolTicksInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("GetCLPoolTicksInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > GetCLPoolTicksInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`GetCLPoolTicks`](self) contract instance.

See the [wrapper's documentation](`GetCLPoolTicksInstance`) for more details.*/
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
        ) -> alloy_contract::Result<GetCLPoolTicksInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> GetCLPoolTicksInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> GetCLPoolTicksInstance<T, P, N> {
            GetCLPoolTicksInstance {
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
    > GetCLPoolTicksInstance<T, P, N> {
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
    > GetCLPoolTicksInstance<T, P, N> {
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
