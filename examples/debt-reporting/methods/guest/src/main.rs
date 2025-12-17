// Copyright 2025 RISC Zero, Inc.//
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

#![allow(unused_doc_comments)]
#![no_main]
#![no_std]

extern crate alloc;

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec::Vec;


// use alloy_primitives::{U256};
// use alloy_sol_types::SolValue;
use alloy_primitives::{Address, U256};
use alloy_sol_types::SolValue;

use risc0_steel::{
    ethereum::{EthEvmInput, ETH_MAINNET_CHAIN_SPEC},
    Contract,
    // Commitment,
};
use debt_reporting_abi::{
    AutopoolAddressConstants, DestinationVaultKey, IMinimalAutoPool, IMinimalDestinationVault,
    IMinimalSystemRegistry,  IMinimalRootPriceOracle, DestinationsZKPricesCommitment, ComputedGetRangePriceLP
};
use risc0_zkvm::guest::env;


risc0_zkvm::guest::entry!(main);

// if any of the spot prices are not safe it is not safe
// otherwise average the spot prices

/*
Because of Fulu upgrade on consensus just getting added to steel, this just validates the last part 
not that every single call came from that block. wait for the latest version of steel to become the stable release
before using in production, other wise there are no garentuees that the prior blocks are the blocks before the top block 
on mainnet.

they are just valid blocks, of (any valid evm) possible at a different block or at a different state

*/

fn main() {

    let autopool: Address = env::read();
    let several_inputs: Vec<EthEvmInput> = env::read();
    let num_blocks_sampled = several_inputs.len() as u64;
    let mut iter = several_inputs.into_iter();
    let first_input: EthEvmInput = iter.next().expect("need at least one EthEvmInput");
    let env = first_input.into_env(&ETH_MAINNET_CHAIN_SPEC);

    let mut latest_block_safe_prices: BTreeMap<DestinationVaultKey, U256> = BTreeMap::new();
    let mut unsafe_spot_prices_destinations: BTreeSet<DestinationVaultKey> = BTreeSet::new();
    let mut all_spot_prices_instances: Vec<(DestinationVaultKey, U256)> = Vec::new();


    // I would put this into a seperate function if I could, 
    // but there are issues with telling the complier what env type is so this will have to work for now 
    // I think you can have seperate envs with the same block and combine them later? can use threads?

    let autopool_constants = {    
        let autopool_contract = Contract::new(autopool, &env);
        
        let destination_vaults: Vec<Address> = autopool_contract
            .call_builder(&IMinimalAutoPool::getDestinationsCall {})
            .call();
        let base_asset: Address = autopool_contract
            .call_builder(&IMinimalAutoPool::assetCall {})
            .call();
        let system_registry: Address = autopool_contract
            .call_builder(&IMinimalAutoPool::getSystemRegistryCall {})
            .call();

        let system_registry_contract = Contract::new(system_registry, &env);

        let root_price_oracle: Address = system_registry_contract
            .call_builder(&IMinimalSystemRegistry::rootPriceOracleCall {})
            .call();

        let mut destination_vault_keys: Vec<DestinationVaultKey> =
            Vec::with_capacity(destination_vaults.len());

        for dv in destination_vaults {
            let destination_vault_contract = Contract::new(dv, &env);

            let token: Address = destination_vault_contract
                .call_builder(&IMinimalDestinationVault::underlyingCall {})
                .call();

            let pool: Address = destination_vault_contract
                .call_builder(&IMinimalDestinationVault::getPoolCall {})
                .call();

            let key = DestinationVaultKey {
                token: token,
                pool: pool,
                baseAsset: base_asset,
                destinationVault: dv,
            };

            destination_vault_keys.push(key.clone());
            
            // add the latest spot price tuple
            let get_range_prices_lp_call: IMinimalRootPriceOracle::getRangePricesLPCall =
                IMinimalRootPriceOracle::getRangePricesLPCall {
                    lpToken: key.token,
                    pool: key.pool,
                    quoteToken: key.baseAsset,
                };

            let root_price_oracle_contract = Contract::new(root_price_oracle, &env);

            let (spot_price_in_quote, safe_price_in_quote, is_spot_safe): (U256, U256, bool) =
                root_price_oracle_contract
                    .call_builder(&get_range_prices_lp_call)
                    .call()
                    .into();

            if !is_spot_safe {
                unsafe_spot_prices_destinations.insert(key.clone());
            }

            latest_block_safe_prices.insert(key.clone(), safe_price_in_quote);
            all_spot_prices_instances.push((key.clone(), spot_price_in_quote));
        }

        AutopoolAddressConstants {
            autopool: autopool,
            systemRegistry: system_registry,
            rootPriceOracle: root_price_oracle,
            baseAsset: base_asset,
            destinationVaultKeys: destination_vault_keys,
        }


    };
    
    for prior_block_input in iter {
        let prior_block_env = prior_block_input.into_env(&ETH_MAINNET_CHAIN_SPEC); 

        // need to connect the envs here
        // TODO we would connect this to the prior env but can't yet due to fulu upgrade not being
        // supported in the current stable steel version. 
        // as of Dec 15th it is supported just not yet in the stable

        for key in &autopool_constants.destinationVaultKeys {
            let get_range_prices_lp_call: IMinimalRootPriceOracle::getRangePricesLPCall =
                IMinimalRootPriceOracle::getRangePricesLPCall {
                    lpToken: key.token,
                    pool: key.pool,
                    quoteToken: key.baseAsset,
                };

            let root_price_oracle_contract = Contract::new(autopool_constants.rootPriceOracle.clone(), &prior_block_env);
            let (spot_price_in_quote, _safe_price_in_quote, is_spot_safe): (U256, U256, bool) =
                root_price_oracle_contract
                    .call_builder(&get_range_prices_lp_call)
                    .call()
                    .into();

            if !is_spot_safe {
                unsafe_spot_prices_destinations.insert(key.clone());
            }
            all_spot_prices_instances.push((key.clone(), spot_price_in_quote));
        }
    }

    let key_to_average_spot_price = compute_average_spot_price_by_destination_vault(all_spot_prices_instances, num_blocks_sampled);
    let mut price_info: Vec<(DestinationVaultKey, ComputedGetRangePriceLP)> = Vec::with_capacity(autopool_constants.destinationVaultKeys.len());

    for key in &autopool_constants.destinationVaultKeys {

        let avg_spot: U256 = *key_to_average_spot_price
        .get(&key)
        .expect("missing avg spot price for key");

        let latest_safe: U256 = *latest_block_safe_prices
            .get(&key)
            .expect("missing latest safe price for key");

        price_info.push((
            key.clone(),
            ComputedGetRangePriceLP {
                averageSpotPriceInQuote: avg_spot,
                latestSafePriceInQuote: latest_safe,
                isSpotSafeZK: unsafe_spot_prices_destinations.contains(&key),
            },
        ));


    }
  
    let journal = DestinationsZKPricesCommitment {
        commitment: env.into_commitment(),
        autopoolConstants: autopool_constants,
        priceInfo: price_info
    };

    env::commit_slice(&journal.abi_encode());
}


