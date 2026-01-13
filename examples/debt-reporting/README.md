# Debt Reporting Example

run using the debugger

see

https://www.notion.so/tokemak/ZK-Debt-Reporting-plan-issue-1037-2aeb758fdbdc80529c5ad00a9b0693fc



### quick fixes

- uint256 encodes (isSpotSafe, Spot, Safe) (change the order)
- use consistant names. match already deployed contracts
- remove unused code, a bunch of struts are not needed

- unit tests for smart contract
- unit tests for rust part



- write a helper lint alais that lints everything
 formats everything in examples/debt-reporting (Rust + Foundry)
alias debt-fmt='(
  cd /Users/pb/Documents/Github/Tokemak/steel-zk-example/examples/debt-reporting \
  && cargo fmt --all \
  && forge fmt --root . --libs lib
)'



## Open Problems:
- Figure out what happens if there are chain reorgs


eg something like:

[dependencies]
risc0-steel = { 
    git = "https://github.com/boundless-xyz/steel.git",
    rev = "2bddda146c2b6ad6b20e900644c815d77a3f667a"
}


eg these are causing the problem

in host

        let mut env = EthEvmEnv::builder()
            .provider(provider.clone())
            .block_number(*block)
            .beacon_api(args.beacon_api_url.clone())
            .chain_spec(&ETH_MAINNET_CHAIN_SPEC)
            .build()
            .await?;


and in guest

if block_number != first_block {
SteelVerifier::new(&current_execution_environment)
    .verify(previous_execution_environment.expect("There should be a previous env by this point").commitment());
} else {
// maybe check just this state?
}