use autonomy::asset::{Asset, AssetInfo};
use autonomy::types::{OrderBy, RequestStatus};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
    from_binary, to_binary, Addr, BankMsg, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    Reply, ReplyOn, Response, StdError, StdResult, SubMsg, SubMsgResult, Uint128, WasmMsg,
};
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};

use crate::msg::{
    ConfigResponse, Cw20HookMsg, EpochInfoResponse, ExecuteMsg, InstantiateMsg, MigrateMsg,
    QueryMsg, RequestInfoResponse, RequestsResponse, StakeAmountResponse, StakesResponse,
    StateResponse,
};
use crate::state::{
    read_balance, read_config, read_request, read_requests, read_state, store_balance,
    store_config, store_request, store_state, Config, Request, State, BLOCKS_IN_EPOCH, STAN_STAKE,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    store_config(
        deps.storage,
        &Config {
            fee_amount: msg.fee_amount,
            fee_denom: msg.fee_denom,
            auto: msg.auto,
        },
    )?;
    store_state(
        deps.storage,
        &State {
            curr_executing_request_id: u64::MAX,
            total_requests: 0,
            last_epoch: 0,
            executor: "".to_string(),
            stakes: vec![],
            total_staked: Uint128::zero(),
        },
    )?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::SetFeeInfo {
            fee_amount,
            fee_denom
        } => set_fee_info(deps, env, info, fee_amount, fee_denom),
        ExecuteMsg::CreateRequest {
            target,
            msg,
            input_asset,
        } => create_request(deps, env, info, target, msg, input_asset),
        ExecuteMsg::CancelRequest { id } => cancel_request(deps, env, info, id),
        ExecuteMsg::ExecuteRequest { id } => execute_request(deps, info, id),

        ExecuteMsg::Receive(msg) => receive_cw20(deps, env, info, msg),
        ExecuteMsg::StakeDenom { num_stakes } => receive_denom(deps, env, info, num_stakes),
        ExecuteMsg::Unstake { idxs } => unstake(deps, env, info, idxs),
        ExecuteMsg::UpdateExecutor {} => update_executor(deps, env),
    }
}

pub fn set_fee_info(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    fee_amount: Uint128,
    fee_denom: String
) -> StdResult<Response> {
    let mut config = read_config(deps.storage)?;
    config.fee_amount = fee_amount;
    config.fee_denom = fee_denom;
    store_config(deps.storage, &config)?;

    Ok(Response::default())
}

pub fn create_request(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    target: String,
    msg: Binary,
    input_asset: Asset,
) -> StdResult<Response> {
    let target_addr = deps.api.addr_validate(&target)?;
    let config = read_config(deps.storage)?;
    let mut state = read_state(deps.storage)?;

    let mut msgs: Vec<CosmosMsg> = vec![];

    let mut funds = info.funds.clone();
    if funds.is_empty() {
        return Err(StdError::generic_err("invalid input assets"));
    }

    if let Some(fee_fund_index) = funds.iter().position(|f| f.denom == config.fee_denom) {
        if funds[fee_fund_index].amount < config.fee_amount {
            return Err(StdError::generic_err("insufficient fee"));
        }
        funds[fee_fund_index].amount -= config.fee_amount;
    } else {
        return Err(StdError::generic_err("no fee paid"));
    }

    /* Check fund tokens */
    match input_asset.clone().info {
        AssetInfo::NativeToken { denom } => {
            if let Some(asset_index) = funds.iter().position(|f| f.denom == denom) {
                if funds[asset_index].amount < input_asset.amount {
                    return Err(StdError::generic_err("invalid input assets"));
                }
            } else {
                return Err(StdError::generic_err("invalid input assets"));
            }
        }
        AssetInfo::Token { contract_addr } => {
            msgs.push(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: contract_addr.to_string(),
                msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
                    owner: info.sender.to_string(),
                    recipient: env.contract.address.to_string(),
                    amount: input_asset.amount,
                })?,
                funds: vec![],
            }));
        }
    }

    // read executor
    let cur_epoch = env.block.height / BLOCKS_IN_EPOCH * BLOCKS_IN_EPOCH;
    if cur_epoch != state.last_epoch {
        _update_executor(&mut state, env.clone());

        // if state.executor == "" {
        //     return Err(StdError::generic_err("no executor"));
        // }
    }

    let id = state.total_requests;
    let request = Request {
        user: info.sender.to_string(),
        executor: state.executor.to_string(),
        target: target_addr.to_string(),
        msg,
        input_asset,
        created_at: env.block.time.seconds(),
        status: RequestStatus::Created,
    };

    state.total_requests += 1;
    store_request(deps.storage, id, &request)?;
    store_state(deps.storage, &state)?;

    Ok(Response::new().add_messages(msgs).add_attributes(vec![
        ("action", "create_request"),
        ("id", id.to_string().as_str()),
    ]))
}

