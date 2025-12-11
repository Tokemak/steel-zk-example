
use debt_reporting_abi::{
    AverageSafePriceCommitment
};

pub fn stub_post_commitment_on_chain(average_safe_price_commitment: AverageSafePriceCommitment) {
    /*
    After this, we should push received price data
    (along with the proof in a transient storage), not certain what transient storage means here
    to a new ZK executor contract.
    */
    println!("Stub for submitting a transaction to validate debt reporting");

    let c = &average_safe_price_commitment;
    println!("commitment: {:?}", c.commitment);

    for (lp_token, avg_price) in &c.priceInfo {
        println!("lp_token: {lp_token:?}, avg_safe_price: {avg_price}");
    }
    println!("blocks: {:?}", c.blocks);
}

