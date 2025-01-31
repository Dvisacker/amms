///Module containing a contract's types and functions.
/**

```solidity
library PoolUtils {
    struct PopulatedTick { int24 tick; int128 liquidityNet; uint128 liquidityGross; uint256 feeGrowthOutside0X128; uint256 feeGrowthOutside1X128; }
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
    struct PopulatedTick {
        int24 tick;
        int128 liquidityNet;
        uint128 liquidityGross;
        uint256 feeGrowthOutside0X128;
        uint256 feeGrowthOutside1X128;
    }
}

interface GetCLPoolTicksInRange {
    type DEX is uint8;
    type V3PoolCallee is address;

    constructor(DEX dex, V3PoolCallee pool, int24 tickLower, int24 tickUpper) payable;

    function getPopulatedTicksInRange(DEX dex, V3PoolCallee pool, int24 tickLower, int24 tickUpper) external payable returns (PoolUtils.PopulatedTick[] memory populatedTicks);
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
        "type": "tuple[]",
        "internalType": "struct PoolUtils.PopulatedTick[]",
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
    ///0x6080604052604051610ddc380380610ddc83398181016040528101906100259190610797565b5f6100388585858561006560201b60201c565b90505f8160405160200161004c9190610975565b6040516020818303038152906040529050805160208201fd5b60608160020b8360020b1315610079575f80fd5b5f8473ffffffffffffffffffffffffffffffffffffffff1663d0c93a7c6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156100c3573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906100e79190610995565b90505f806100fc8686856101d160201b60201c565b915091505f8061011389858561021560201b60201c565b915091508067ffffffffffffffff811115610131576101306109c0565b5b60405190808252806020026020018201604052801561016a57816020015b61015761062b565b81526020019060019003908161014f5790505b5095505f808590505b8460010b8160010b136101c2576101b58c8c838a888b870361ffff16815181106101a05761019f6109ed565b5b60200260200101518d8861032260201b60201c565b9150806001019050610173565b50505050505050949350505050565b5f805f6101e4868561039560201b60201c565b905060088160020b901d9250610200858561039560201b60201c565b905060088160020b901d915050935093915050565b60605f600184846102269190610a53565b6102309190610aac565b61ffff1667ffffffffffffffff81111561024d5761024c6109c0565b5b60405190808252806020026020018201604052801561027b5781602001602082028036833780820191505090505b5091505f8490505b8360010b8160010b13610319575f6102ba828873ffffffffffffffffffffffffffffffffffffffff166103a760201b90919060201c565b9050808487846102ca9190610a53565b61ffff16815181106102df576102de6109ed565b5b6020026020010181815250506102fa816103d560201b60201c565b836103059190610b05565b9250508061031290610b38565b9050610283565b50935093915050565b5f805b610100811015610386575f816001901b86161461037b575f818860081b01870290506103798a8a83888880600101995081518110610366576103656109ed565b5b602002602001015161041e60201b60201c565b505b806001019050610325565b50819050979650505050505050565b5f808284071282840503905092915050565b5f808260010b90506103ca84635339c29660e01b835f60206105d660201b60201c565b5f5191505092915050565b5f8019808314600382048460011c1684039350600582048460021c16600583048516019350601182048460041c850116935060ff8204840260f81c8160081b1792505050919050565b81815f019060020b908160020b8152505060028081111561044257610441610b61565b5b84600281111561045557610454610b61565b5b0361053d578273ffffffffffffffffffffffffffffffffffffffff1663f30dba93836040518263ffffffff1660e01b81526004016104939190610b9d565b61014060405180830381865afa1580156104af573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906104d39190610d02565b909192939495965090919293509091925090915090505084604001856020018660600187608001848152508481525084600f0b600f0b815250846fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff16815250505050506105d0565b5f610567838573ffffffffffffffffffffffffffffffffffffffff166105f460201b90919060201c565b905080602001518260200190600f0b9081600f0b81525050805f015182604001906fffffffffffffffffffffffffffffffff1690816fffffffffffffffffffffffffffffffff168152505080604001518260600181815250508060600151826080018181525050505b50505050565b835f5282600452808260245f885afa6105ed575f80fd5b5050505050565b6105fc61066d565b5f808360020b91508290506106238563f30dba9360e01b84846101006105d660201b60201c565b505092915050565b6040518060a001604052805f60020b81526020015f600f0b81526020015f6fffffffffffffffffffffffffffffffff1681526020015f81526020015f81525090565b6040518061010001604052805f6fffffffffffffffffffffffffffffffff1681526020015f600f0b81526020015f81526020015f81526020015f60060b81526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f63ffffffff1681526020015f151581525090565b5f80fd5b600381106106f0575f80fd5b50565b5f81519050610701816106e4565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61073082610707565b9050919050565b61074081610726565b811461074a575f80fd5b50565b5f8151905061075b81610737565b92915050565b5f8160020b9050919050565b61077681610761565b8114610780575f80fd5b50565b5f815190506107918161076d565b92915050565b5f805f80608085870312156107af576107ae6106e0565b5b5f6107bc878288016106f3565b94505060206107cd8782880161074d565b93505060406107de87828801610783565b92505060606107ef87828801610783565b91505092959194509250565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b61082d81610761565b82525050565b5f81600f0b9050919050565b61084881610833565b82525050565b5f6fffffffffffffffffffffffffffffffff82169050919050565b6108728161084e565b82525050565b5f819050919050565b61088a81610878565b82525050565b60a082015f8201516108a45f850182610824565b5060208201516108b7602085018261083f565b5060408201516108ca6040850182610869565b5060608201516108dd6060850182610881565b5060808201516108f06080850182610881565b50505050565b5f6109018383610890565b60a08301905092915050565b5f602082019050919050565b5f610923826107fb565b61092d8185610805565b935061093883610815565b805f5b8381101561096857815161094f88826108f6565b975061095a8361090d565b92505060018101905061093b565b5085935050505092915050565b5f6020820190508181035f83015261098d8184610919565b905092915050565b5f602082840312156109aa576109a96106e0565b5b5f6109b784828501610783565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f8160010b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f610a5d82610a1a565b9150610a6883610a1a565b92508282039050617fff81137fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff800082121715610aa657610aa5610a26565b5b92915050565b5f610ab682610a1a565b9150610ac183610a1a565b925082820190507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80008112617fff82131715610aff57610afe610a26565b5b92915050565b5f610b0f82610878565b9150610b1a83610878565b9250828201905080821115610b3257610b31610a26565b5b92915050565b5f610b4282610a1a565b9150617fff8203610b5657610b55610a26565b5b600182019050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b610b9781610761565b82525050565b5f602082019050610bb05f830184610b8e565b92915050565b610bbf8161084e565b8114610bc9575f80fd5b50565b5f81519050610bda81610bb6565b92915050565b610be981610833565b8114610bf3575f80fd5b50565b5f81519050610c0481610be0565b92915050565b610c1381610878565b8114610c1d575f80fd5b50565b5f81519050610c2e81610c0a565b92915050565b5f8160060b9050919050565b610c4981610c34565b8114610c53575f80fd5b50565b5f81519050610c6481610c40565b92915050565b610c7381610707565b8114610c7d575f80fd5b50565b5f81519050610c8e81610c6a565b92915050565b5f63ffffffff82169050919050565b610cac81610c94565b8114610cb6575f80fd5b50565b5f81519050610cc781610ca3565b92915050565b5f8115159050919050565b610ce181610ccd565b8114610ceb575f80fd5b50565b5f81519050610cfc81610cd8565b92915050565b5f805f805f805f805f806101408b8d031215610d2157610d206106e0565b5b5f610d2e8d828e01610bcc565b9a50506020610d3f8d828e01610bf6565b9950506040610d508d828e01610bf6565b9850506060610d618d828e01610c20565b9750506080610d728d828e01610c20565b96505060a0610d838d828e01610c20565b95505060c0610d948d828e01610c56565b94505060e0610da58d828e01610c80565b935050610100610db78d828e01610cb9565b925050610120610dc98d828e01610cee565b9150509295989b9194979a509295985056fe
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`@Qa\r\xDC8\x03\x80a\r\xDC\x839\x81\x81\x01`@R\x81\x01\x90a\0%\x91\x90a\x07\x97V[_a\08\x85\x85\x85\x85a\0e` \x1B` \x1CV[\x90P_\x81`@Q` \x01a\0L\x91\x90a\tuV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80Q` \x82\x01\xFD[``\x81`\x02\x0B\x83`\x02\x0B\x13\x15a\0yW_\x80\xFD[_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xC3W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xE7\x91\x90a\t\x95V[\x90P_\x80a\0\xFC\x86\x86\x85a\x01\xD1` \x1B` \x1CV[\x91P\x91P_\x80a\x01\x13\x89\x85\x85a\x02\x15` \x1B` \x1CV[\x91P\x91P\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x011Wa\x010a\t\xC0V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01jW\x81` \x01[a\x01Wa\x06+V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x01OW\x90P[P\x95P_\x80\x85\x90P[\x84`\x01\x0B\x81`\x01\x0B\x13a\x01\xC2Wa\x01\xB5\x8C\x8C\x83\x8A\x88\x8B\x87\x03a\xFF\xFF\x16\x81Q\x81\x10a\x01\xA0Wa\x01\x9Fa\t\xEDV[[` \x02` \x01\x01Q\x8D\x88a\x03\"` \x1B` \x1CV[\x91P\x80`\x01\x01\x90Pa\x01sV[PPPPPPP\x94\x93PPPPV[_\x80_a\x01\xE4\x86\x85a\x03\x95` \x1B` \x1CV[\x90P`\x08\x81`\x02\x0B\x90\x1D\x92Pa\x02\0\x85\x85a\x03\x95` \x1B` \x1CV[\x90P`\x08\x81`\x02\x0B\x90\x1D\x91PP\x93P\x93\x91PPV[``_`\x01\x84\x84a\x02&\x91\x90a\nSV[a\x020\x91\x90a\n\xACV[a\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02MWa\x02La\t\xC0V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02{W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x91P_\x84\x90P[\x83`\x01\x0B\x81`\x01\x0B\x13a\x03\x19W_a\x02\xBA\x82\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03\xA7` \x1B\x90\x91\x90` \x1CV[\x90P\x80\x84\x87\x84a\x02\xCA\x91\x90a\nSV[a\xFF\xFF\x16\x81Q\x81\x10a\x02\xDFWa\x02\xDEa\t\xEDV[[` \x02` \x01\x01\x81\x81RPPa\x02\xFA\x81a\x03\xD5` \x1B` \x1CV[\x83a\x03\x05\x91\x90a\x0B\x05V[\x92PP\x80a\x03\x12\x90a\x0B8V[\x90Pa\x02\x83V[P\x93P\x93\x91PPV[_\x80[a\x01\0\x81\x10\x15a\x03\x86W_\x81`\x01\x90\x1B\x86\x16\x14a\x03{W_\x81\x88`\x08\x1B\x01\x87\x02\x90Pa\x03y\x8A\x8A\x83\x88\x88\x80`\x01\x01\x99P\x81Q\x81\x10a\x03fWa\x03ea\t\xEDV[[` \x02` \x01\x01Qa\x04\x1E` \x1B` \x1CV[P[\x80`\x01\x01\x90Pa\x03%V[P\x81\x90P\x97\x96PPPPPPPV[_\x80\x82\x84\x07\x12\x82\x84\x05\x03\x90P\x92\x91PPV[_\x80\x82`\x01\x0B\x90Pa\x03\xCA\x84cS9\xC2\x96`\xE0\x1B\x83_` a\x05\xD6` \x1B` \x1CV[_Q\x91PP\x92\x91PPV[_\x80\x19\x80\x83\x14`\x03\x82\x04\x84`\x01\x1C\x16\x84\x03\x93P`\x05\x82\x04\x84`\x02\x1C\x16`\x05\x83\x04\x85\x16\x01\x93P`\x11\x82\x04\x84`\x04\x1C\x85\x01\x16\x93P`\xFF\x82\x04\x84\x02`\xF8\x1C\x81`\x08\x1B\x17\x92PPP\x91\x90PV[\x81\x81_\x01\x90`\x02\x0B\x90\x81`\x02\x0B\x81RPP`\x02\x80\x81\x11\x15a\x04BWa\x04Aa\x0BaV[[\x84`\x02\x81\x11\x15a\x04UWa\x04Ta\x0BaV[[\x03a\x05=W\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF3\r\xBA\x93\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x93\x91\x90a\x0B\x9DV[a\x01@`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xAFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xD3\x91\x90a\r\x02V[\x90\x91\x92\x93\x94\x95\x96P\x90\x91\x92\x93P\x90\x91\x92P\x90\x91P\x90PP\x84`@\x01\x85` \x01\x86``\x01\x87`\x80\x01\x84\x81RP\x84\x81RP\x84`\x0F\x0B`\x0F\x0B\x81RP\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPPPPa\x05\xD0V[_a\x05g\x83\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x05\xF4` \x1B\x90\x91\x90` \x1CV[\x90P\x80` \x01Q\x82` \x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP\x80_\x01Q\x82`@\x01\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x80`@\x01Q\x82``\x01\x81\x81RPP\x80``\x01Q\x82`\x80\x01\x81\x81RPPP[PPPPV[\x83_R\x82`\x04R\x80\x82`$_\x88Z\xFAa\x05\xEDW_\x80\xFD[PPPPPV[a\x05\xFCa\x06mV[_\x80\x83`\x02\x0B\x91P\x82\x90Pa\x06#\x85c\xF3\r\xBA\x93`\xE0\x1B\x84\x84a\x01\0a\x05\xD6` \x1B` \x1CV[PP\x92\x91PPV[`@Q\x80`\xA0\x01`@R\x80_`\x02\x0B\x81R` \x01_`\x0F\x0B\x81R` \x01_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01\0\x01`@R\x80_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_`\x0F\x0B\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x06\x0B\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x15\x15\x81RP\x90V[_\x80\xFD[`\x03\x81\x10a\x06\xF0W_\x80\xFD[PV[_\x81Q\x90Pa\x07\x01\x81a\x06\xE4V[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x070\x82a\x07\x07V[\x90P\x91\x90PV[a\x07@\x81a\x07&V[\x81\x14a\x07JW_\x80\xFD[PV[_\x81Q\x90Pa\x07[\x81a\x077V[\x92\x91PPV[_\x81`\x02\x0B\x90P\x91\x90PV[a\x07v\x81a\x07aV[\x81\x14a\x07\x80W_\x80\xFD[PV[_\x81Q\x90Pa\x07\x91\x81a\x07mV[\x92\x91PPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a\x07\xAFWa\x07\xAEa\x06\xE0V[[_a\x07\xBC\x87\x82\x88\x01a\x06\xF3V[\x94PP` a\x07\xCD\x87\x82\x88\x01a\x07MV[\x93PP`@a\x07\xDE\x87\x82\x88\x01a\x07\x83V[\x92PP``a\x07\xEF\x87\x82\x88\x01a\x07\x83V[\x91PP\x92\x95\x91\x94P\x92PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x08-\x81a\x07aV[\x82RPPV[_\x81`\x0F\x0B\x90P\x91\x90PV[a\x08H\x81a\x083V[\x82RPPV[_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x08r\x81a\x08NV[\x82RPPV[_\x81\x90P\x91\x90PV[a\x08\x8A\x81a\x08xV[\x82RPPV[`\xA0\x82\x01_\x82\x01Qa\x08\xA4_\x85\x01\x82a\x08$V[P` \x82\x01Qa\x08\xB7` \x85\x01\x82a\x08?V[P`@\x82\x01Qa\x08\xCA`@\x85\x01\x82a\x08iV[P``\x82\x01Qa\x08\xDD``\x85\x01\x82a\x08\x81V[P`\x80\x82\x01Qa\x08\xF0`\x80\x85\x01\x82a\x08\x81V[PPPPV[_a\t\x01\x83\x83a\x08\x90V[`\xA0\x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\t#\x82a\x07\xFBV[a\t-\x81\x85a\x08\x05V[\x93Pa\t8\x83a\x08\x15V[\x80_[\x83\x81\x10\x15a\thW\x81Qa\tO\x88\x82a\x08\xF6V[\x97Pa\tZ\x83a\t\rV[\x92PP`\x01\x81\x01\x90Pa\t;V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\t\x8D\x81\x84a\t\x19V[\x90P\x92\x91PPV[_` \x82\x84\x03\x12\x15a\t\xAAWa\t\xA9a\x06\xE0V[[_a\t\xB7\x84\x82\x85\x01a\x07\x83V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81`\x01\x0B\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\n]\x82a\n\x1AV[\x91Pa\nh\x83a\n\x1AV[\x92P\x82\x82\x03\x90Pa\x7F\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\x82\x12\x17\x15a\n\xA6Wa\n\xA5a\n&V[[\x92\x91PPV[_a\n\xB6\x82a\n\x1AV[\x91Pa\n\xC1\x83a\n\x1AV[\x92P\x82\x82\x01\x90P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\x81\x12a\x7F\xFF\x82\x13\x17\x15a\n\xFFWa\n\xFEa\n&V[[\x92\x91PPV[_a\x0B\x0F\x82a\x08xV[\x91Pa\x0B\x1A\x83a\x08xV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x0B2Wa\x0B1a\n&V[[\x92\x91PPV[_a\x0BB\x82a\n\x1AV[\x91Pa\x7F\xFF\x82\x03a\x0BVWa\x0BUa\n&V[[`\x01\x82\x01\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[a\x0B\x97\x81a\x07aV[\x82RPPV[_` \x82\x01\x90Pa\x0B\xB0_\x83\x01\x84a\x0B\x8EV[\x92\x91PPV[a\x0B\xBF\x81a\x08NV[\x81\x14a\x0B\xC9W_\x80\xFD[PV[_\x81Q\x90Pa\x0B\xDA\x81a\x0B\xB6V[\x92\x91PPV[a\x0B\xE9\x81a\x083V[\x81\x14a\x0B\xF3W_\x80\xFD[PV[_\x81Q\x90Pa\x0C\x04\x81a\x0B\xE0V[\x92\x91PPV[a\x0C\x13\x81a\x08xV[\x81\x14a\x0C\x1DW_\x80\xFD[PV[_\x81Q\x90Pa\x0C.\x81a\x0C\nV[\x92\x91PPV[_\x81`\x06\x0B\x90P\x91\x90PV[a\x0CI\x81a\x0C4V[\x81\x14a\x0CSW_\x80\xFD[PV[_\x81Q\x90Pa\x0Cd\x81a\x0C@V[\x92\x91PPV[a\x0Cs\x81a\x07\x07V[\x81\x14a\x0C}W_\x80\xFD[PV[_\x81Q\x90Pa\x0C\x8E\x81a\x0CjV[\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x0C\xAC\x81a\x0C\x94V[\x81\x14a\x0C\xB6W_\x80\xFD[PV[_\x81Q\x90Pa\x0C\xC7\x81a\x0C\xA3V[\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a\x0C\xE1\x81a\x0C\xCDV[\x81\x14a\x0C\xEBW_\x80\xFD[PV[_\x81Q\x90Pa\x0C\xFC\x81a\x0C\xD8V[\x92\x91PPV[_\x80_\x80_\x80_\x80_\x80a\x01@\x8B\x8D\x03\x12\x15a\r!Wa\r a\x06\xE0V[[_a\r.\x8D\x82\x8E\x01a\x0B\xCCV[\x9APP` a\r?\x8D\x82\x8E\x01a\x0B\xF6V[\x99PP`@a\rP\x8D\x82\x8E\x01a\x0B\xF6V[\x98PP``a\ra\x8D\x82\x8E\x01a\x0C V[\x97PP`\x80a\rr\x8D\x82\x8E\x01a\x0C V[\x96PP`\xA0a\r\x83\x8D\x82\x8E\x01a\x0C V[\x95PP`\xC0a\r\x94\x8D\x82\x8E\x01a\x0CVV[\x94PP`\xE0a\r\xA5\x8D\x82\x8E\x01a\x0C\x80V[\x93PPa\x01\0a\r\xB7\x8D\x82\x8E\x01a\x0C\xB9V[\x92PPa\x01 a\r\xC9\x8D\x82\x8E\x01a\x0C\xEEV[\x91PP\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV\xFE",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361061001d575f3560e01c8063fa5b11d414610021575b5f80fd5b61003b6004803603810190610036919061074d565b610051565b604051610048919061092b565b60405180910390f35b60608160020b8360020b1315610065575f80fd5b5f8473ffffffffffffffffffffffffffffffffffffffff1663d0c93a7c6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156100af573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906100d3919061095f565b90505f806100e28686856101ab565b915091505f806100f38985856101e3565b915091508067ffffffffffffffff8111156101115761011061098a565b5b60405190808252806020026020018201604052801561014a57816020015b6101376105e1565b81526020019060019003908161012f5790505b5095505f808590505b8460010b8160010b1361019c5761018f8c8c838a888b870361ffff16815181106101805761017f6109b7565b5b60200260200101518d886102ea565b9150806001019050610153565b50505050505050949350505050565b5f805f6101b88685610357565b905060088160020b901d92506101ce8585610357565b905060088160020b901d915050935093915050565b60605f600184846101f49190610a1d565b6101fe9190610a76565b61ffff1667ffffffffffffffff81111561021b5761021a61098a565b5b6040519080825280602002602001820160405280156102495781602001602082028036833780820191505090505b5091505f8490505b8360010b8160010b136102e1575f610288828873ffffffffffffffffffffffffffffffffffffffff1661036990919063ffffffff16565b9050808487846102989190610a1d565b61ffff16815181106102ad576102ac6109b7565b5b6020026020010181815250506102c281610391565b836102cd9190610acf565b925050806102da90610b02565b9050610251565b50935093915050565b5f805b610100811015610348575f816001901b86161461033d575f818860081b018702905061033b8a8a8388888060010199508151811061032e5761032d6109b7565b5b60200260200101516103da565b505b8060010190506102ed565b50819050979650505050505050565b5f808284071282840503905092915050565b5f808260010b905061038684635339c29660e01b835f6020610592565b5f5191505092915050565b5f8019808314600382048460011c1684039350600582048460021c16600583048516019350601182048460041c850116935060ff8204840260f81c8160081b1792505050919050565b81815f019060020b908160020b815250506002808111156103fe576103fd610b2b565b5b84600281111561041157610410610b2b565b5b036104f9578273ffffffffffffffffffffffffffffffffffffffff1663f30dba93836040518263ffffffff1660e01b815260040161044f9190610b67565b61014060405180830381865afa15801561046b573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061048f9190610ccc565b909192939495965090919293509091925090915090505084604001856020018660600187608001848152508481525084600f0b600f0b815250846fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff168152505050505061058c565b5f610523838573ffffffffffffffffffffffffffffffffffffffff166105b090919063ffffffff16565b905080602001518260200190600f0b9081600f0b81525050805f015182604001906fffffffffffffffffffffffffffffffff1690816fffffffffffffffffffffffffffffffff168152505080604001518260600181815250508060600151826080018181525050505b50505050565b835f5282600452808260245f885afa6105a9575f80fd5b5050505050565b6105b8610623565b5f808360020b91508290506105d98563f30dba9360e01b8484610100610592565b505092915050565b6040518060a001604052805f60020b81526020015f600f0b81526020015f6fffffffffffffffffffffffffffffffff1681526020015f81526020015f81525090565b6040518061010001604052805f6fffffffffffffffffffffffffffffffff1681526020015f600f0b81526020015f81526020015f81526020015f60060b81526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f63ffffffff1681526020015f151581525090565b5f80fd5b600381106106a6575f80fd5b50565b5f813590506106b78161069a565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6106e6826106bd565b9050919050565b6106f6816106dc565b8114610700575f80fd5b50565b5f81359050610711816106ed565b92915050565b5f8160020b9050919050565b61072c81610717565b8114610736575f80fd5b50565b5f8135905061074781610723565b92915050565b5f805f806080858703121561076557610764610696565b5b5f610772878288016106a9565b945050602061078387828801610703565b935050604061079487828801610739565b92505060606107a587828801610739565b91505092959194509250565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b6107e381610717565b82525050565b5f81600f0b9050919050565b6107fe816107e9565b82525050565b5f6fffffffffffffffffffffffffffffffff82169050919050565b61082881610804565b82525050565b5f819050919050565b6108408161082e565b82525050565b60a082015f82015161085a5f8501826107da565b50602082015161086d60208501826107f5565b506040820151610880604085018261081f565b5060608201516108936060850182610837565b5060808201516108a66080850182610837565b50505050565b5f6108b78383610846565b60a08301905092915050565b5f602082019050919050565b5f6108d9826107b1565b6108e381856107bb565b93506108ee836107cb565b805f5b8381101561091e57815161090588826108ac565b9750610910836108c3565b9250506001810190506108f1565b5085935050505092915050565b5f6020820190508181035f83015261094381846108cf565b905092915050565b5f8151905061095981610723565b92915050565b5f6020828403121561097457610973610696565b5b5f6109818482850161094b565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f8160010b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f610a27826109e4565b9150610a32836109e4565b92508282039050617fff81137fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff800082121715610a7057610a6f6109f0565b5b92915050565b5f610a80826109e4565b9150610a8b836109e4565b925082820190507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80008112617fff82131715610ac957610ac86109f0565b5b92915050565b5f610ad98261082e565b9150610ae48361082e565b9250828201905080821115610afc57610afb6109f0565b5b92915050565b5f610b0c826109e4565b9150617fff8203610b2057610b1f6109f0565b5b600182019050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b610b6181610717565b82525050565b5f602082019050610b7a5f830184610b58565b92915050565b610b8981610804565b8114610b93575f80fd5b50565b5f81519050610ba481610b80565b92915050565b610bb3816107e9565b8114610bbd575f80fd5b50565b5f81519050610bce81610baa565b92915050565b610bdd8161082e565b8114610be7575f80fd5b50565b5f81519050610bf881610bd4565b92915050565b5f8160060b9050919050565b610c1381610bfe565b8114610c1d575f80fd5b50565b5f81519050610c2e81610c0a565b92915050565b610c3d816106bd565b8114610c47575f80fd5b50565b5f81519050610c5881610c34565b92915050565b5f63ffffffff82169050919050565b610c7681610c5e565b8114610c80575f80fd5b50565b5f81519050610c9181610c6d565b92915050565b5f8115159050919050565b610cab81610c97565b8114610cb5575f80fd5b50565b5f81519050610cc681610ca2565b92915050565b5f805f805f805f805f806101408b8d031215610ceb57610cea610696565b5b5f610cf88d828e01610b96565b9a50506020610d098d828e01610bc0565b9950506040610d1a8d828e01610bc0565b9850506060610d2b8d828e01610bea565b9750506080610d3c8d828e01610bea565b96505060a0610d4d8d828e01610bea565b95505060c0610d5e8d828e01610c20565b94505060e0610d6f8d828e01610c4a565b935050610100610d818d828e01610c83565b925050610120610d938d828e01610cb8565b9150509295989b9194979a509295985056fea2646970667358221220488aabe0e51b769a9225aad20a05f737198f685561dfde8b578842d24c46ae1864736f6c634300081a0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\0\x1DW_5`\xE0\x1C\x80c\xFA[\x11\xD4\x14a\0!W[_\x80\xFD[a\0;`\x04\x806\x03\x81\x01\x90a\x006\x91\x90a\x07MV[a\0QV[`@Qa\0H\x91\x90a\t+V[`@Q\x80\x91\x03\x90\xF3[``\x81`\x02\x0B\x83`\x02\x0B\x13\x15a\0eW_\x80\xFD[_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xAFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xD3\x91\x90a\t_V[\x90P_\x80a\0\xE2\x86\x86\x85a\x01\xABV[\x91P\x91P_\x80a\0\xF3\x89\x85\x85a\x01\xE3V[\x91P\x91P\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\x11Wa\x01\x10a\t\x8AV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01JW\x81` \x01[a\x017a\x05\xE1V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x01/W\x90P[P\x95P_\x80\x85\x90P[\x84`\x01\x0B\x81`\x01\x0B\x13a\x01\x9CWa\x01\x8F\x8C\x8C\x83\x8A\x88\x8B\x87\x03a\xFF\xFF\x16\x81Q\x81\x10a\x01\x80Wa\x01\x7Fa\t\xB7V[[` \x02` \x01\x01Q\x8D\x88a\x02\xEAV[\x91P\x80`\x01\x01\x90Pa\x01SV[PPPPPPP\x94\x93PPPPV[_\x80_a\x01\xB8\x86\x85a\x03WV[\x90P`\x08\x81`\x02\x0B\x90\x1D\x92Pa\x01\xCE\x85\x85a\x03WV[\x90P`\x08\x81`\x02\x0B\x90\x1D\x91PP\x93P\x93\x91PPV[``_`\x01\x84\x84a\x01\xF4\x91\x90a\n\x1DV[a\x01\xFE\x91\x90a\nvV[a\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\x1BWa\x02\x1Aa\t\x8AV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02IW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x91P_\x84\x90P[\x83`\x01\x0B\x81`\x01\x0B\x13a\x02\xE1W_a\x02\x88\x82\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03i\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80\x84\x87\x84a\x02\x98\x91\x90a\n\x1DV[a\xFF\xFF\x16\x81Q\x81\x10a\x02\xADWa\x02\xACa\t\xB7V[[` \x02` \x01\x01\x81\x81RPPa\x02\xC2\x81a\x03\x91V[\x83a\x02\xCD\x91\x90a\n\xCFV[\x92PP\x80a\x02\xDA\x90a\x0B\x02V[\x90Pa\x02QV[P\x93P\x93\x91PPV[_\x80[a\x01\0\x81\x10\x15a\x03HW_\x81`\x01\x90\x1B\x86\x16\x14a\x03=W_\x81\x88`\x08\x1B\x01\x87\x02\x90Pa\x03;\x8A\x8A\x83\x88\x88\x80`\x01\x01\x99P\x81Q\x81\x10a\x03.Wa\x03-a\t\xB7V[[` \x02` \x01\x01Qa\x03\xDAV[P[\x80`\x01\x01\x90Pa\x02\xEDV[P\x81\x90P\x97\x96PPPPPPPV[_\x80\x82\x84\x07\x12\x82\x84\x05\x03\x90P\x92\x91PPV[_\x80\x82`\x01\x0B\x90Pa\x03\x86\x84cS9\xC2\x96`\xE0\x1B\x83_` a\x05\x92V[_Q\x91PP\x92\x91PPV[_\x80\x19\x80\x83\x14`\x03\x82\x04\x84`\x01\x1C\x16\x84\x03\x93P`\x05\x82\x04\x84`\x02\x1C\x16`\x05\x83\x04\x85\x16\x01\x93P`\x11\x82\x04\x84`\x04\x1C\x85\x01\x16\x93P`\xFF\x82\x04\x84\x02`\xF8\x1C\x81`\x08\x1B\x17\x92PPP\x91\x90PV[\x81\x81_\x01\x90`\x02\x0B\x90\x81`\x02\x0B\x81RPP`\x02\x80\x81\x11\x15a\x03\xFEWa\x03\xFDa\x0B+V[[\x84`\x02\x81\x11\x15a\x04\x11Wa\x04\x10a\x0B+V[[\x03a\x04\xF9W\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF3\r\xBA\x93\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04O\x91\x90a\x0BgV[a\x01@`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04kW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x8F\x91\x90a\x0C\xCCV[\x90\x91\x92\x93\x94\x95\x96P\x90\x91\x92\x93P\x90\x91\x92P\x90\x91P\x90PP\x84`@\x01\x85` \x01\x86``\x01\x87`\x80\x01\x84\x81RP\x84\x81RP\x84`\x0F\x0B`\x0F\x0B\x81RP\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPPPPa\x05\x8CV[_a\x05#\x83\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x05\xB0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80` \x01Q\x82` \x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP\x80_\x01Q\x82`@\x01\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x80`@\x01Q\x82``\x01\x81\x81RPP\x80``\x01Q\x82`\x80\x01\x81\x81RPPP[PPPPV[\x83_R\x82`\x04R\x80\x82`$_\x88Z\xFAa\x05\xA9W_\x80\xFD[PPPPPV[a\x05\xB8a\x06#V[_\x80\x83`\x02\x0B\x91P\x82\x90Pa\x05\xD9\x85c\xF3\r\xBA\x93`\xE0\x1B\x84\x84a\x01\0a\x05\x92V[PP\x92\x91PPV[`@Q\x80`\xA0\x01`@R\x80_`\x02\x0B\x81R` \x01_`\x0F\x0B\x81R` \x01_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01\0\x01`@R\x80_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_`\x0F\x0B\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x06\x0B\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x15\x15\x81RP\x90V[_\x80\xFD[`\x03\x81\x10a\x06\xA6W_\x80\xFD[PV[_\x815\x90Pa\x06\xB7\x81a\x06\x9AV[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x06\xE6\x82a\x06\xBDV[\x90P\x91\x90PV[a\x06\xF6\x81a\x06\xDCV[\x81\x14a\x07\0W_\x80\xFD[PV[_\x815\x90Pa\x07\x11\x81a\x06\xEDV[\x92\x91PPV[_\x81`\x02\x0B\x90P\x91\x90PV[a\x07,\x81a\x07\x17V[\x81\x14a\x076W_\x80\xFD[PV[_\x815\x90Pa\x07G\x81a\x07#V[\x92\x91PPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a\x07eWa\x07da\x06\x96V[[_a\x07r\x87\x82\x88\x01a\x06\xA9V[\x94PP` a\x07\x83\x87\x82\x88\x01a\x07\x03V[\x93PP`@a\x07\x94\x87\x82\x88\x01a\x079V[\x92PP``a\x07\xA5\x87\x82\x88\x01a\x079V[\x91PP\x92\x95\x91\x94P\x92PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x07\xE3\x81a\x07\x17V[\x82RPPV[_\x81`\x0F\x0B\x90P\x91\x90PV[a\x07\xFE\x81a\x07\xE9V[\x82RPPV[_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x08(\x81a\x08\x04V[\x82RPPV[_\x81\x90P\x91\x90PV[a\x08@\x81a\x08.V[\x82RPPV[`\xA0\x82\x01_\x82\x01Qa\x08Z_\x85\x01\x82a\x07\xDAV[P` \x82\x01Qa\x08m` \x85\x01\x82a\x07\xF5V[P`@\x82\x01Qa\x08\x80`@\x85\x01\x82a\x08\x1FV[P``\x82\x01Qa\x08\x93``\x85\x01\x82a\x087V[P`\x80\x82\x01Qa\x08\xA6`\x80\x85\x01\x82a\x087V[PPPPV[_a\x08\xB7\x83\x83a\x08FV[`\xA0\x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x08\xD9\x82a\x07\xB1V[a\x08\xE3\x81\x85a\x07\xBBV[\x93Pa\x08\xEE\x83a\x07\xCBV[\x80_[\x83\x81\x10\x15a\t\x1EW\x81Qa\t\x05\x88\x82a\x08\xACV[\x97Pa\t\x10\x83a\x08\xC3V[\x92PP`\x01\x81\x01\x90Pa\x08\xF1V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\tC\x81\x84a\x08\xCFV[\x90P\x92\x91PPV[_\x81Q\x90Pa\tY\x81a\x07#V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\ttWa\tsa\x06\x96V[[_a\t\x81\x84\x82\x85\x01a\tKV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81`\x01\x0B\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\n'\x82a\t\xE4V[\x91Pa\n2\x83a\t\xE4V[\x92P\x82\x82\x03\x90Pa\x7F\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\x82\x12\x17\x15a\npWa\noa\t\xF0V[[\x92\x91PPV[_a\n\x80\x82a\t\xE4V[\x91Pa\n\x8B\x83a\t\xE4V[\x92P\x82\x82\x01\x90P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\x81\x12a\x7F\xFF\x82\x13\x17\x15a\n\xC9Wa\n\xC8a\t\xF0V[[\x92\x91PPV[_a\n\xD9\x82a\x08.V[\x91Pa\n\xE4\x83a\x08.V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\n\xFCWa\n\xFBa\t\xF0V[[\x92\x91PPV[_a\x0B\x0C\x82a\t\xE4V[\x91Pa\x7F\xFF\x82\x03a\x0B Wa\x0B\x1Fa\t\xF0V[[`\x01\x82\x01\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[a\x0Ba\x81a\x07\x17V[\x82RPPV[_` \x82\x01\x90Pa\x0Bz_\x83\x01\x84a\x0BXV[\x92\x91PPV[a\x0B\x89\x81a\x08\x04V[\x81\x14a\x0B\x93W_\x80\xFD[PV[_\x81Q\x90Pa\x0B\xA4\x81a\x0B\x80V[\x92\x91PPV[a\x0B\xB3\x81a\x07\xE9V[\x81\x14a\x0B\xBDW_\x80\xFD[PV[_\x81Q\x90Pa\x0B\xCE\x81a\x0B\xAAV[\x92\x91PPV[a\x0B\xDD\x81a\x08.V[\x81\x14a\x0B\xE7W_\x80\xFD[PV[_\x81Q\x90Pa\x0B\xF8\x81a\x0B\xD4V[\x92\x91PPV[_\x81`\x06\x0B\x90P\x91\x90PV[a\x0C\x13\x81a\x0B\xFEV[\x81\x14a\x0C\x1DW_\x80\xFD[PV[_\x81Q\x90Pa\x0C.\x81a\x0C\nV[\x92\x91PPV[a\x0C=\x81a\x06\xBDV[\x81\x14a\x0CGW_\x80\xFD[PV[_\x81Q\x90Pa\x0CX\x81a\x0C4V[\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x0Cv\x81a\x0C^V[\x81\x14a\x0C\x80W_\x80\xFD[PV[_\x81Q\x90Pa\x0C\x91\x81a\x0CmV[\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a\x0C\xAB\x81a\x0C\x97V[\x81\x14a\x0C\xB5W_\x80\xFD[PV[_\x81Q\x90Pa\x0C\xC6\x81a\x0C\xA2V[\x92\x91PPV[_\x80_\x80_\x80_\x80_\x80a\x01@\x8B\x8D\x03\x12\x15a\x0C\xEBWa\x0C\xEAa\x06\x96V[[_a\x0C\xF8\x8D\x82\x8E\x01a\x0B\x96V[\x9APP` a\r\t\x8D\x82\x8E\x01a\x0B\xC0V[\x99PP`@a\r\x1A\x8D\x82\x8E\x01a\x0B\xC0V[\x98PP``a\r+\x8D\x82\x8E\x01a\x0B\xEAV[\x97PP`\x80a\r<\x8D\x82\x8E\x01a\x0B\xEAV[\x96PP`\xA0a\rM\x8D\x82\x8E\x01a\x0B\xEAV[\x95PP`\xC0a\r^\x8D\x82\x8E\x01a\x0C V[\x94PP`\xE0a\ro\x8D\x82\x8E\x01a\x0CJV[\x93PPa\x01\0a\r\x81\x8D\x82\x8E\x01a\x0C\x83V[\x92PPa\x01 a\r\x93\x8D\x82\x8E\x01a\x0C\xB8V[\x91PP\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV\xFE\xA2dipfsX\"\x12 H\x8A\xAB\xE0\xE5\x1Bv\x9A\x92%\xAA\xD2\n\x05\xF77\x19\x8FhUa\xDF\xDE\x8BW\x88B\xD2LF\xAE\x18dsolcC\0\x08\x1A\x003",
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
function getPopulatedTicksInRange(DEX dex, V3PoolCallee pool, int24 tickLower, int24 tickUpper) external payable returns (PoolUtils.PopulatedTick[] memory populatedTicks);
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
        pub populatedTicks: alloy::sol_types::private::Vec<
            <PoolUtils::PopulatedTick as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Array<PoolUtils::PopulatedTick>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <PoolUtils::PopulatedTick as alloy::sol_types::SolType>::RustType,
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
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<PoolUtils::PopulatedTick>,
            );
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