pub fn cancel_request(deps: DepsMut, _env: Env, info: MessageInfo, id: u64) -> StdResult<Response> {
    let config = read_config(deps.storage)?;
    let mut request = read_request(deps.storage, id)?;
    if deps.api.addr_canonicalize(request.user.as_str())?
        != deps.api.addr_canonicalize(info.sender.as_str())?
    {
        return Err(StdError::generic_err("unauthorized"));
    }

    if request.status != RequestStatus::Created {
        return Err(StdError::generic_err("request is executed or canceled"));
    }

    request.status = RequestStatus::Canceled;
    store_request(deps.storage, id, &request)?;

    let mut msgs: Vec<CosmosMsg> = vec![];
    let input_asset = request.input_asset.clone();
    match input_asset.info {
        AssetInfo::NativeToken { denom: _ } => {
            msgs.push(CosmosMsg::Bank(BankMsg::Send {
                to_address: request.user.to_string(),
                amount: vec![input_asset.deduct_tax(&deps.querier)?],
            }));
        }
        AssetInfo::Token { contract_addr } => {
            if !request.input_asset.amount.is_zero() {
                msgs.push(CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: contract_addr.to_string(),
                    msg: to_binary(&Cw20ExecuteMsg::Transfer {
                        recipient: request.user.to_string(),
                        amount: request.input_asset.amount,
                    })?,
                    funds: vec![],
                }));
            }
        }
    }

    let fee_asset = Asset {
        info: AssetInfo::NativeToken {
            denom: config.fee_denom,
        },
        amount: config.fee_amount,
    };
    msgs.push(CosmosMsg::Bank(BankMsg::Send {
        to_address: request.user.to_string(),
        amount: vec![fee_asset.deduct_tax(&deps.querier)?],
    }));

    Ok(Response::new().add_messages(msgs).add_attributes(vec![
        ("action", "cancel_request"),
        ("id", id.to_string().as_str()),
    ]))
}

