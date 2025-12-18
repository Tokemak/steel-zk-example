mod preflight;
use preflight::{preflight_autopool_constants, preflight_prices};

mod push_prices_onchain;
use push_prices_onchain::stub_post_commitment_onchain;

use alloy_primitives::{address, Address};
use alloy_sol_types::SolValue;
use anyhow::{Context, Result};
use clap::Parser;
use debt_reporting_abi::DestinationsZKPricesCommitment;
use debt_reporting_methods::DEBT_REPORTING_GUEST_ELF;
use risc0_steel::alloy::providers::Provider;
use risc0_steel::alloy::providers::{ProviderBuilder, RootProvider};
use risc0_steel::ethereum::EthEvmInput;
use risc0_zkvm::{default_executor, ExecutorEnv};
use std::sync::Arc;
use std::time::Instant;
use tokio::task::JoinSet;
use tracing_subscriber::EnvFilter;
use url::Url;

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
    let full_start = Instant::now();
    println!("Starting Debt Reporting Host");
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let args = Args::parse();
    let provider: RootProvider = ProviderBuilder::default().connect_http(args.rpc_url);
    // autoETH  note: wstETH destination is broken
    let autopool: Address = address!("0x0A2b94F6871c1D7A32Fe58E1ab5e6deA2f114E56");
    // let autopool: Address = address!("0xa7569A44f348d3D70d8ad5889e50F78E33d80D35"); // autoUSD
    let latest = provider.get_block_number().await?;
    println!("Starting Address Preflight");
    let t = Instant::now();
    let (autopool_constants_input, autopool_constants) =
        preflight_autopool_constants(autopool, provider.clone(), latest).await?;
    println!("Finished Address Preflight in {:?}", t.elapsed());
    let autopool_constants = Arc::new(autopool_constants);

    let historical_blocks: Vec<u64> = (0..100).map(|i| latest - i).collect();
    let mut set = JoinSet::new();
    let t = Instant::now();
    
    println!("Starting {:?} blocks Prices Preflight", ( historical_blocks.len() as u64));
    

    for block in historical_blocks {
        let provider = provider.clone();
        let autopool_constants = autopool_constants.clone();
        set.spawn(async move {
            let input = preflight_prices(&autopool_constants, provider, block).await?;
            Ok::<EthEvmInput, anyhow::Error>(input)

        });
    }

    let mut inputs_as_vector = Vec::new();
    inputs_as_vector.push(autopool_constants_input);
    while let Some(res) = set.join_next().await {
        let input = res??;
        inputs_as_vector.push(input);
        // 70ish seconds for autoETH per block
        // can put a progress bar here if inclined
    }

    println!("Finished Prices Preflight in {:?}", t.elapsed());

    let destinations_zk_prices_commitment = {
        println!("Starting Guest!");
        let t = Instant::now();
        let session_info = {
            let mut builder = ExecutorEnv::builder();

            builder
                .write(&autopool)
                .context("Failed to write autopool Address")?;

            builder
                .write(&inputs_as_vector)
                .context("Failed to write input envs")?;

            let env = builder.build().context("failed to build executor env")?;

            let exec = default_executor();
            exec.execute(env, DEBT_REPORTING_GUEST_ELF)
                .context("failed to run executor")?
        };
        println!("Guest execution took {:?}", t.elapsed());

        let destinations_zk_prices_commitment =
            DestinationsZKPricesCommitment::abi_decode(session_info.journal.as_ref())
                .context("failed to decode journal")?;
        destinations_zk_prices_commitment
    };

    stub_post_commitment_onchain(destinations_zk_prices_commitment);

    println!("End to end {:?}", full_start.elapsed());
    Ok(())
}
