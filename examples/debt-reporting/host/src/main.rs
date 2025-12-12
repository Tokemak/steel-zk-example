mod preflight_fetch_contract_addresses;
use preflight_fetch_contract_addresses::fetch_and_print_base_asset;

use alloy_primitives::{address, Address};
use anyhow::Result;
use clap::Parser;
use risc0_steel::alloy::providers::{ProviderBuilder, RootProvider};

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
    // minimal main for imports
    println!("Starting minimal imports example...");
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    let provider: RootProvider = ProviderBuilder::default().connect_http(args.rpc_url);
    let autopool: Address = address!("0x0A2b94F6871c1D7A32Fe58E1ab5e6deA2f114E56"); // autoETH
    let _base_asset = fetch_and_print_base_asset(autopool, provider.clone(), 20000000).await?;

    Ok(())
}

// #[tokio::main]
// async fn main() -> Result<()> {
//     println!("Starting Debt Reporting example...");
//     tracing_subscriber::fmt()
//         .with_env_filter(EnvFilter::from_default_env())
//         .init();

//     let args = Args::parse();
//     let provider: RootProvider = ProviderBuilder::default().connect_http(args.rpc_url);

//     let latest = provider.get_block_number().await?;
//     let mut blocks = Vec::with_capacity(3);
//     for i in 0..3 {
//         blocks.push(latest - (i * 2));
//     }

//     // IMPORTANT: store them as HostEvmEnv
//     let mut envs: Vec<EthEvmEnv<_, _>> = Vec::with_capacity(blocks.len());

//     for block in &blocks {
//         let provider = provider.clone();
//         let env = EthEvmEnv::builder()
//             .provider(provider)
//             .block_number(*block)
//             .chain_spec(&ETH_MAINNET_CHAIN_SPEC)
//             .build()
//             .await?;
//         envs.push(env);
//     }

//     // We want the newest env (associated with the largest block)
//     let most_recent_env = envs
//         .iter_mut()
//         .max_by_key(|env| {
//             // TODO: pick whatever way Steel exposes the block number, e.g. env.block_number()
//             // For now stub 0 to satisfy the type. Replace with real accessor.
//             0u64
//         })
//         .expect("at least one env");

//     let autopool: Address = address!("0x0A2b94F6871c1D7A32Fe58E1ab5e6deA2f114E56"); // autoETH

//     let autopool_address_constants: AutopoolAddressConstants =
//         fetch_constants_for_autopool(autopool, most_recent_env).await?;

//     // Now you can reuse the SAME envs to preflight getRangePricesLP
//     preflight_getRangePricesLP::preflight_get_range_prices_lp(
//         autopool_address_constants.baseAsset,
//         autopool_address_constants.rootPriceOracle,
//         &autopool_address_constants.destinationVaultKeys,
//         &mut envs,
//     )
//     .await?;

//     println!("Address preflight Success!!!");
//     Ok(())
// }