pub fn execute_request(deps: DepsMut, info: MessageInfo, id: u64) -> StdResult<Response> {
    let config = read_config(deps.storage)?;
    let mut request = read_request(deps.storage, id)?;

    if request.executor != "" {
        if deps.api.addr_canonicalize(request.executor.as_str())?
            != deps.api.addr_canonicalize(info.sender.as_str())?
        {
            return Err(StdError::generic_err("unauthorized"));
        }
    }

    if request.status != RequestStatus::Created {
        return Err(StdError::generic_err("request is executed or canceled"));
    }

    request.status = RequestStatus::Executed;
    store_request(deps.storage, id, &request)?;

    let mut state = read_state(deps.storage)?;
    state.curr_executing_request_id = id;
    store_state(deps.storage, &state)?;
    let mut msgs = vec![];

    let input_asset = request.input_asset.clone();
    match input_asset.info {
        AssetInfo::NativeToken { denom: _ } => {
            msgs.push(SubMsg {
                id: 1,
                msg: CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: request.target.to_string(),
                    funds: vec![input_asset.deduct_tax(&deps.querier)?],
                    msg: request.msg,
                }),
                gas_limit: None,
                reply_on: ReplyOn::Success,
            });
        }
        AssetInfo::Token { contract_addr } => {
            if !request.input_asset.amount.is_zero() {
                msgs.push(SubMsg {
                    id: 0,
                    msg: CosmosMsg::Wasm(WasmMsg::Execute {
                        contract_addr: contract_addr.to_string(),
                        msg: to_binary(&Cw20ExecuteMsg::Transfer {
                            recipient: request.target.to_string(),
                            amount: request.input_asset.amount,
                        })?,
                        funds: vec![],
                    }),
                    gas_limit: None,
                    reply_on: ReplyOn::Never,
                });
            }
            msgs.push(SubMsg {
                id: 1,
                msg: CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: request.target.to_string(),
                    funds: vec![],
                    msg: request.msg,
                }),
                gas_limit: None,
                reply_on: ReplyOn::Success,
            });
        }
    }

    let fee_asset = Asset {
        info: AssetInfo::NativeToken {
            denom: config.fee_denom,
        },
        amount: config.fee_amount,
    };
    msgs.push(SubMsg {
        id: 0,
        msg: CosmosMsg::Bank(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![fee_asset.deduct_tax(&deps.querier)?],
        }),
        gas_limit: None,
        reply_on: ReplyOn::Never,
    });

    Ok(Response::new().add_submessages(msgs).add_attributes(vec![
        ("action", "execute_request"),
        ("id", id.to_string().as_str()),
    ]))
}

