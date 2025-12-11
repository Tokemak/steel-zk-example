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


use anyhow::{Context, Result};
use clap::Parser;
use alloy_primitives::{Address};
use debt_reporting_abi::{
    AverageSafePriceCommitment, IRootPriceOracle, DestinationVaultKey, ComputedGetRangePriceLP, 
    AutopoolAddressConstants,
        IMinimalSystemRegistry, IMinimalAutoPool, IMinimalRootPriceOracle, IMinimalDestinationVault
};
use futures::{StreamExt, TryStreamExt};
use risc0_steel::{
    ethereum::{EthEvmEnv, ETH_MAINNET_CHAIN_SPEC},
    Contract,
};


pub async fn fetch_constants_for_autopool(autopool: Address, most_recent_env: &mut EthEvmEnv) -> Result<AutopoolAddressConstants> {
    let mut autopool_contract = Contract::preflight(autopool, most_recent_env);

    let base_asset: Address = autopool_contract.call_builder(&IMinimalAutoPool::assetCall{}).call().await?;
    let system_registry:Address = autopool_contract.call_builder(&IMinimalAutoPool::getSystemRegistryCall{}).call().await?;
    let destination_vaults: Vec<Address> = autopool_contract.call_builder(&IMinimalAutoPool::getDestinationsCall{}).call().await?;

    let mut system_registry_contract = Contract::preflight(system_registry, &mut most_recent_env);
    let root_price_oracle: Address = system_registry_contract.call_builder(&IMinimalSystemRegistry::rootPriceOracleCall{}).call().await?;

    let mut destination_vault_keys: Vec<DestinationVaultKey> =  Vec::with_capacity(destination_vaults.len());

    // consider use futures::{StreamExt, TryStreamExt};
    // consider streaming theses instead since it might take a bit otherwise
    for dv in destination_vaults {
        let key = fetch_constants_for_destination_vault(dv, base_asset, most_recent_env).await?;
        destination_vault_keys.push(key);
    }

    Ok(
        AutopoolAddressConstants{
            autopool:autopool,
            systemRegistry:system_registry,
            rootPriceOracle:root_price_oracle,
            baseAsset: base_asset,
            destinationVaultKeys:destination_vault_keys
        }
    )
}

// internal to this file only
async fn fetch_constants_for_destination_vault(destination_vault: Address, base_asset:Address, most_recent_env: &mut  EthEvmEnv) -> Result<DestinationVaultKey> {
    let mut destination_vault_contract = Contract::preflight(destination_vault, most_recent_env);

    let token: Address = destination_vault_contract.call_builder(&IMinimalDestinationVault::underlyingCall{}).call().await?;
    let pool: Address = destination_vault_contract.call_builder(&IMinimalDestinationVault::getPoolCall{}).call().await?;

    let destination_vault_key = DestinationVaultKey {
        token: token,
        pool: pool,
        baseAsset: base_asset,
        destinationVault: destination_vault,
    };

    Ok(destination_vault_key)
}

