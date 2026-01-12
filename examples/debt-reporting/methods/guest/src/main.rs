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
use alloc::collections::{BTreeMap, BTreeSet}; // required since we are in no_std
use alloc::vec::Vec;

use alloy_primitives::{
    aliases::{U112},
    ruint::UintTryFrom,
    Address, U256, B256
    keccak256,
};
use alloy_sol_types::SolValue;

use debt_reporting_abi::{
    ChainAddressConstants, DestinationVaultKey, DestinationsZKPricesCommitment, IMinimalAutoPool,
    IMinimalDestinationVault, IMinimalRootPriceOracle, PackedComputedGetRangePriceLP,
};
use risc0_steel::{
    ethereum::{EthEvmInput, ETH_MAINNET_CHAIN_SPEC},
    Contract,
};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

/*
Because of Fulu upgrade on consensus just getting added to steel, this just validateSs the last part
not that every single call came from that block. wait for the latest version of steel to become the stable release
before using in production, other wise there are no garentuees that the prior blocks are the blocks before the top block
on mainnet.

they are just valid blocks, of (any valid evm) possible at a block. they can be on different chains, or different blocks

So as is this does not connect the blocks (eg prove the blocks came after each other on the same chain)
TODO add that before pushing to production

// if any of the spot prices are not safe it is not safe
// write the average of the spot prices

there are issues with telling the complier the type of`some_input.into_env(&ETH_MAINNET_CHAIN_SPEC)`

*/

// fn revert_if_invalid_chain_address_constants(chain_address_constants: ChainAddressConstants ) {
//     // todo
//     // make sure that these addresses are right
//     println!("In Guest, placeholder to validate that the ChainAddressConstants are correct");
// }


fn main() {
    // TODO figure out how to pass an env to a seperate function for clarity
    let chain_address_constants: ChainAddressConstants = env::read();
    let preflighted_inputs: Vec<EthEvmInput> = env::read();
    // 2 envs for the first block, one was used to get all the destination addreses the other was used for prices
    let num_blocks_sampled = (preflighted_inputs.len() as u64) - 1;
    let mut preflighted_inputs_iter = preflighted_inputs.into_iter();
    let destination_vault_keys_input: EthEvmInput = preflighted_inputs_iter
        .next()
        .expect("need at least one EthEvmInput");

    let destination_vault_keys_env = destination_vault_keys_input.into_env(&ETH_MAINNET_CHAIN_SPEC);
    let constants_block_number = destination_vault_keys_env.header().number;

    let destination_vault_keys: BTreeSet<DestinationVaultKey> = {
        let mut destination_vault_keys: BTreeSet<DestinationVaultKey> = BTreeSet::new();

        for autopool in &chain_address_constants.autopools {
            let autopool_contract = Contract::new(*autopool, &destination_vault_keys_env);
            // might be fine consuming autopools here
            let destination_vaults: Vec<Address> = autopool_contract
                .call_builder(&IMinimalAutoPool::getDestinationsCall {})
                .call();

            let base_asset: Address = autopool_contract
                .call_builder(&IMinimalAutoPool::assetCall {})
                .call();

            for destination_vault in destination_vaults {
                let destination_vault_contract =
                    Contract::new(destination_vault, &destination_vault_keys_env);
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
                    destinationVault: destination_vault,
                };
                destination_vault_keys.insert(key.clone());
            }
        }
        destination_vault_keys
    };

    let (latest_block_safe_prices, unsafe_spot_prices_destinations, all_spot_prices_instances) = {
        let mut latest_block_safe_prices: BTreeMap<DestinationVaultKey, U256> = BTreeMap::new();
        let mut unsafe_spot_prices_destinations: BTreeSet<DestinationVaultKey> = BTreeSet::new();
        let mut all_spot_prices_instances: Vec<(DestinationVaultKey, U256)> = Vec::new();

        for prices_input in preflighted_inputs_iter {
            let prices_env = prices_input.into_env(&ETH_MAINNET_CHAIN_SPEC);
            let prices_block_number = prices_env.header().number;
            // do I have to make a new contract every time? or is one per env good enough?
            let root_price_oracle_contract =
                Contract::new(chain_address_constants.rootPriceOracle.clone(), &prices_env);

            for key in &destination_vault_keys {
                let get_range_prices_lp_call: IMinimalRootPriceOracle::getRangePricesLPCall =
                    IMinimalRootPriceOracle::getRangePricesLPCall {
                        lpToken: key.token,
                        pool: key.pool,
                        quoteToken: key.baseAsset,
                    };

                let (spot_price_in_quote, safe_price_in_quote, is_spot_safe): (U256, U256, bool) =
                    root_price_oracle_contract
                        .call_builder(&get_range_prices_lp_call)
                        .call()
                        .into();

                if prices_block_number == constants_block_number {
                    // only save the latest safe price
                    latest_block_safe_prices.insert(key.clone(), safe_price_in_quote);
                }

                if !is_spot_safe {
                    unsafe_spot_prices_destinations.insert(key.clone());
                }
                all_spot_prices_instances.push((key.clone(), spot_price_in_quote));
            }
        }

        (
            latest_block_safe_prices,
            unsafe_spot_prices_destinations,
            all_spot_prices_instances,
        )
    };

    let key_to_average_spot_price = compute_average_spot_price_by_destination_vault(
        all_spot_prices_instances,
        num_blocks_sampled,
    );

    let price_info = {
        // keccak key, packed version of prices
        let mut price_info: Vec<(B256, PackedComputedGetRangePriceLP)> = Vec::new();

        for key in &destination_vault_keys {
            // this should be a u256 here instead of PackedComputedGetRangePriceLP
            let packed: = build_PackedComputedGetRangePriceLP(
                &key,
                &key_to_average_spot_price,
                &latest_block_safe_prices,
                &unsafe_spot_prices_destinations,
            );
            let keccak256_key: B256 = compute_transient_storage_slot(key.lp_token, key.pool, key.base_asset)
            let key_price_tuple = (keccak256_key, packed);

            price_info.push(key_price_tuple);
        }
        price_info
    };

    // note need to connect the autopool_constants_env with the prices_env here
    let journal = DestinationsZKPricesCommitment {
        commitment: destination_vault_keys_env.into_commitment(),
        priceInfo: price_info, // to be written into a transient storage dictionary
    };

    env::commit_slice(&journal.abi_encode());
}


