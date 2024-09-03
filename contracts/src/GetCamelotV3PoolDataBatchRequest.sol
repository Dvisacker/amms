//SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface ICamelotV3Pool {
    /**
     * @notice The contract that stores all the timepoints and can perform actions with them
     * @return The operator address
     */
    function dataStorageOperator() external view returns (address);

    /**
     * @notice The contract that deployed the pool, which must adhere to the IAlgebraFactory interface
     * @return The contract address
     */
    function factory() external view returns (address);

    /**
     * @notice The first of the two tokens of the pool, sorted by address
     * @return The token contract address
     */
    function token0() external view returns (address);

    /**
     * @notice The second of the two tokens of the pool, sorted by address
     * @return The token contract address
     */
    function token1() external view returns (address);

    /**
     * @notice The maximum amount of position liquidity that can use any tick in the range
     * @dev This parameter is enforced per tick to prevent liquidity from overflowing a uint128 at any point, and
     * also prevents out-of-range liquidity from being used to prevent adding in-range liquidity to a pool
     * @return The max amount of liquidity per tick
     */
    function maxLiquidityPerTick() external view returns (uint128);
    /**
     * @notice The globalState structure in the pool stores many values but requires only one slot
     * and is exposed as a single method to save gas when accessed externally.
     * @return price The current price of the pool as a sqrt(token1/token0) Q64.96 value;
     * Returns tick The current tick of the pool, i.e. according to the last tick transition that was run;
     * Returns This value may not always be equal to SqrtTickMath.getTickAtSqrtRatio(price) if the price is on a tick
     * boundary;
     * Returns feeZto The last pool fee value for ZtO swaps in hundredths of a bip, i.e. 1e-6;
     * Returns feeOtz The last pool fee value for OtZ swaps in hundredths of a bip, i.e. 1e-6;
     * Returns timepointIndex The index of the last written timepoint;
     * Returns communityFeeToken0 The community fee percentage of the swap fee in thousandths (1e-3) for token0;
     * Returns communityFeeToken1 The community fee percentage of the swap fee in thousandths (1e-3) for token1;
     * Returns unlocked Whether the pool is currently locked to reentrancy;
     */
    function globalState()
        external
        view
        returns (
            uint160 price,
            int24 tick,
            uint16 feeZto,
            uint16 feeOtz,
            uint16 timepointIndex,
            uint8 communityFeeToken0,
            uint8 communityFeeToken1,
            bool unlocked
        );

    /**
     * @notice The fee growth as a Q128.128 fees of token0 collected per unit of liquidity for the entire life of the pool
     * @dev This value can overflow the uint256
     */
    function totalFeeGrowth0Token() external view returns (uint256);

    /**
     * @notice The fee growth as a Q128.128 fees of token1 collected per unit of liquidity for the entire life of the pool
     * @dev This value can overflow the uint256
     */
    function totalFeeGrowth1Token() external view returns (uint256);

    /**
     * @notice The currently in range liquidity available to the pool
     * @dev This value has no relationship to the total liquidity across all ticks.
     * Returned value cannot exceed type(uint128).max
     */
    function liquidity() external view returns (uint128);

    /**
     * @notice Look up information about a specific tick in the pool
     * @dev This is a public structure, so the `return` natspec tags are omitted.
     * @param tick The tick to look up
     * @return liquidityTotal the total amount of position liquidity that uses the pool either as tick lower or
     * tick upper;
     * Returns liquidityDelta how much liquidity changes when the pool price crosses the tick;
     * Returns outerFeeGrowth0Token the fee growth on the other side of the tick from the current tick in token0;
     * Returns outerFeeGrowth1Token the fee growth on the other side of the tick from the current tick in token1;
     * Returns outerTickCumulative the cumulative tick value on the other side of the tick from the current tick;
     * Returns outerSecondsPerLiquidity the seconds spent per liquidity on the other side of the tick from the current tick;
     * Returns outerSecondsSpent the seconds spent on the other side of the tick from the current tick;
     * Returns initialized Set to true if the tick is initialized, i.e. liquidityTotal is greater than 0
     * otherwise equal to false. Outside values can only be used if the tick is initialized.
     * In addition, these values are only relative and must be used only in comparison to previous snapshots for
     * a specific position.
     */
    function ticks(int24 tick)
        external
        view
        returns (
            uint128 liquidityTotal,
            int128 liquidityDelta,
            uint256 outerFeeGrowth0Token,
            uint256 outerFeeGrowth1Token,
            int56 outerTickCumulative,
            uint160 outerSecondsPerLiquidity,
            uint32 outerSecondsSpent,
            bool initialized
        );

    /**
     * @notice Returns 256 packed tick initialized boolean values. See TickTable for more information
     */
    function tickTable(int16 wordPosition) external view returns (uint256);

    /**
     * @notice Returns the information about a position by the position's key
     * @dev This is a public mapping of structures, so the `return` natspec tags are omitted.
     * @param key The position's key is a hash of a preimage composed by the owner, bottomTick and topTick
     * @return liquidityAmount The amount of liquidity in the position;
     * Returns lastLiquidityAddTimestamp Timestamp of last adding of liquidity;
     * Returns innerFeeGrowth0Token Fee growth of token0 inside the tick range as of the last mint/burn/poke;
     * Returns innerFeeGrowth1Token Fee growth of token1 inside the tick range as of the last mint/burn/poke;
     * Returns fees0 The computed amount of token0 owed to the position as of the last mint/burn/poke;
     * Returns fees1 The computed amount of token1 owed to the position as of the last mint/burn/poke
     */
    function positions(bytes32 key)
        external
        view
        returns (
            uint128 liquidityAmount,
            uint32 lastLiquidityAddTimestamp,
            uint256 innerFeeGrowth0Token,
            uint256 innerFeeGrowth1Token,
            uint128 fees0,
            uint128 fees1
        );

    /**
     * @notice Returns data about a specific timepoint index
     * @param index The element of the timepoints array to fetch
     * @dev You most likely want to use #getTimepoints() instead of this method to get an timepoint as of some amount of time
     * ago, rather than at a specific index in the array.
     * This is a public mapping of structures, so the `return` natspec tags are omitted.
     * @return initialized whether the timepoint has been initialized and the values are safe to use;
     * Returns blockTimestamp The timestamp of the timepoint;
     * Returns tickCumulative the tick multiplied by seconds elapsed for the life of the pool as of the timepoint timestamp;
     * Returns secondsPerLiquidityCumulative the seconds per in range liquidity for the life of the pool as of the timepoint timestamp;
     * Returns volatilityCumulative Cumulative standard deviation for the life of the pool as of the timepoint timestamp;
     * Returns averageTick Time-weighted average tick;
     * Returns volumePerLiquidityCumulative Cumulative swap volume per liquidity for the life of the pool as of the timepoint timestamp;
     */
    function timepoints(uint256 index)
        external
        view
        returns (
            bool initialized,
            uint32 blockTimestamp,
            int56 tickCumulative,
            uint160 secondsPerLiquidityCumulative,
            uint88 volatilityCumulative,
            int24 averageTick,
            uint144 volumePerLiquidityCumulative
        );

    /**
     * @notice Returns the information about active incentive
     * @dev if there is no active incentive at the moment, virtualPool,endTimestamp,startTimestamp would be equal to 0
     * @return virtualPool The address of a virtual pool associated with the current active incentive
     */
    function activeIncentive() external view returns (address virtualPool);

    /**
     * @notice Returns the lock time for added liquidity
     */
    function liquidityCooldown() external view returns (uint32 cooldownInSeconds);

    /**
     * @notice The pool tick spacing
     * @dev Ticks can only be used at multiples of this value
     * e.g.: a tickSpacing of 60 means ticks can be initialized every 60th tick, i.e., ..., -120, -60, 0, 60, 120, ...
     * This value is an int24 to avoid casting even though it is always positive.
     * @return The tick spacing
     */
    function tickSpacing() external view returns (int24);
}

