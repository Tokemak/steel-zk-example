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

mod preflight_fetch_contract_addresses;


use alloy_sol_types::SolType;
use anyhow::{Context, Result};
use clap::Parser;

use debt_reporting_abi::{
    AverageSafePriceCommitment, IRootPriceOracle, AutopoolAddressConstants
}
// use debt_reporting_methods::DEBT_REPORTING_GUEST_ELF; // not sure what this is and what it is used for
// use futures::{StreamExt, TryStreamExt};
use risc0_steel::{
    alloy::providers::{Provider, ProviderBuilder, RootProvider},
    ethereum::{EthEvmEnv, ETH_MAINNET_CHAIN_SPEC},
};
// use risc0_zkvm::{default_executor, ExecutorEnv};
use tracing_subscriber::EnvFilter;
use url::Url;
use alloy_primitives::{Address, address};
use preflight_fetch_contract_addresses::fetch_constants_for_autopool;

/// Simple program to show the use of Ethereum contract data inside the guest.
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[arg(short, long, env = "RPC_URL")]
    rpc_url: Url,
    #[arg(long, env = "BEACON_API_URL")]
    beacon_api_url: Url,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting Debt Reporting example...");
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    let provider: RootProvider = ProviderBuilder::default().connect_http(args.rpc_url);
    let latest = provider.get_block_number().await?;
    let mut blocks = Vec::with_capacity(3);
    let mut envs = Vec::with_capacity(blocks.len());

    for i in 0..3 {
        blocks.push(latest - (i * 2));
    }

    for block in blocks {
        let provider = provider.clone();
        let mut env = EthEvmEnv::builder()
            .provider(provider)
            .block_number(block_num)
            .chain_spec(&ETH_MAINNET_CHAIN_SPEC)
            .build()
            .await?;
        envs.push(env);
    }

    // in the 0th position
    let most_recent_env = envs.get(0); // not sure if this is right, I want a mutable refernce to the env
    let autopool: Address = address!("0x0A2b94F6871c1D7A32Fe58E1ab5e6deA2f114E56"); // autoETH

    let autopool_address_contants: AutopoolAddressConstants =
        fetch_constants_for_autopool(autopool, most_recent_env).await?;

    println!("Address preflight Success!!!");

    Ok(())

    // println!("Running Debt Reporting in the guest for {blocks:?}");

    // let session_info = {
    //     let mut builder = ExecutorEnv::builder();

    //     builder
    //         .write(&inputs)
    //         .context("failed to write inputs into executor env")?;

    //     let env = builder.build().context("failed to build executor env")?;

    //     let exec = default_executor();
    //     exec.execute(env, DEBT_REPORTING_GUEST_ELF)
    //         .context("failed to run executor")?
    // };

    // // The journal should be the ABI encoded commitment.
    // let average_safe_price_commitment =
    //     AverageSafePriceCommitment::abi_decode(session_info.journal.as_ref())
    //         .context("failed to decode journal")?;

    // stub_post_commitment_on_chain(average_safe_price_commitment);

    // Ok(())
}

// let inputs: Vec<_> = futures::stream::iter(blocks.iter().cloned())
// .map(|block_num| {
//     let provider = provider.clone();
//     let call = call.clone();
//     async move {
//         let mut env = EthEvmEnv::builder()
//             .provider(provider)
//             .block_number(block_num)
//             .chain_spec(&ETH_MAINNET_CHAIN_SPEC)
//             .build()
//             .await?;

//         let mut contract = Contract::preflight(ROOT_PRICE_ORACLE, &mut env);
//         // we don't care about the output here just that we feteched the data
//         // might be helpful for debugging
//         let _ = contract.call_builder(&call).call().await?;
//         let input = env.into_input().await?;
//         println!("did a block! {block_num}");
//         Ok::<_, anyhow::Error>(input)
//     }
// })
// .buffered(8) // at most 8 concurrent
// .try_collect() // Vec<_>, same order as `blocks`
// .await?;
