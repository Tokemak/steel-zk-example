// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.24;

import {IZKRootPriceOracle} from "./IZKRootPriceOracle.sol";

contract ZKRootPriceOracle is IZKRootPriceOracle {
    address public immutable systemRegistry;
    string public constant NAMESPACE = "AutoFinanceZKDebtReportingV1";

    constructor(address _systemRegistry) {
        systemRegistry = _systemRegistry;
    
    }

    function getRangePricesLP(address lpToken, address pool, address quoteToken)
        external
        returns (uint256 spotPriceInQuote, uint256 safePriceInQuote, bool isSpotSafe)
    {
        // map (destination vault address) -> 

        // I first need this mapping 
        // (lpToken, pool, quoteToken) -> destination vault address -> prices

        bytes32 transientStorageSlot = keccak256(abi.encodePacked(lpToken, pool, quoteToken));


        uint256 packedPrices = 


    }
}
