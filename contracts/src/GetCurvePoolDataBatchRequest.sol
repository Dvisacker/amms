pragma solidity ^0.8.0;

interface ICurvePool {
    function coins(uint256 i) external view returns (address);
    function balances(uint256 i) external view returns (uint256);
    function get_virtual_price() external view returns (uint256);
    function A() external view returns (uint256);
    function fee() external view returns (uint256);
}

interface IERC20 {
    function decimals() external view returns (uint8);
    function symbol() external view returns (string memory);
}

contract GetCurvePoolBatchRequest {
    struct PoolData {
        address[8] coins;
        uint256[8] balances;
        uint8[8] decimals;
        bytes32[8] symbols;
        uint256 virtualPrice;
        uint256 A;
        uint256 fee;
    }

    constructor(address[] memory pools) {
        PoolData[] memory allPoolData = new PoolData[](pools.length);

        for (uint256 i = 0; i < pools.length; ++i) {
            address poolAddress = pools[i];
            if (codeSizeIsZero(poolAddress)) continue;

            PoolData memory poolData;

            for (uint256 j = 0; j < 8; j++) {
                try ICurvePool(poolAddress).coins(j) returns (address coin) {
                    poolData.coins[j] = coin;
                    poolData.balances[j] = ICurvePool(poolAddress).balances(j);

                    if (coin != address(0)) {
                        try IERC20(coin).decimals() returns (uint8 dec) {
                            poolData.decimals[j] = dec;
                        } catch {}

                        try IERC20(coin).symbol() returns (string memory sym) {
                            poolData.symbols[j] = stringToBytes32(sym);
                        } catch {}
                    }
                } catch {
                    break;
                }
            }

            try ICurvePool(poolAddress).get_virtual_price() returns (uint256 vp) {
                poolData.virtualPrice = vp;
            } catch {}

            try ICurvePool(poolAddress).A() returns (uint256 a) {
                poolData.A = a;
            } catch {}

            try ICurvePool(poolAddress).fee() returns (uint256 f) {
                poolData.fee = f;
            } catch {}

            allPoolData[i] = poolData;
        }

        bytes memory _abiEncodedData = abi.encode(allPoolData);
        assembly {
            let dataStart := add(_abiEncodedData, 0x20)
            return(dataStart, sub(msize(), dataStart))
        }
    }

    function codeSizeIsZero(address target) internal view returns (bool) {
        return target.code.length == 0;
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
