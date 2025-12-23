use alloy_primitives::Address;
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

pub async fn preflight_prices(
    autopool_constants: &AutopoolAddressConstants,
    multicall3: Address,
    provider: RootProvider,
    block: u64,
) -> Result<EthEvmInput, anyhow::Error> {
    let mut env = EthEvmEnv::builder()
        .provider(provider)
        .block_number(block)
        .chain_spec(&ETH_MAINNET_CHAIN_SPEC)
        .build()
        .await?;

    let calls = {
        let mut calls: Vec<IMulticall3::Call3> =
            Vec::with_capacity(autopool_constants.destinationVaultKeys.len());

        for key in &autopool_constants.destinationVaultKeys {
            {
                let get_range_prices_lp_call = IMinimalRootPriceOracle::getRangePricesLPCall {
                    lpToken: key.token,
                    pool: key.pool,
                    quoteToken: key.baseAsset,
                };

                calls.push(IMulticall3::Call3 {
                    target: autopool_constants.rootPriceOracle,
                    allowFailure: false,
                    callData: get_range_prices_lp_call.abi_encode().into(),
                });
            }
        }
        calls
    };

    let mut multicall3_contract = Contract::preflight(multicall3, &mut env);
    // just make the contract calls, we don't care about the values themselves in the preflight
    let _results: Vec<IMulticall3::Result> = multicall3_contract
        .call_builder(&IMulticall3::aggregate3Call { calls })
        .call_with_prefetch() // .call() also works, prefetch should be faster though
        .await?
        .into();

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
        let (destination_vaults, base_asset, system_registry) = {
            let calls: Vec<IMulticall3::Call3> = vec![
                IMulticall3::Call3 {
                    target: autopool,
                    allowFailure: false,
                    callData: IMinimalAutoPool::getDestinationsCall {}.abi_encode().into(),
                },
                IMulticall3::Call3 {
                    target: autopool,
                    allowFailure: false,
                    callData: IMinimalAutoPool::assetCall {}.abi_encode().into(),
                },
                IMulticall3::Call3 {
                    target: autopool,
                    allowFailure: false,
                    callData: IMinimalAutoPool::getSystemRegistryCall {}
                        .abi_encode()
                        .into(),
                },
            ];

            let mut multicall3_contract = Contract::preflight(multicall3, &mut env);

            let results: Vec<IMulticall3::Result> = multicall3_contract
                .call_builder(&IMulticall3::aggregate3Call { calls })
                .call_with_prefetch()
                .await?
                .into();

            let destination_vaults: Vec<Address> =
                IMinimalAutoPool::getDestinationsCall::abi_decode_returns(&results[0].returnData)?;
            let base_asset: Address =
                IMinimalAutoPool::assetCall::abi_decode_returns(&results[1].returnData)?;
            let system_registry: Address =
                IMinimalAutoPool::getSystemRegistryCall::abi_decode_returns(
                    &results[2].returnData,
                )?;

            (destination_vaults, base_asset, system_registry)
        };

        let mut system_registry_contract = Contract::preflight(system_registry, &mut env);

        let root_price_oracle: Address = system_registry_contract
            .call_builder(&IMinimalSystemRegistry::rootPriceOracleCall {})
            .call_with_prefetch()
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
            .call_with_prefetch()
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
                .into();

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
