/**

Generated by the following Solidity interface...
```solidity
interface GetDetailedPoolDataBatchRequest {
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
#[allow(non_camel_case_types, non_snake_case, clippy::style)]
pub mod GetDetailedPoolDataBatchRequest {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b5060405161118138038061118183398181016040528101906100319190610c6e565b5f815167ffffffffffffffff81111561004d5761004c610ad8565b5b60405190808252806020026020018201604052801561008657816020015b6100736109f7565b81526020019060019003908161006b5790505b5090505f5b8251811015610972575f8382815181106100a8576100a7610cb5565b5b602002602001015190506100c1816109a060201b60201c565b156100cc5750610967565b6100d46109f7565b8173ffffffffffffffffffffffffffffffffffffffff16630dfe16816040518163ffffffff1660e01b8152600401602060405180830381865afa15801561011d573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101419190610ce2565b815f019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff16815250508173ffffffffffffffffffffffffffffffffffffffff1663d21220a76040518163ffffffff1660e01b8152600401602060405180830381865afa1580156101c0573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101e49190610ce2565b816060019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff168152505061022d815f01516109a060201b60201c565b15610239575050610967565b61024c81606001516109a060201b60201c565b15610258575050610967565b5f80825f015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f313ce567000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506040516103079190610d5f565b5f604051808303815f8787f1925050503d805f8114610341576040519150601f19603f3d011682016040523d82523d5f602084013e610346565b606091505b509150915081156103b5575f60208251036103a5578180602001905181019061036f9190610da8565b90505f81148061037f575060ff81115b1561038e575050505050610967565b80846040019060ff16908160ff16815250506103af565b5050505050610967565b506103be565b50505050610967565b5f80846060015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f313ce567000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff838183161783525050505060405161046e9190610d5f565b5f604051808303815f8787f1925050503d805f81146104a8576040519150601f19603f3d011682016040523d82523d5f602084013e6104ad565b606091505b50915091508115610520575f602082510361050e57818060200190518101906104d69190610da8565b90505f8114806104e6575060ff81115b156104f75750505050505050610967565b808660a0019060ff16908160ff168152505061051a565b50505050505050610967565b5061052b565b505050505050610967565b5f80865f015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f95d89b41000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506040516105da9190610d5f565b5f604051808303815f8787f1925050503d805f8114610614576040519150601f19603f3d011682016040523d82523d5f602084013e610619565b606091505b50915091508115610683575f602082510361064957818060200190518101906106429190610e06565b9050610673565b5f8280602001905181019061065e9190610ed3565b905061066f816109d160201b60201c565b9150505b8088602001818152505050610690565b5f801b8760200181815250505b5f80886060015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f95d89b41000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff83818316178352505050506040516107409190610d5f565b5f604051808303815f8787f1925050503d805f811461077a576040519150601f19603f3d011682016040523d82523d5f602084013e61077f565b606091505b509150915081156107e9575f60208251036107af57818060200190518101906107a89190610e06565b90506107d9565b5f828060200190518101906107c49190610ed3565b90506107d5816109d160201b60201c565b9150505b808a6080018181525050506107f6565b5f801b8960800181815250505b8973ffffffffffffffffffffffffffffffffffffffff1663c45a01556040518163ffffffff1660e01b8152600401602060405180830381865afa15801561083f573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108639190610ce2565b8960c0019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff16815250508973ffffffffffffffffffffffffffffffffffffffff1663ddca3f436040518163ffffffff1660e01b8152600401602060405180830381865afa92505050801561090257506040513d601f19601f820116820180604052508101906108ff9190610f52565b60015b61092457610bb889610120019062ffffff16908162ffffff168152505061093d565b808a610120019062ffffff16908162ffffff1681525050505b888c8c8151811061095157610950610cb5565b5b6020026020010181905250505050505050505050505b80600101905061008b565b505f816040516020016109859190611160565b60405160208183030381529060405290506020810180590381f35b5f808273ffffffffffffffffffffffffffffffffffffffff163b036109c857600190506109cc565b5f90505b919050565b5f808290505f8151036109e9575f801b9150506109f2565b60208301519150505b919050565b6040518061014001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f80191681526020015f60ff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f80191681526020015f60ff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f6dffffffffffffffffffffffffffff1681526020015f6dffffffffffffffffffffffffffff1681526020015f62ffffff1681525090565b5f604051905090565b5f80fd5b5f80fd5b5f80fd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b610b0e82610ac8565b810181811067ffffffffffffffff82111715610b2d57610b2c610ad8565b5b80604052505050565b5f610b3f610ab3565b9050610b4b8282610b05565b919050565b5f67ffffffffffffffff821115610b6a57610b69610ad8565b5b602082029050602081019050919050565b5f80fd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f610ba882610b7f565b9050919050565b610bb881610b9e565b8114610bc2575f80fd5b50565b5f81519050610bd381610baf565b92915050565b5f610beb610be684610b50565b610b36565b90508083825260208201905060208402830185811115610c0e57610c0d610b7b565b5b835b81811015610c375780610c238882610bc5565b845260208401935050602081019050610c10565b5050509392505050565b5f82601f830112610c5557610c54610ac4565b5b8151610c65848260208601610bd9565b91505092915050565b5f60208284031215610c8357610c82610abc565b5b5f82015167ffffffffffffffff811115610ca057610c9f610ac0565b5b610cac84828501610c41565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f60208284031215610cf757610cf6610abc565b5b5f610d0484828501610bc5565b91505092915050565b5f81519050919050565b5f81905092915050565b8281835e5f83830152505050565b5f610d3982610d0d565b610d438185610d17565b9350610d53818560208601610d21565b80840191505092915050565b5f610d6a8284610d2f565b915081905092915050565b5f819050919050565b610d8781610d75565b8114610d91575f80fd5b50565b5f81519050610da281610d7e565b92915050565b5f60208284031215610dbd57610dbc610abc565b5b5f610dca84828501610d94565b91505092915050565b5f819050919050565b610de581610dd3565b8114610def575f80fd5b50565b5f81519050610e0081610ddc565b92915050565b5f60208284031215610e1b57610e1a610abc565b5b5f610e2884828501610df2565b91505092915050565b5f80fd5b5f67ffffffffffffffff821115610e4f57610e4e610ad8565b5b610e5882610ac8565b9050602081019050919050565b5f610e77610e7284610e35565b610b36565b905082815260208101848484011115610e9357610e92610e31565b5b610e9e848285610d21565b509392505050565b5f82601f830112610eba57610eb9610ac4565b5b8151610eca848260208601610e65565b91505092915050565b5f60208284031215610ee857610ee7610abc565b5b5f82015167ffffffffffffffff811115610f0557610f04610ac0565b5b610f1184828501610ea6565b91505092915050565b5f62ffffff82169050919050565b610f3181610f1a565b8114610f3b575f80fd5b50565b5f81519050610f4c81610f28565b92915050565b5f60208284031215610f6757610f66610abc565b5b5f610f7484828501610f3e565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b610faf81610b9e565b82525050565b610fbe81610dd3565b82525050565b5f60ff82169050919050565b610fd981610fc4565b82525050565b5f6dffffffffffffffffffffffffffff82169050919050565b61100181610fdf565b82525050565b61101081610f1a565b82525050565b61014082015f82015161102b5f850182610fa6565b50602082015161103e6020850182610fb5565b5060408201516110516040850182610fd0565b5060608201516110646060850182610fa6565b5060808201516110776080850182610fb5565b5060a082015161108a60a0850182610fd0565b5060c082015161109d60c0850182610fa6565b5060e08201516110b060e0850182610ff8565b506101008201516110c5610100850182610ff8565b506101208201516110da610120850182611007565b50505050565b5f6110eb8383611016565b6101408301905092915050565b5f602082019050919050565b5f61110e82610f7d565b6111188185610f87565b935061112383610f97565b805f5b8381101561115357815161113a88826110e0565b9750611145836110f8565b925050600181019050611126565b5085935050505092915050565b5f6020820190508181035f8301526111788184611104565b90509291505056fe
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`@Qa\x11\x818\x03\x80a\x11\x81\x839\x81\x81\x01`@R\x81\x01\x90a\x001\x91\x90a\x0CnV[_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0MWa\0La\n\xD8V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\x86W\x81` \x01[a\0sa\t\xF7V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\0kW\x90P[P\x90P_[\x82Q\x81\x10\x15a\trW_\x83\x82\x81Q\x81\x10a\0\xA8Wa\0\xA7a\x0C\xB5V[[` \x02` \x01\x01Q\x90Pa\0\xC1\x81a\t\xA0` \x1B` \x1CV[\x15a\0\xCCWPa\tgV[a\0\xD4a\t\xF7V[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\x1DW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01A\x91\x90a\x0C\xE2V[\x81_\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD2\x12 \xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xC0W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xE4\x91\x90a\x0C\xE2V[\x81``\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\x02-\x81_\x01Qa\t\xA0` \x1B` \x1CV[\x15a\x029WPPa\tgV[a\x02L\x81``\x01Qa\t\xA0` \x1B` \x1CV[\x15a\x02XWPPa\tgV[_\x80\x82_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x03\x07\x91\x90a\r_V[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x03AW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x03FV[``\x91P[P\x91P\x91P\x81\x15a\x03\xB5W_` \x82Q\x03a\x03\xA5W\x81\x80` \x01\x90Q\x81\x01\x90a\x03o\x91\x90a\r\xA8V[\x90P_\x81\x14\x80a\x03\x7FWP`\xFF\x81\x11[\x15a\x03\x8EWPPPPPa\tgV[\x80\x84`@\x01\x90`\xFF\x16\x90\x81`\xFF\x16\x81RPPa\x03\xAFV[PPPPPa\tgV[Pa\x03\xBEV[PPPPa\tgV[_\x80\x84``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x04n\x91\x90a\r_V[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x04\xA8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x04\xADV[``\x91P[P\x91P\x91P\x81\x15a\x05 W_` \x82Q\x03a\x05\x0EW\x81\x80` \x01\x90Q\x81\x01\x90a\x04\xD6\x91\x90a\r\xA8V[\x90P_\x81\x14\x80a\x04\xE6WP`\xFF\x81\x11[\x15a\x04\xF7WPPPPPPPa\tgV[\x80\x86`\xA0\x01\x90`\xFF\x16\x90\x81`\xFF\x16\x81RPPa\x05\x1AV[PPPPPPPa\tgV[Pa\x05+V[PPPPPPa\tgV[_\x80\x86_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F\x95\xD8\x9BA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x05\xDA\x91\x90a\r_V[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x06\x14W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x06\x19V[``\x91P[P\x91P\x91P\x81\x15a\x06\x83W_` \x82Q\x03a\x06IW\x81\x80` \x01\x90Q\x81\x01\x90a\x06B\x91\x90a\x0E\x06V[\x90Pa\x06sV[_\x82\x80` \x01\x90Q\x81\x01\x90a\x06^\x91\x90a\x0E\xD3V[\x90Pa\x06o\x81a\t\xD1` \x1B` \x1CV[\x91PP[\x80\x88` \x01\x81\x81RPPPa\x06\x90V[_\x80\x1B\x87` \x01\x81\x81RPP[_\x80\x88``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F\x95\xD8\x9BA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x07@\x91\x90a\r_V[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x07zW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x07\x7FV[``\x91P[P\x91P\x91P\x81\x15a\x07\xE9W_` \x82Q\x03a\x07\xAFW\x81\x80` \x01\x90Q\x81\x01\x90a\x07\xA8\x91\x90a\x0E\x06V[\x90Pa\x07\xD9V[_\x82\x80` \x01\x90Q\x81\x01\x90a\x07\xC4\x91\x90a\x0E\xD3V[\x90Pa\x07\xD5\x81a\t\xD1` \x1B` \x1CV[\x91PP[\x80\x8A`\x80\x01\x81\x81RPPPa\x07\xF6V[_\x80\x1B\x89`\x80\x01\x81\x81RPP[\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC4Z\x01U`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08?W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08c\x91\x90a\x0C\xE2V[\x89`\xC0\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xDD\xCA?C`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\t\x02WP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xFF\x91\x90a\x0FRV[`\x01[a\t$Wa\x0B\xB8\x89a\x01 \x01\x90b\xFF\xFF\xFF\x16\x90\x81b\xFF\xFF\xFF\x16\x81RPPa\t=V[\x80\x8Aa\x01 \x01\x90b\xFF\xFF\xFF\x16\x90\x81b\xFF\xFF\xFF\x16\x81RPPP[\x88\x8C\x8C\x81Q\x81\x10a\tQWa\tPa\x0C\xB5V[[` \x02` \x01\x01\x81\x90RPPPPPPPPPPP[\x80`\x01\x01\x90Pa\0\x8BV[P_\x81`@Q` \x01a\t\x85\x91\x90a\x11`V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P` \x81\x01\x80Y\x03\x81\xF3[_\x80\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a\t\xC8W`\x01\x90Pa\t\xCCV[_\x90P[\x91\x90PV[_\x80\x82\x90P_\x81Q\x03a\t\xE9W_\x80\x1B\x91PPa\t\xF2V[` \x83\x01Q\x91PP[\x91\x90PV[`@Q\x80a\x01@\x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x80\x19\x16\x81R` \x01_`\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x80\x19\x16\x81R` \x01_`\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_b\xFF\xFF\xFF\x16\x81RP\x90V[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x0B\x0E\x82a\n\xC8V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0B-Wa\x0B,a\n\xD8V[[\x80`@RPPPV[_a\x0B?a\n\xB3V[\x90Pa\x0BK\x82\x82a\x0B\x05V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0BjWa\x0Bia\n\xD8V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x80\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x0B\xA8\x82a\x0B\x7FV[\x90P\x91\x90PV[a\x0B\xB8\x81a\x0B\x9EV[\x81\x14a\x0B\xC2W_\x80\xFD[PV[_\x81Q\x90Pa\x0B\xD3\x81a\x0B\xAFV[\x92\x91PPV[_a\x0B\xEBa\x0B\xE6\x84a\x0BPV[a\x0B6V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x0C\x0EWa\x0C\ra\x0B{V[[\x83[\x81\x81\x10\x15a\x0C7W\x80a\x0C#\x88\x82a\x0B\xC5V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x0C\x10V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x0CUWa\x0CTa\n\xC4V[[\x81Qa\x0Ce\x84\x82` \x86\x01a\x0B\xD9V[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0C\x83Wa\x0C\x82a\n\xBCV[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xA0Wa\x0C\x9Fa\n\xC0V[[a\x0C\xAC\x84\x82\x85\x01a\x0CAV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\x0C\xF7Wa\x0C\xF6a\n\xBCV[[_a\r\x04\x84\x82\x85\x01a\x0B\xC5V[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\r9\x82a\r\rV[a\rC\x81\x85a\r\x17V[\x93Pa\rS\x81\x85` \x86\x01a\r!V[\x80\x84\x01\x91PP\x92\x91PPV[_a\rj\x82\x84a\r/V[\x91P\x81\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[a\r\x87\x81a\ruV[\x81\x14a\r\x91W_\x80\xFD[PV[_\x81Q\x90Pa\r\xA2\x81a\r~V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\r\xBDWa\r\xBCa\n\xBCV[[_a\r\xCA\x84\x82\x85\x01a\r\x94V[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[a\r\xE5\x81a\r\xD3V[\x81\x14a\r\xEFW_\x80\xFD[PV[_\x81Q\x90Pa\x0E\0\x81a\r\xDCV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0E\x1BWa\x0E\x1Aa\n\xBCV[[_a\x0E(\x84\x82\x85\x01a\r\xF2V[\x91PP\x92\x91PPV[_\x80\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0EOWa\x0ENa\n\xD8V[[a\x0EX\x82a\n\xC8V[\x90P` \x81\x01\x90P\x91\x90PV[_a\x0Ewa\x0Er\x84a\x0E5V[a\x0B6V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x0E\x93Wa\x0E\x92a\x0E1V[[a\x0E\x9E\x84\x82\x85a\r!V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x0E\xBAWa\x0E\xB9a\n\xC4V[[\x81Qa\x0E\xCA\x84\x82` \x86\x01a\x0EeV[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0E\xE8Wa\x0E\xE7a\n\xBCV[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x05Wa\x0F\x04a\n\xC0V[[a\x0F\x11\x84\x82\x85\x01a\x0E\xA6V[\x91PP\x92\x91PPV[_b\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x0F1\x81a\x0F\x1AV[\x81\x14a\x0F;W_\x80\xFD[PV[_\x81Q\x90Pa\x0FL\x81a\x0F(V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x0FgWa\x0Ffa\n\xBCV[[_a\x0Ft\x84\x82\x85\x01a\x0F>V[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x0F\xAF\x81a\x0B\x9EV[\x82RPPV[a\x0F\xBE\x81a\r\xD3V[\x82RPPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\x0F\xD9\x81a\x0F\xC4V[\x82RPPV[_m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x10\x01\x81a\x0F\xDFV[\x82RPPV[a\x10\x10\x81a\x0F\x1AV[\x82RPPV[a\x01@\x82\x01_\x82\x01Qa\x10+_\x85\x01\x82a\x0F\xA6V[P` \x82\x01Qa\x10>` \x85\x01\x82a\x0F\xB5V[P`@\x82\x01Qa\x10Q`@\x85\x01\x82a\x0F\xD0V[P``\x82\x01Qa\x10d``\x85\x01\x82a\x0F\xA6V[P`\x80\x82\x01Qa\x10w`\x80\x85\x01\x82a\x0F\xB5V[P`\xA0\x82\x01Qa\x10\x8A`\xA0\x85\x01\x82a\x0F\xD0V[P`\xC0\x82\x01Qa\x10\x9D`\xC0\x85\x01\x82a\x0F\xA6V[P`\xE0\x82\x01Qa\x10\xB0`\xE0\x85\x01\x82a\x0F\xF8V[Pa\x01\0\x82\x01Qa\x10\xC5a\x01\0\x85\x01\x82a\x0F\xF8V[Pa\x01 \x82\x01Qa\x10\xDAa\x01 \x85\x01\x82a\x10\x07V[PPPPV[_a\x10\xEB\x83\x83a\x10\x16V[a\x01@\x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x11\x0E\x82a\x0F}V[a\x11\x18\x81\x85a\x0F\x87V[\x93Pa\x11#\x83a\x0F\x97V[\x80_[\x83\x81\x10\x15a\x11SW\x81Qa\x11:\x88\x82a\x10\xE0V[\x97Pa\x11E\x83a\x10\xF8V[\x92PP`\x01\x81\x01\x90Pa\x11&V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x11x\x81\x84a\x11\x04V[\x90P\x92\x91PPV\xFE",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040525f80fdfea2646970667358221220ca6f2e2b9be7c764f7ddfe955537d2935696261e7cc27edcaf2d7750d7d2575064736f6c634300081a0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R_\x80\xFD\xFE\xA2dipfsX\"\x12 \xCAo.+\x9B\xE7\xC7d\xF7\xDD\xFE\x95U7\xD2\x93V\x96&\x1E|\xC2~\xDC\xAF-wP\xD7\xD2WPdsolcC\0\x08\x1A\x003",
    );
    /**Constructor`.
```solidity
constructor(address[] pools);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct constructorCall {
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
    /**Creates a new wrapper around an on-chain [`GetDetailedPoolDataBatchRequest`](self) contract instance.

See the [wrapper's documentation](`GetDetailedPoolDataBatchRequestInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> GetDetailedPoolDataBatchRequestInstance<T, P, N> {
        GetDetailedPoolDataBatchRequestInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<GetDetailedPoolDataBatchRequestInstance<T, P, N>>,
    > {
        GetDetailedPoolDataBatchRequestInstance::<T, P, N>::deploy(provider, pools)
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
        GetDetailedPoolDataBatchRequestInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, pools)
    }
    /**A [`GetDetailedPoolDataBatchRequest`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`GetDetailedPoolDataBatchRequest`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct GetDetailedPoolDataBatchRequestInstance<
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
    for GetDetailedPoolDataBatchRequestInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("GetDetailedPoolDataBatchRequestInstance")
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
    > GetDetailedPoolDataBatchRequestInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`GetDetailedPoolDataBatchRequest`](self) contract instance.

See the [wrapper's documentation](`GetDetailedPoolDataBatchRequestInstance`) for more details.*/
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
        ) -> alloy_contract::Result<GetDetailedPoolDataBatchRequestInstance<T, P, N>> {
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
    > GetDetailedPoolDataBatchRequestInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(
            self,
        ) -> GetDetailedPoolDataBatchRequestInstance<T, P, N> {
            GetDetailedPoolDataBatchRequestInstance {
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
    > GetDetailedPoolDataBatchRequestInstance<T, P, N> {
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
    > GetDetailedPoolDataBatchRequestInstance<T, P, N> {
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
