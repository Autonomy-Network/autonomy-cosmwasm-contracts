use std::convert::TryInto;

use autonomy::{
    asset::{Asset, AssetInfo},
    types::{OrderBy, RequestStatus},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Binary, StdResult, Storage, Uint128};
use cosmwasm_storage::{bucket, bucket_read, singleton, singleton_read, ReadonlyBucket};

const KEY_CONFIG: &[u8] = b"config";
const PREFIX_KEY_REQUEST_INFO: &[u8] = b"request_info";
const KEY_STATE: &[u8] = b"state";
const PREFIX_KEY_STAKE_BALANCE: &[u8] = b"stake_balance";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub fee_amount: Uint128,
    pub fee_denom: String,
    pub auto: AssetInfo,
}

pub fn store_config(storage: &mut dyn Storage, config: &Config) -> StdResult<()> {
    singleton::<Config>(storage, KEY_CONFIG).save(config)
}

pub fn read_config(storage: &dyn Storage) -> StdResult<Config> {
    singleton_read::<Config>(storage, KEY_CONFIG).load()
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub curr_executing_request_id: u64,
    pub total_requests: u64,
    pub last_epoch: u64,
    pub executor: String,
    pub stakes: Vec<String>,
    pub total_staked: Uint128,
}

pub fn store_state(storage: &mut dyn Storage, state: &State) -> StdResult<()> {
    singleton::<State>(storage, KEY_STATE).save(state)
}

pub fn read_state(storage: &dyn Storage) -> StdResult<State> {
    singleton_read::<State>(storage, KEY_STATE).load()
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Request {
    // The user who registered this request.
    pub user: String,
    // The executor who is gonna run this request.
    pub executor: String,
    // Target contract.
    pub target: String,
    // Msg to call the target
    pub msg: Binary,
    // Asset sent in advance
    pub input_asset: Asset,
    // Timestamp of creation
    pub created_at: u64,
    // Status of Request
    pub status: RequestStatus,
}

pub fn read_request(storage: &dyn Storage, id: u64) -> StdResult<Request> {
    bucket_read::<Request>(storage, PREFIX_KEY_REQUEST_INFO).load(&id.to_le_bytes())
}

pub fn store_request(storage: &mut dyn Storage, id: u64, request: &Request) -> StdResult<()> {
    bucket::<Request>(storage, PREFIX_KEY_REQUEST_INFO).save(&id.to_le_bytes(), request)
}

pub fn remove_request(storage: &mut dyn Storage, id: u64) -> StdResult<()> {
    Ok(bucket::<Request>(storage, PREFIX_KEY_REQUEST_INFO).remove(&id.to_le_bytes()))
}

const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 10;
pub fn read_requests<'a>(
    storage: &'a dyn Storage,
    start_after: Option<u64>,
    limit: Option<u32>,
    order_by: Option<OrderBy>,
) -> StdResult<Vec<(u64, Request)>> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let (start, end, order_by) = match order_by {
        Some(OrderBy::Asc) => (calc_range_start_id(start_after), None, OrderBy::Asc),
        _ => (None, calc_range_end_id(start_after), OrderBy::Desc),
    };

    let lock_accounts: ReadonlyBucket<'a, Request> =
        ReadonlyBucket::new(storage, PREFIX_KEY_REQUEST_INFO);

    lock_accounts
        .range(start.as_deref(), end.as_deref(), order_by.into())
        .take(limit)
        .map(|item| {
            let (k, v) = item?;
            Ok((u64::from_le_bytes(k.try_into().unwrap()), v))
        })
        .collect()
}

// this will set the first key after the provided key, by appending a 1 byte
fn calc_range_start_id(start_after: Option<u64>) -> Option<Vec<u8>> {
    start_after.map(|id| {
        let mut v = id.to_le_bytes().to_vec();
        v.push(1);
        v
    })
}

// this will set the first key after the provided key, by appending a 1 byte
fn calc_range_end_id(start_after: Option<u64>) -> Option<Vec<u8>> {
    start_after.map(|id| id.to_le_bytes().to_vec())
}

pub fn read_balance(storage: &dyn Storage, addr: Addr) -> Uint128 {
    bucket_read::<Uint128>(storage, PREFIX_KEY_STAKE_BALANCE)
        .load(&addr.as_bytes())
        .unwrap_or_default()
}

pub fn store_balance(storage: &mut dyn Storage, addr: Addr, amount: &Uint128) -> StdResult<()> {
    bucket::<Uint128>(storage, PREFIX_KEY_STAKE_BALANCE).save(&addr.as_bytes(), amount)
}

pub const STAN_STAKE: u64 = 1000000;
pub const BLOCKS_IN_EPOCH: u64 = 100;
