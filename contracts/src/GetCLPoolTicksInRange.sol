// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

import {DEX} from "./PoolHelpers.sol";
import "./PoolHelpers.sol";
import "./IAerodromeCLPool.sol";



/* 
Original contract by Aperture Finance
*/
contract GetCLPoolTicksInRange is PoolHelpers {
    constructor(DEX dex, V3PoolCallee pool, int24 tickLower, int24 tickUpper) payable {
        PopulatedTicks memory populatedTicks = getPopulatedTicksInRange(dex, pool, tickLower, tickUpper);
        bytes memory returnData = abi.encode(populatedTicks);
        assembly ("memory-safe") {
            revert(add(returnData, 0x20), mload(returnData))
        }
    }

    /// @notice Get all the tick data for the populated ticks from tickLower to tickUpper
    /// @param pool The address of the pool for which to fetch populated tick data
    /// @param tickLower The lower tick boundary of the populated ticks to fetch
    /// @param tickUpper The upper tick boundary of the populated ticks to fetch
    /// @return populatedTicks An array of tick data for the given word in the tick bitmap
    function getPopulatedTicksInRange(
        DEX dex,
        V3PoolCallee pool,
        int24 tickLower,
        int24 tickUpper
    ) public payable returns (PopulatedTicks memory populatedTicks) {
        require(tickLower <= tickUpper);
        // checks that the pool exists
        int24 tickSpacing = IUniswapV3Pool(V3PoolCallee.unwrap(pool)).tickSpacing();
        (int16 wordPosLower, int16 wordPosUpper) = getWordPositions(tickLower, tickUpper, tickSpacing);
        unchecked {
            (uint256[] memory tickBitmap, uint256 count) = getTickBitmapAndCount(pool, wordPosLower, wordPosUpper);
            // fetch populated tick data
            populatedTicks.ticks = new PopulatedTick[](count);
            uint256 idx;
            for (int16 wordPos = wordPosLower; wordPos <= wordPosUpper; ++wordPos) {
                idx = populateTicksInWord(
                    dex,
                    pool,
                    wordPos,
                    tickSpacing,
                    tickBitmap[uint16(wordPos - wordPosLower)],
                    populatedTicks.ticks,
                    idx
                );
            }
        }
    }

    /// @notice Get the tick data for all populated ticks in a word of the tick bitmap
    function populateTicksInWord(
        DEX dex,
        V3PoolCallee pool,
        int16 wordPos,
        int24 tickSpacing,
        uint256 bitmap,
        PopulatedTick[] memory populatedTicks,
        uint256 idx
    ) internal view returns (uint256) {
        unchecked {
            for (uint256 bitPos; bitPos < 256; ++bitPos) {
                //slither-disable-next-line incorrect-shift
                if (bitmap & (1 << bitPos) != 0) {
                    int24 tick;
                    assembly {
                        tick := mul(tickSpacing, add(shl(8, wordPos), bitPos))
                    }
                    populateTick(dex, pool, tick, populatedTicks[idx++]);
                }
            }
            return idx;
        }
    }

    function populateTick(DEX dex, V3PoolCallee pool, int24 tick, PopulatedTick memory populatedTick) internal view {
        populatedTick.tick = tick;
        if (dex == DEX.SlipStream) {
            (
                populatedTick.liquidityGross,
                populatedTick.liquidityNet,
                ,
                populatedTick.feeGrowthOutside0X128,
                populatedTick.feeGrowthOutside1X128,
                ,
                ,
                ,
                ,

            ) = IAerodromeCLPool(V3PoolCallee.unwrap(pool)).ticks(tick);
        } else {
            PoolCaller.TickInfo memory info = pool.ticks(tick);
            populatedTick.liquidityNet = info.liquidityNet;
            populatedTick.liquidityGross = info.liquidityGross;
            populatedTick.feeGrowthOutside0X128 = info.feeGrowthOutside0X128;
            populatedTick.feeGrowthOutside1X128 = info.feeGrowthOutside1X128;
        }
    }
}