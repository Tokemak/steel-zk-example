extern crate alloc;

use alloy_primitives::Address;
use alloy_sol_types::SolCall;
use debt_reporting_abi::{
    ChainAddressConstants, DestinationVaultKey, IMinimalAutoPool, IMinimalDestinationVault,
    IMinimalRootPriceOracle, IMulticall3,
};

use alloc::collections::BTreeSet;

use risc0_steel::{
    alloy::providers::RootProvider,
    ethereum::{EthEvmEnv, EthEvmInput, ETH_MAINNET_CHAIN_SPEC},
    Contract,
};

// todo, some kind of validation that the root price oracle -> system registry, and reverse,
// and that they all point at each other
// this uses multicall while the guest uses naive seqential logic, not certain on if I want it to be like that
// the guest is determanistic, and makes no external calls so it is not faster to use multicall

pub async fn preflight_autopool_constants(
    chain_address_constants: ChainAddressConstants,
    provider: RootProvider,
    block: u64,
) -> Result<(EthEvmInput, Vec<DestinationVaultKey>), anyhow::Error> {
    let mut env = EthEvmEnv::builder()
        .provider(provider)
        .block_number(block)
        .chain_spec(&ETH_MAINNET_CHAIN_SPEC)
        .build()
        .await?;

    let destinations_with_base_asset: BTreeSet<(Address, Address)> = {
        let destinations_and_base_asset_calls = {
            let mut destinations_and_base_asset_calls: Vec<IMulticall3::Call3> =
                Vec::with_capacity(chain_address_constants.autopools.len() * 2);

            for autopool in &chain_address_constants.autopools {
                destinations_and_base_asset_calls.push(IMulticall3::Call3 {
                    target: *autopool,
                    allowFailure: false,
                    callData: IMinimalAutoPool::getDestinationsCall {}.abi_encode().into(),
                });

                destinations_and_base_asset_calls.push(IMulticall3::Call3 {
                    target: *autopool,
                    allowFailure: false,
                    callData: IMinimalAutoPool::assetCall {}.abi_encode().into(),
                });
            }
            destinations_and_base_asset_calls
        };

        let destinations_and_base_asset = {
            let mut multicall3_contract =
                Contract::preflight(chain_address_constants.multicall3, &mut env);
            let results: Vec<IMulticall3::Result> = multicall3_contract
                .call_builder(&IMulticall3::aggregate3Call {
                    calls: destinations_and_base_asset_calls,
                })
                .call_with_prefetch()
                .await?
                .into();

            let mut destinations_and_base_asset: BTreeSet<(Address, Address)> = BTreeSet::new();

            for (index, _autopool) in chain_address_constants.autopools.iter().enumerate() {
                let destinations_index = index * 2;
                let base_asset_index = (index * 2) + 1;
                let destination_vaults: Vec<Address> =
                    IMinimalAutoPool::getDestinationsCall::abi_decode_returns(
                        &results[destinations_index].returnData,
                    )?;

                let base_asset: Address = IMinimalAutoPool::assetCall::abi_decode_returns(
                    &results[base_asset_index].returnData,
                )?;

                for destination in destination_vaults {
                    destinations_and_base_asset.insert((destination, base_asset));
                }
            }
            destinations_and_base_asset
        };
        destinations_and_base_asset
    };

    let destination_vault_keys = {
        let calls: Vec<IMulticall3::Call3> = {
            let mut calls: Vec<IMulticall3::Call3> =
                Vec::with_capacity(destinations_with_base_asset.len() * 2);

            for (destination, _base_asset) in &destinations_with_base_asset {
                calls.push(IMulticall3::Call3 {
                    target: *destination,
                    allowFailure: false,
                    callData: IMinimalDestinationVault::getPoolCall {}.abi_encode().into(),
                });

                calls.push(IMulticall3::Call3 {
                    target: *destination,
                    allowFailure: false,
                    callData: IMinimalDestinationVault::underlyingCall {}
                        .abi_encode()
                        .into(),
                });
            }
            calls
        };

        let mut multicall3_contract =
            Contract::preflight(chain_address_constants.multicall3, &mut env);
        let results: Vec<IMulticall3::Result> = multicall3_contract
            .call_builder(&IMulticall3::aggregate3Call { calls })
            .call_with_prefetch()
            .await?
            .into();

        let mut destination_vault_keys: Vec<DestinationVaultKey> =
            Vec::with_capacity(destinations_with_base_asset.len());
        for (index, (destination_vault, base_asset)) in
            destinations_with_base_asset.iter().enumerate()
        {
            let pool: Address = IMinimalDestinationVault::getPoolCall::abi_decode_returns(
                &results[index * 2].returnData,
            )?
            .into();

            let token: Address = IMinimalDestinationVault::underlyingCall::abi_decode_returns(
                &results[(index * 2) + 1].returnData,
            )?
            .into();

            destination_vault_keys.push(DestinationVaultKey {
                token,
                pool,
                baseAsset: *base_asset,
                destinationVault: *destination_vault,
            });
        }

        destination_vault_keys
    };

    let input = env.into_input().await?;
    Ok((input, destination_vault_keys))
}

pub async fn preflight_prices(
    chain_address_constants: ChainAddressConstants,
    destination_vault_keys: Vec<DestinationVaultKey>,
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
        // maybe put this into a seperate function?
        let mut calls: Vec<IMulticall3::Call3> = Vec::new();

        for key in destination_vault_keys {
            {
                let get_range_prices_lp_call = IMinimalRootPriceOracle::getRangePricesLPCall {
                    lpToken: key.token,
                    pool: key.pool,
                    quoteToken: key.baseAsset,
                };

                calls.push(IMulticall3::Call3 {
                    target: chain_address_constants.rootPriceOracle,
                    allowFailure: false,
                    callData: get_range_prices_lp_call.abi_encode().into(),
                });
            }
        }
        calls
    };

    // TODO include some limits to make sure that we are not overwhelming the gas limits with too many calls
    let mut multicall3_contract = Contract::preflight(chain_address_constants.multicall3, &mut env);

    // just make the contract calls, we don't care about the results themselves in the preflight
    let _results: Vec<IMulticall3::Result> = multicall3_contract
        .call_builder(&IMulticall3::aggregate3Call { calls })
        .call_with_prefetch()
        .await?
        .into();

    let input = env.into_input().await?;
    Ok(input)
}
