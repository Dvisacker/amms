//SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./IUniswapV2Pool.sol";
import "./PoolHelpers.sol";

interface IERC20 {
    function decimals() external view returns (uint8);
    function symbol() external view returns (string memory);
}

/**
 * @dev This contract is not meant to be deployed. Instead, use a static call with the
 *       deployment bytecode as payload.
 */
contract GetUniV2PoolData is PoolHelpers {

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

            poolData.tokenA = IUniswapV2Pool(poolAddress).token0();
            poolData.tokenB = IUniswapV2Pool(poolAddress).token1();

            //Check that tokenA and tokenB do not have codesize of 0
            if (codeSizeIsZero(poolData.tokenA)) continue;
            if (codeSizeIsZero(poolData.tokenB)) continue;

            //Get tokenA decimals
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

            (bool tokenASymbolSuccess, bytes memory tokenASymbolData) =
                poolData.tokenA.call{gas: 20000}(abi.encodeWithSignature("symbol()"));

            if (tokenASymbolSuccess) {
                bytes32 symbol;
                if (tokenASymbolData.length == 32) {
                    symbol = abi.decode(tokenASymbolData, (bytes32));
                } else {
                    string memory symbolString = abi.decode(tokenASymbolData, (string));
                    symbol = stringToBytes32(symbolString);
                }
                poolData.tokenASymbol = symbol;
            } else {
                poolData.tokenASymbol = bytes32(0);
            }

            (bool tokenBSymbolSuccess, bytes memory tokenBSymbolData) =
                poolData.tokenB.call{gas: 20000}(abi.encodeWithSignature("symbol()"));

            if (tokenBSymbolSuccess) {
                bytes32 symbol;
                if (tokenBSymbolData.length == 32) {
                    symbol = abi.decode(tokenBSymbolData, (bytes32));
                } else {
                    string memory symbolString = abi.decode(tokenBSymbolData, (string));
                    symbol = stringToBytes32(symbolString);
                }
                poolData.tokenBSymbol = symbol;
            } else {
                poolData.tokenBSymbol = bytes32(0);
            }

            try IUniswapV2Pool(poolAddress).factory() returns (address _factory) {
                poolData.factory = _factory;
            } catch {
                poolData.factory = address(0);
            }

            try IUniswapV2Pool(poolAddress).getReserves() returns (
                uint112 _reserve0, uint112 _reserve1, uint32 /*_blockTimestampLast*/
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

    function stringToBytes32(string memory source) internal pure returns (bytes32 result) {
        bytes memory tempEmptyStringTest = bytes(source);
        if (tempEmptyStringTest.length == 0) {
            return 0x0;
        }

        assembly {
            result := mload(add(source, 32))
        }
    }
}