pub fn receive_cw20(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> StdResult<Response> {
    let config = read_config(deps.storage)?;

    match from_binary(&cw20_msg.msg) {
        Ok(Cw20HookMsg::Stake { num_stakes }) => {
            match config.auto {
                AssetInfo::Token { contract_addr } => {
                    // only auto token contract can execute this message
                    if contract_addr.to_string() != info.sender.to_string() {
                        return Err(StdError::generic_err("unauthorized"));
                    }
                }
                AssetInfo::NativeToken { denom: _ } => {
                    return Err(StdError::generic_err("invalid auto token"));
                }
            }

            let cw20_sender = deps.api.addr_validate(&cw20_msg.sender)?;
            stake(deps, env, info, &cw20_sender, num_stakes, cw20_msg.amount)
        }
        Err(_) => Err(StdError::generic_err("data should be given")),
    }
}

pub fn receive_denom(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    num_stakes: u64,
) -> StdResult<Response> {
    let config = read_config(deps.storage)?;

    match config.auto {
        AssetInfo::Token { contract_addr: _ } => {
            return Err(StdError::generic_err("unauthorized"));
        }
        AssetInfo::NativeToken { denom } => {
            let received_auto = info
                .funds
                .iter()
                .find(|c| c.denom == denom)
                .map(|c| c.amount)
                .unwrap_or(Uint128::zero());
            let staker = info.clone().sender;
            stake(deps, env, info, &staker, num_stakes, received_auto)
        }
    }
}

pub fn stake(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    sender: &Addr,
    num_stakes: u64,
    amount: Uint128,
) -> StdResult<Response> {
    if Uint128::from(STAN_STAKE) * Uint128::from(num_stakes) != amount {
        return Err(StdError::generic_err("invalid stake info"));
    }

    let mut state = read_state(deps.storage)?;
    _update_executor(&mut state, env);
    store_state(deps.storage, &state)?;

    // update stakes array
    for _ in 0..num_stakes {
        state.stakes.push(sender.to_string());
    }
    // add amount to stake balance
    let balance = read_balance(deps.storage, sender.clone()) + amount;
    store_balance(deps.storage, sender.clone(), &balance)?;
    state.total_staked += amount;
    store_state(deps.storage, &state)?;

    Ok(Response::new().add_attributes(vec![
        ("action", "stake"),
        ("user", sender.to_string().as_str()),
        ("num_stakes", num_stakes.to_string().as_str()),
    ]))
}

pub fn unstake(deps: DepsMut, env: Env, info: MessageInfo, idxs: Vec<u64>) -> StdResult<Response> {
    let mut state = read_state(deps.storage)?;
    let config = read_config(deps.storage)?;

    _update_executor(&mut state, env);
    store_state(deps.storage, &state)?;

    for i in 0..idxs.len() {
        let idx = idxs[i] as usize;
        if deps.api.addr_canonicalize(&state.stakes[idx])?
            != deps.api.addr_canonicalize(info.sender.as_str())?
        {
            return Err(StdError::generic_err("idx is not you"));
        }
        if idx >= state.stakes.len() {
            return Err(StdError::generic_err("idx out of bounds"));
        }
        state.stakes.swap_remove(idx);
    }
    // add amount to stake balance
    let amount = Uint128::from(idxs.len() as u64) * Uint128::from(STAN_STAKE);
    let balance = read_balance(deps.storage, info.sender.clone()) - amount;
    store_balance(deps.storage, info.sender.clone(), &balance)?;
    state.total_staked -= amount;
    store_state(deps.storage, &state)?;

    let mut msgs: Vec<CosmosMsg> = vec![];
    match config.auto {
        AssetInfo::Token { contract_addr: _ } => {
            msgs.push(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: config.auto.to_string(),
                msg: to_binary(&Cw20ExecuteMsg::Transfer {
                    recipient: info.sender.to_string(),
                    amount,
                })?,
                funds: vec![],
            }));
        }
        AssetInfo::NativeToken { denom } => {
            let asset = Asset {
                info: AssetInfo::NativeToken { denom },
                amount,
            };
            msgs.push(CosmosMsg::Bank(BankMsg::Send {
                to_address: info.sender.to_string(),
                amount: vec![asset.deduct_tax(&deps.querier)?],
            }));
        }
    }

    Ok(Response::new().add_messages(msgs).add_attributes(vec![
        ("action", "unstake"),
        ("user", info.sender.to_string().as_str()),
        ("count", idxs.len().to_string().as_str()),
    ]))
}

fn _update_executor(state: &mut State, env: Env) {
    let last_epoch = env.block.height / BLOCKS_IN_EPOCH * BLOCKS_IN_EPOCH;
    if state.last_epoch != last_epoch {
        let len = state.stakes.len() as u64;

        if len > 0 {
            let mut rng = oorandom::Rand64::new(env.block.height as u128);
            let index = rng.rand_u64() % len;
            state.executor = state.stakes[index as usize].clone();
            state.last_epoch = last_epoch;
        } else {
            state.executor = "".to_string();
        }
    }
}

