use osmo_bindings::{Step, Swap};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Uint128;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Swap {
        user: String,
        first: Swap,
        route: Vec<Step>,
        amount: Uint128,
        min_output: Uint128,
        max_output: Uint128,
    },
    CheckRange {
        user: String,
        denom: String,
        balance_before: Uint128,
        min_output: Uint128,
        max_output: Uint128,
    },
}
