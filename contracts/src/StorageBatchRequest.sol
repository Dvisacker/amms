// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/* 
Original contract by Aperture Finance
*/
contract StorageBatchRequest {
    function getStorage(bytes32[] calldata slots) external payable returns (bytes32[] memory) {
        assembly ("memory-safe") {
            // abi offset for dynamic array
            mstore(0, 0x20)
            mstore(0x20, slots.length)
            let end := add(0x40, shl(5, slots.length))
            let memptr := 0x40
            let calldataptr := slots.offset
            // prettier-ignore
            for { } 1 { } {
                mstore(memptr, sload(calldataload(calldataptr)))
                memptr := add(memptr, 0x20)
                calldataptr := add(calldataptr, 0x20)
                if iszero(lt(memptr, end)) { break }
            }
            return(0, end)
        }
    }
}