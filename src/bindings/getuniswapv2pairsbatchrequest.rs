/**

Generated by the following Solidity interface...
```solidity
interface GetUniswapV2PairsBatchRequest {
    constructor(uint256 from, uint256 step, address factory);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "from",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "step",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "factory",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "nonpayable"
  }
]
```*/
#[allow(non_camel_case_types, non_snake_case, clippy::style)]
pub mod GetUniswapV2PairsBatchRequest {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b5060405161049b38038061049b83398181016040528101906100319190610233565b5f838361003e91906102b0565b90505f8167ffffffffffffffff81111561005b5761005a6102e3565b5b6040519080825280602002602001820160405280156100895781602001602082028036833780820191505090505b5090505f5b82811015610174578373ffffffffffffffffffffffffffffffffffffffff16631e3dd18b82886100be9190610310565b6040518263ffffffff1660e01b81526004016100da9190610352565b6020604051808303815f875af11580156100f6573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061011a919061036b565b82828151811061012d5761012c610396565b5b602002602001019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff1681525050808060010191505061008e565b505f81604051602001610187919061047a565b60405160208183030381529060405290506020810180590381f35b5f80fd5b5f819050919050565b6101b8816101a6565b81146101c2575f80fd5b50565b5f815190506101d3816101af565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610202826101d9565b9050919050565b610212816101f8565b811461021c575f80fd5b50565b5f8151905061022d81610209565b92915050565b5f805f6060848603121561024a576102496101a2565b5b5f610257868287016101c5565b9350506020610268868287016101c5565b92505060406102798682870161021f565b9150509250925092565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f6102ba826101a6565b91506102c5836101a6565b92508282039050818111156102dd576102dc610283565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b5f61031a826101a6565b9150610325836101a6565b925082820190508082111561033d5761033c610283565b5b92915050565b61034c816101a6565b82525050565b5f6020820190506103655f830184610343565b92915050565b5f602082840312156103805761037f6101a2565b5b5f61038d8482850161021f565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b6103f5816101f8565b82525050565b5f61040683836103ec565b60208301905092915050565b5f602082019050919050565b5f610428826103c3565b61043281856103cd565b935061043d836103dd565b805f5b8381101561046d57815161045488826103fb565b975061045f83610412565b925050600181019050610440565b5085935050505092915050565b5f6020820190508181035f830152610492818461041e565b90509291505056fe
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`@Qa\x04\x9B8\x03\x80a\x04\x9B\x839\x81\x81\x01`@R\x81\x01\x90a\x001\x91\x90a\x023V[_\x83\x83a\0>\x91\x90a\x02\xB0V[\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0[Wa\0Za\x02\xE3V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\x89W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_[\x82\x81\x10\x15a\x01tW\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x1E=\xD1\x8B\x82\x88a\0\xBE\x91\x90a\x03\x10V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\0\xDA\x91\x90a\x03RV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\0\xF6W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x1A\x91\x90a\x03kV[\x82\x82\x81Q\x81\x10a\x01-Wa\x01,a\x03\x96V[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x80\x80`\x01\x01\x91PPa\0\x8EV[P_\x81`@Q` \x01a\x01\x87\x91\x90a\x04zV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P` \x81\x01\x80Y\x03\x81\xF3[_\x80\xFD[_\x81\x90P\x91\x90PV[a\x01\xB8\x81a\x01\xA6V[\x81\x14a\x01\xC2W_\x80\xFD[PV[_\x81Q\x90Pa\x01\xD3\x81a\x01\xAFV[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x02\x02\x82a\x01\xD9V[\x90P\x91\x90PV[a\x02\x12\x81a\x01\xF8V[\x81\x14a\x02\x1CW_\x80\xFD[PV[_\x81Q\x90Pa\x02-\x81a\x02\tV[\x92\x91PPV[_\x80_``\x84\x86\x03\x12\x15a\x02JWa\x02Ia\x01\xA2V[[_a\x02W\x86\x82\x87\x01a\x01\xC5V[\x93PP` a\x02h\x86\x82\x87\x01a\x01\xC5V[\x92PP`@a\x02y\x86\x82\x87\x01a\x02\x1FV[\x91PP\x92P\x92P\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x02\xBA\x82a\x01\xA6V[\x91Pa\x02\xC5\x83a\x01\xA6V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x02\xDDWa\x02\xDCa\x02\x83V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[_a\x03\x1A\x82a\x01\xA6V[\x91Pa\x03%\x83a\x01\xA6V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x03=Wa\x03<a\x02\x83V[[\x92\x91PPV[a\x03L\x81a\x01\xA6V[\x82RPPV[_` \x82\x01\x90Pa\x03e_\x83\x01\x84a\x03CV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x03\x80Wa\x03\x7Fa\x01\xA2V[[_a\x03\x8D\x84\x82\x85\x01a\x02\x1FV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x03\xF5\x81a\x01\xF8V[\x82RPPV[_a\x04\x06\x83\x83a\x03\xECV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x04(\x82a\x03\xC3V[a\x042\x81\x85a\x03\xCDV[\x93Pa\x04=\x83a\x03\xDDV[\x80_[\x83\x81\x10\x15a\x04mW\x81Qa\x04T\x88\x82a\x03\xFBV[\x97Pa\x04_\x83a\x04\x12V[\x92PP`\x01\x81\x01\x90Pa\x04@V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x04\x92\x81\x84a\x04\x1EV[\x90P\x92\x91PPV\xFE",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040525f80fdfea2646970667358221220a17bdf131d3b540cdb04533f360bdb64b5f1951371e7a8795a85a1370690afd464736f6c634300081a0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R_\x80\xFD\xFE\xA2dipfsX\"\x12 \xA1{\xDF\x13\x1D;T\x0C\xDB\x04S?6\x0B\xDBd\xB5\xF1\x95\x13q\xE7\xA8yZ\x85\xA17\x06\x90\xAF\xD4dsolcC\0\x08\x1A\x003",
    );
    /**Constructor`.
```solidity
constructor(uint256 from, uint256 step, address factory);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub from: alloy::sol_types::private::primitives::aliases::U256,
        pub step: alloy::sol_types::private::primitives::aliases::U256,
        pub factory: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value.from, value.step, value.factory)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        from: tuple.0,
                        step: tuple.1,
                        factory: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.from),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.step),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.factory,
                    ),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`GetUniswapV2PairsBatchRequest`](self) contract instance.

See the [wrapper's documentation](`GetUniswapV2PairsBatchRequestInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> GetUniswapV2PairsBatchRequestInstance<T, P, N> {
        GetUniswapV2PairsBatchRequestInstance::<T, P, N>::new(address, provider)
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
        from: alloy::sol_types::private::primitives::aliases::U256,
        step: alloy::sol_types::private::primitives::aliases::U256,
        factory: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<GetUniswapV2PairsBatchRequestInstance<T, P, N>>,
    > {
        GetUniswapV2PairsBatchRequestInstance::<
            T,
            P,
            N,
        >::deploy(provider, from, step, factory)
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
        from: alloy::sol_types::private::primitives::aliases::U256,
        step: alloy::sol_types::private::primitives::aliases::U256,
        factory: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        GetUniswapV2PairsBatchRequestInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, from, step, factory)
    }
    /**A [`GetUniswapV2PairsBatchRequest`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`GetUniswapV2PairsBatchRequest`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct GetUniswapV2PairsBatchRequestInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for GetUniswapV2PairsBatchRequestInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("GetUniswapV2PairsBatchRequestInstance")
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
    > GetUniswapV2PairsBatchRequestInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`GetUniswapV2PairsBatchRequest`](self) contract instance.

See the [wrapper's documentation](`GetUniswapV2PairsBatchRequestInstance`) for more details.*/
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
            from: alloy::sol_types::private::primitives::aliases::U256,
            step: alloy::sol_types::private::primitives::aliases::U256,
            factory: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<GetUniswapV2PairsBatchRequestInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, from, step, factory);
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
            from: alloy::sol_types::private::primitives::aliases::U256,
            step: alloy::sol_types::private::primitives::aliases::U256,
            factory: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            from,
                            step,
                            factory,
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
    impl<T, P: ::core::clone::Clone, N> GetUniswapV2PairsBatchRequestInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(
            self,
        ) -> GetUniswapV2PairsBatchRequestInstance<T, P, N> {
            GetUniswapV2PairsBatchRequestInstance {
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
    > GetUniswapV2PairsBatchRequestInstance<T, P, N> {
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
    > GetUniswapV2PairsBatchRequestInstance<T, P, N> {
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
