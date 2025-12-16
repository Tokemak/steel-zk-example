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
    IMinimalSystemRegistry,  IMinimalRootPriceOracle, AutopoolAddressConstantsCommitment
};
use risc0_zkvm::guest::env; // DestinationsZKPricesCommitment

risc0_zkvm::guest::entry!(main);

// todo offcahin is spot price safe check

/*

NOTE

Because of Fulu upgrade on consensus just getting added to steel, this just validates the last part 
not that every single call came from that block. wait for the latest version of steel to become the stable release
before using in production, other wise there are no garentuees that the prior blocks are the blocks before the top block 
on mainnet.

they are just valid blocks, of some other EVM, at some other point point

it seems the host part of the progarm is the expensive part


*/




fn main() {

    let autopool: Address = env::read();
    let several_inputs: Vec<EthEvmInput> = env::read();
    let _num_blocks_sampled = several_inputs.len();

    let mut iter = several_inputs.into_iter();
    let first_input: EthEvmInput = iter.next().expect("need at least one EthEvmInput");
    let env = first_input.into_env(&ETH_MAINNET_CHAIN_SPEC);

    let mut latest_safe_price_tuples: Vec<(DestinationVaultKey, U256)> =  Vec::new(); // size does not matter
    // we do a groupby avg later this is just a list of tuples of (DestinationVaultKey, U256) of spot price of the destination vault
    let mut all_spot_prices_instances: Vec<(DestinationVaultKey, U256)> =  Vec::new();

    // I would put this into a seperate function if I could, 
    // but there are issues with telling the complier what env type is so this will have to work for now 
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

            let (spot_price_in_quote, safe_price_in_quote, _is_spot_safe): (U256, U256, bool) =
                root_price_oracle_contract
                    .call_builder(&get_range_prices_lp_call)
                    .call()
                    .into();
                


            latest_safe_price_tuples.push((key.clone(), safe_price_in_quote));
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

        let prior_block_env = prior_block_input.into_env(&ETH_MAINNET_CHAIN_SPEC); // need to connect the envs here
        for key in &autopool_constants.destinationVaultKeys {
            // let key = DestinationVaultKey {
            //     // token: token,
            //     // pool: pool,
            //     // baseAsset: base_asset,
            //     destinationVault: dv,
            // };

            let get_range_prices_lp_call: IMinimalRootPriceOracle::getRangePricesLPCall =
                IMinimalRootPriceOracle::getRangePricesLPCall {
                    lpToken: key.token,
                    pool: key.pool,
                    quoteToken: key.baseAsset,
                };

            let root_price_oracle_contract = Contract::new(autopool_constants.rootPriceOracle.clone(), &prior_block_env);

            let (spot_price_in_quote, _safe_price_in_quote, _is_spot_safe): (U256, U256, bool) =
                root_price_oracle_contract
                    .call_builder(&get_range_prices_lp_call)
                    .call()
                    .into();
                
            all_spot_prices_instances.push((key.clone(), spot_price_in_quote));
        }

    }
            


    // latest_safe_price_tuples.push((key, safe_price_in_quote));
    // all_spot_prices_instances.push((key, spot_price_in_quote));

    
   
    // let journal = DestinationsZKPricesCommitment {
    //     commitment: env.into_commitment(),
    //     autopoolConstants: autopool_constants,
    //     priceInfo: // (DestinationVaultKey, ComputedGetRangePriceLP)[]
    //     // missing commit ment here of destiatnion 
    // };

    let journal = AutopoolAddressConstantsCommitment {
        commitment: env.into_commitment(),
        autopoolConstants: autopool_constants,
    };

    env::commit_slice(&journal.abi_encode());
}

// fn main() {

//     println!("{base_asset:?} found in helper on host");
//     // let blocks: Vec<u64>= env::read();
//     // // let blocks: Vec<U256> = blocks.into_iter().map(U256::from).collect();

//     // let mut ethereum_envs: Vec<EthEvmInput> = env::read();
//     // let call: IRootPriceOracle::getRangePricesLPCall = IRootPriceOracle::getRangePricesLPCall {
//     //     lpToken: A_LP_TOKEN,
//     //     pool: A_LP_TOKEN,
//     //     quoteToken: USDC_MAINNET,
//     // };

//     // let mut safe_prices: Vec<U256> = Vec::with_capacity(blocks.len());
//     // let mut previous_execution_environment: Option<EthEvmEnv<_, Commitment>> = None;

//     // for (_block_number, ethereum_input) in blocks.iter().zip(ethereum_envs.iter_mut()) {
//     //     let current_execution_environment = ethereum_input.clone().into_env(&ETH_MAINNET_CHAIN_SPEC);
//     //     let root_price_oracle_contract =
//     //         Contract::new(ROOT_PRICE_ORACLE, &current_execution_environment);

//     //     let (_spot_price_in_quote, safe_price_in_quote, _is_spot_safe): (U256, U256, bool) =
//     //         root_price_oracle_contract
//     //             .call_builder(&call)
//     //             .call()
//     //             .into();

//     //     safe_prices.push(safe_price_in_quote);

//     //     previous_execution_environment = Some(current_execution_environment);
//     // }

//     // let sum: U256 = safe_prices.iter().copied().sum();
//     // let denom = U256::from(safe_prices.len() as u64);
//     // let average_safe_price: U256 = sum / denom;

//     // let blocks: Vec<U256> = blocks.into_iter().map(U256::from).collect();

//     // let last_env = previous_execution_environment.expect("There should be an environment here");

//     // let journal = AverageSafePriceCommitment {
//     //     commitment: last_env.into_commitment(),
//     //     priceInfo: vec![(A_LP_TOKEN, average_safe_price)],
//     //     blocks: blocks,
//     // };
    

//     // env::commit_slice(&journal.abi_encode());

// }
