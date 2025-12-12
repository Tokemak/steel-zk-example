// use alloy_primitives::Address;
// use anyhow::Result;
// use debt_reporting_abi::{DestinationVaultKey, IMinimalRootPriceOracle};
// use risc0_steel::{ethereum::HostEvmEnv, Contract};

// /// Optional: check env ordering (stub for now)
// pub fn verify_envs_are_in_order_largest_to_smallest<D, F, C>(
//     _envs: &[HostEvmEnv<D, F, C>],
// ) -> Result<()> {
//     // TODO: read block numbers from env and assert descending order
//     Ok(())
// }

// /// Preflight getRangePricesLP on each env for all destination vaults.
// pub async fn preflight_get_range_prices_lp<D, F, C>(
//     base_asset: Address,
//     root_price_oracle: Address,
//     destination_vault_keys: &[DestinationVaultKey],
//     envs: &mut [HostEvmEnv<D, F, C>],
// ) -> Result<()> {
//     // Optional ordering check
//     verify_envs_are_in_order_largest_to_smallest(envs)?;

//     for env in envs.iter_mut() {
//         let mut root_price_oracle_contract = Contract::preflight(root_price_oracle, env);

//         for dv in destination_vault_keys {
//             let call = IMinimalRootPriceOracle::getRangePricesLPCall {
//                 lpToken: dv.token,
//                 pool: dv.pool,
//                 quoteToken: base_asset,
//             };

//             // We just need to make the call and prime the env
//             let _ = root_price_oracle_contract
//                 .call_builder(&call)
//                 .call()
//                 .await?;
//         }
//     }

//     Ok(())
// }
