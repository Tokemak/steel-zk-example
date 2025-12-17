use debt_reporting_abi::DestinationsZKPricesCommitment;

pub fn stub_post_commitment_onchain(c: DestinationsZKPricesCommitment) {
    println!("Stub for submitting a transaction to validate debt reporting\n");

    // High-level summary
    println!("Commitment: {:#?}", c.commitment);

    let ac = &c.autopoolConstants;
    println!("Autopool constants:");
    println!("  autopool:       {:?}", ac.autopool);
    println!("  systemRegistry: {:?}", ac.systemRegistry);
    println!("  rootPriceOracle:{:?}", ac.rootPriceOracle);
    println!("  baseAsset:      {:?}", ac.baseAsset);
    println!("  destinationVaultKeys: {}", ac.destinationVaultKeys.len());
    println!("  priceInfo rows:        {}", c.priceInfo.len());
    println!();

    // Table header
    println!(
        "{:<4} {:<42} {:<42} {:<42} {:<42} {:>24} {:>24} {:<10}",
        "idx",
        "token",
        "pool",
        "baseAsset",
        "destinationVault",
        "avgSpot",
        "latestSafe",
        "spotSafe?"
    );
    println!("{}", "-".repeat(240));

    // Rows
    for (i, (key, price)) in c.priceInfo.iter().enumerate() {
        println!(
            "{:<4} {:<42} {:<42} {:<42} {:<42} {:>24} {:>24} {:<10}",
            i,
            format!("{:?}", key.token),
            format!("{:?}", key.pool),
            format!("{:?}", key.baseAsset),
            format!("{:?}", key.destinationVault),
            price.averageSpotPriceInQuote,
            price.latestSafePriceInQuote,
            price.isSpotSafeZK,
        );
    }

    println!("\n(Stub) would now build + send tx with proof + commitment payload...");
}
