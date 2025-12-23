use std::collections::{BTreeMap, BTreeSet};

use debt_reporting_abi::{AutopoolAddressConstants, DestinationVaultKey, DestinationsZKPricesCommitment};

pub fn stub_post_commitment_onchain(c: DestinationsZKPricesCommitment) {
    println!("Stub for submitting a transaction to validate debt reporting\n");

    // High-level summary
    println!("Commitment: {:#?}", c.commitment);
    println!();

    // Build a lookup: DestinationVaultKey -> (index into priceInfo, price)
    // So we can quickly print rows per autopool.
    let mut price_by_key: BTreeMap<DestinationVaultKey, usize> = BTreeMap::new();
    for (i, (key, _price)) in c.priceInfo.iter().enumerate() {
        price_by_key.insert(key.clone(), i);
    }

    // Iterate autopools (constants) and print each section.
    for (ap_idx, ac) in c.allAutopoolConstants.iter().enumerate() {
        print_autopool_section_header(ap_idx, ac);

        // Collect the rows for this autopool in the order of destinationVaultKeys.
        let mut any_rows = false;

        // Table header
        println!(
            "{:<4} {:<42} {:>24} {:>24} {:<10}",
            "idx", "destinationVault", "avgSpot", "latestSafe", "spotSafe?"
        );
        println!("{}", "-".repeat(140));

        for (i, key) in ac.destinationVaultKeys.iter().enumerate() {
            if let Some(pi) = price_by_key.get(key) {
                let (k, price) = &c.priceInfo[*pi];
                any_rows = true;
                println!(
                    "{:<4} {:<42} {:>24} {:>24} {:<10}",
                    i,
                    format!("{:?}", k.destinationVault),
                    price.averageSpotPriceInQuote,
                    price.latestSafePriceInQuote,
                    price.isSpotSafeZK,
                );
            } else {
                // Key exists in constants but no price row found
                println!(
                    "{:<4} {:<42} {:>24} {:>24} {:<10}",
                    i,
                    format!("{:?}", key.destinationVault),
                    "(missing)",
                    "(missing)",
                    "(missing)"
                );
            }
        }

        if !any_rows {
            println!("(no priceInfo rows matched this autopool’s destinationVaultKeys)");
        }

        println!("\n(Stub) would now build + send tx with proof + commitment payload...");
        println!();
    }
}

fn print_autopool_section_header(ap_idx: usize, ac: &AutopoolAddressConstants) {
    // "a bunch of new lines" + loud header
    println!("\n\n\n\n\n");
    println!("================================================================================");
    println!("AUT0POOL #{}: {:?}", ap_idx, ac.autopool);
    println!("================================================================================");
    println!("systemRegistry:  {:?}", ac.systemRegistry);
    println!("rootPriceOracle: {:?}", ac.rootPriceOracle);
    println!("baseAsset:       {:?}", ac.baseAsset);
    println!("destinationVaultKeys: {}", ac.destinationVaultKeys.len());
    println!();
}
