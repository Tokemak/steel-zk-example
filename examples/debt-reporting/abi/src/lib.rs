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
// TODO clean up these interfaces
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

    struct AutopoolAddressConstants {
        address autopool;
        address systemRegistry;
        address rootPriceOracle;
        address baseAsset; // is redundent, don't worry about for now
        DestinationVaultKey[] destinationVaultKeys;
    }

    struct AutopoolAddressConstantsCommitment {
        Commitment commitment;
        AutopoolAddressConstants autopoolConstants;
    }

    struct ComputedGetRangePriceLP{
        uint256 averageSpotPriceInQuote;
        uint256 latestSafePriceInQuote;
        bool isSpotSafeZK;
    }

    struct DestinationsZKPricesCommitment {
        Commitment commitment;
        AutopoolAddressConstants autopoolConstants;
        (DestinationVaultKey, ComputedGetRangePriceLP)[] priceInfo;
    }
}




use core::cmp::Ordering;

impl Ord for DestinationVaultKey {
    // required for the BTreeSet and BTreeMap
    fn cmp(&self, other: &Self) -> Ordering {
        (self.token, self.pool, self.baseAsset, self.destinationVault)
            .cmp(&(other.token, other.pool, other.baseAsset, other.destinationVault))
    }
}

impl PartialOrd for DestinationVaultKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// might not be needed
// pub fn keccak256_destination_vault_key(dvk: &DestinationVaultKey) -> B256 {
//     keccak256(dvk.abi_encode())
// }