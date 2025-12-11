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



// todo
// pass in an autopool vault address
// outside of the zk
// get the destinations of that autopool
// for each destination in autopool.getDestinations():
//    for each block in [10 recent blocks]:
//      _ = RootPriceOracle.getRangePriceLP(destination.lp_token, destination.pool, autopool.baseAsset)?

// do the same in the guest but average all the safe prices for lp token and pool
//


use alloy_sol_types::SolType;
use anyhow::{Context, Result};
use clap::Parser;
use debt_reporting_abi::{
    AverageSafePriceCommitment, IRootPriceOracle, A_LP_TOKEN, ROOT_PRICE_ORACLE, USDC_MAINNET,
};
use debt_reporting_methods::DEBT_REPORTING_GUEST_ELF; // not sure what this is and what it is used for
use futures::{StreamExt, TryStreamExt};
use risc0_steel::{
    alloy::providers::{Provider, ProviderBuilder, RootProvider},
    ethereum::{EthEvmEnv, ETH_MAINNET_CHAIN_SPEC},
    Contract,
};
use risc0_zkvm::{default_executor, ExecutorEnv};
use tracing_subscriber::EnvFilter;
use url::Url;

/// Simple program to show the use of Ethereum contract data inside the guest.
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[arg(short, long, env = "RPC_URL")]
    rpc_url: Url,
    #[arg(long, env = "BEACON_API_URL")]
    beacon_api_url: Url,
}

fn stub_post_commitment_on_chain(average_safe_price_commitment: AverageSafePriceCommitment) {
    /*
    After this, we should push received price data
    (along with the proof in a transient storage)
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

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting Debt Reporting example...");
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    let provider: RootProvider = ProviderBuilder::default().connect_http(args.rpc_url);
    let latest = provider.get_block_number().await?;
    let mut blocks = Vec::with_capacity(10);

    let call: IRootPriceOracle::getRangePricesLPCall = IRootPriceOracle::getRangePricesLPCall {
        lpToken: A_LP_TOKEN,
        pool: A_LP_TOKEN,
        quoteToken: USDC_MAINNET,
    };

    for i in 0..10 {
        blocks.push(latest - (i * 2));
    }

    let inputs: Vec<_> = futures::stream::iter(blocks.iter().cloned())
        .map(|block_num| {
            let provider = provider.clone();
            let call = call.clone();
            async move {
                let mut env = EthEvmEnv::builder()
                    .provider(provider)
                    .block_number(block_num)
                    .chain_spec(&ETH_MAINNET_CHAIN_SPEC)
                    .build()
                    .await?;

                let mut contract = Contract::preflight(ROOT_PRICE_ORACLE, &mut env);
                // we don't care about the output here just that we feteched the data
                // might be helpful for debugging
                let _ = contract.call_builder(&call).call().await?;
                let input = env.into_input().await?;
                println!("did a block! {block_num}");
                Ok::<_, anyhow::Error>(input)
            }
        })
        .buffered(8) // at most 8 concurrent
        .try_collect() // Vec<_>, same order as `blocks`
        .await?;

    println!("Running Debt Reporting in the guest for {blocks:?}");

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

    stub_post_commitment_on_chain(average_safe_price_commitment);

    Ok(())
}
