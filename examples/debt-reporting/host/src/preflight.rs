use alloy_primitives::{Address, U256};
use alloy_sol_types::SolCall;
use debt_reporting_abi::{
    AutopoolAddressConstants, DestinationVaultKey, IMinimalAutoPool, IMinimalDestinationVault,
    IMinimalRootPriceOracle, IMinimalSystemRegistry, IMulticall3,
};

use risc0_steel::{
    alloy::providers::RootProvider,
    ethereum::{EthEvmEnv, EthEvmInput, ETH_MAINNET_CHAIN_SPEC},
    Contract,
};
// use anyhow::{anyhow, bail};

// pub async fn _preflight_prices2(
//     autopool_constants: &AutopoolAddressConstants,
//     multicall3: &Address,
//     provider: RootProvider,
//     block: u64,
// ) -> Result<EthEvmInput, anyhow::Error> {
//     let mut env = EthEvmEnv::builder()
//             .provider(provider)
//             .block_number(block)
//             .chain_spec(&ETH_MAINNET_CHAIN_SPEC)
//             .build()
//             .await?;

//     let mut price_calls: Vec<Call3> = Vec::with_capacity(&autopool_constants.destinationVaultKeys.len());

//     {
//         for key in &autopool_constants.destinationVaultKeys {
//             {
//                 let get_range_prices_lp_call: IMinimalRootPriceOracle::getRangePricesLPCall =
//                     IMinimalRootPriceOracle::getRangePricesLPCall {
//                         lpToken: key.token,
//                         pool: key.pool,
//                         quoteToken: key.baseAsset,
//                     };

//                 let price_call = Call3 {
//                     target: autopool_constants.rootPriceOracle,
//                     allowFailure: false,
//                     callData: get_range_prices_lp_call.abi_encode(), // selector + args
//                 };

//                 price_calls.push(price_call);

//             }
//     }

//     let mut multicall_contract = Contract::preflight(multicall3, &mut env);

//     let _results: Vec<IMulticall3::Result> = multicall_contract
//         .call_builder(&IMulticall3::aggregate3Call { calls })
//         .call()
//         .await?
//         .into();

//     let input = env.into_input().await?;
//     Ok(input)
// }
// }

pub async fn preflight_prices(
    autopool_constants: &AutopoolAddressConstants,
    provider: RootProvider,
    block: u64,
) -> Result<EthEvmInput, anyhow::Error> {
    let mut env = EthEvmEnv::builder()
        .provider(provider)
        .block_number(block)
        .chain_spec(&ETH_MAINNET_CHAIN_SPEC)
        .build()
        .await?;

    for key in &autopool_constants.destinationVaultKeys {
        {
            let get_range_prices_lp_call: IMinimalRootPriceOracle::getRangePricesLPCall =
                IMinimalRootPriceOracle::getRangePricesLPCall {
                    lpToken: key.token,
                    pool: key.pool,
                    quoteToken: key.baseAsset,
                };

            let mut root_price_oracle_contract =
                Contract::preflight(autopool_constants.rootPriceOracle, &mut env);

            let (_spot_price_in_quote, _safe_price_in_quote, _is_spot_safe): (U256, U256, bool) =
                root_price_oracle_contract
                    .call_builder(&get_range_prices_lp_call)
                    .call_with_prefetch()
                    .await?
                    .into();
        };
    }
    let input = env.into_input().await?;
    Ok(input)
}