fn compute_average_spot_price_by_destination_vault(
    all_spot_prices_instances: Vec<(DestinationVaultKey, U256)>,
    expected_count_per_key: u64,
) -> BTreeMap<DestinationVaultKey, U256> {
    // same concept as .groupby(destination_vault_key)[spot_price].avg() -> dict[key] : average spot price

    let expected_count_per_key_u256: U256 = U256::from(expected_count_per_key);
    let mut running_sum_by_key: BTreeMap<DestinationVaultKey, U256> = BTreeMap::new(); 

    for (destination_vault_key, spot_price_value) in all_spot_prices_instances {
        // Get the slot for this key (create it with 0 if it's new).
        let running_sum_for_key: &mut U256 = running_sum_by_key
            .entry(destination_vault_key)
            .or_insert(U256::ZERO);

        // * means the actual value, not the reference, this is the same concept as ``` some_dict[key] += some_value ```  # in python
        *running_sum_for_key += spot_price_value; 
    }

    let average_price_by_key : Vec<(DestinationVaultKey, U256)> = running_sum_by_key
        .into_iter()
        .map(|(destination_vault_key, running_sum_u256)| {
            let average_price_u256: U256 = running_sum_u256 / expected_count_per_key_u256;
            (destination_vault_key, average_price_u256)
        })
        .collect();

    let mut key_to_average_price = BTreeMap::new();

    for (key, average_price) in average_price_by_key {
        key_to_average_price.insert(key, average_price);
    }
    key_to_average_price
}