pub fn update_executor(deps: DepsMut, env: Env) -> StdResult<Response> {
    let mut state = read_state(deps.storage)?;
    _update_executor(&mut state, env);
    store_state(deps.storage, &state)?;

    Ok(Response::new().add_attributes(vec![
        ("action", "update_executor"),
        ("epoch", state.last_epoch.to_string().as_str()),
        ("executor", state.executor.to_string().as_str()),
    ]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> StdResult<Response> {
    match msg.id {
        1 => execute_reply(deps, env, msg.result),
        _ => Err(StdError::generic_err("Unauthorized")),
    }
}

pub fn execute_reply(deps: DepsMut, _env: Env, _msg: SubMsgResult) -> StdResult<Response> {
    let mut state = read_state(deps.storage)?;
    state.curr_executing_request_id = u64::MAX;
    store_state(deps.storage, &state)?;
    Ok(Response::new().add_attributes(vec![("action", "finialize_execute")]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => Ok(to_binary(&query_config(deps)?)?),
        QueryMsg::RequestInfo { id } => Ok(to_binary(&query_request_info(deps, id)?)?),
        QueryMsg::Requests {
            start_after,
            limit,
            order_by,
        } => Ok(to_binary(&query_requests(
            deps,
            start_after,
            limit,
            order_by,
        )?)?),
        QueryMsg::State {} => Ok(to_binary(&query_state(deps)?)?),
        QueryMsg::EpochInfo {} => Ok(to_binary(&query_epoch_info(deps, env)?)?),
        QueryMsg::StakeAmount { user } => Ok(to_binary(&query_stake_amount(deps, user)?)?),
        QueryMsg::Stakes { start, limit } => Ok(to_binary(&query_stakes(deps, start, limit)?)?),
    }
}

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = read_config(deps.storage)?;
    let resp = ConfigResponse {
        auto: config.auto,
        stan_stake: Uint128::from(STAN_STAKE),
        blocks_in_epoch: BLOCKS_IN_EPOCH,
        fee_amount: config.fee_amount,
        fee_denom: config.fee_denom,
    };

    Ok(resp)
}

pub fn query_request_info(deps: Deps, id: u64) -> StdResult<RequestInfoResponse> {
    let info = read_request(deps.storage, id).unwrap_or(Request {
        user: "".to_string(),
        executor: "".to_string(),
        target: "".to_string(),
        msg: to_binary("")?,
        input_asset: Asset {
            info: AssetInfo::NativeToken {
                denom: "uluna".to_string(),
            },
            amount: Uint128::zero(),
        },
        created_at: 0,
        status: RequestStatus::Created,
    });
    Ok(RequestInfoResponse { id, request: info })
}

pub fn query_requests(
    deps: Deps,
    start_after: Option<u64>,
    limit: Option<u32>,
    order_by: Option<OrderBy>,
) -> StdResult<RequestsResponse> {
    let requests = if let Some(start_after) = start_after {
        read_requests(deps.storage, Some(start_after), limit, order_by)?
    } else {
        read_requests(deps.storage, None, limit, order_by)?
    };

    let requests_responses: StdResult<Vec<RequestInfoResponse>> = requests
        .iter()
        .map(|request| {
            Ok(RequestInfoResponse {
                id: request.0,
                request: request.1.clone(),
            })
        })
        .collect();

    Ok(RequestsResponse {
        requests: requests_responses?,
    })
}

pub fn query_state(deps: Deps) -> StdResult<StateResponse> {
    let state = read_state(deps.storage)?;
    let resp = StateResponse {
        curr_executing_request_id: state.curr_executing_request_id,
        total_requests: state.total_requests,
        total_stake_amount: state.total_staked,
        stakes_len: state.stakes.len() as u64,
    };

    Ok(resp)
}

pub fn query_epoch_info(deps: Deps, env: Env) -> StdResult<EpochInfoResponse> {
    let state = read_state(deps.storage)?;
    let cur_epoch = env.block.height / BLOCKS_IN_EPOCH * BLOCKS_IN_EPOCH;
    let resp = EpochInfoResponse {
        cur_epoch,
        last_epoch: state.last_epoch,
        executor: state.executor,
    };

    Ok(resp)
}

pub fn query_stake_amount(deps: Deps, user: String) -> StdResult<StakeAmountResponse> {
    let amount = read_balance(deps.storage, deps.api.addr_validate(&user)?);
    let resp = StakeAmountResponse { amount };

    Ok(resp)
}

pub fn query_stakes(deps: Deps, start: u64, limit: u64) -> StdResult<StakesResponse> {
    let state = read_state(deps.storage)?;

    let mut end = (start + limit) as usize;
    if end > state.stakes.len() {
        end = state.stakes.len()
    };
    let start = start as usize;

    Ok(StakesResponse {
        stakes: state.stakes[start..end].to_vec(),
    })
}
