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
    AverageSafePriceCommitment, IRootPriceOracle, A_LP_TOKEN, ROOT_PRICE_ORACLE, USDC_MAINNET,
};
use debt_reporting_methods::DEBT_REPORTING_GUEST_ELF; // not sure what this is and what it is used for
use risc0_steel::{
    alloy::providers::{Provider, ProviderBuilder, RootProvider},
    ethereum::{EthEvmEnv, ETH_MAINNET_CHAIN_SPEC},
    Contract, SteelVerifier,
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
    let provider: RootProvider = ProviderBuilder::default().connect_http(args.rpc_url);

    let mut latest = provider.get_block_number().await?;
    latest = latest - 10; 
    let mut blocks = Vec::with_capacity(3);
    for i in 0..3 {
        let offset = 1 * (9 - i);
        blocks.push(latest - offset);
    }

    let mut inputs = Vec::with_capacity(blocks.len());

    for block in blocks.iter() {
        let builder = EthEvmEnv::builder()
            .provider(provider.clone())
            .chain_spec(&ETH_MAINNET_CHAIN_SPEC)
            .block_number(*block);

        let mut env = builder.build().await?;

        let mut contract = Contract::preflight(ROOT_PRICE_ORACLE, &mut env);

        let call: IRootPriceOracle::getRangePricesLPCall = IRootPriceOracle::getRangePricesLPCall {
            lpToken: A_LP_TOKEN,
            pool: A_LP_TOKEN,
            quoteToken: USDC_MAINNET,
        };

        // we don't need to care about the output of this call here
        // just need to fetch data
        // so that we use it in the guest
        let _ = contract.call_builder(&call).call().await?;
        println!("success getRangePricesLPCall {block:?} ");
        let commitment = env.commitment();
        println!("success commitment {block:?} ");
        println!("raw commitment: {:?}", commitment);

        SteelVerifier::preflight(&mut env)
            .verify(&commitment)
            .await?;
        println!("success preflight {block:?} ");
        let input = env.into_input().await?;
        inputs.push(input)
    }

    println!("Running the guest with the constructed input:");
    let session_info = {
        let mut builder = ExecutorEnv::builder();

        builder
            .write(&blocks)
            .context("failed to write blocks into executor env")?;
        builder
            .write(&inputs)
            .context("failed to write inputs into executor env")?;

        let env = builder.build().context("failed to build executor env")?;

        let exec = default_executor();
        exec.execute(env, DEBT_REPORTING_GUEST_ELF)
            .context("failed to run executor")?
    };

    // The journal should be the ABI encoded commitment.
    let average_safe_price_commitment =
        AverageSafePriceCommitment::abi_decode(session_info.journal.as_ref())
            .context("failed to decode journal")?;
        

    let c = &average_safe_price_commitment;

    println!("commitment: {:?}", c.commitment);

    for (lp_token, avg_price) in &c.priceInfo {
        println!("lp_token: {lp_token:?}, avg_safe_price: {avg_price}");
    }

    println!("blocks: {:?}", c.blocks);

    // post the commitment and answers on chain with a write
    // eg post the (destination_vault: lp token safe price data)

    Ok(())
}

// and then decode it in the guest

// you can write this here, you can write a struct

// then as the proof
// the proof is then debt params,
// include assets of the right block?
// in the guest?

// let params = DebtParams {
//     num_prior_blocks: 2,
//     gap_between_blocks: 100,
// };

// println!("Running the guest with the constructed input:");
// let session_info = {
//     let mut builder = ExecutorEnv::builder();

//     // 1. Write the params struct
//     builder
//         .write(&params)
//         .context("failed to write DebtParams")?;
