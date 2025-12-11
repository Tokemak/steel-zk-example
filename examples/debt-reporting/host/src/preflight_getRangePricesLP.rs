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
    IRootPriceOracle, DestinationVaultKey, ComputedGetRangePriceLP, 
    AutopoolAddressConstants, IMinimalRootPriceOracle
         
};
use futures::{StreamExt, TryStreamExt};
use risc0_steel::{
    alloy::providers::{Provider, ProviderBuilder, RootProvider},
    ethereum::{EthEvmEnv, ETH_MAINNET_CHAIN_SPEC},
    Contract,
};


pub async fn verify_envs_are_in_order_largest_to_smallest( envs: Vec<&mut EthEvmEnv>) {
    // stub, throw an error if the order is wrong, 
    // should be 100, 90, 80
    // not 80, 90, 100
}

// not sure if I want to consume destination_vault_keys or not
// I think I don't want to consume them
pub async fn preflight_getRangePricesLP(
    base_asset:Address, root_price_oracle:Address;
    destination_vault_keys: Vec<DestinationVaultKey>, envs: Vec<&mut EthEvmEnv>
) {

    for env in envs {

        let mut root_price_oracle_contract = Contract::preflight(ROOT_PRICE_ORACLE, env);

        for destination_vault_key in destination_vault_keys {
            let get_range_prices_lp_call: IRootPriceOracle::getRangePricesLPCall = IRootPriceOracle::getRangePricesLPCall {
                lpToken: destination_vault_key.token,
                pool: destination_vault_key.pool,
                quoteToken: base_asset,
            };
            // we just need to make the call, don't care about the result here
            // TODO edge case, what if it fails on this block, we are doing a bunch of calls
            let _ = root_price_oracle_contract.call_builder(&get_range_prices_lp_call).call().await?;
        }
    }

}

