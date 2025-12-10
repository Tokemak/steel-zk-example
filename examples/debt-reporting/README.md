# Debt Reporting Example


see

https://www.notion.so/tokemak/ZK-Debt-Reporting-plan-issue-1037-2aeb758fdbdc80529c5ad00a9b0693fc



Open Problems:
- Either use the Dec 9th, version of boundless-xyz/steel with fulu support. 
- or
- Don't validate with steel preflights, just assume the blocks come from a valid source.





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