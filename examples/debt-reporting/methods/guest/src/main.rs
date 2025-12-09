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

use alloy_primitives::{address, Address, U256};
use risc0_steel::{
    ethereum::{EthEvmInput, ETH_MAINNET_CHAIN_SPEC},
    Contract,
};
use alloy_sol_types::{SolValue};

use debt_reporting_abi::{IRootPriceOracle, AverageSafePriceCommitment};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

// const EOA_TOKEMAK_WALLET: Address = address!("91aa2CcE6B22Ec9eCd8A56C830566e67187fe07E");
const USDC_MAINNET: Address = address!("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48");
const ROOT_PRICE_ORACLE: Address = address!("61F8BE7FD721e80C0249829eaE6f0DAf21bc2CaC");


fn main() {
    // get those from the inputs? idk here on what, prob doesn't matter
    let num_prior_blocks: U256 = U256::from(2);
    let gap_between_blocks: U256 = U256::from(100);
    let input: EthEvmInput = env::read();
    // Converts the input into a `EvmEnv` for execution. It checks that the state matches the state
    let env = input.into_env(&ETH_MAINNET_CHAIN_SPEC);
    let pool: Address = address!("64273624eb57c5cA961d366CBF3968e760Bf0452");

    let call : IRootPriceOracle::getRangePricesLPCall  =  IRootPriceOracle::getRangePricesLPCall{
        lpToken: pool,
        pool: pool,
        quoteToken: USDC_MAINNET,
    };

    let _safe_prices : Vec::<U256>  = Vec::new();

    let contract = Contract::new(ROOT_PRICE_ORACLE, &env);
    // TODO double check this is the order
    let (_spot_price_in_quote, safe_price_in_quote, _is_spot_safe): (U256, U256, bool) = contract.call_builder(&call).call().into();

    let journal = AverageSafePriceCommitment {
        commitment: env.into_commitment(),
        priceInfo: vec![(pool, safe_price_in_quote)],
        numPriorBlocks: num_prior_blocks,
        gapBetweenBlocks: gap_between_blocks,
    };

    env::commit_slice(&journal.abi_encode());

}

