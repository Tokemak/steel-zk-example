use alloy_primitives::{Address, address};
use alloy_sol_types::{SolCall, SolType, sol};

sol! {
    /// This must match the signature in the guest.
    interface IRootPriceOracle {
        function getRangePricesLP(
            address lpToken,
            address pool,
            address quoteToken
        ) external returns (uint, uint, bool);
    }
}
