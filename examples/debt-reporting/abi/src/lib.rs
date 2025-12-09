use alloy_sol_types::sol;
use risc0_steel::Commitment;

sol! {
    interface IRootPriceOracle {
        function getRangePricesLP(
            address lpToken,
            address pool,
            address quoteToken
        ) external returns (uint, uint, bool); // spotPriceInQuote, safePriceInQuote, isSpotSafe
    }
}

sol! {
    struct AverageSafePriceCommitment {
        Commitment commitment;
        (address, uint)[] priceInfo; // lpToken, averageSafePrice
        uint numPriorBlocks;
        uint gapBetweenBlocks;
    }
}
