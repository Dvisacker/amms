// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {TickMath} from "../lib/uni-v3-lib/src/TickMath.sol";
import "./PoolHelpers.sol";

/* 
Original contract by Aperture Finance
*/
contract GetCLPoolTickBitmap is PoolHelpers {
    constructor(DEX dex, V3PoolCallee pool) payable {
        Slot[] memory slots = getTickBitmap(dex, pool);
        bytes memory returnData = abi.encode(slots);
        assembly ("memory-safe") {
            revert(add(returnData, 0x20), mload(returnData))
        }
    }

    function getTickBitmapSlot(DEX dex) internal pure virtual returns (uint256) {
        if (dex == DEX.UniswapV3) return 6;
        if (dex == DEX.PancakeSwapV3) return 7;
        revert("EphemeralPoolTickBitmap: invalid or unsupported DEX");
    }

    /// @notice Get the tick bitmap for a pool
    /// @dev Public function to expose the abi for easier decoding using TypeChain
    /// @param pool The address of the pool for which to fetch the tick bitmap
    /// @return slots An array of storage slots and their raw data
    function getTickBitmap(DEX dex, V3PoolCallee pool) public payable returns (Slot[] memory slots) {
        // checks that the pool exists
        int24 tickSpacing = IUniswapV3Pool(V3PoolCallee.unwrap(pool)).tickSpacing();
        (int16 wordPosLower, int16 wordPosUpper) = getWordPositions(TickMath.MIN_TICK, TickMath.MAX_TICK, tickSpacing);
        uint256 TICKBITMAP_SLOT = getTickBitmapSlot(dex);
        unchecked {
            slots = new Slot[](uint16(wordPosUpper - wordPosLower + 1));
            for (int16 wordPos = wordPosLower; wordPos <= wordPosUpper; ++wordPos) {
                // calculate the storage slot corresponding to the word position
                // the slot of tickBitmap[wordPos] is keccak256(abi.encode(wordPos, tickBitmap.slot))
                uint256 slot;
                assembly ("memory-safe") {
                    mstore(0, wordPos)
                    mstore(0x20, TICKBITMAP_SLOT)
                    slot := keccak256(0, 0x40)
                }
                slots[uint16(wordPos - wordPosLower)] = Slot(slot, pool.tickBitmap(wordPos));
            }
        }
    }
}