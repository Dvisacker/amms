//SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./PoolHelpers.sol";
interface IUniswapV2Pair {
    function token0() external view returns (address);

    function token1() external view returns (address);

    function getReserves() external view returns (uint256 reserve0, uint256 reserve1, uint32 blockTimestampLast);
}

interface IERC20 {
    function decimals() external view returns (uint8);
}

/**
 * @dev This contract is not meant to be deployed. Instead, use a static call with the
 *       deployment bytecode as payload.
 */
contract GetUniswapV2PoolDataBatchRequest is PoolHelpers {

    constructor(address[] memory pools) {
        UniswapV2PoolData[] memory allPoolData = getPoolData(pools);
        bytes memory _abiEncodedData = abi.encode(allPoolData);

        assembly {
            // Return from the start of the data (discarding the original data address)
            // up to the end of the memory used
            let dataStart := add(_abiEncodedData, 0x20)
            return(dataStart, sub(msize(), dataStart))
        }
        
    }

    function getPoolData(address[] memory pools) public returns (UniswapV2PoolData[] memory) {
        UniswapV2PoolData[] memory allPoolData = new UniswapV2PoolData[](pools.length);

        for (uint256 i = 0; i < pools.length; ++i) {
            address poolAddress = pools[i];

            if (codeSizeIsZero(poolAddress)) continue;

            UniswapV2PoolData memory poolData;

            // Get tokens A and B
            poolData.tokenA = IUniswapV2Pair(poolAddress).token0();
            poolData.tokenB = IUniswapV2Pair(poolAddress).token1();

            // Check that tokenA and tokenB do not have codesize of 0
            if (codeSizeIsZero(poolData.tokenA)) continue;
            if (codeSizeIsZero(poolData.tokenB)) continue;

            // Get tokenA decimals
            (bool tokenADecimalsSuccess, bytes memory tokenADecimalsData) =
                poolData.tokenA.call{gas: 20000}(abi.encodeWithSignature("decimals()"));

            if (tokenADecimalsSuccess) {
                uint256 tokenADecimals;

                if (tokenADecimalsData.length == 32) {
                    (tokenADecimals) = abi.decode(tokenADecimalsData, (uint256));

                    if (tokenADecimals == 0 || tokenADecimals > 255) {
                        continue;
                    } else {
                        poolData.tokenADecimals = uint8(tokenADecimals);
                    }
                } else {
                    continue;
                }
            } else {
                continue;
            }

            // Get tokenB decimals
            (bool tokenBDecimalsSuccess, bytes memory tokenBDecimalsData) =
                poolData.tokenB.call{gas: 20000}(abi.encodeWithSignature("decimals()"));

            if (tokenBDecimalsSuccess) {
                uint256 tokenBDecimals;

                if (tokenBDecimalsData.length == 32) {
                    (tokenBDecimals) = abi.decode(tokenBDecimalsData, (uint256));

                    if (tokenBDecimals == 0 || tokenBDecimals > 255) {
                        continue;
                    } else {
                        poolData.tokenBDecimals = uint8(tokenBDecimals);
                    }
                } else {
                    continue;
                }
            } else {
                continue;
            }

            // Get reserves
            try IUniswapV2Pair(poolAddress).getReserves() returns (
                uint256 _reserve0, uint256 _reserve1, uint32 /*_blockTimestampLast*/
            ) {
                if (_reserve0 > type(uint112).max || _reserve1 > type(uint112).max) {
                    poolData.reserve0 = 0;
                    poolData.reserve1 = 0;
                } else {
                    poolData.reserve0 = uint112(_reserve0);
                    poolData.reserve1 = uint112(_reserve1);
                }
            } catch {
                poolData.reserve0 = 0;
                poolData.reserve1 = 0;
            }

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