pub async fn preflight_autopool_constants(
    autopool: Address,
    multicall3: Address,
    provider: RootProvider,
    block: u64,
) -> Result<(EthEvmInput, AutopoolAddressConstants), anyhow::Error> {
    let mut env = EthEvmEnv::builder()
        .provider(provider)
        .block_number(block)
        .chain_spec(&ETH_MAINNET_CHAIN_SPEC)
        .build()
        .await?;

    let (destination_vaults, base_asset, system_registry, root_price_oracle) = {
        let mut autopool_contract = Contract::preflight(autopool, &mut env);

        let destination_vaults: Vec<Address> = autopool_contract
            .call_builder(&IMinimalAutoPool::getDestinationsCall {})
            .call()
            .await?;

        let base_asset: Address = autopool_contract
            .call_builder(&IMinimalAutoPool::assetCall {})
            .call()
            .await?;

        let system_registry: Address = autopool_contract
            .call_builder(&IMinimalAutoPool::getSystemRegistryCall {})
            .call()
            .await?;

        let mut system_registry_contract = Contract::preflight(system_registry, &mut env);

        let root_price_oracle: Address = system_registry_contract
            .call_builder(&IMinimalSystemRegistry::rootPriceOracleCall {})
            .call()
            .await?;
        (
            destination_vaults,
            base_asset,
            system_registry,
            root_price_oracle,
        )
    };

    let destination_vault_keys = {
        let get_pool_call = IMinimalDestinationVault::getPoolCall {};
        let underlying_call = IMinimalDestinationVault::underlyingCall {};

        let calls = {
            let mut calls: Vec<IMulticall3::Call3> =
                Vec::with_capacity(2 * destination_vaults.len());

            for destination_vault in &destination_vaults {
                calls.push(IMulticall3::Call3 {
                    target: *destination_vault,
                    allowFailure: false,
                    callData: get_pool_call.abi_encode().into(),
                });

                calls.push(IMulticall3::Call3 {
                    target: *destination_vault,
                    allowFailure: false,
                    callData: underlying_call.abi_encode().into(),
                })
            }
            calls
        };

        let mut multicall3_contract = Contract::preflight(multicall3, &mut env);
        let results: Vec<IMulticall3::Result> = multicall3_contract
            .call_builder(&IMulticall3::aggregate3Call { calls })
            .call()
            .await?
            .into();

        let destination_vault_keys = {
            let mut destination_vault_keys = Vec::with_capacity(destination_vaults.len());

            for (index, destination_vault) in destination_vaults.iter().enumerate() {
                let pool_result = &results[index * 2];
                let underlying_result = &results[(index * 2) + 1];

                let pool: Address = IMinimalDestinationVault::getPoolCall::abi_decode_returns(
                    &pool_result.returnData,
                )?
                .into(); // maybe abi_decode_returns
                let token: Address = IMinimalDestinationVault::underlyingCall::abi_decode_returns(
                    &underlying_result.returnData,
                )?
                .into();

                destination_vault_keys.push(DestinationVaultKey {
                    token,
                    pool,
                    baseAsset: base_asset,
                    destinationVault: *destination_vault,
                });
            }
            destination_vault_keys
        };

        destination_vault_keys
    };

    let autopool_address_constants = AutopoolAddressConstants {
        autopool: autopool,
        systemRegistry: system_registry,
        rootPriceOracle: root_price_oracle,
        baseAsset: base_asset,
        destinationVaultKeys: destination_vault_keys,
    };

    let input = env.into_input().await?;
    Ok((input, autopool_address_constants))
}

// fn assert_same_keys(
//     a: &[DestinationVaultKey],
//     b: &[DestinationVaultKey],
// ) -> anyhow::Result<()> {
//     if a.len() != b.len() {
//         bail!("len mismatch: a={} b={}", a.len(), b.len());
//     }

//     for (i, (ka, kb)) in a.iter().zip(b.iter()).enumerate() {
//         if ka != kb {
//             // `DestinationVaultKey` from sol! usually derives PartialEq if you used all_derives
//             return Err(anyhow!(
//                 "mismatch at index {i}\n  a={:?}\n  b={:?}",
//                 ka,
//                 kb
//             ));
//         }
//     }

//     Ok(())
// }

// let mut secondary_destination_vault_keys: Vec<DestinationVaultKey>  =  Vec::with_capacity(destination_vaults.len());
// for dv in destination_vaults {
//     let (token, pool) = {
//         let mut destination_vault_contract = Contract::preflight(dv, &mut env);

//         let token: Address = destination_vault_contract
//             .call_builder(&IMinimalDestinationVault::underlyingCall {})
//             .call()
//             .await?;

//         let pool: Address = destination_vault_contract
//             .call_builder(&IMinimalDestinationVault::getPoolCall {})
//             .call()
//             .await?;

//         (token, pool)
//     };

//     secondary_destination_vault_keys.push(DestinationVaultKey {
//         token: token,
//         pool: pool,
//         baseAsset: base_asset,
//         destinationVault: dv,
//     });
// }

// assert_same_keys(&destination_vault_keys, &secondary_destination_vault_keys)?;
