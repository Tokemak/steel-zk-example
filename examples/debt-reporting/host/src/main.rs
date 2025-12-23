mod preflight;
use preflight::{preflight_autopool_constants, preflight_prices};

mod push_prices_onchain;
use push_prices_onchain::stub_post_commitment_onchain;

use debt_reporting_abi::ChainAddressConstants;

use alloy_primitives::{address};
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

fn stub_read_cli_args() -> ChainAddressConstants {
    let systemRegistry = address!("0x2218f90a98b0c070676f249ef44834686daa4285");
    let rootPriceOracle = address!("0x61f8be7fd721e80c0249829eae6f0daf21bc2cac");
    let multicall3 = address!("0xcA11bde05977b3631167028862bE2a173976CA11");

    let autopools = vec![
        address!("0x0A2b94F6871c1D7A32Fe58E1ab5e6deA2f114E56"), // autoETH
        address!("0xa7569A44f348d3D70d8ad5889e50F78E33d80D35"), // autoUSD
        address!("0x1ABD0403591bE494771115d74ED9E120530f356E"), // anchrgUSD
        address!("0x79eB84B5E30Ef2481c8f00fD0Aa7aAd6Ac0AA54d"), // autoDOLA
    ];

    ChainAddressConstants {
        multicall3,
        systemRegistry,
        rootPriceOracle,
        autopools,
    }
}

// fn _stub_pseudo_random_block_selection() -> Vec<u64> {
//     // some function that takes in teh block hash of latest block, and uses that to
//     // select 3 of the prior blocks in the last minute
//     // not essential but could be helpful to make it less exploitable

//     let historical_blocks: Vec<u64> = (0..3).map(|i| latest - i).collect();
//     historical_blocks
// }

#[tokio::main]
async fn main() -> Result<()> {
    let full_start = Instant::now();
    println!("Starting Debt Reporting Host");
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let args = Args::parse();
    let chain_address_constants: ChainAddressConstants = stub_read_cli_args();
    let provider: RootProvider = ProviderBuilder::default().connect_http(args.rpc_url);

    let latest = provider.get_block_number().await?;
    println!("Starting Destination Vault Keys Preflight");
    let t = Instant::now();
    let (destination_vault_keys_input, destination_vault_keys) =
        preflight_autopool_constants(chain_address_constants.clone(), provider.clone(), latest)
            .await?;

    println!(
        "Finished Destination Vault Keys Preflight in {:?} found {:?} unique destinations",
        t.elapsed(),
        (destination_vault_keys.len() as u64)
    );
    let input_vector = {
        let historical_blocks: Vec<u64> =  (0..3).map(|i| latest - i).collect();
        let mut set = JoinSet::new();
        let t = Instant::now();

        let destination_vault_keys = Arc::new(destination_vault_keys); // needed to use in threads, not sure why
        println!(
            "Starting {:?} blocks Prices Preflight",
            (historical_blocks.len() as u64)
        );

        for block in historical_blocks {
            let provider = provider.clone();
            let chain_address_constants = chain_address_constants.clone();
            let destination_vault_keys = destination_vault_keys.clone().to_vec();
            set.spawn(async move {
                let input = preflight_prices(
                    chain_address_constants,
                    destination_vault_keys,
                    provider,
                    block,
                )
                .await?;
                Ok::<EthEvmInput, anyhow::Error>(input)
            });
        }

        let mut input_vector = Vec::new();
        input_vector.push(destination_vault_keys_input);

        while let Some(res) = set.join_next().await {
            let prices_input = res??;
            input_vector.push(prices_input);
        }
        println!("Finished Prices Preflight in {:?}", t.elapsed());
    };



    let destinations_zk_prices_commitment = {
        println!("Starting Guest!");
        let t = Instant::now();
        let session_info = {
            let mut builder = ExecutorEnv::builder();
            builder
                .write(&chain_address_constants)
                .context("Failed to write autopool Address")?;

            builder
                .write(&input_vector)
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
