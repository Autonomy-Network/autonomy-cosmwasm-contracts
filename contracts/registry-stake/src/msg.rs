use autonomy::{
    asset::{Asset, AssetInfo},
    types::OrderBy,
};
use cosmwasm_std::{Binary, Uint128};
use cw20::Cw20ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::Request;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub fee_amount: Uint128,
    pub fee_denom: String,
    pub auto: AssetInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetFeeInfo {
        fee_amount: Uint128,
        fee_denom: String,
    },
    CreateRequest {
        target: String,
        msg: Binary,
        input_asset: Asset,
    },
    CancelRequest {
        id: u64,
    },
    ExecuteRequest {
        id: u64,
    },
    Receive(Cw20ReceiveMsg),
    StakeDenom {
        num_stakes: u64,
    },
    Unstake {
        idxs: Vec<u64>,
    },
    UpdateExecutor {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cw20HookMsg {
    Stake { num_stakes: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    RequestInfo {
        id: u64,
    },
    Requests {
        start_after: Option<u64>,
        limit: Option<u32>,
        order_by: Option<OrderBy>,
    },
    State {},
    EpochInfo {},
    StakeAmount {
        user: String,
    },
    Stakes {
        start: u64,
        limit: u64,
    },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub auto: AssetInfo,
    pub stan_stake: Uint128,
    pub blocks_in_epoch: u64,
    pub fee_amount: Uint128,
    pub fee_denom: String,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RequestInfoResponse {
    pub id: u64,
    pub request: Request,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RequestsResponse {
    pub requests: Vec<RequestInfoResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StateResponse {
    pub curr_executing_request_id: u64,
    pub total_requests: u64,
    pub total_stake_amount: Uint128,
    pub stakes_len: u64,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EpochInfoResponse {
    pub cur_epoch: u64,
    pub last_epoch: u64,
    pub executor: String,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakeAmountResponse {
    pub amount: Uint128,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakesResponse {
    pub stakes: Vec<String>,
}
