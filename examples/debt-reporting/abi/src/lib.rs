// use alloy_primitives::{address};
use alloy_sol_types::sol;
use risc0_steel::Commitment;

sol! {
    interface IMinimalRootPriceOracle {
        function getRangePricesLP(
            address lpToken,
            address pool,
            address quoteToken
        )
            external
            view
            returns (uint256 spotPriceInQuote, uint256 safePriceInQuote, bool isSpotSafe);
    }

    /// Minimal interface for an Autopool.
    interface IMinimalAutoPool {
        function getSystemRegistry() external view returns (address systemRegistry);

        function getDestinations() external view returns (address[] memory _destinations);

        /// Base asset of the Autopool.
        function asset() external view returns (address pool);
    }

    interface IMinimalDestinationVault {
        /// Underlying pool
        function getPool() external view returns (address pool);

        /// LP token for this destination vault. Often the same as the pool
        function underlying() external view returns (address underlying);
    }

    interface IMinimalSystemRegistry {
        function rootPriceOracle() external view returns (address rootPriceOracle_);
    }
}

sol! {
    #![sol(all_derives)]
    #![sol(extra_derives(serde::Serialize, serde::Deserialize))]
    // can we store this as a sha256(token, pool, baseAsset, destinationVault) instead?
    // so we are writing one slot instead of 4?
    struct DestinationVaultKey {
        address token;
        address pool;
        address baseAsset; // redundent
        address destinationVault;
    }

    struct ComputedGetRangePriceLP{
        uint256 averageSpotPriceInQuote;
        uint256 recentSafePriceInQuote;
        bool isSpotSafeZK;
    }

    struct AutopoolAddressConstants {
        address autopool;
        address systemRegistry;
        address rootPriceOracle;
        address baseAsset; // is redundent, don't worry about for now
        DestinationVaultKey[] destinationVaultKeys;
    }

    struct AutopoolAddressConstantsCommitment {
        Commitment commitment;
        AutopoolAddressConstants autopool_address_constants;
    }

    struct AverageSafePriceCommitment {
        Commitment commitment;
        (DestinationVaultKey, ComputedGetRangePriceLP)[] priceInfo; // lpToken, averageSafePrice
    }

    struct BaseAssetCommitment {
        Commitment commitment;
        address baseAsset;
    }
}

// const EOA_TOKEMAK_WALLET: Address = address!("91aa2CcE6B22Ec9eCd8A56C830566e67187fe07E");
// pub const USDC_MAINNET: Address = address!("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48");
// pub const ROOT_PRICE_ORACLE: Address = address!("61F8BE7FD721e80C0249829eaE6f0DAf21bc2CaC");
// pub const A_LP_TOKEN: Address = address!("64273624eb57c5cA961d366CBF3968e760Bf0452");
// pub const BAL_AAVE_GHO_USDT_USDC_DESTINATION_VAULT: Address =
//     address!("0x366C094C5563CD12AF27b9AfFF2200B0E0D056E0");
