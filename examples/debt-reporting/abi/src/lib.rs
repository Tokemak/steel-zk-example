use alloy_primitives::{Address, address};
use alloy_sol_types::sol;
use risc0_steel::Commitment;

sol! {
    interface IRootPriceOracle {
        function getRangePricesLP(
            address lpToken,
            address pool,
            address quoteToken
        ) external returns (uint, uint, bool); // spotPriceInQuote, safePriceInQuote, isSpotSafe
    }
}

sol! {
    struct AverageSafePriceCommitment {
        Commitment commitment;
        (address, uint)[] priceInfo; // lpToken, averageSafePrice
        uint[] blocks;
    }
}

// #[derive(Serialize, Deserialize)]
// pub struct DebtReportingInputData {
//     pub blocks: Vec<u64>,
//     pub ethereum_envs: Vec<EthEvmInput>,
// }

// const EOA_TOKEMAK_WALLET: Address = address!("91aa2CcE6B22Ec9eCd8A56C830566e67187fe07E");
pub const USDC_MAINNET: Address = address!("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48");
pub const ROOT_PRICE_ORACLE: Address = address!("61F8BE7FD721e80C0249829eaE6f0DAf21bc2CaC");
pub const A_LP_TOKEN: Address = address!("64273624eb57c5cA961d366CBF3968e760Bf0452");
