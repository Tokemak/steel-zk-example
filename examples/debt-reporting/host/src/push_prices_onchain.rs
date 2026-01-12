use debt_reporting_abi::DestinationsZKPricesCommitment;
use alloy_primitives::{aliases::U112, ruint::UintTryFrom, B256, U256};

fn mask_112() -> U256 {
    (U256::from(1u8) << 112) - U256::from(1u8)
}

fn fmt_u256_hex_32(x: U256) -> String {
    // always 32 bytes / 64 hex chars, so you can see the zeros
    format!("0x{:064x}", x)
}

fn unpack_packed_prices_stub(x: U256) -> (U112, U112, bool, u8) {
    let m = mask_112();

    let avg_u256 = x & m;
    let latest_u256 = (x >> 112u32) & m;

    // extract u8 at bits [224..232)
    let flag_u8: u8 = (x >> 224u32).as_limbs()[0] as u8;

    // stub policy: accept 0/1; otherwise treat as false
    let is_spot_safe = matches!(flag_u8, 1);

    let avg_u112: U112 = U112::uint_try_from(avg_u256).unwrap_or(U112::ZERO);
    let latest_u112: U112 = U112::uint_try_from(latest_u256).unwrap_or(U112::ZERO);

    (avg_u112, latest_u112, is_spot_safe, flag_u8)
}

fn print_packed_layout(x: U256) {
    let m = mask_112();

    let avg = x & m;
    let latest = (x >> 112u32) & m;

    // fields above
    let flag_u256 = (x >> 224u32) & U256::from(0xffu16);
    let top_u256 = x >> 232u32; // top 24 bits (should be zero)

    // low limb is enough after shifting/masking
    let flag_u8: u8 = flag_u256.as_limbs()[0] as u8;
    let top_u32: u32 = top_u256.as_limbs()[0] as u32; // should be <= 0xFFFFFF

    println!("  packedPrices (dec): {}", x);
    println!("  packedPrices (hex): {}", fmt_u256_hex_32(x));

    // show the exact bit layout with separators
    println!(
        "  layout [top24][flag8][latest112][avg112]: 0x{:06x}_{:02x}_{:028x}_{:028x}",
        top_u32, flag_u8, latest, avg
    );

    // optional: show parts explicitly
    println!("    top24     : 0x{:06x} (should be 0)", top_u32);
    println!("    flag8     : 0x{:02x}", flag_u8);
    println!("    latest112 : 0x{:028x}", latest);
    println!("    avg112    : 0x{:028x}", avg);
}

pub fn stub_post_commitment_onchain(c: DestinationsZKPricesCommitment) {
    println!("\n=== Stub: Submit Debt Reporting Commitment ===\n");

    println!("Commitment:");
    println!("  {:?}", c.commitment);
    println!();

    println!("Price Info: {} entries", c.priceInfo.len());
    println!("(keyHash = keccak256(abi.encodePacked(lpToken,pool,quoteToken)))");
    println!("(packedPrices layout: avg:u112 | latest:u112<<112 | flag:u8<<224)");
    println!("------------------------------------------------------------------");

    for (i, (key_hash, packed_prices)) in c.priceInfo.iter().enumerate() {
        let key_hash: &B256 = key_hash;

        let (avg_u112, latest_u112, is_spot_safe, raw_flag_u8) =
            unpack_packed_prices_stub(*packed_prices);

        println!("Entry #{i}:");
        println!("  keyHash:        {:?}", key_hash);

        // print packed as full 32-byte hex + segmented view so zeros are obvious
        print_packed_layout(*packed_prices);

        // decoded (human-readable)
        if raw_flag_u8 != 0 && raw_flag_u8 != 1 {
            println!("  decoded: (WARNING flag {}, expected 0/1)", raw_flag_u8);
        } else {
            println!("  decoded:");
        }
        println!("    avgSpot:      {}", U256::from(avg_u112));
        println!("    latestSafe:   {}", U256::from(latest_u112));
        println!("    isSpotSafe:   {} (raw flag {})", is_spot_safe, raw_flag_u8);

        println!("------------------------------------------------------------------");
    }

    println!("End commitment stub\n");
}
