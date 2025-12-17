mod preflight;
use preflight::{preflight_autopool_constants_and_prices, preflight_prices_calls};

mod push_prices_onchain;
use push_prices_onchain::stub_post_commitment_onchain;

use alloy_primitives::{address, Address};
use anyhow::{Context, Result};

use alloy_sol_types::SolValue;
use clap::Parser;
use debt_reporting_abi::DestinationsZKPricesCommitment;
use debt_reporting_methods::DEBT_REPORTING_GUEST_ELF;
use risc0_steel::alloy::providers::Provider;
use risc0_steel::alloy::providers::{ProviderBuilder, RootProvider};
use risc0_steel::ethereum::EthEvmInput;
use risc0_zkvm::{default_executor, ExecutorEnv};

use tracing_subscriber::EnvFilter;
use url::Url;

use std::time::Instant;

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
    // minimal main for imports
    println!("Starting debt reporting host");
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    let provider: RootProvider = ProviderBuilder::default().connect_http(args.rpc_url);
    let autopool: Address = address!("0x0A2b94F6871c1D7A32Fe58E1ab5e6deA2f114E56"); // autoETH
                                                                                    // let autopool: Address = address!("0xa7569A44f348d3D70d8ad5889e50F78E33d80D35"); // autoUSD
    let latest = provider.get_block_number().await?;

    println!("Starting Address Preflight");
    let t = Instant::now();
    let (latest_input, autopool_constants) =
        preflight_autopool_constants_and_prices(autopool, provider.clone(), latest).await?;
    println!("Finished Address Preflight in {:?}", t.elapsed());

    let mut inputs_as_vector: Vec<EthEvmInput> = Vec::with_capacity(3);

    inputs_as_vector.push(latest_input);

    let historical_blocks = vec![latest - 2, latest - 1];

    for block in historical_blocks {
        let t = Instant::now();
        println!("Starting Prices Preflight for block {block:?}");
        let just_prices_input =
            preflight_prices_calls(&autopool_constants, provider.clone(), block).await?;
        inputs_as_vector.push(just_prices_input);
        println!(
            "Finished Prices Preflight for block {block:?} in {:?}",
            t.elapsed()
        );
    }

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

    stub_post_commitment_onchain(destinations_zk_prices_commitment);

    Ok(())
}
