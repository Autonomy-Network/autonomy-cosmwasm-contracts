use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use registry_stake::msg::{
    ConfigResponse, EpochInfoResponse, ExecuteMsg, InstantiateMsg, QueryMsg, RequestInfoResponse,
    RequestsResponse, StakeAmountResponse, StakesResponse, StateResponse,
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(ConfigResponse), &out_dir);
    export_schema(&schema_for!(RequestInfoResponse), &out_dir);
    export_schema(&schema_for!(RequestsResponse), &out_dir);
    export_schema(&schema_for!(StateResponse), &out_dir);
    export_schema(&schema_for!(StakeAmountResponse), &out_dir);
    export_schema(&schema_for!(StakesResponse), &out_dir);
    export_schema(&schema_for!(EpochInfoResponse), &out_dir);
}
