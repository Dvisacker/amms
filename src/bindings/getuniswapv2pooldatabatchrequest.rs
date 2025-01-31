/**

Generated by the following Solidity interface...
```solidity
interface GetUniswapV2PoolDataBatchRequest {
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
pub mod GetUniswapV2PoolDataBatchRequest {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b50604051610c4a380380610c4a833981810160405281019061003191906108e1565b5f815167ffffffffffffffff81111561004d5761004c61074b565b5b60405190808252806020026020018201604052801561008657816020015b6100736106c4565b81526020019060019003908161006b5790505b5090505f5b8251811015610665575f8382815181106100a8576100a7610928565b5b602002602001015190506100c18161069360201b60201c565b156100cc575061065a565b6100d46106c4565b8173ffffffffffffffffffffffffffffffffffffffff16630dfe16816040518163ffffffff1660e01b8152600401602060405180830381865afa15801561011d573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101419190610955565b815f019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff16815250508173ffffffffffffffffffffffffffffffffffffffff1663d21220a76040518163ffffffff1660e01b8152600401602060405180830381865afa1580156101c0573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101e49190610955565b816040019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff168152505061022d815f015161069360201b60201c565b1561023957505061065a565b61024c816040015161069360201b60201c565b1561025857505061065a565b5f80825f015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f313ce567000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff838183161783525050505060405161030791906109d2565b5f604051808303815f8787f1925050503d805f8114610341576040519150601f19603f3d011682016040523d82523d5f602084013e610346565b606091505b509150915081156103b5575f60208251036103a5578180602001905181019061036f9190610a1b565b90505f81148061037f575060ff81115b1561038e57505050505061065a565b80846020019060ff16908160ff16815250506103af565b505050505061065a565b506103be565b5050505061065a565b5f80846040015173ffffffffffffffffffffffffffffffffffffffff16614e206040516024016040516020818303038152906040527f313ce567000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff838183161783525050505060405161046e91906109d2565b5f604051808303815f8787f1925050503d805f81146104a8576040519150601f19603f3d011682016040523d82523d5f602084013e6104ad565b606091505b50915091508115610520575f602082510361050e57818060200190518101906104d69190610a1b565b90505f8114806104e6575060ff81115b156104f7575050505050505061065a565b80866060019060ff16908160ff168152505061051a565b5050505050505061065a565b5061052b565b50505050505061065a565b8573ffffffffffffffffffffffffffffffffffffffff16630902f1ac6040518163ffffffff1660e01b8152600401606060405180830381865afa92505050801561059357506040513d601f19601f820116820180604052508101906105909190610a7f565b60015b6105b0575f8560800181815250505f8560a0018181525050610634565b6dffffffffffffffffffffffffffff80168311806105dd57506dffffffffffffffffffffffffffff801682115b156105fb575f8860800181815250505f8860a0018181525050610630565b826dffffffffffffffffffffffffffff16886080018181525050816dffffffffffffffffffffffffffff168860a00181815250505b5050505b8488888151811061064857610647610928565b5b60200260200101819052505050505050505b80600101905061008b565b505f816040516020016106789190610c29565b60405160208183030381529060405290506020810180590381f35b5f808273ffffffffffffffffffffffffffffffffffffffff163b036106bb57600190506106bf565b5f90505b919050565b6040518060c001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f60ff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f60ff1681526020015f81526020015f81525090565b5f604051905090565b5f80fd5b5f80fd5b5f80fd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6107818261073b565b810181811067ffffffffffffffff821117156107a05761079f61074b565b5b80604052505050565b5f6107b2610726565b90506107be8282610778565b919050565b5f67ffffffffffffffff8211156107dd576107dc61074b565b5b602082029050602081019050919050565b5f80fd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61081b826107f2565b9050919050565b61082b81610811565b8114610835575f80fd5b50565b5f8151905061084681610822565b92915050565b5f61085e610859846107c3565b6107a9565b90508083825260208201905060208402830185811115610881576108806107ee565b5b835b818110156108aa57806108968882610838565b845260208401935050602081019050610883565b5050509392505050565b5f82601f8301126108c8576108c7610737565b5b81516108d884826020860161084c565b91505092915050565b5f602082840312156108f6576108f561072f565b5b5f82015167ffffffffffffffff81111561091357610912610733565b5b61091f848285016108b4565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f6020828403121561096a5761096961072f565b5b5f61097784828501610838565b91505092915050565b5f81519050919050565b5f81905092915050565b8281835e5f83830152505050565b5f6109ac82610980565b6109b6818561098a565b93506109c6818560208601610994565b80840191505092915050565b5f6109dd82846109a2565b915081905092915050565b5f819050919050565b6109fa816109e8565b8114610a04575f80fd5b50565b5f81519050610a15816109f1565b92915050565b5f60208284031215610a3057610a2f61072f565b5b5f610a3d84828501610a07565b91505092915050565b5f63ffffffff82169050919050565b610a5e81610a46565b8114610a68575f80fd5b50565b5f81519050610a7981610a55565b92915050565b5f805f60608486031215610a9657610a9561072f565b5b5f610aa386828701610a07565b9350506020610ab486828701610a07565b9250506040610ac586828701610a6b565b9150509250925092565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b610b0181610811565b82525050565b5f60ff82169050919050565b610b1c81610b07565b82525050565b610b2b816109e8565b82525050565b60c082015f820151610b455f850182610af8565b506020820151610b586020850182610b13565b506040820151610b6b6040850182610af8565b506060820151610b7e6060850182610b13565b506080820151610b916080850182610b22565b5060a0820151610ba460a0850182610b22565b50505050565b5f610bb58383610b31565b60c08301905092915050565b5f602082019050919050565b5f610bd782610acf565b610be18185610ad9565b9350610bec83610ae9565b805f5b83811015610c1c578151610c038882610baa565b9750610c0e83610bc1565b925050600181019050610bef565b5085935050505092915050565b5f6020820190508181035f830152610c418184610bcd565b90509291505056fe
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`@Qa\x0CJ8\x03\x80a\x0CJ\x839\x81\x81\x01`@R\x81\x01\x90a\x001\x91\x90a\x08\xE1V[_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0MWa\0La\x07KV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\x86W\x81` \x01[a\0sa\x06\xC4V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\0kW\x90P[P\x90P_[\x82Q\x81\x10\x15a\x06eW_\x83\x82\x81Q\x81\x10a\0\xA8Wa\0\xA7a\t(V[[` \x02` \x01\x01Q\x90Pa\0\xC1\x81a\x06\x93` \x1B` \x1CV[\x15a\0\xCCWPa\x06ZV[a\0\xD4a\x06\xC4V[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\x1DW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01A\x91\x90a\tUV[\x81_\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD2\x12 \xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xC0W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xE4\x91\x90a\tUV[\x81`@\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\x02-\x81_\x01Qa\x06\x93` \x1B` \x1CV[\x15a\x029WPPa\x06ZV[a\x02L\x81`@\x01Qa\x06\x93` \x1B` \x1CV[\x15a\x02XWPPa\x06ZV[_\x80\x82_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x03\x07\x91\x90a\t\xD2V[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x03AW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x03FV[``\x91P[P\x91P\x91P\x81\x15a\x03\xB5W_` \x82Q\x03a\x03\xA5W\x81\x80` \x01\x90Q\x81\x01\x90a\x03o\x91\x90a\n\x1BV[\x90P_\x81\x14\x80a\x03\x7FWP`\xFF\x81\x11[\x15a\x03\x8EWPPPPPa\x06ZV[\x80\x84` \x01\x90`\xFF\x16\x90\x81`\xFF\x16\x81RPPa\x03\xAFV[PPPPPa\x06ZV[Pa\x03\xBEV[PPPPa\x06ZV[_\x80\x84`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aN `@Q`$\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x7F1<\xE5g\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x83\x16\x17\x83RPPPP`@Qa\x04n\x91\x90a\t\xD2V[_`@Q\x80\x83\x03\x81_\x87\x87\xF1\x92PPP=\x80_\x81\x14a\x04\xA8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x04\xADV[``\x91P[P\x91P\x91P\x81\x15a\x05 W_` \x82Q\x03a\x05\x0EW\x81\x80` \x01\x90Q\x81\x01\x90a\x04\xD6\x91\x90a\n\x1BV[\x90P_\x81\x14\x80a\x04\xE6WP`\xFF\x81\x11[\x15a\x04\xF7WPPPPPPPa\x06ZV[\x80\x86``\x01\x90`\xFF\x16\x90\x81`\xFF\x16\x81RPPa\x05\x1AV[PPPPPPPa\x06ZV[Pa\x05+V[PPPPPPa\x06ZV[\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\t\x02\xF1\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x05\x93WP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x90\x91\x90a\n\x7FV[`\x01[a\x05\xB0W_\x85`\x80\x01\x81\x81RPP_\x85`\xA0\x01\x81\x81RPPa\x064V[m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x83\x11\x80a\x05\xDDWPm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x82\x11[\x15a\x05\xFBW_\x88`\x80\x01\x81\x81RPP_\x88`\xA0\x01\x81\x81RPPa\x060V[\x82m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88`\x80\x01\x81\x81RPP\x81m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88`\xA0\x01\x81\x81RPP[PPP[\x84\x88\x88\x81Q\x81\x10a\x06HWa\x06Ga\t(V[[` \x02` \x01\x01\x81\x90RPPPPPPP[\x80`\x01\x01\x90Pa\0\x8BV[P_\x81`@Q` \x01a\x06x\x91\x90a\x0C)V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P` \x81\x01\x80Y\x03\x81\xF3[_\x80\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a\x06\xBBW`\x01\x90Pa\x06\xBFV[_\x90P[\x91\x90PV[`@Q\x80`\xC0\x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_`\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_`\xFF\x16\x81R` \x01_\x81R` \x01_\x81RP\x90V[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x07\x81\x82a\x07;V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x07\xA0Wa\x07\x9Fa\x07KV[[\x80`@RPPPV[_a\x07\xB2a\x07&V[\x90Pa\x07\xBE\x82\x82a\x07xV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x07\xDDWa\x07\xDCa\x07KV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x80\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x08\x1B\x82a\x07\xF2V[\x90P\x91\x90PV[a\x08+\x81a\x08\x11V[\x81\x14a\x085W_\x80\xFD[PV[_\x81Q\x90Pa\x08F\x81a\x08\"V[\x92\x91PPV[_a\x08^a\x08Y\x84a\x07\xC3V[a\x07\xA9V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x08\x81Wa\x08\x80a\x07\xEEV[[\x83[\x81\x81\x10\x15a\x08\xAAW\x80a\x08\x96\x88\x82a\x088V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x08\x83V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x08\xC8Wa\x08\xC7a\x077V[[\x81Qa\x08\xD8\x84\x82` \x86\x01a\x08LV[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x08\xF6Wa\x08\xF5a\x07/V[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\x13Wa\t\x12a\x073V[[a\t\x1F\x84\x82\x85\x01a\x08\xB4V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\tjWa\tia\x07/V[[_a\tw\x84\x82\x85\x01a\x088V[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\t\xAC\x82a\t\x80V[a\t\xB6\x81\x85a\t\x8AV[\x93Pa\t\xC6\x81\x85` \x86\x01a\t\x94V[\x80\x84\x01\x91PP\x92\x91PPV[_a\t\xDD\x82\x84a\t\xA2V[\x91P\x81\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[a\t\xFA\x81a\t\xE8V[\x81\x14a\n\x04W_\x80\xFD[PV[_\x81Q\x90Pa\n\x15\x81a\t\xF1V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\n0Wa\n/a\x07/V[[_a\n=\x84\x82\x85\x01a\n\x07V[\x91PP\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\n^\x81a\nFV[\x81\x14a\nhW_\x80\xFD[PV[_\x81Q\x90Pa\ny\x81a\nUV[\x92\x91PPV[_\x80_``\x84\x86\x03\x12\x15a\n\x96Wa\n\x95a\x07/V[[_a\n\xA3\x86\x82\x87\x01a\n\x07V[\x93PP` a\n\xB4\x86\x82\x87\x01a\n\x07V[\x92PP`@a\n\xC5\x86\x82\x87\x01a\nkV[\x91PP\x92P\x92P\x92V[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x0B\x01\x81a\x08\x11V[\x82RPPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\x0B\x1C\x81a\x0B\x07V[\x82RPPV[a\x0B+\x81a\t\xE8V[\x82RPPV[`\xC0\x82\x01_\x82\x01Qa\x0BE_\x85\x01\x82a\n\xF8V[P` \x82\x01Qa\x0BX` \x85\x01\x82a\x0B\x13V[P`@\x82\x01Qa\x0Bk`@\x85\x01\x82a\n\xF8V[P``\x82\x01Qa\x0B~``\x85\x01\x82a\x0B\x13V[P`\x80\x82\x01Qa\x0B\x91`\x80\x85\x01\x82a\x0B\"V[P`\xA0\x82\x01Qa\x0B\xA4`\xA0\x85\x01\x82a\x0B\"V[PPPPV[_a\x0B\xB5\x83\x83a\x0B1V[`\xC0\x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x0B\xD7\x82a\n\xCFV[a\x0B\xE1\x81\x85a\n\xD9V[\x93Pa\x0B\xEC\x83a\n\xE9V[\x80_[\x83\x81\x10\x15a\x0C\x1CW\x81Qa\x0C\x03\x88\x82a\x0B\xAAV[\x97Pa\x0C\x0E\x83a\x0B\xC1V[\x92PP`\x01\x81\x01\x90Pa\x0B\xEFV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x0CA\x81\x84a\x0B\xCDV[\x90P\x92\x91PPV\xFE",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040525f80fdfea2646970667358221220fcf32cd5180c7d6b27fdbdeb9e7662fd95075cc8546b8851838fc40409844d7764736f6c634300081a0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R_\x80\xFD\xFE\xA2dipfsX\"\x12 \xFC\xF3,\xD5\x18\x0C}k'\xFD\xBD\xEB\x9Evb\xFD\x95\x07\\\xC8Tk\x88Q\x83\x8F\xC4\x04\t\x84MwdsolcC\0\x08\x1A\x003",
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
