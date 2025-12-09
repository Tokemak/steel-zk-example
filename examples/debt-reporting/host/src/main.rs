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
    Commitment, Contract,
};
use risc0_zkvm::{default_executor, ExecutorEnv};
use tracing_subscriber::EnvFilter;

use alloy_sol_types::{sol, SolCall, SolType}; 
use alloy_primitives::{address, Address};

use clap::Parser;
use debt_reporting_abi::IRootPriceOracle;
use debt_reporting_methods::ERC20_GUEST_ELF;
use url::Url;

/// Simple program to show the use of Ethereum contract data inside the guest.
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// URL of the RPC endpoint
    #[arg(short, long, env = "RPC_URL")]
    rpc_url: Url,
}

// in general it looks like this should live in the guest?

// const EOA_TOKEMAK_WALLET: Address = address!("91aa2CcE6B22Ec9eCd8A56C830566e67187fe07E");
const USDC_MAINNET: Address = address!("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48");
const ROOT_PRICE_ORACLE: Address = address!("61F8BE7FD721e80C0249829eaE6f0DAf21bc2CaC");

fn make_get_range_prices_lp_token_calls(
    lp_tokens: Vec<Address>,
    pools: Vec<Address>,
    quote_token: Address,
) -> Vec<IRootPriceOracle::getRangePricesLPCall> {
    if lp_tokens.len() != pools.len() {
        panic!(
            "lp_tokens and pools must have the same length. lp_tokens={}, pools={}",
            lp_tokens.len(),
            pools.len(),
        );
    }

    let mut calls = Vec::new();
    for (lp_token, pool) in lp_tokens.into_iter().zip(pools.into_iter()) {
        calls.push(IRootPriceOracle::getRangePricesLPCall {
            lpToken: lp_token,
            pool: pool,
            quoteToken: quote_token,
        });
    }

    return calls;
}

// not sure if needed
type RangePricesReturn =
    <IRootPriceOracle::getRangePricesLPCall as SolCall>::Return;


#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting Debt Reporting example...");
    // not certain what this does here
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    // Create an EVM environment from an RPC endpoint defaulting to the latest block.
    let mut env = EthEvmEnv::builder()
        .rpc(args.rpc_url)
        .chain_spec(&ETH_MAINNET_CHAIN_SPEC)
        .build()
        .await?;

    let some_vaults: Vec<Address> = vec![
        address!("64273624eb57c5cA961d366CBF3968e760Bf0452"),
        address!("0x85B2b559bC2D21104C4DEFdd6EFcA8A20343361D"),
    ];
    let some_pools: Vec<Address> = vec![
        address!("64273624eb57c5cA961d366CBF3968e760Bf0452"),
        address!("0x85B2b559bC2D21104C4DEFdd6EFcA8A20343361D"),
    ];

    // I think this has to live on teh guest
    let calls: Vec<IRootPriceOracle::getRangePricesLPCall> =
        make_get_range_prices_lp_token_calls(some_vaults, some_pools, USDC_MAINNET);

    // let mut results: Vec<RangePricesReturn> = Vec::with_capacity(calls.len());
    let mut contract = Contract::preflight(ROOT_PRICE_ORACLE, &mut env);
    println!("building calls!");

    // Make a call for each item in the list
    for call in calls {
        let mut builder = contract.call_builder(&call);
        // let ret: RangePricesReturn = builder.call().await?;
        // results.push(ret);
    } 
    // not certain what this does
    let input = env.into_input().await?; // Finally, construct the input from the environment.

    println!("Running the guest with the constructed input...");
    let session_info = {
        let env = ExecutorEnv::builder()
            .write(&input)
            .unwrap()
            .build()
            .context("failed to build executor env")?;

        let exec = default_executor();
        exec.execute(env, ERC20_GUEST_ELF)
            .context("failed to run executor")?
    };

    // The journal should be the ABI encoded commitment.
    let commitment = Commitment::abi_decode(session_info.journal.as_ref())
        .context("failed to decode journal")?;

    println!("{commitment:?}");

    Ok(())
}

// #[tokio::main]
// async fn main() -> Result<()> {
//     println!("Starting Debt Reporting example...");
//     tracing_subscriber::fmt()
//         .with_env_filter(EnvFilter::from_default_env())
//         .init();

//     // Parse the command line arguments.
//     let args = Args::parse();

//     // Create an EVM environment from an RPC endpoint defaulting to the latest block.
//     let mut env = EthEvmEnv::builder()
//         .rpc(args.rpc_url)
//         .chain_spec(&ETH_MAINNET_CHAIN_SPEC)
//         .build()
//         .await?;

//     // Preflight the call to prepare the input that is required to execute the function in
//     // the guest without RPC access. It also returns the result of the call.
//     let mut contract = Contract::preflight(ROOT_PRICE_ORACLE, &mut env);
//     let mut builder = contract.call_builder(&CALL);
//     builder.tx.caller = EOA_TOKEMAK_WALLET;
//     let returns = builder.call().await?;

//     println!("returns: {:?}", returns._0);
//     println!("returns: {:?}", returns._1);
//     println!("returns: {:?}", returns._2);

//     let input = env.into_input().await?; // Finally, construct the input from the environment.

//     println!("Running the guest with the constructed input...");
//     let session_info = {
//         let env = ExecutorEnv::builder()
//             .write(&input)
//             .unwrap()
//             .build()
//             .context("failed to build executor env")?;

//         let exec = default_executor();
//         exec.execute(env, ERC20_GUEST_ELF)
//             .context("failed to run executor")?
//     };

//     // The journal should be the ABI encoded commitment.
//     let commitment = Commitment::abi_decode(session_info.journal.as_ref())
//         .context("failed to decode journal")?;

//     println!("{commitment:?}");

//     Ok(())
// }

// args autopool root price oracle
// autopool.getDestinations()
// autopool. base asset
// for destination in destinations:
// get lp token and pool into list
// we now have a list of calls to target at the root price oracle
// -> that gives
