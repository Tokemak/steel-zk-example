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

use alloy_sol_types::SolType;
use anyhow::{Context, Result};
use clap::Parser;
use debt_reporting_abi::{
    IRootPriceOracle, A_LP_TOKEN, ROOT_PRICE_ORACLE, USDC_MAINNET,
};
use debt_reporting_methods::DEBT_REPORTING_GUEST_ELF; // not sure what this is and what it is used for
use risc0_steel::{
    ethereum::{EthEvmEnv, ETH_MAINNET_CHAIN_SPEC},
    Commitment, Contract,
};
use risc0_zkvm::{default_executor, ExecutorEnv};
use tracing_subscriber::EnvFilter;
use url::Url;

/// Simple program to show the use of Ethereum contract data inside the guest.
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// URL of the RPC endpoint
    #[arg(short, long, env = "RPC_URL")]
    rpc_url: Url,
}

// the host has to preflight the input data, passes the guest the needed EVM data

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting Debt Reporting example...");
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    let mut env = EthEvmEnv::builder()
        .rpc(args.rpc_url)
        .chain_spec(&ETH_MAINNET_CHAIN_SPEC)
        .build()
        .await?;

    let mut contract = Contract::preflight(ROOT_PRICE_ORACLE, &mut env);

    let call: IRootPriceOracle::getRangePricesLPCall = IRootPriceOracle::getRangePricesLPCall {
        lpToken: A_LP_TOKEN,
        pool: A_LP_TOKEN,
        quoteToken: USDC_MAINNET,
    };

    let _ = contract.call_builder(&call).call().await?;
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