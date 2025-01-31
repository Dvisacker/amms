/**

Generated by the following Solidity interface...
```solidity
interface GetUniV2PoolData {
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
pub mod GetUniV2PoolData {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b5060405161124938038061124983398181016040528101906100319190610d0a565b5f815167ffffffffffffffff81111561004d5761004c610b74565b5b60405190808252806020026020018201604052801561008657816020015b610073610abe565b81526020019060019003908161006b5790505b5090505f5b8251811015610a39575f8382815181106100a8576100a7610d51565b5b602002602001015190506100c181610a6760201b60201c565b156100cc5750610a2e565b6100d4610abe565b8173ffffffffffffffffffffffffffffffffffffffff16630dfe16816040518163ffffffff1660e01b8152600401602060405180830381865afa15801561011d573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101419190610d7e565b815f019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff16815250508173ffffffffffffffffffffffffffffffffffffffff1663d21220a76040518163ffffffff1660e01b8152600401602060405180830381865afa1580156101c0573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101e49190610d7e565b816060019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff168152505061022d815f0151610a6760201b60201c565b15610239575050610a2e565b61024c8160600151610a6760201b60201c565b15610258575050610a2e565b5f80825f015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f313ce567000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506040516103079190610dfb565b5f604051808303815f8787f1925050503d805f8114610341576040519150601f19603f3d011682016040523d82523d5f602084013e610346565b606091505b509150915081156103b5575f60208251036103a5578180602001905181019061036f9190610e44565b90505f81148061037f575060ff81115b1561038e575050505050610a2e565b80846040019060ff16908160ff16815250506103af565b5050505050610a2e565b506103be565b50505050610a2e565b5f80846060015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f313ce567000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff838183161783525050505060405161046e9190610dfb565b5f604051808303815f8787f1925050503d805f81146104a8576040519150601f19603f3d011682016040523d82523d5f602084013e6104ad565b606091505b50915091508115610520575f602082510361050e57818060200190518101906104d69190610e44565b90505f8114806104e6575060ff81115b156104f75750505050505050610a2e565b808660a0019060ff16908160ff168152505061051a565b50505050505050610a2e565b5061052b565b505050505050610a2e565b5f80865f015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f95d89b41000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506040516105da9190610dfb565b5f604051808303815f8787f1925050503d805f8114610614576040519150601f19603f3d011682016040523d82523d5f602084013e610619565b606091505b50915091508115610683575f602082510361064957818060200190518101906106429190610ea2565b9050610673565b5f8280602001905181019061065e9190610f6f565b905061066f81610a9860201b60201c565b9150505b8088602001818152505050610690565b5f801b8760200181815250505b5f80886060015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f95d89b41000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506040516107409190610dfb565b5f604051808303815f8787f1925050503d805f811461077a576040519150601f19603f3d011682016040523d82523d5f602084013e61077f565b606091505b509150915081156107e9575f60208251036107af57818060200190518101906107a89190610ea2565b90506107d9565b5f828060200190518101906107c49190610f6f565b90506107d581610a9860201b60201c565b9150505b808a6080018181525050506107f6565b5f801b8960800181815250505b8973ffffffffffffffffffffffffffffffffffffffff1663c45a01556040518163ffffffff1660e01b8152600401602060405180830381865afa92505050801561085e57506040513d601f19601f8201168201806040525081019061085b9190610d7e565b60015b6108a0575f89610100019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff16815250506108db565b808a610100019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff1681525050505b8973ffffffffffffffffffffffffffffffffffffffff16630902f1ac6040518163ffffffff1660e01b8152600401606060405180830381865afa92505050801561094357506040513d601f19601f820116820180604052508101906109409190611032565b60015b610960575f8960c00181815250505f8960e0018181525050610a04565b6dffffffffffffffffffffffffffff8016836dffffffffffffffffffffffffffff1611806109ad57506dffffffffffffffffffffffffffff8016826dffffffffffffffffffffffffffff16115b156109cb575f8c60c00181815250505f8c60e0018181525050610a00565b826dffffffffffffffffffffffffffff168c60c0018181525050816dffffffffffffffffffffffffffff168c60e00181815250505b5050505b888c8c81518110610a1857610a17610d51565b5b6020026020010181905250505050505050505050505b80600101905061008b565b505f81604051602001610a4c9190611228565b60405160208183030381529060405290506020810180590381f35b5f808273ffffffffffffffffffffffffffffffffffffffff163b03610a8f5760019050610a93565b5f90505b919050565b5f808290505f815103610ab0575f801b915050610ab9565b60208301519150505b919050565b6040518061012001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f80191681526020015f60ff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f80191681526020015f60ff1681526020015f81526020015f81526020015f73ffffffffffffffffffffffffffffffffffffffff1681525090565b5f604051905090565b5f80fd5b5f80fd5b5f80fd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b610baa82610b64565b810181811067ffffffffffffffff82111715610bc957610bc8610b74565b5b80604052505050565b5f610bdb610b4f565b9050610be78282610ba1565b919050565b5f67ffffffffffffffff821115610c0657610c05610b74565b5b602082029050602081019050919050565b5f80fd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610c4482610c1b565b9050919050565b610c5481610c3a565b8114610c5e575f80fd5b50565b5f81519050610c6f81610c4b565b92915050565b5f610c87610c8284610bec565b610bd2565b90508083825260208201905060208402830185811115610caa57610ca9610c17565b5b835b81811015610cd35780610cbf8882610c61565b845260208401935050602081019050610cac565b5050509392505050565b5f82601f830112610cf157610cf0610b60565b5b8151610d01848260208601610c75565b91505092915050565b5f60208284031215610d1f57610d1e610b58565b5b5f82015167ffffffffffffffff811115610d3c57610d3b610b5c565b5b610d4884828501610cdd565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f60208284031215610d9357610d92610b58565b5b5f610da084828501610c61565b91505092915050565b5f81519050919050565b5f81905092915050565b8281835e5f83830152505050565b5f610dd582610da9565b610ddf8185610db3565b9350610def818560208601610dbd565b80840191505092915050565b5f610e068284610dcb565b915081905092915050565b5f819050919050565b610e2381610e11565b8114610e2d575f80fd5b50565b5f81519050610e3e81610e1a565b92915050565b5f60208284031215610e5957610e58610b58565b5b5f610e6684828501610e30565b91505092915050565b5f819050919050565b610e8181610e6f565b8114610e8b575f80fd5b50565b5f81519050610e9c81610e78565b92915050565b5f60208284031215610eb757610eb6610b58565b5b5f610ec484828501610e8e565b91505092915050565b5f80fd5b5f67ffffffffffffffff821115610eeb57610eea610b74565b5b610ef482610b64565b9050602081019050919050565b5f610f13610f0e84610ed1565b610bd2565b905082815260208101848484011115610f2f57610f2e610ecd565b5b610f3a848285610dbd565b509392505050565b5f82601f830112610f5657610f55610b60565b5b8151610f66848260208601610f01565b91505092915050565b5f60208284031215610f8457610f83610b58565b5b5f82015167ffffffffffffffff811115610fa157610fa0610b5c565b5b610fad84828501610f42565b91505092915050565b5f6dffffffffffffffffffffffffffff82169050919050565b610fd881610fb6565b8114610fe2575f80fd5b50565b5f81519050610ff381610fcf565b92915050565b5f63ffffffff82169050919050565b61101181610ff9565b811461101b575f80fd5b50565b5f8151905061102c81611008565b92915050565b5f805f6060848603121561104957611048610b58565b5b5f61105686828701610fe5565b935050602061106786828701610fe5565b92505060406110788682870161101e565b9150509250925092565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b6110b481610c3a565b82525050565b6110c381610e6f565b82525050565b5f60ff82169050919050565b6110de816110c9565b82525050565b6110ed81610e11565b82525050565b61012082015f8201516111085f8501826110ab565b50602082015161111b60208501826110ba565b50604082015161112e60408501826110d5565b50606082015161114160608501826110ab565b50608082015161115460808501826110ba565b5060a082015161116760a08501826110d5565b5060c082015161117a60c08501826110e4565b5060e082015161118d60e08501826110e4565b506101008201516111a26101008501826110ab565b50505050565b5f6111b383836110f3565b6101208301905092915050565b5f602082019050919050565b5f6111d682611082565b6111e0818561108c565b93506111eb8361109c565b805f5b8381101561121b57815161120288826111a8565b975061120d836111c0565b9250506001810190506111ee565b5085935050505092915050565b5f6020820190508181035f83015261124081846111cc565b90509291505056fe
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`@Qa\x12I8\x03\x80a\x12I\x839\x81\x81\x01`@R\x81\x01\x90a\x001\x91\x90a\r\nV[_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0MWa\0La\x0BtV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\x86W\x81` \x01[a\0sa\n\xBEV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\0kW\x90P[P\x90P_[\x82Q\x81\x10\x15a\n9W_\x83\x82\x81Q\x81\x10a\0\xA8Wa\0\xA7a\rQV[[` \x02` \x01\x01Q\x90Pa\0\xC1\x81a\ng` \x1B` \x1CV[\x15a\0\xCCWPa\n.V[a\0\xD4a\n\xBEV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\x1DW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01A\x91\x90a\r~V[\x81_\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD2\x12 \xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xC0W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xE4\x91\x90a\r~V[\x81``\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\x02-\x81_\x01Qa\ng` \x1B` \x1CV[\x15a\x029WPPa\n.V[a\x02L\x81``\x01Qa\ng` \x1B` \x1CV[\x15a\x02XWPPa\n.V[_\x80\x82_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x03\x07\x91\x90a\r\xFBV[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x03AW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x03FV[``\x91P[P\x91P\x91P\x81\x15a\x03\xB5W_` \x82Q\x03a\x03\xA5W\x81\x80` \x01\x90Q\x81\x01\x90a\x03o\x91\x90a\x0EDV[\x90P_\x81\x14\x80a\x03\x7FWP`\xFF\x81\x11[\x15a\x03\x8EWPPPPPa\n.V[\x80\x84`@\x01\x90`\xFF\x16\x90\x81`\xFF\x16\x81RPPa\x03\xAFV[PPPPPa\n.V[Pa\x03\xBEV[PPPPa\n.V[_\x80\x84``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x04n\x91\x90a\r\xFBV[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x04\xA8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x04\xADV[``\x91P[P\x91P\x91P\x81\x15a\x05 W_` \x82Q\x03a\x05\x0EW\x81\x80` \x01\x90Q\x81\x01\x90a\x04\xD6\x91\x90a\x0EDV[\x90P_\x81\x14\x80a\x04\xE6WP`\xFF\x81\x11[\x15a\x04\xF7WPPPPPPPa\n.V[\x80\x86`\xA0\x01\x90`\xFF\x16\x90\x81`\xFF\x16\x81RPPa\x05\x1AV[PPPPPPPa\n.V[Pa\x05+V[PPPPPPa\n.V[_\x80\x86_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F\x95\xD8\x9BA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x05\xDA\x91\x90a\r\xFBV[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x06\x14W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x06\x19V[``\x91P[P\x91P\x91P\x81\x15a\x06\x83W_` \x82Q\x03a\x06IW\x81\x80` \x01\x90Q\x81\x01\x90a\x06B\x91\x90a\x0E\xA2V[\x90Pa\x06sV[_\x82\x80` \x01\x90Q\x81\x01\x90a\x06^\x91\x90a\x0FoV[\x90Pa\x06o\x81a\n\x98` \x1B` \x1CV[\x91PP[\x80\x88` \x01\x81\x81RPPPa\x06\x90V[_\x80\x1B\x87` \x01\x81\x81RPP[_\x80\x88``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F\x95\xD8\x9BA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x07@\x91\x90a\r\xFBV[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x07zW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x07\x7FV[``\x91P[P\x91P\x91P\x81\x15a\x07\xE9W_` \x82Q\x03a\x07\xAFW\x81\x80` \x01\x90Q\x81\x01\x90a\x07\xA8\x91\x90a\x0E\xA2V[\x90Pa\x07\xD9V[_\x82\x80` \x01\x90Q\x81\x01\x90a\x07\xC4\x91\x90a\x0FoV[\x90Pa\x07\xD5\x81a\n\x98` \x1B` \x1CV[\x91PP[\x80\x8A`\x80\x01\x81\x81RPPPa\x07\xF6V[_\x80\x1B\x89`\x80\x01\x81\x81RPP[\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC4Z\x01U`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x08^WP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08[\x91\x90a\r~V[`\x01[a\x08\xA0W_\x89a\x01\0\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\x08\xDBV[\x80\x8Aa\x01\0\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPP[\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\t\x02\xF1\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\tCWP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t@\x91\x90a\x102V[`\x01[a\t`W_\x89`\xC0\x01\x81\x81RPP_\x89`\xE0\x01\x81\x81RPPa\n\x04V[m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x83m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x80a\t\xADWPm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x82m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11[\x15a\t\xCBW_\x8C`\xC0\x01\x81\x81RPP_\x8C`\xE0\x01\x81\x81RPPa\n\0V[\x82m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8C`\xC0\x01\x81\x81RPP\x81m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8C`\xE0\x01\x81\x81RPP[PPP[\x88\x8C\x8C\x81Q\x81\x10a\n\x18Wa\n\x17a\rQV[[` \x02` \x01\x01\x81\x90RPPPPPPPPPPP[\x80`\x01\x01\x90Pa\0\x8BV[P_\x81`@Q` \x01a\nL\x91\x90a\x12(V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P` \x81\x01\x80Y\x03\x81\xF3[_\x80\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a\n\x8FW`\x01\x90Pa\n\x93V[_\x90P[\x91\x90PV[_\x80\x82\x90P_\x81Q\x03a\n\xB0W_\x80\x1B\x91PPa\n\xB9V[` \x83\x01Q\x91PP[\x91\x90PV[`@Q\x80a\x01 \x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x80\x19\x16\x81R` \x01_`\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x80\x19\x16\x81R` \x01_`\xFF\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x0B\xAA\x82a\x0BdV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0B\xC9Wa\x0B\xC8a\x0BtV[[\x80`@RPPPV[_a\x0B\xDBa\x0BOV[\x90Pa\x0B\xE7\x82\x82a\x0B\xA1V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0C\x06Wa\x0C\x05a\x0BtV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x80\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x0CD\x82a\x0C\x1BV[\x90P\x91\x90PV[a\x0CT\x81a\x0C:V[\x81\x14a\x0C^W_\x80\xFD[PV[_\x81Q\x90Pa\x0Co\x81a\x0CKV[\x92\x91PPV[_a\x0C\x87a\x0C\x82\x84a\x0B\xECV[a\x0B\xD2V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x0C\xAAWa\x0C\xA9a\x0C\x17V[[\x83[\x81\x81\x10\x15a\x0C\xD3W\x80a\x0C\xBF\x88\x82a\x0CaV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x0C\xACV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x0C\xF1Wa\x0C\xF0a\x0B`V[[\x81Qa\r\x01\x84\x82` \x86\x01a\x0CuV[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\r\x1FWa\r\x1Ea\x0BXV[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r<Wa\r;a\x0B\\V[[a\rH\x84\x82\x85\x01a\x0C\xDDV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\r\x93Wa\r\x92a\x0BXV[[_a\r\xA0\x84\x82\x85\x01a\x0CaV[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\r\xD5\x82a\r\xA9V[a\r\xDF\x81\x85a\r\xB3V[\x93Pa\r\xEF\x81\x85` \x86\x01a\r\xBDV[\x80\x84\x01\x91PP\x92\x91PPV[_a\x0E\x06\x82\x84a\r\xCBV[\x91P\x81\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x0E#\x81a\x0E\x11V[\x81\x14a\x0E-W_\x80\xFD[PV[_\x81Q\x90Pa\x0E>\x81a\x0E\x1AV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0EYWa\x0EXa\x0BXV[[_a\x0Ef\x84\x82\x85\x01a\x0E0V[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x0E\x81\x81a\x0EoV[\x81\x14a\x0E\x8BW_\x80\xFD[PV[_\x81Q\x90Pa\x0E\x9C\x81a\x0ExV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0E\xB7Wa\x0E\xB6a\x0BXV[[_a\x0E\xC4\x84\x82\x85\x01a\x0E\x8EV[\x91PP\x92\x91PPV[_\x80\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0E\xEBWa\x0E\xEAa\x0BtV[[a\x0E\xF4\x82a\x0BdV[\x90P` \x81\x01\x90P\x91\x90PV[_a\x0F\x13a\x0F\x0E\x84a\x0E\xD1V[a\x0B\xD2V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x0F/Wa\x0F.a\x0E\xCDV[[a\x0F:\x84\x82\x85a\r\xBDV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x0FVWa\x0FUa\x0B`V[[\x81Qa\x0Ff\x84\x82` \x86\x01a\x0F\x01V[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0F\x84Wa\x0F\x83a\x0BXV[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\xA1Wa\x0F\xA0a\x0B\\V[[a\x0F\xAD\x84\x82\x85\x01a\x0FBV[\x91PP\x92\x91PPV[_m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x0F\xD8\x81a\x0F\xB6V[\x81\x14a\x0F\xE2W_\x80\xFD[PV[_\x81Q\x90Pa\x0F\xF3\x81a\x0F\xCFV[\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x10\x11\x81a\x0F\xF9V[\x81\x14a\x10\x1BW_\x80\xFD[PV[_\x81Q\x90Pa\x10,\x81a\x10\x08V[\x92\x91PPV[_\x80_``\x84\x86\x03\x12\x15a\x10IWa\x10Ha\x0BXV[[_a\x10V\x86\x82\x87\x01a\x0F\xE5V[\x93PP` a\x10g\x86\x82\x87\x01a\x0F\xE5V[\x92PP`@a\x10x\x86\x82\x87\x01a\x10\x1EV[\x91PP\x92P\x92P\x92V[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x10\xB4\x81a\x0C:V[\x82RPPV[a\x10\xC3\x81a\x0EoV[\x82RPPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\x10\xDE\x81a\x10\xC9V[\x82RPPV[a\x10\xED\x81a\x0E\x11V[\x82RPPV[a\x01 \x82\x01_\x82\x01Qa\x11\x08_\x85\x01\x82a\x10\xABV[P` \x82\x01Qa\x11\x1B` \x85\x01\x82a\x10\xBAV[P`@\x82\x01Qa\x11.`@\x85\x01\x82a\x10\xD5V[P``\x82\x01Qa\x11A``\x85\x01\x82a\x10\xABV[P`\x80\x82\x01Qa\x11T`\x80\x85\x01\x82a\x10\xBAV[P`\xA0\x82\x01Qa\x11g`\xA0\x85\x01\x82a\x10\xD5V[P`\xC0\x82\x01Qa\x11z`\xC0\x85\x01\x82a\x10\xE4V[P`\xE0\x82\x01Qa\x11\x8D`\xE0\x85\x01\x82a\x10\xE4V[Pa\x01\0\x82\x01Qa\x11\xA2a\x01\0\x85\x01\x82a\x10\xABV[PPPPV[_a\x11\xB3\x83\x83a\x10\xF3V[a\x01 \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x11\xD6\x82a\x10\x82V[a\x11\xE0\x81\x85a\x10\x8CV[\x93Pa\x11\xEB\x83a\x10\x9CV[\x80_[\x83\x81\x10\x15a\x12\x1BW\x81Qa\x12\x02\x88\x82a\x11\xA8V[\x97Pa\x12\r\x83a\x11\xC0V[\x92PP`\x01\x81\x01\x90Pa\x11\xEEV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x12@\x81\x84a\x11\xCCV[\x90P\x92\x91PPV\xFE",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040525f80fdfea2646970667358221220236284ca5711a1baf7cb2291d5c2be481ebdd76409679ad7ffc8e6065a19e32c64736f6c634300081a0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R_\x80\xFD\xFE\xA2dipfsX\"\x12 #b\x84\xCAW\x11\xA1\xBA\xF7\xCB\"\x91\xD5\xC2\xBEH\x1E\xBD\xD7d\tg\x9A\xD7\xFF\xC8\xE6\x06Z\x19\xE3,dsolcC\0\x08\x1A\x003",
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