interface IERC20 {
    function decimals() external view returns (uint8);
}

/**
 * @dev This contract is not meant to be deployed. Instead, use a static call with the
 *       deployment bytecode as payload.
 */
contract GetCamelotV3PoolDataBatchRequest {
    struct PoolData {
        address tokenA;
        uint8 tokenADecimals;
        address tokenB;
        uint8 tokenBDecimals;
        uint128 liquidity;
        uint160 sqrtPrice;
        int24 tick;
        int24 tickSpacing;
        uint24 fee;
        int128 liquidityNet;
    }

    constructor(address[] memory pools) {
        PoolData[] memory allPoolData = new PoolData[](pools.length);

        for (uint256 i = 0; i < pools.length; ++i) {
            address poolAddress = pools[i];

            if (codeSizeIsZero(poolAddress)) continue;

            PoolData memory poolData;

            poolData.tokenA = ICamelotV3Pool(poolAddress).token0();
            poolData.tokenB = ICamelotV3Pool(poolAddress).token1();

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

            (uint160 sqrtPriceX96, int24 tick,,,,,,) = ICamelotV3Pool(poolAddress).globalState();

            (, int128 liquidityNet,,,,,,) = ICamelotV3Pool(poolAddress).ticks(tick);

            poolData.liquidity = ICamelotV3Pool(poolAddress).liquidity();
            poolData.tickSpacing = ICamelotV3Pool(poolAddress).tickSpacing();
            poolData.fee = 0;

            poolData.sqrtPrice = sqrtPriceX96;
            poolData.tick = tick;

            poolData.liquidityNet = liquidityNet;

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
}
