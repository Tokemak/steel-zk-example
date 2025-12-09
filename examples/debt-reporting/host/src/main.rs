// Copyright 2025 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use anyhow::{Context, Result};
use risc0_steel::{
    ethereum::{EthEvmEnv, ETH_MAINNET_CHAIN_SPEC},
    Commitment,
};
use risc0_zkvm::{default_executor, ExecutorEnv};
use tracing_subscriber::EnvFilter;

use alloy_sol_types::SolType;

use clap::Parser;
use debt_reporting_methods::DEBT_REPORTING_GUEST_ELF; // not sure what this is and what it is used for
use url::Url;

/// Simple program to show the use of Ethereum contract data inside the guest.
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// URL of the RPC endpoint
    #[arg(short, long, env = "RPC_URL")]
    rpc_url: Url,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting Debt Reporting example...");
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    let env = EthEvmEnv::builder()
        .rpc(args.rpc_url)
        .chain_spec(&ETH_MAINNET_CHAIN_SPEC)
        .build()
        .await?;

    let input = env.into_input().await?;

    println!("Running the guest with the constructed input...");
    let session_info = {
        let env = ExecutorEnv::builder()
            .write(&input)
            .unwrap()
            .build()
            .context("failed to build executor env")?;

        let exec = default_executor();
        exec.execute(env, DEBT_REPORTING_GUEST_ELF)
            .context("failed to run executor")?
    };

    // The journal should be the ABI encoded commitment.
    let commitment = Commitment::abi_decode(session_info.journal.as_ref())
        .context("failed to decode journal")?;

    println!("{commitment:?}");

    Ok(())
}

// fn _make_get_range_prices_lp_token_calls(
//     lp_tokens: Vec<Address>,
//     pools: Vec<Address>,
//     quote_token: Address,
// ) -> Vec<IRootPriceOracle::getRangePricesLPCall> {
//     if lp_tokens.len() != pools.len() {
//         panic!(
//             "lp_tokens and pools must have the same length. lp_tokens={}, pools={}",
//             lp_tokens.len(),
//             pools.len(),
//         );
//     }

//     let mut calls = Vec::new();
//     for (lp_token, pool) in lp_tokens.into_iter().zip(pools.into_iter()) {
//         calls.push(IRootPriceOracle::getRangePricesLPCall {
//             lpToken: lp_token,
//             pool: pool,
//             quoteToken: quote_token,
//         });
//     }

//     return calls;
// }

// fn _simple_make_calls() -> Vec<IRootPriceOracle::getRangePricesLPCall> {
//     let some_vaults: Vec<Address> = vec![
//         address!("64273624eb57c5cA961d366CBF3968e760Bf0452"),
//         address!("0x85B2b559bC2D21104C4DEFdd6EFcA8A20343361D"),
//     ];
//     let some_pools: Vec<Address> = vec![
//         address!("64273624eb57c5cA961d366CBF3968e760Bf0452"),
//         address!("0x85B2b559bC2D21104C4DEFdd6EFcA8A20343361D"),
//     ];

//     let calls: Vec<IRootPriceOracle::getRangePricesLPCall> =
//         make_get_range_prices_lp_token_calls(some_vaults, some_pools, USDC_MAINNET);

//     return calls
// }

// args autopool root price oracle
// autopool.getDestinations()
// autopool. base asset
// for destination in destinations:
// get lp token and pool into list
// we now have a list of calls to target at the root price oracle
// -> that gives

// error[E0432]: unresolved import `risc0_steel`
//  --> abi/src/lib.rs:2:5
//   |
// 2 | use risc0_steel::Commitment;
//   |     ^^^^^^^^^^^ use of unresolved module or unlinked crate `risc0_steel`
