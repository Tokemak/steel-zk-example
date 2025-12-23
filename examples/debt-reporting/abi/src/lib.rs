use alloy_sol_types::sol;
use core::cmp::Ordering;
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

    interface IMinimalAutoPool { // capitalized
        function getSystemRegistry() external view returns (address systemRegistry);

        function getDestinations() external view returns (address[] memory _destinations);

        function asset() external view returns (address pool);
    }

    interface IMinimalDestinationVault {
        function getPool() external view returns (address pool);

        // LP token for this destination vault. Often the same as the pool
        function underlying() external view returns (address underlying);
    }

    interface IMinimalSystemRegistry {
        function rootPriceOracle() external view returns (address rootPriceOracle_);
    }

    // source  https://vscode.blockscan.com/ethereum/0xcA11bde05977b3631167028862bE2a173976CA11

    interface IMulticall3 {
        struct Call3 {
            address target;
            bool allowFailure;
            bytes callData;
        }

        struct Result {
            bool success;
            bytes returnData;
        }

        function aggregate3(Call3[] calldata calls)
            external
            payable
            returns (Result[] memory returnData);
    }
}

sol! {
    #![sol(all_derives)]
    #![sol(extra_derives(serde::Serialize, serde::Deserialize))]

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

// required for the BTreeSet and BTreeMap
impl Ord for DestinationVaultKey {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.token, self.pool, self.baseAsset, self.destinationVault).cmp(&(
            other.token, // todo rename to underlying
            other.pool,
            other.baseAsset,
            other.destinationVault,
        ))
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
