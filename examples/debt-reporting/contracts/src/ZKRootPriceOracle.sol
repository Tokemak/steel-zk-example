// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.24;

import {Steel} from "contracts/src/Steel.sol"; // note required for  NAMESPACE.erc7201Slot()
import {IZKRootPriceOracle} from "./IZKRootPriceOracle.sol";
import {PackedGetRangePriceLPCodec} from "./PackedGetRangePriceLPCodec.sol";

import {TransientSlot} from "lib/openzeppelin-contracts/contracts/utils/TransientSlot.sol";
import {SlotDerivation} from "lib/openzeppelin-contracts/contracts/utils/SlotDerivation.sol";

contract ZKRootPriceOracle is IZKRootPriceOracle {
    using SlotDerivation for bytes32;

    address public immutable systemRegistry;
    string public constant NAMESPACE = "AutoFinanceZKDebtReportingV1";

    error PriceNotFoundInTransientStorage();

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
        uint256 packedPrices = NAMESPACE.erc7201Slot().deriveMapping(transientStorageSlot).getAddressSlot().value;

        // not sure if this is the right pattern here
        // maybe instead of reverting we just say safe != spot?
        // reverting ot be safe

        if (packedPrices == 0) revert PriceNotFoundInTransientStorage();
        PackedComputedGetRangePriceLP memory unpacked = PackedGetRangePriceLPCodec.unpack(packedPrices);
        return (unpacked.averageSpotPriceInQuote, unpacked.latestSafePriceInQuote, unpacked.isSpotSafeZK);
    }
}
