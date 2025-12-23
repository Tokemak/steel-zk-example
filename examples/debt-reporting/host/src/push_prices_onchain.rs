use debt_reporting_abi::DestinationsZKPricesCommitment;

pub fn stub_post_commitment_onchain(c: DestinationsZKPricesCommitment) {
    println!("\n=== Stub: Submit Debt Reporting Commitment ===\n");

    // Commitment header
    println!("Commitment:");
    println!("  {:?}", c.commitment);
    println!();

    println!("Price Info ({} entries):", c.priceInfo.len());
    println!("---------------------------------------------");

    for (i, (key, computed)) in c.priceInfo.iter().enumerate() {
        println!("Entry #{i}");
        println!("  DestinationVaultKey:");
        println!("    token:              {:?}", key.token);
        println!("    pool:               {:?}", key.pool);
        println!("    baseAsset:          {:?}", key.baseAsset);
        println!("    destinationVault:   {:?}", key.destinationVault);

        println!("  ComputedGetRangePriceLP:");
        println!(
            "    averageSpotPriceInQuote: {}",
            computed.averageSpotPriceInQuote
        );
        println!(
            "    latestSafePriceInQuote:  {}",
            computed.latestSafePriceInQuote
        );
        println!("    isSpotSafeZK:            {}", computed.isSpotSafeZK);

        println!("---------------------------------------------");
    }

    println!("End commitment stub\n");
}
