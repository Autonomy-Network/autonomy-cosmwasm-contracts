use crate::contract::{execute, instantiate, query, reply};
use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg, StateResponse};
use crate::testing::mock_querier::mock_dependencies;

use autonomy::asset::{Asset, AssetInfo};
use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{
    attr, from_binary, to_binary, Addr, BankMsg, Coin, CosmosMsg, Decimal, Event, Reply, ReplyOn,
    StdError, SubMsg, SubMsgResponse, SubMsgResult, Uint128, WasmMsg,
};

#[test]
fn proper_initialization() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        auto: AssetInfo::Token {
            contract_addr: Addr::unchecked("auto"),
        },
        fee_amount: Uint128::from(0u128),
        fee_denom: "uosmo".to_string(),
    };

    let info = mock_info("addr0000", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    assert_eq!(
        from_binary::<ConfigResponse>(
            &query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap()
        )
        .unwrap(),
        ConfigResponse {
            auto: AssetInfo::Token {
                contract_addr: Addr::unchecked("auto")
            },
            stan_stake: Uint128::from(1000000u128),
            blocks_in_epoch: 100,
            fee_amount: Uint128::from(0u128),
            fee_denom: "uosmo".to_string(),
        }
    );
}

#[test]
fn create_execute_request() {
    let mut deps = mock_dependencies(&[]);
    deps.querier.with_tax(
        Decimal::percent(5),
        &[
            (&"uluna".to_string(), &Uint128::new(1000000u128)),
            (&"ukrw".to_string(), &Uint128::new(1000000u128)),
        ],
    );

    let msg = InstantiateMsg {
        auto: AssetInfo::Token {
            contract_addr: Addr::unchecked("auto"),
        },
        fee_amount: Uint128::from(10u128),
        fee_denom: "uosmo".to_string(),
    };

    let info = mock_info("addr0000", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let msg = ExecuteMsg::CreateRequest {
        target: "contract0000".to_string(),
        msg: to_binary("").unwrap(),
        input_asset: Asset {
            info: AssetInfo::NativeToken {
                denom: "uluna".to_string(),
            },
            amount: Uint128::zero(),
        },
    };

    let info = mock_info(
        "token",
        &[
            Coin {
                denom: "uluna".to_string(),
                amount: Uint128::from(10u128),
            },
            Coin {
                denom: "uosmo".to_string(),
                amount: Uint128::from(10u128),
            },
        ],
    );
    let env = mock_env();
    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(
        res.attributes,
        vec![attr("action", "create_request"), attr("id", "0")]
    );

    let msg = ExecuteMsg::ExecuteRequest { id: 0 };

    let info = mock_info("executor", &[]);
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();
    assert_eq!(
        res.attributes,
        vec![attr("action", "execute_request"), attr("id", "0"),]
    );
    assert_eq!(
        res.messages,
        vec![
            SubMsg {
                id: 1,
                msg: CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: "contract0000".to_string(),
                    msg: to_binary("").unwrap(),
                    funds: vec![Coin {
                        denom: "uluna".to_string(),
                        amount: Uint128::zero()
                    }],
                }),
                gas_limit: None,
                reply_on: ReplyOn::Success,
            },
            SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
                to_address: "executor".to_string(),
                amount: vec![Coin {
                    denom: "uosmo".to_string(),
                    amount: Uint128::from(10u128)
                }],
            }))
        ],
    );
}

#[test]
fn test_curr_executing_request_id() {
    let mut deps = mock_dependencies(&[]);
    deps.querier.with_tax(
        Decimal::percent(5),
        &[
            (&"uluna".to_string(), &Uint128::new(1000000u128)),
            (&"ukrw".to_string(), &Uint128::new(1000000u128)),
        ],
    );

    let msg = InstantiateMsg {
        auto: AssetInfo::Token {
            contract_addr: Addr::unchecked("auto"),
        },
        fee_amount: Uint128::from(10u128),
        fee_denom: "uosmo".to_string(),
    };

    let info = mock_info("addr0000", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let msg = ExecuteMsg::CreateRequest {
        target: "contract0000".to_string(),
        msg: to_binary("First request").unwrap(),
        input_asset: Asset {
            info: AssetInfo::NativeToken {
                denom: "uluna".to_string(),
            },
            amount: Uint128::zero(),
        },
    };

    let info = mock_info(
        "token",
        &[
            Coin {
                denom: "uluna".to_string(),
                amount: Uint128::from(10u128),
            },
            Coin {
                denom: "uosmo".to_string(),
                amount: Uint128::from(10u128),
            },
        ],
    );
    let env = mock_env();
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    // Create request 2 times
    let msg = ExecuteMsg::CreateRequest {
        target: "contract0000".to_string(),
        msg: to_binary("Second request").unwrap(),
        input_asset: Asset {
            info: AssetInfo::NativeToken {
                denom: "uluna".to_string(),
            },
            amount: Uint128::zero(),
        },
    };

    let info = mock_info(
        "token",
        &[
            Coin {
                denom: "uluna".to_string(),
                amount: Uint128::from(10u128),
            },
            Coin {
                denom: "uosmo".to_string(),
                amount: Uint128::from(10u128),
            },
        ],
    );
    let env = mock_env();
    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(
        res.attributes,
        vec![attr("action", "create_request"), attr("id", "1"),]
    );

    let msg = ExecuteMsg::ExecuteRequest { id: 1 };

    let info = mock_info("executor", &[]);
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();
    assert_eq!(
        res.attributes,
        vec![attr("action", "execute_request"), attr("id", "1"),]
    );

    // Query the "curr_executing_request_id"
    let res = query(deps.as_ref(), mock_env(), QueryMsg::State {}).unwrap();
    let state: StateResponse = from_binary(&res).unwrap();
    assert_eq!(state.curr_executing_request_id, 1);

    // test the reply method
    let events = vec![Event::new("wasm")];
    let result = SubMsgResult::Ok(SubMsgResponse { events, data: None });
    let subcall = Reply { id: 1, result };
    let _res = reply(deps.as_mut(), mock_env(), subcall).unwrap();

    // Query the "curr_executing_request_id"
    let res = query(deps.as_ref(), mock_env(), QueryMsg::State {}).unwrap();
    let state: StateResponse = from_binary(&res).unwrap();
    assert_eq!(state.curr_executing_request_id, u64::MAX);
}
