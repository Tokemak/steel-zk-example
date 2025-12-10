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

use alloy_primitives::{U256};
use alloy_sol_types::SolValue;
use risc0_steel::{
    ethereum::{EthEvmInput, ETH_MAINNET_CHAIN_SPEC, EthEvmEnv},
    Contract,
    Commitment,
    SteelVerifier,
};

use debt_reporting_abi::{AverageSafePriceCommitment, IRootPriceOracle, USDC_MAINNET, ROOT_PRICE_ORACLE, A_LP_TOKEN};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {

    let blocks: Vec<u64>= env::read();
    // let blocks: Vec<U256> = blocks.into_iter().map(U256::from).collect();
    let first_block: &u64 = blocks.get(0).expect("Blocks should have at least one block");

    let mut ethereum_envs: Vec<EthEvmInput> = env::read();
    let call: IRootPriceOracle::getRangePricesLPCall = IRootPriceOracle::getRangePricesLPCall {
        lpToken: A_LP_TOKEN,
        pool: A_LP_TOKEN,
        quoteToken: USDC_MAINNET,
    };

    let mut safe_prices: Vec<U256> = Vec::with_capacity(blocks.len());
    let mut previous_execution_environment: Option<EthEvmEnv<_, Commitment>> = None;

    for (block_number, ethereum_input) in blocks.iter().zip(ethereum_envs.iter_mut()) {
        let current_execution_environment = ethereum_input.clone().into_env(&ETH_MAINNET_CHAIN_SPEC);

        if block_number != first_block {
            SteelVerifier::new(&current_execution_environment)
                .verify(previous_execution_environment.expect("There should be a previous env by this point").commitment());
        } else {
            // maybe check just this state?
        }
        
        // assert_eq!( // syntax is wrong
        //     current_execution_environment.header().number,
        //     block_number,
        //     "Mismatched block number between expected blocks list and EVM environment"
        // );

        let root_price_oracle_contract =
            Contract::new(ROOT_PRICE_ORACLE, &current_execution_environment);

        let (_spot_price_in_quote, safe_price_in_quote, _is_spot_safe): (U256, U256, bool) =
            root_price_oracle_contract
                .call_builder(&call)
                .call()
                .into();

        safe_prices.push(safe_price_in_quote);

        previous_execution_environment = Some(current_execution_environment);
    }

    let sum: U256 = safe_prices.iter().copied().sum();
    let denom = U256::from(safe_prices.len() as u64);
    let average_safe_price: U256 = sum / denom;

    let blocks: Vec<U256> = blocks.into_iter().map(U256::from).collect();

    let last_env = previous_execution_environment.expect("There should be an environment here");

    let journal = AverageSafePriceCommitment {
        commitment: last_env.into_commitment(),
        priceInfo: vec![(A_LP_TOKEN, average_safe_price)],
        blocks: blocks,
    };

    env::commit_slice(&journal.abi_encode());

}
