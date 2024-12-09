//SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./IAerodromePool.sol";

/**
 * @dev This contract is not meant to be deployed. Instead, use a static call with the
 *       deployment bytecode as payload.
 */
contract GetVe33PoolDataBatchRequest {
    struct PoolData {
        address tokenA;
        uint8 tokenADecimals;
        address tokenB;
        uint8 tokenBDecimals;
        uint256 reserve0;
        uint256 reserve1;
        bool stable;
    }

    constructor(address[] memory pools) {
        PoolData[] memory allPoolData = new PoolData[](pools.length);

        for (uint256 i = 0; i < pools.length; ++i) {
            address poolAddress = pools[i];

            if (codeSizeIsZero(poolAddress)) continue;

            PoolData memory poolData;

            // Get tokens A and B
            poolData.tokenA = IAerodromePool(poolAddress).token0();
            poolData.tokenB = IAerodromePool(poolAddress).token1();

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
            try IAerodromePool(poolAddress).getReserves() returns (
                uint256 _reserve0, uint256 _reserve1, uint256 /*_blockTimestampLast*/
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

            try IAerodromePool(poolAddress).stable() returns (bool _stable) {
                poolData.stable = _stable;
            } catch {
                poolData.stable = false;
            }

            allPoolData[i] = poolData;
        }

        // ensure abi encoding, not needed here but increase reusability for different return types
        // note: abi.encode add a first 32 bytes word with the address of the original data
        bytes memory _abiEncodedData = abi.encode(allPoolData);

        assembly {
            // Return from the start of the data (discarding the original data address)
            // up to the end of the memory used
            let dataStart := add(_abiEncodedData, 0x20)
            return(dataStart, sub(msize(), dataStart))
        }
    }

    function codeSizeIsZero(address target) internal view returns (bool) {
        if (target.code.length == 0) {
            return true;
        } else {
            return false;
        }
    }
}
