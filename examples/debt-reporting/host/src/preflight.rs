// host/src/preflight_fetch_contract_addresses.rs

use anyhow::Result;
use std::sync::Arc;
use tokio::{sync::Semaphore, task::JoinSet};

use alloy_primitives::{Address, U256};
use debt_reporting_abi::{
    AutopoolAddressConstants, DestinationVaultKey, IMinimalAutoPool, IMinimalDestinationVault,
    IMinimalRootPriceOracle, IMinimalSystemRegistry,
};

use risc0_steel::{
    alloy::providers::RootProvider,
    ethereum::{EthEvmEnv, EthEvmInput, ETH_MAINNET_CHAIN_SPEC},
    Contract,
};

// TODO all the loops should be done in threads, order does not matter
// very slow other wise

pub async fn preflight_prices_calls(
    autopool_constants: &AutopoolAddressConstants,
    provider: RootProvider,
    block: u64,
) -> Result<EthEvmInput> {
    let mut env = EthEvmEnv::builder()
        .provider(provider)
        .block_number(block)
        .chain_spec(&ETH_MAINNET_CHAIN_SPEC)
        .build()
        .await?;

    for key in &autopool_constants.destinationVaultKeys {
        {
            let get_range_prices_lp_call: IMinimalRootPriceOracle::getRangePricesLPCall =
                IMinimalRootPriceOracle::getRangePricesLPCall {
                    lpToken: key.token,
                    pool: key.pool,
                    quoteToken: key.baseAsset,
                };

            let mut root_price_oracle_contract =
                Contract::preflight(autopool_constants.rootPriceOracle, &mut env);

            let (_spot_price_in_quote, _safe_price_in_quote, _is_spot_safe): (U256, U256, bool) =
                root_price_oracle_contract
                    .call_builder(&get_range_prices_lp_call)
                    .call()
                    .await?
                    .into();
        };
    }
    let input = env.into_input().await?;
    Ok(input)
}

// just use for the latest block
pub async fn preflight_autopool_constants_and_prices(
    autopool: Address,
    provider: RootProvider,
    block: u64,
) -> Result<(EthEvmInput, AutopoolAddressConstants)> {
    let mut env = EthEvmEnv::builder()
        .provider(provider)
        .block_number(block)
        .chain_spec(&ETH_MAINNET_CHAIN_SPEC)
        .build()
        .await?;

    let mut autopool_contract = Contract::preflight(autopool, &mut env);

    let destination_vaults: Vec<Address> = autopool_contract
        .call_builder(&IMinimalAutoPool::getDestinationsCall {})
        .call()
        .await?;

    let base_asset: Address = autopool_contract
        .call_builder(&IMinimalAutoPool::assetCall {})
        .call()
        .await?;

    let system_registry: Address = autopool_contract
        .call_builder(&IMinimalAutoPool::getSystemRegistryCall {})
        .call()
        .await?;

    let mut system_registry_contract = Contract::preflight(system_registry, &mut env);

    let root_price_oracle: Address = system_registry_contract
        .call_builder(&IMinimalSystemRegistry::rootPriceOracleCall {})
        .call()
        .await?;

    let mut destination_vault_keys: Vec<DestinationVaultKey> =
        Vec::with_capacity(destination_vaults.len());

    for dv in destination_vaults {
        // weird things with mutable borrows so we have to make it be in nested scopes like this
        let (token, pool) = {
            let mut destination_vault_contract = Contract::preflight(dv, &mut env);

            let token: Address = destination_vault_contract
                .call_builder(&IMinimalDestinationVault::underlyingCall {})
                .call()
                .await?;

            let pool: Address = destination_vault_contract
                .call_builder(&IMinimalDestinationVault::getPoolCall {})
                .call()
                .await?;

            (token, pool)
        };

        {
            let get_range_prices_lp_call: IMinimalRootPriceOracle::getRangePricesLPCall =
                IMinimalRootPriceOracle::getRangePricesLPCall {
                    lpToken: token,
                    pool: pool,
                    quoteToken: base_asset,
                };

            let mut root_price_oracle_contract = Contract::preflight(root_price_oracle, &mut env);

            // get it working then make it not redundent
            // make the call but we don't use it here
            // just so that we can use this later
            let (_spot_price_in_quote, _safe_price_in_quote, _is_spot_safe): (U256, U256, bool) =
                root_price_oracle_contract
                    .call_builder(&get_range_prices_lp_call)
                    .call()
                    .await?
                    .into();
        };

        destination_vault_keys.push(DestinationVaultKey {
            token: token,
            pool: pool,
            baseAsset: base_asset,
            destinationVault: dv,
        });
    }

    let autopool_address_constants = AutopoolAddressConstants {
        autopool: autopool,
        systemRegistry: system_registry,
        rootPriceOracle: root_price_oracle,
        baseAsset: base_asset,
        destinationVaultKeys: destination_vault_keys,
    };

    // Or print specific fields (works even without Debug on the whole struct):
    println!("Inside of helper in the host!");

    println!("autopool: {:?}", autopool_address_constants.autopool);
    println!(
        "systemRegistry: {:?}",
        autopool_address_constants.systemRegistry
    );
    println!(
        "rootPriceOracle: {:?}",
        autopool_address_constants.rootPriceOracle
    );
    println!("baseAsset: {:?}", autopool_address_constants.baseAsset);
    println!(
        "destinationVaultKeys len: {}",
        autopool_address_constants.destinationVaultKeys.len()
    );

    let input = env.into_input().await?;
    Ok((input, autopool_address_constants))
}
