// use osmo_bindings::{Step, Swap};
use osmosis_std::types::osmosis::gamm::v1beta1::{ SwapAmountInRoute, SwapAmountOutRoute, MsgSwapExactAmountIn};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Uint128;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Swap {
        // Address of the user for this swap
        user: String,
        // Swap routes
        route: Vec<SwapAmountInRoute>,
        // Input token
        token_in: String,
        // Minimum output amount
        min_output: Uint128,
        // Maximum output amount
        max_output: Uint128,
    },
    CheckRange {
        // Address of the user for this swap
        user: String,
        // Denom of the output asset
        denom: String,
        // Balance before this swap
        balance_before: Uint128,
        // Minimum output amount
        min_output: Uint128,
        // Maximum output amount
        max_output: Uint128,
    },
}
