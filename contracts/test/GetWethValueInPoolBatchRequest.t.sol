// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/GetWethValueInPoolBatchRequest.sol";

contract GetWethValueInPoolBatchRequestTest is Test {
    address constant weth = 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;
    uint256 constant wethInPoolThreshold = 0.01 ether;

    function setUp() public {
        string memory rpc_url = vm.envString("ETHEREUM_RPC_URL");
        vm.createSelectFork(rpc_url);
    }

    function testUniV3GoodLiquidity() public {
        address[] memory pools = new address[](1);
        pools[0] = address(0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640);

        address[] memory dexes = new address[](1);
        dexes[0] = address(0x1F98431c8aD98523631AE4a59f267346ea31F984);

        bool[] memory dexIsUniV3 = new bool[](1);
        dexIsUniV3[0] = true;

        GetWethValueInPoolBatchRequest data =
            new GetWethValueInPoolBatchRequest(pools, dexes, dexIsUniV3, weth, wethInPoolThreshold);

        uint256[] memory weth_values = abi.decode(address(data).code, (uint256[]));

        // expecting weth value to be non 0
        assert(weth_values[0] != 0);
    }

    function testUniV3VerySmallLiquidity() public {
        address[] memory pools = new address[](1);
        pools[0] = address(0x697C1CcA83174363e9B6758B8CD616474487C192);

        address[] memory dexes = new address[](1);
        dexes[0] = address(0x1F98431c8aD98523631AE4a59f267346ea31F984);

        bool[] memory dexIsUniV3 = new bool[](1);
        dexIsUniV3[0] = true;

        GetWethValueInPoolBatchRequest data =
            new GetWethValueInPoolBatchRequest(pools, dexes, dexIsUniV3, weth, wethInPoolThreshold);

        uint256[] memory weth_values = abi.decode(address(data).code, (uint256[]));

        // expecting weth value to be 0
        assert(weth_values[0] == 0);
    }

    function testUniV3ZeroLiquidity() public {
        address[] memory pools = new address[](1);
        pools[0] = address(0xc53489F27F4d8A1cdceD3BFe397CAF628e8aBC13);

        address[] memory dexes = new address[](1);
        dexes[0] = address(0x1F98431c8aD98523631AE4a59f267346ea31F984);

        bool[] memory dexIsUniV3 = new bool[](1);
        dexIsUniV3[0] = true;

        GetWethValueInPoolBatchRequest data =
            new GetWethValueInPoolBatchRequest(pools, dexes, dexIsUniV3, weth, wethInPoolThreshold);

        uint256[] memory weth_values = abi.decode(address(data).code, (uint256[]));

        // expecting weth value to be non 0
        assert(weth_values[0] == 0);
    }

    function testUniV3EdgeCase() public {
        string memory rpc_url = vm.envString("ARBITRUM_RPC_URL");
        vm.createSelectFork(rpc_url);

        address[] memory pools = new address[](1);
        pools[0] = address(0x0184CEdd5a45A483c28A3703C4555627aa5a8e51);

        address[] memory dexes = new address[](1);
        dexes[0] = address(0x1F98431c8aD98523631AE4a59f267346ea31F984);

        bool[] memory dexIsUniV3 = new bool[](1);
        dexIsUniV3[0] = true;

        address arbitrum_weth = address(0x82aF49447D8a07e3bd95BD0d56f35241523fBab1);

        GetWethValueInPoolBatchRequest data =
            new GetWethValueInPoolBatchRequest(pools, dexes, dexIsUniV3, arbitrum_weth, wethInPoolThreshold);

        uint256[] memory weth_values = abi.decode(address(data).code, (uint256[]));

        console.log("Weth values in pools: %s", weth_values[0]);
        // expecting weth value to be non 0
        assert(weth_values[0] == 0);
    }
}
