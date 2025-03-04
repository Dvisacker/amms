/**

Generated by the following Solidity interface...
```solidity
interface SyncCamelotV3PoolBatchRequest {
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
pub mod SyncCamelotV3PoolBatchRequest {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b50604051610b04380380610b0483398181016040528101906100319190610564565b5f815167ffffffffffffffff81111561004d5761004c6103ce565b5b60405190808252806020026020018201604052801561008657816020015b610073610357565b81526020019060019003908161006b5790505b5090505f5b82518110156102f8575f8382815181106100a8576100a76105ab565b5b602002602001015190506100c18161032660201b60201c565b156100cc57506102ed565b6100d4610357565b5f808373ffffffffffffffffffffffffffffffffffffffff1663e76c01e46040518163ffffffff1660e01b815260040161010060405180830381865afa158015610120573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061014491906106da565b505050505050915091505f8473ffffffffffffffffffffffffffffffffffffffff1663f30dba93836040518263ffffffff1660e01b8152600401610188919061079a565b61010060405180830381865afa1580156101a4573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101c891906108d0565b5050505050509150508473ffffffffffffffffffffffffffffffffffffffff16631a6865026040518163ffffffff1660e01b8152600401602060405180830381865afa15801561021a573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061023e9190610981565b845f01906fffffffffffffffffffffffffffffffff1690816fffffffffffffffffffffffffffffffff168152505082846020019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff168152505081846040019060020b908160020b81525050808460600190600f0b9081600f0b81525050838787815181106102dc576102db6105ab565b5b602002602001018190525050505050505b80600101905061008b565b505f8160405160200161030b9190610ae3565b60405160208183030381529060405290506020810180590381f35b5f808273ffffffffffffffffffffffffffffffffffffffff163b0361034e5760019050610352565b5f90505b919050565b60405180608001604052805f6fffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f60020b81526020015f600f0b81525090565b5f604051905090565b5f80fd5b5f80fd5b5f80fd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b610404826103be565b810181811067ffffffffffffffff82111715610423576104226103ce565b5b80604052505050565b5f6104356103a9565b905061044182826103fb565b919050565b5f67ffffffffffffffff8211156104605761045f6103ce565b5b602082029050602081019050919050565b5f80fd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61049e82610475565b9050919050565b6104ae81610494565b81146104b8575f80fd5b50565b5f815190506104c9816104a5565b92915050565b5f6104e16104dc84610446565b61042c565b9050808382526020820190506020840283018581111561050457610503610471565b5b835b8181101561052d578061051988826104bb565b845260208401935050602081019050610506565b5050509392505050565b5f82601f83011261054b5761054a6103ba565b5b815161055b8482602086016104cf565b91505092915050565b5f60208284031215610579576105786103b2565b5b5f82015167ffffffffffffffff811115610596576105956103b6565b5b6105a284828501610537565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b6105e181610475565b81146105eb575f80fd5b50565b5f815190506105fc816105d8565b92915050565b5f8160020b9050919050565b61061781610602565b8114610621575f80fd5b50565b5f815190506106328161060e565b92915050565b5f61ffff82169050919050565b61064e81610638565b8114610658575f80fd5b50565b5f8151905061066981610645565b92915050565b5f60ff82169050919050565b6106848161066f565b811461068e575f80fd5b50565b5f8151905061069f8161067b565b92915050565b5f8115159050919050565b6106b9816106a5565b81146106c3575f80fd5b50565b5f815190506106d4816106b0565b92915050565b5f805f805f805f80610100898b0312156106f7576106f66103b2565b5b5f6107048b828c016105ee565b98505060206107158b828c01610624565b97505060406107268b828c0161065b565b96505060606107378b828c0161065b565b95505060806107488b828c0161065b565b94505060a06107598b828c01610691565b93505060c061076a8b828c01610691565b92505060e061077b8b828c016106c6565b9150509295985092959890939650565b61079481610602565b82525050565b5f6020820190506107ad5f83018461078b565b92915050565b5f6fffffffffffffffffffffffffffffffff82169050919050565b6107d7816107b3565b81146107e1575f80fd5b50565b5f815190506107f2816107ce565b92915050565b5f81600f0b9050919050565b61080d816107f8565b8114610817575f80fd5b50565b5f8151905061082881610804565b92915050565b5f819050919050565b6108408161082e565b811461084a575f80fd5b50565b5f8151905061085b81610837565b92915050565b5f8160060b9050919050565b61087681610861565b8114610880575f80fd5b50565b5f815190506108918161086d565b92915050565b5f63ffffffff82169050919050565b6108af81610897565b81146108b9575f80fd5b50565b5f815190506108ca816108a6565b92915050565b5f805f805f805f80610100898b0312156108ed576108ec6103b2565b5b5f6108fa8b828c016107e4565b985050602061090b8b828c0161081a565b975050604061091c8b828c0161084d565b965050606061092d8b828c0161084d565b955050608061093e8b828c01610883565b94505060a061094f8b828c016105ee565b93505060c06109608b828c016108bc565b92505060e06109718b828c016106c6565b9150509295985092959890939650565b5f60208284031215610996576109956103b2565b5b5f6109a3848285016107e4565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b6109de816107b3565b82525050565b6109ed81610475565b82525050565b6109fc81610602565b82525050565b610a0b816107f8565b82525050565b608082015f820151610a255f8501826109d5565b506020820151610a3860208501826109e4565b506040820151610a4b60408501826109f3565b506060820151610a5e6060850182610a02565b50505050565b5f610a6f8383610a11565b60808301905092915050565b5f602082019050919050565b5f610a91826109ac565b610a9b81856109b6565b9350610aa6836109c6565b805f5b83811015610ad6578151610abd8882610a64565b9750610ac883610a7b565b925050600181019050610aa9565b5085935050505092915050565b5f6020820190508181035f830152610afb8184610a87565b90509291505056fe
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`@Qa\x0B\x048\x03\x80a\x0B\x04\x839\x81\x81\x01`@R\x81\x01\x90a\x001\x91\x90a\x05dV[_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0MWa\0La\x03\xCEV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\x86W\x81` \x01[a\0sa\x03WV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\0kW\x90P[P\x90P_[\x82Q\x81\x10\x15a\x02\xF8W_\x83\x82\x81Q\x81\x10a\0\xA8Wa\0\xA7a\x05\xABV[[` \x02` \x01\x01Q\x90Pa\0\xC1\x81a\x03&` \x1B` \x1CV[\x15a\0\xCCWPa\x02\xEDV[a\0\xD4a\x03WV[_\x80\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xE7l\x01\xE4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01 W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01D\x91\x90a\x06\xDAV[PPPPPP\x91P\x91P_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF3\r\xBA\x93\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\x88\x91\x90a\x07\x9AV[a\x01\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xA4W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xC8\x91\x90a\x08\xD0V[PPPPPP\x91PP\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x1Ahe\x02`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x1AW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02>\x91\x90a\t\x81V[\x84_\x01\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x82\x84` \x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81\x84`@\x01\x90`\x02\x0B\x90\x81`\x02\x0B\x81RPP\x80\x84``\x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP\x83\x87\x87\x81Q\x81\x10a\x02\xDCWa\x02\xDBa\x05\xABV[[` \x02` \x01\x01\x81\x90RPPPPPP[\x80`\x01\x01\x90Pa\0\x8BV[P_\x81`@Q` \x01a\x03\x0B\x91\x90a\n\xE3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P` \x81\x01\x80Y\x03\x81\xF3[_\x80\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a\x03NW`\x01\x90Pa\x03RV[_\x90P[\x91\x90PV[`@Q\x80`\x80\x01`@R\x80_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_`\x02\x0B\x81R` \x01_`\x0F\x0B\x81RP\x90V[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x04\x04\x82a\x03\xBEV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x04#Wa\x04\"a\x03\xCEV[[\x80`@RPPPV[_a\x045a\x03\xA9V[\x90Pa\x04A\x82\x82a\x03\xFBV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x04`Wa\x04_a\x03\xCEV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x80\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x04\x9E\x82a\x04uV[\x90P\x91\x90PV[a\x04\xAE\x81a\x04\x94V[\x81\x14a\x04\xB8W_\x80\xFD[PV[_\x81Q\x90Pa\x04\xC9\x81a\x04\xA5V[\x92\x91PPV[_a\x04\xE1a\x04\xDC\x84a\x04FV[a\x04,V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x05\x04Wa\x05\x03a\x04qV[[\x83[\x81\x81\x10\x15a\x05-W\x80a\x05\x19\x88\x82a\x04\xBBV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x05\x06V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x05KWa\x05Ja\x03\xBAV[[\x81Qa\x05[\x84\x82` \x86\x01a\x04\xCFV[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x05yWa\x05xa\x03\xB2V[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\x96Wa\x05\x95a\x03\xB6V[[a\x05\xA2\x84\x82\x85\x01a\x057V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[a\x05\xE1\x81a\x04uV[\x81\x14a\x05\xEBW_\x80\xFD[PV[_\x81Q\x90Pa\x05\xFC\x81a\x05\xD8V[\x92\x91PPV[_\x81`\x02\x0B\x90P\x91\x90PV[a\x06\x17\x81a\x06\x02V[\x81\x14a\x06!W_\x80\xFD[PV[_\x81Q\x90Pa\x062\x81a\x06\x0EV[\x92\x91PPV[_a\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x06N\x81a\x068V[\x81\x14a\x06XW_\x80\xFD[PV[_\x81Q\x90Pa\x06i\x81a\x06EV[\x92\x91PPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\x06\x84\x81a\x06oV[\x81\x14a\x06\x8EW_\x80\xFD[PV[_\x81Q\x90Pa\x06\x9F\x81a\x06{V[\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a\x06\xB9\x81a\x06\xA5V[\x81\x14a\x06\xC3W_\x80\xFD[PV[_\x81Q\x90Pa\x06\xD4\x81a\x06\xB0V[\x92\x91PPV[_\x80_\x80_\x80_\x80a\x01\0\x89\x8B\x03\x12\x15a\x06\xF7Wa\x06\xF6a\x03\xB2V[[_a\x07\x04\x8B\x82\x8C\x01a\x05\xEEV[\x98PP` a\x07\x15\x8B\x82\x8C\x01a\x06$V[\x97PP`@a\x07&\x8B\x82\x8C\x01a\x06[V[\x96PP``a\x077\x8B\x82\x8C\x01a\x06[V[\x95PP`\x80a\x07H\x8B\x82\x8C\x01a\x06[V[\x94PP`\xA0a\x07Y\x8B\x82\x8C\x01a\x06\x91V[\x93PP`\xC0a\x07j\x8B\x82\x8C\x01a\x06\x91V[\x92PP`\xE0a\x07{\x8B\x82\x8C\x01a\x06\xC6V[\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[a\x07\x94\x81a\x06\x02V[\x82RPPV[_` \x82\x01\x90Pa\x07\xAD_\x83\x01\x84a\x07\x8BV[\x92\x91PPV[_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x07\xD7\x81a\x07\xB3V[\x81\x14a\x07\xE1W_\x80\xFD[PV[_\x81Q\x90Pa\x07\xF2\x81a\x07\xCEV[\x92\x91PPV[_\x81`\x0F\x0B\x90P\x91\x90PV[a\x08\r\x81a\x07\xF8V[\x81\x14a\x08\x17W_\x80\xFD[PV[_\x81Q\x90Pa\x08(\x81a\x08\x04V[\x92\x91PPV[_\x81\x90P\x91\x90PV[a\x08@\x81a\x08.V[\x81\x14a\x08JW_\x80\xFD[PV[_\x81Q\x90Pa\x08[\x81a\x087V[\x92\x91PPV[_\x81`\x06\x0B\x90P\x91\x90PV[a\x08v\x81a\x08aV[\x81\x14a\x08\x80W_\x80\xFD[PV[_\x81Q\x90Pa\x08\x91\x81a\x08mV[\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x08\xAF\x81a\x08\x97V[\x81\x14a\x08\xB9W_\x80\xFD[PV[_\x81Q\x90Pa\x08\xCA\x81a\x08\xA6V[\x92\x91PPV[_\x80_\x80_\x80_\x80a\x01\0\x89\x8B\x03\x12\x15a\x08\xEDWa\x08\xECa\x03\xB2V[[_a\x08\xFA\x8B\x82\x8C\x01a\x07\xE4V[\x98PP` a\t\x0B\x8B\x82\x8C\x01a\x08\x1AV[\x97PP`@a\t\x1C\x8B\x82\x8C\x01a\x08MV[\x96PP``a\t-\x8B\x82\x8C\x01a\x08MV[\x95PP`\x80a\t>\x8B\x82\x8C\x01a\x08\x83V[\x94PP`\xA0a\tO\x8B\x82\x8C\x01a\x05\xEEV[\x93PP`\xC0a\t`\x8B\x82\x8C\x01a\x08\xBCV[\x92PP`\xE0a\tq\x8B\x82\x8C\x01a\x06\xC6V[\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[_` \x82\x84\x03\x12\x15a\t\x96Wa\t\x95a\x03\xB2V[[_a\t\xA3\x84\x82\x85\x01a\x07\xE4V[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a\t\xDE\x81a\x07\xB3V[\x82RPPV[a\t\xED\x81a\x04uV[\x82RPPV[a\t\xFC\x81a\x06\x02V[\x82RPPV[a\n\x0B\x81a\x07\xF8V[\x82RPPV[`\x80\x82\x01_\x82\x01Qa\n%_\x85\x01\x82a\t\xD5V[P` \x82\x01Qa\n8` \x85\x01\x82a\t\xE4V[P`@\x82\x01Qa\nK`@\x85\x01\x82a\t\xF3V[P``\x82\x01Qa\n^``\x85\x01\x82a\n\x02V[PPPPV[_a\no\x83\x83a\n\x11V[`\x80\x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\n\x91\x82a\t\xACV[a\n\x9B\x81\x85a\t\xB6V[\x93Pa\n\xA6\x83a\t\xC6V[\x80_[\x83\x81\x10\x15a\n\xD6W\x81Qa\n\xBD\x88\x82a\ndV[\x97Pa\n\xC8\x83a\n{V[\x92PP`\x01\x81\x01\x90Pa\n\xA9V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\n\xFB\x81\x84a\n\x87V[\x90P\x92\x91PPV\xFE",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040525f80fdfea264697066735822122008f079947df85a345597edc4c0f2f178f24c5cd73baa09b513ce912829aac5aa64736f6c634300081a0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R_\x80\xFD\xFE\xA2dipfsX\"\x12 \x08\xF0y\x94}\xF8Z4U\x97\xED\xC4\xC0\xF2\xF1x\xF2L\\\xD7;\xAA\t\xB5\x13\xCE\x91()\xAA\xC5\xAAdsolcC\0\x08\x1A\x003",
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
    /**Creates a new wrapper around an on-chain [`SyncCamelotV3PoolBatchRequest`](self) contract instance.

See the [wrapper's documentation](`SyncCamelotV3PoolBatchRequestInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> SyncCamelotV3PoolBatchRequestInstance<T, P, N> {
        SyncCamelotV3PoolBatchRequestInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<SyncCamelotV3PoolBatchRequestInstance<T, P, N>>,
    > {
        SyncCamelotV3PoolBatchRequestInstance::<T, P, N>::deploy(provider, pools)
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
        SyncCamelotV3PoolBatchRequestInstance::<T, P, N>::deploy_builder(provider, pools)
    }
    /**A [`SyncCamelotV3PoolBatchRequest`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`SyncCamelotV3PoolBatchRequest`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct SyncCamelotV3PoolBatchRequestInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for SyncCamelotV3PoolBatchRequestInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("SyncCamelotV3PoolBatchRequestInstance")
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
    > SyncCamelotV3PoolBatchRequestInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`SyncCamelotV3PoolBatchRequest`](self) contract instance.

See the [wrapper's documentation](`SyncCamelotV3PoolBatchRequestInstance`) for more details.*/
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
        ) -> alloy_contract::Result<SyncCamelotV3PoolBatchRequestInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> SyncCamelotV3PoolBatchRequestInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(
            self,
        ) -> SyncCamelotV3PoolBatchRequestInstance<T, P, N> {
            SyncCamelotV3PoolBatchRequestInstance {
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
    > SyncCamelotV3PoolBatchRequestInstance<T, P, N> {
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
    > SyncCamelotV3PoolBatchRequestInstance<T, P, N> {
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
