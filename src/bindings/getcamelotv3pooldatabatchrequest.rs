/**

Generated by the following Solidity interface...
```solidity
interface GetCamelotV3PoolDataBatchRequest {
    constructor(address[] pools);
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
pub mod GetCamelotV3PoolDataBatchRequest {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b506040516111f13803806111f183398181016040528101906100319190610ab5565b5f815167ffffffffffffffff81111561004d5761004c61091f565b5b60405190808252806020026020018201604052801561008657816020015b610073610849565b81526020019060019003908161006b5790505b5090505f5b82518110156107ea575f8382815181106100a8576100a7610afc565b5b602002602001015190506100c18161081860201b60201c565b156100cc57506107df565b6100d4610849565b8173ffffffffffffffffffffffffffffffffffffffff16630dfe16816040518163ffffffff1660e01b8152600401602060405180830381865afa15801561011d573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101419190610b29565b815f019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff16815250508173ffffffffffffffffffffffffffffffffffffffff1663d21220a76040518163ffffffff1660e01b8152600401602060405180830381865afa1580156101c0573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101e49190610b29565b816040019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff168152505061022d815f015161081860201b60201c565b156102395750506107df565b61024c816040015161081860201b60201c565b156102585750506107df565b5f80825f015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f313ce567000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506040516103079190610ba6565b5f604051808303815f8787f1925050503d805f8114610341576040519150601f19603f3d011682016040523d82523d5f602084013e610346565b606091505b509150915081156103b5575f60208251036103a5578180602001905181019061036f9190610bef565b90505f81148061037f575060ff81115b1561038e5750505050506107df565b80846020019060ff16908160ff16815250506103af565b50505050506107df565b506103be565b505050506107df565b5f80846040015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f313ce567000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff838183161783525050505060405161046e9190610ba6565b5f604051808303815f8787f1925050503d805f81146104a8576040519150601f19603f3d011682016040523d82523d5f602084013e6104ad565b606091505b50915091508115610520575f602082510361050e57818060200190518101906104d69190610bef565b90505f8114806104e6575060ff81115b156104f757505050505050506107df565b80866060019060ff16908160ff168152505061051a565b505050505050506107df565b5061052b565b5050505050506107df565b5f808773ffffffffffffffffffffffffffffffffffffffff1663e76c01e46040518163ffffffff1660e01b815260040161010060405180830381865afa158015610577573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061059b9190610d1c565b505050505050915091505f8873ffffffffffffffffffffffffffffffffffffffff1663f30dba93836040518263ffffffff1660e01b81526004016105df9190610ddc565b61010060405180830381865afa1580156105fb573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061061f9190610edf565b5050505050509150508873ffffffffffffffffffffffffffffffffffffffff16631a6865026040518163ffffffff1660e01b8152600401602060405180830381865afa158015610671573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106959190610f90565b88608001906fffffffffffffffffffffffffffffffff1690816fffffffffffffffffffffffffffffffff16815250508873ffffffffffffffffffffffffffffffffffffffff1663d0c93a7c6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561070d573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107319190610fbb565b8860e0019060020b908160020b815250505f88610100019062ffffff16908162ffffff1681525050828860a0019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff1681525050818860c0019060020b908160020b8152505080886101200190600f0b9081600f0b81525050878b8b815181106107ca576107c9610afc565b5b60200260200101819052505050505050505050505b80600101905061008b565b505f816040516020016107fd91906111d0565b60405160208183030381529060405290506020810180590381f35b5f808273ffffffffffffffffffffffffffffffffffffffff163b036108405760019050610844565b5f90505b919050565b6040518061014001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f60ff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f60ff1681526020015f6fffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f60020b81526020015f60020b81526020015f62ffffff1681526020015f600f0b81525090565b5f604051905090565b5f80fd5b5f80fd5b5f80fd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6109558261090f565b810181811067ffffffffffffffff821117156109745761097361091f565b5b80604052505050565b5f6109866108fa565b9050610992828261094c565b919050565b5f67ffffffffffffffff8211156109b1576109b061091f565b5b602082029050602081019050919050565b5f80fd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6109ef826109c6565b9050919050565b6109ff816109e5565b8114610a09575f80fd5b50565b5f81519050610a1a816109f6565b92915050565b5f610a32610a2d84610997565b61097d565b90508083825260208201905060208402830185811115610a5557610a546109c2565b5b835b81811015610a7e5780610a6a8882610a0c565b845260208401935050602081019050610a57565b5050509392505050565b5f82601f830112610a9c57610a9b61090b565b5b8151610aac848260208601610a20565b91505092915050565b5f60208284031215610aca57610ac9610903565b5b5f82015167ffffffffffffffff811115610ae757610ae6610907565b5b610af384828501610a88565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f60208284031215610b3e57610b3d610903565b5b5f610b4b84828501610a0c565b91505092915050565b5f81519050919050565b5f81905092915050565b8281835e5f83830152505050565b5f610b8082610b54565b610b8a8185610b5e565b9350610b9a818560208601610b68565b80840191505092915050565b5f610bb18284610b76565b915081905092915050565b5f819050919050565b610bce81610bbc565b8114610bd8575f80fd5b50565b5f81519050610be981610bc5565b92915050565b5f60208284031215610c0457610c03610903565b5b5f610c1184828501610bdb565b91505092915050565b610c23816109c6565b8114610c2d575f80fd5b50565b5f81519050610c3e81610c1a565b92915050565b5f8160020b9050919050565b610c5981610c44565b8114610c63575f80fd5b50565b5f81519050610c7481610c50565b92915050565b5f61ffff82169050919050565b610c9081610c7a565b8114610c9a575f80fd5b50565b5f81519050610cab81610c87565b92915050565b5f60ff82169050919050565b610cc681610cb1565b8114610cd0575f80fd5b50565b5f81519050610ce181610cbd565b92915050565b5f8115159050919050565b610cfb81610ce7565b8114610d05575f80fd5b50565b5f81519050610d1681610cf2565b92915050565b5f805f805f805f80610100898b031215610d3957610d38610903565b5b5f610d468b828c01610c30565b9850506020610d578b828c01610c66565b9750506040610d688b828c01610c9d565b9650506060610d798b828c01610c9d565b9550506080610d8a8b828c01610c9d565b94505060a0610d9b8b828c01610cd3565b93505060c0610dac8b828c01610cd3565b92505060e0610dbd8b828c01610d08565b9150509295985092959890939650565b610dd681610c44565b82525050565b5f602082019050610def5f830184610dcd565b92915050565b5f6fffffffffffffffffffffffffffffffff82169050919050565b610e1981610df5565b8114610e23575f80fd5b50565b5f81519050610e3481610e10565b92915050565b5f81600f0b9050919050565b610e4f81610e3a565b8114610e59575f80fd5b50565b5f81519050610e6a81610e46565b92915050565b5f8160060b9050919050565b610e8581610e70565b8114610e8f575f80fd5b50565b5f81519050610ea081610e7c565b92915050565b5f63ffffffff82169050919050565b610ebe81610ea6565b8114610ec8575f80fd5b50565b5f81519050610ed981610eb5565b92915050565b5f805f805f805f80610100898b031215610efc57610efb610903565b5b5f610f098b828c01610e26565b9850506020610f1a8b828c01610e5c565b9750506040610f2b8b828c01610bdb565b9650506060610f3c8b828c01610bdb565b9550506080610f4d8b828c01610e92565b94505060a0610f5e8b828c01610c30565b93505060c0610f6f8b828c01610ecb565b92505060e0610f808b828c01610d08565b9150509295985092959890939650565b5f60208284031215610fa557610fa4610903565b5b5f610fb284828501610e26565b91505092915050565b5f60208284031215610fd057610fcf610903565b5b5f610fdd84828501610c66565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b611018816109e5565b82525050565b61102781610cb1565b82525050565b61103681610df5565b82525050565b611045816109c6565b82525050565b61105481610c44565b82525050565b5f62ffffff82169050919050565b6110718161105a565b82525050565b61108081610e3a565b82525050565b61014082015f82015161109b5f85018261100f565b5060208201516110ae602085018261101e565b5060408201516110c1604085018261100f565b5060608201516110d4606085018261101e565b5060808201516110e7608085018261102d565b5060a08201516110fa60a085018261103c565b5060c082015161110d60c085018261104b565b5060e082015161112060e085018261104b565b50610100820151611135610100850182611068565b5061012082015161114a610120850182611077565b50505050565b5f61115b8383611086565b6101408301905092915050565b5f602082019050919050565b5f61117e82610fe6565b6111888185610ff0565b935061119383611000565b805f5b838110156111c35781516111aa8882611150565b97506111b583611168565b925050600181019050611196565b5085935050505092915050565b5f6020820190508181035f8301526111e88184611174565b90509291505056fe
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`@Qa\x11\xF18\x03\x80a\x11\xF1\x839\x81\x81\x01`@R\x81\x01\x90a\x001\x91\x90a\n\xB5V[_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0MWa\0La\t\x1FV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\x86W\x81` \x01[a\0sa\x08IV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\0kW\x90P[P\x90P_[\x82Q\x81\x10\x15a\x07\xEAW_\x83\x82\x81Q\x81\x10a\0\xA8Wa\0\xA7a\n\xFCV[[` \x02` \x01\x01Q\x90Pa\0\xC1\x81a\x08\x18` \x1B` \x1CV[\x15a\0\xCCWPa\x07\xDFV[a\0\xD4a\x08IV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\x1DW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01A\x91\x90a\x0B)V[\x81_\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD2\x12 \xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xC0W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xE4\x91\x90a\x0B)V[\x81`@\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\x02-\x81_\x01Qa\x08\x18` \x1B` \x1CV[\x15a\x029WPPa\x07\xDFV[a\x02L\x81`@\x01Qa\x08\x18` \x1B` \x1CV[\x15a\x02XWPPa\x07\xDFV[_\x80\x82_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x03\x07\x91\x90a\x0B\xA6V[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x03AW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x03FV[``\x91P[P\x91P\x91P\x81\x15a\x03\xB5W_` \x82Q\x03a\x03\xA5W\x81\x80` \x01\x90Q\x81\x01\x90a\x03o\x91\x90a\x0B\xEFV[\x90P_\x81\x14\x80a\x03\x7FWP`\xFF\x81\x11[\x15a\x03\x8EWPPPPPa\x07\xDFV[\x80\x84` \x01\x90`\xFF\x16\x90\x81`\xFF\x16\x81RPPa\x03\xAFV[PPPPPa\x07\xDFV[Pa\x03\xBEV[PPPPa\x07\xDFV[_\x80\x84`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x04n\x91\x90a\x0B\xA6V[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x04\xA8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x04\xADV[``\x91P[P\x91P\x91P\x81\x15a\x05 W_` \x82Q\x03a\x05\x0EW\x81\x80` \x01\x90Q\x81\x01\x90a\x04\xD6\x91\x90a\x0B\xEFV[\x90P_\x81\x14\x80a\x04\xE6WP`\xFF\x81\x11[\x15a\x04\xF7WPPPPPPPa\x07\xDFV[\x80\x86``\x01\x90`\xFF\x16\x90\x81`\xFF\x16\x81RPPa\x05\x1AV[PPPPPPPa\x07\xDFV[Pa\x05+V[PPPPPPa\x07\xDFV[_\x80\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE7l\x01\xE4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05wW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x9B\x91\x90a\r\x1CV[PPPPPP\x91P\x91P_\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF3\r\xBA\x93\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\xDF\x91\x90a\r\xDCV[a\x01\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xFBW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x1F\x91\x90a\x0E\xDFV[PPPPPP\x91PP\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x1Ahe\x02`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06qW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x95\x91\x90a\x0F\x90V[\x88`\x80\x01\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\rW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x071\x91\x90a\x0F\xBBV[\x88`\xE0\x01\x90`\x02\x0B\x90\x81`\x02\x0B\x81RPP_\x88a\x01\0\x01\x90b\xFF\xFF\xFF\x16\x90\x81b\xFF\xFF\xFF\x16\x81RPP\x82\x88`\xA0\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81\x88`\xC0\x01\x90`\x02\x0B\x90\x81`\x02\x0B\x81RPP\x80\x88a\x01 \x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP\x87\x8B\x8B\x81Q\x81\x10a\x07\xCAWa\x07\xC9a\n\xFCV[[` \x02` \x01\x01\x81\x90RPPPPPPPPPP[\x80`\x01\x01\x90Pa\0\x8BV[P_\x81`@Q` \x01a\x07\xFD\x91\x90a\x11\xD0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P` \x81\x01\x80Y\x03\x81\xF3[_\x80\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a\x08@W`\x01\x90Pa\x08DV[_\x90P[\x91\x90PV[`@Q\x80a\x01@\x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_`\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_`\xFF\x16\x81R` \x01_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_`\x02\x0B\x81R` \x01_`\x02\x0B\x81R` \x01_b\xFF\xFF\xFF\x16\x81R` \x01_`\x0F\x0B\x81RP\x90V[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\tU\x82a\t\x0FV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\ttWa\tsa\t\x1FV[[\x80`@RPPPV[_a\t\x86a\x08\xFAV[\x90Pa\t\x92\x82\x82a\tLV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\t\xB1Wa\t\xB0a\t\x1FV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x80\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\t\xEF\x82a\t\xC6V[\x90P\x91\x90PV[a\t\xFF\x81a\t\xE5V[\x81\x14a\n\tW_\x80\xFD[PV[_\x81Q\x90Pa\n\x1A\x81a\t\xF6V[\x92\x91PPV[_a\n2a\n-\x84a\t\x97V[a\t}V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\nUWa\nTa\t\xC2V[[\x83[\x81\x81\x10\x15a\n~W\x80a\nj\x88\x82a\n\x0CV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\nWV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\n\x9CWa\n\x9Ba\t\x0BV[[\x81Qa\n\xAC\x84\x82` \x86\x01a\n V[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\n\xCAWa\n\xC9a\t\x03V[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n\xE7Wa\n\xE6a\t\x07V[[a\n\xF3\x84\x82\x85\x01a\n\x88V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\x0B>Wa\x0B=a\t\x03V[[_a\x0BK\x84\x82\x85\x01a\n\x0CV[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x0B\x80\x82a\x0BTV[a\x0B\x8A\x81\x85a\x0B^V[\x93Pa\x0B\x9A\x81\x85` \x86\x01a\x0BhV[\x80\x84\x01\x91PP\x92\x91PPV[_a\x0B\xB1\x82\x84a\x0BvV[\x91P\x81\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x0B\xCE\x81a\x0B\xBCV[\x81\x14a\x0B\xD8W_\x80\xFD[PV[_\x81Q\x90Pa\x0B\xE9\x81a\x0B\xC5V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0C\x04Wa\x0C\x03a\t\x03V[[_a\x0C\x11\x84\x82\x85\x01a\x0B\xDBV[\x91PP\x92\x91PPV[a\x0C#\x81a\t\xC6V[\x81\x14a\x0C-W_\x80\xFD[PV[_\x81Q\x90Pa\x0C>\x81a\x0C\x1AV[\x92\x91PPV[_\x81`\x02\x0B\x90P\x91\x90PV[a\x0CY\x81a\x0CDV[\x81\x14a\x0CcW_\x80\xFD[PV[_\x81Q\x90Pa\x0Ct\x81a\x0CPV[\x92\x91PPV[_a\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x0C\x90\x81a\x0CzV[\x81\x14a\x0C\x9AW_\x80\xFD[PV[_\x81Q\x90Pa\x0C\xAB\x81a\x0C\x87V[\x92\x91PPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\x0C\xC6\x81a\x0C\xB1V[\x81\x14a\x0C\xD0W_\x80\xFD[PV[_\x81Q\x90Pa\x0C\xE1\x81a\x0C\xBDV[\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a\x0C\xFB\x81a\x0C\xE7V[\x81\x14a\r\x05W_\x80\xFD[PV[_\x81Q\x90Pa\r\x16\x81a\x0C\xF2V[\x92\x91PPV[_\x80_\x80_\x80_\x80a\x01\0\x89\x8B\x03\x12\x15a\r9Wa\r8a\t\x03V[[_a\rF\x8B\x82\x8C\x01a\x0C0V[\x98PP` a\rW\x8B\x82\x8C\x01a\x0CfV[\x97PP`@a\rh\x8B\x82\x8C\x01a\x0C\x9DV[\x96PP``a\ry\x8B\x82\x8C\x01a\x0C\x9DV[\x95PP`\x80a\r\x8A\x8B\x82\x8C\x01a\x0C\x9DV[\x94PP`\xA0a\r\x9B\x8B\x82\x8C\x01a\x0C\xD3V[\x93PP`\xC0a\r\xAC\x8B\x82\x8C\x01a\x0C\xD3V[\x92PP`\xE0a\r\xBD\x8B\x82\x8C\x01a\r\x08V[\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[a\r\xD6\x81a\x0CDV[\x82RPPV[_` \x82\x01\x90Pa\r\xEF_\x83\x01\x84a\r\xCDV[\x92\x91PPV[_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x0E\x19\x81a\r\xF5V[\x81\x14a\x0E#W_\x80\xFD[PV[_\x81Q\x90Pa\x0E4\x81a\x0E\x10V[\x92\x91PPV[_\x81`\x0F\x0B\x90P\x91\x90PV[a\x0EO\x81a\x0E:V[\x81\x14a\x0EYW_\x80\xFD[PV[_\x81Q\x90Pa\x0Ej\x81a\x0EFV[\x92\x91PPV[_\x81`\x06\x0B\x90P\x91\x90PV[a\x0E\x85\x81a\x0EpV[\x81\x14a\x0E\x8FW_\x80\xFD[PV[_\x81Q\x90Pa\x0E\xA0\x81a\x0E|V[\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x0E\xBE\x81a\x0E\xA6V[\x81\x14a\x0E\xC8W_\x80\xFD[PV[_\x81Q\x90Pa\x0E\xD9\x81a\x0E\xB5V[\x92\x91PPV[_\x80_\x80_\x80_\x80a\x01\0\x89\x8B\x03\x12\x15a\x0E\xFCWa\x0E\xFBa\t\x03V[[_a\x0F\t\x8B\x82\x8C\x01a\x0E&V[\x98PP` a\x0F\x1A\x8B\x82\x8C\x01a\x0E\\V[\x97PP`@a\x0F+\x8B\x82\x8C\x01a\x0B\xDBV[\x96PP``a\x0F<\x8B\x82\x8C\x01a\x0B\xDBV[\x95PP`\x80a\x0FM\x8B\x82\x8C\x01a\x0E\x92V[\x94PP`\xA0a\x0F^\x8B\x82\x8C\x01a\x0C0V[\x93PP`\xC0a\x0Fo\x8B\x82\x8C\x01a\x0E\xCBV[\x92PP`\xE0a\x0F\x80\x8B\x82\x8C\x01a\r\x08V[\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[_` \x82\x84\x03\x12\x15a\x0F\xA5Wa\x0F\xA4a\t\x03V[[_a\x0F\xB2\x84\x82\x85\x01a\x0E&V[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0F\xD0Wa\x0F\xCFa\t\x03V[[_a\x0F\xDD\x84\x82\x85\x01a\x0CfV[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x10\x18\x81a\t\xE5V[\x82RPPV[a\x10'\x81a\x0C\xB1V[\x82RPPV[a\x106\x81a\r\xF5V[\x82RPPV[a\x10E\x81a\t\xC6V[\x82RPPV[a\x10T\x81a\x0CDV[\x82RPPV[_b\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x10q\x81a\x10ZV[\x82RPPV[a\x10\x80\x81a\x0E:V[\x82RPPV[a\x01@\x82\x01_\x82\x01Qa\x10\x9B_\x85\x01\x82a\x10\x0FV[P` \x82\x01Qa\x10\xAE` \x85\x01\x82a\x10\x1EV[P`@\x82\x01Qa\x10\xC1`@\x85\x01\x82a\x10\x0FV[P``\x82\x01Qa\x10\xD4``\x85\x01\x82a\x10\x1EV[P`\x80\x82\x01Qa\x10\xE7`\x80\x85\x01\x82a\x10-V[P`\xA0\x82\x01Qa\x10\xFA`\xA0\x85\x01\x82a\x10<V[P`\xC0\x82\x01Qa\x11\r`\xC0\x85\x01\x82a\x10KV[P`\xE0\x82\x01Qa\x11 `\xE0\x85\x01\x82a\x10KV[Pa\x01\0\x82\x01Qa\x115a\x01\0\x85\x01\x82a\x10hV[Pa\x01 \x82\x01Qa\x11Ja\x01 \x85\x01\x82a\x10wV[PPPPV[_a\x11[\x83\x83a\x10\x86V[a\x01@\x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x11~\x82a\x0F\xE6V[a\x11\x88\x81\x85a\x0F\xF0V[\x93Pa\x11\x93\x83a\x10\0V[\x80_[\x83\x81\x10\x15a\x11\xC3W\x81Qa\x11\xAA\x88\x82a\x11PV[\x97Pa\x11\xB5\x83a\x11hV[\x92PP`\x01\x81\x01\x90Pa\x11\x96V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x11\xE8\x81\x84a\x11tV[\x90P\x92\x91PPV\xFE",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040525f80fdfea26469706673582212208ecfbb2cbca589e4e6e482f2d997461eea44b8ecbeaeeb29b02ad546a28b6e7a64736f6c634300081a0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R_\x80\xFD\xFE\xA2dipfsX\"\x12 \x8E\xCF\xBB,\xBC\xA5\x89\xE4\xE6\xE4\x82\xF2\xD9\x97F\x1E\xEAD\xB8\xEC\xBE\xAE\xEB)\xB0*\xD5F\xA2\x8BnzdsolcC\0\x08\x1A\x003",
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
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`GetCamelotV3PoolDataBatchRequest`](self) contract instance.

See the [wrapper's documentation](`GetCamelotV3PoolDataBatchRequestInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> GetCamelotV3PoolDataBatchRequestInstance<T, P, N> {
        GetCamelotV3PoolDataBatchRequestInstance::<T, P, N>::new(address, provider)
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
            GetCamelotV3PoolDataBatchRequestInstance<T, P, N>,
        >,
    > {
        GetCamelotV3PoolDataBatchRequestInstance::<T, P, N>::deploy(provider, pools)
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
        GetCamelotV3PoolDataBatchRequestInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, pools)
    }
    /**A [`GetCamelotV3PoolDataBatchRequest`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`GetCamelotV3PoolDataBatchRequest`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct GetCamelotV3PoolDataBatchRequestInstance<
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
    for GetCamelotV3PoolDataBatchRequestInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("GetCamelotV3PoolDataBatchRequestInstance")
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
    > GetCamelotV3PoolDataBatchRequestInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`GetCamelotV3PoolDataBatchRequest`](self) contract instance.

See the [wrapper's documentation](`GetCamelotV3PoolDataBatchRequestInstance`) for more details.*/
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
        ) -> alloy_contract::Result<GetCamelotV3PoolDataBatchRequestInstance<T, P, N>> {
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
    > GetCamelotV3PoolDataBatchRequestInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(
            self,
        ) -> GetCamelotV3PoolDataBatchRequestInstance<T, P, N> {
            GetCamelotV3PoolDataBatchRequestInstance {
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
    > GetCamelotV3PoolDataBatchRequestInstance<T, P, N> {
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
    > GetCamelotV3PoolDataBatchRequestInstance<T, P, N> {
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
