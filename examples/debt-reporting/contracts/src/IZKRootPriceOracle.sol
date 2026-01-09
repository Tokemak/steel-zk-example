// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.24;

interface IZKRootPriceOracle {
    /// @notice Retrieve the price of LP token based on the reserves
    /// @param lpToken LP token to get the price of
    /// @param pool liquidity pool to be used for price determination
    /// @param quoteToken token to quote the price in
    // we are writing destination vault address -> price

    // but what this gets here is
    // (lp token, pool, quoteToken) -> price

    function getRangePricesLP(address lpToken, address pool, address quoteToken)
        external
        returns (uint256 spotPriceInQuote, uint256 safePriceInQuote, bool isSpotSafe);
}
