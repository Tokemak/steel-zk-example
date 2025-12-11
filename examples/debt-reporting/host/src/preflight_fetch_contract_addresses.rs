// host/src/preflight_fetch_contract_addresses.rs

use alloy_primitives::Address;
use anyhow::Result;
use debt_reporting_abi::{
    AutopoolAddressConstants, DestinationVaultKey, IMinimalAutoPool, IMinimalDestinationVault,
    IMinimalSystemRegistry,
};
use risc0_steel::{ethereum::HostEvmEnv, Contract};

/// Fetch all the "constant" addresses associated with an Autopool.
///
/// Generic over the INTERNAL type params of the host env, but the env
/// itself is always a HostEvmEnv<_, _, _>.
pub async fn fetch_constants_for_autopool<D, F, C>(
    autopool: Address,
    env: &mut HostEvmEnv<D, F, C>,
) -> Result<AutopoolAddressConstants> {
    let mut autopool_contract = Contract::preflight(autopool, env);

    let base_asset: Address = autopool_contract
        .call_builder(&IMinimalAutoPool::assetCall {})
        .call()
        .await?;

    let system_registry: Address = autopool_contract
        .call_builder(&IMinimalAutoPool::getSystemRegistryCall {})
        .call()
        .await?;

    let destination_vaults: Vec<Address> = autopool_contract
        .call_builder(&IMinimalAutoPool::getDestinationsCall {})
        .call()
        .await?;

    let mut system_registry_contract = Contract::preflight(system_registry, env);
    let root_price_oracle: Address = system_registry_contract
        .call_builder(&IMinimalSystemRegistry::rootPriceOracleCall {})
        .call()
        .await?;

    let mut destination_vault_keys: Vec<DestinationVaultKey> =
        Vec::with_capacity(destination_vaults.len());

    for dv in destination_vaults {
        let key = fetch_constants_for_destination_vault(dv, base_asset, env).await?;
        destination_vault_keys.push(key);
    }

    Ok(AutopoolAddressConstants {
        autopool,
        systemRegistry: system_registry,
        rootPriceOracle: root_price_oracle,
        baseAsset: base_asset,
        destinationVaultKeys: destination_vault_keys,
    })
}

async fn fetch_constants_for_destination_vault<D, F, C>(
    destination_vault: Address,
    base_asset: Address,
    env: &mut HostEvmEnv<D, F, C>,
) -> Result<DestinationVaultKey> {
    let mut destination_vault_contract = Contract::preflight(destination_vault, env);

    let token: Address = destination_vault_contract
        .call_builder(&IMinimalDestinationVault::underlyingCall {})
        .call()
        .await?;

    let pool: Address = destination_vault_contract
        .call_builder(&IMinimalDestinationVault::getPoolCall {})
        .call()
        .await?;

    Ok(DestinationVaultKey {
        token,
        pool,
        baseAsset: base_asset,
        destinationVault: destination_vault,
    })
}
