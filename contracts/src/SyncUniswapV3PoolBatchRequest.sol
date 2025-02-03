//SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./PoolHelpers.sol";

interface IERC20 {
    function decimals() external view returns (uint8);
}

/**
 * @dev This contract is not meant to be deployed. Instead, use a static call with the
 *       deployment bytecode as payload.
 */
contract SyncUniswapV3PoolBatchRequest is PoolHelpers {

    constructor(address[] memory pools) {
        bytes memory _abiEncodedData = abi.encode(getPoolData(pools));

        assembly {
            // Return from the start of the data (discarding the original data address)
            // up to the end of the memory used
            let dataStart := add(_abiEncodedData, 0x20)
            return(dataStart, sub(msize(), dataStart))
        }
    }

    function getPoolData(address[] memory pools) public view returns (UniswapV3PoolPriceData[] memory) {
        UniswapV3PoolPriceData[] memory allPoolData = new UniswapV3PoolPriceData[](pools.length);

        for (uint256 i = 0; i < pools.length; ++i) {
            address poolAddress = pools[i];
            if (codeSizeIsZero(poolAddress)) continue;

            UniswapV3PoolPriceData memory poolData;

            (uint160 sqrtPriceX96, int24 tick,,,,,) = IUniswapV3Pool(poolAddress).slot0();

            (, int128 liquidityNet,,,,,,) = IUniswapV3Pool(poolAddress).ticks(tick);

            poolData.liquidity = IUniswapV3Pool(poolAddress).liquidity();
            poolData.sqrtPrice = sqrtPriceX96;
            poolData.tick = tick;
            poolData.liquidityNet = liquidityNet;

            allPoolData[i] = poolData;
        }

        return allPoolData;
    }

    function codeSizeIsZero(address target) internal view returns (bool) {
        if (target.code.length == 0) {
            return true;
        } else {
            return false;
        }
    }
}
