//SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IUniswapV3Pool {
    function token0() external view returns (address);

    function token1() external view returns (address);

    function factory() external view returns (address);

    function fee() external view returns (uint24);
}

interface IERC20 {
    function decimals() external view returns (uint8);
    function symbol() external view returns (string memory);
}

/**
 * @dev This contract is not meant to be deployed. Instead, use a static call with the
 *       deployment bytecode as payload.
 */
contract GetDetailedPoolDataBatchRequest {
    struct PoolData {
        address tokenA;
        bytes32 tokenASymbol;
        uint8 tokenADecimals;
        address tokenB;
        bytes32 tokenBSymbol;
        uint8 tokenBDecimals;
        address factory;
        uint112 reserve0;
        uint112 reserve1;
        uint24 fee;
    }

    constructor(address[] memory pools) {
        PoolData[] memory allPoolData = new PoolData[](pools.length);

        for (uint256 i = 0; i < pools.length; ++i) {
            address poolAddress = pools[i];

            if (codeSizeIsZero(poolAddress)) continue;

            PoolData memory poolData;

            poolData.tokenA = IUniswapV3Pool(poolAddress).token0();
            poolData.tokenB = IUniswapV3Pool(poolAddress).token1();

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
                string memory symbol = abi.decode(tokenASymbolData, (string));
                poolData.tokenASymbol = stringToBytes32(symbol);
            } else {
                poolData.tokenASymbol = bytes32(0);
            }

            (bool tokenBSymbolSuccess, bytes memory tokenBSymbolData) =
                poolData.tokenB.call{gas: 20000}(abi.encodeWithSignature("symbol()"));

            if (tokenBSymbolSuccess) {
                string memory symbol = abi.decode(tokenBSymbolData, (string));
                poolData.tokenBSymbol = stringToBytes32(symbol);
            } else {
                poolData.tokenBSymbol = bytes32(0);
            }

            poolData.factory = IUniswapV3Pool(poolAddress).factory();

            try IUniswapV3Pool(poolAddress).fee() returns (uint24 _fee) {
                poolData.fee = _fee;
            } catch {
                // If this fails, it might be a V2 pool or not a Uniswap pool at all
                poolData.fee = 3000; // Default to 0.3% for V2 pools
            }

            allPoolData[i] = poolData;
        }

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