pub fn compute_transient_storage_slot(lp_token: Address, pool: Address, quote_token: Address) -> B256 {
    // TODO check these with fuzz tests edge case is where the tokens are not checksum cases
    // Solidity: abi.encodePacked(address,address,address) should be 1:1 with
    let packed = (lp_token, pool, quote_token).abi_encode_packed();
    keccak256(packed)
}

fn build_PackedComputedGetRangePriceLP(
    key: &DestinationVaultKey,
    key_to_average_spot_price: &BTreeMap<DestinationVaultKey, U256>,
    latest_block_safe_prices: &BTreeMap<DestinationVaultKey, U256>,
    unsafe_spot_prices_destinations: &BTreeSet<DestinationVaultKey>,
) -> U256 {
    /*
    Returns the packed (u112,u112,u8) version of the safe, spot price, 

    if any of the spot prices are not safe or they are to large to fit in a u112, then
    the spot price is safe is set to false
    
    I don't expect to ever overflow, not sure on the right path to take for when that occurs. 
    don't want to panic because that breaks the whole debt reporting
    */

     let avg_spot_u256: U256 = *key_to_average_spot_price.get(key).unwrap_or_else(|| {
        panic!(
            "missing average spot price for destination vault key {:?}",
            key
        )
    });

    let latest_safe_u256: U256 = *latest_block_safe_prices.get(key).unwrap_or_else(|| {
        panic!(
            "missing latest safe price for destination vault key {:?}",
            key
        )
    });

    let avg_spot_try: Result<U112, _> = U112::uint_try_from(avg_spot_u256);
    let latest_safe_try: Result<U112, _> = U112::uint_try_from(latest_safe_u256);

    let avg_spot_u112: U112 = avg_spot_try.unwrap_or(U112::from(0u8));
    let latest_safe_u112: U112 = latest_safe_try.unwrap_or(U112::from(0u8));

    let cant_convert_too_big = avg_spot_try.is_err() || latest_safe_try.is_err();

    // not sure if we should panic here
    if cant_convert_too_big { panic!("One of the prices would not fit in a u112 \n {:?} avg_spot_u112 \n {:?} latest_safe_try", avg_spot_u112, latest_safe_try );
}
    let all_spot_prices_are_safe = !unsafe_spot_prices_destinations.contains(key);
    let is_spot_safe = (all_spot_prices_are_safe & !cant_convert_too_big) as u8;

    let packed = pack_u112_u112_u8(avg_spot_u112,  latest_safe_u112,is_spot_safe  )

    packed
}


pub fn pack_u112_u112_u8(avg: U112, latest: U112, is_spot_safe_zk: u8) -> U256 {
    let avg_u256: U256 = U256::from(avg);
    let latest_u256: U256 = U256::from(latest);
    let safe_u256: U256 = U256::from(is_spot_safe_zk);

    avg_u256 | (latest_u256 << 112) | (safe_u256 << 224)
}


// go through again for clarity, rewrite
fn compute_average_spot_price_by_destination_vault(
    all_spot_prices_instances: Vec<(DestinationVaultKey, U256)>,
    expected_count_per_key: u64, // number of blocks
) -> BTreeMap<DestinationVaultKey, U256> {
    let expected_count_per_key_u256: U256 = U256::from(expected_count_per_key);
    let mut running_sum_by_key: BTreeMap<DestinationVaultKey, U256> = BTreeMap::new();

    for (destination_vault_key, spot_price_value) in all_spot_prices_instances {
        let running_sum_for_key: &mut U256 = running_sum_by_key
            .entry(destination_vault_key)
            .or_insert(U256::ZERO);

        *running_sum_for_key += spot_price_value;
    }

    let average_price_by_key: Vec<(DestinationVaultKey, U256)> = running_sum_by_key
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
