#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_info, mock_env};
    use empty_contract::{msg::{InstantiateMsg, ExecuteMsg}, contract::{instantiate, execute}};

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg{};
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn test_execute() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &[]);
        let msg = ExecuteMsg::DoNothing{};
        let res = execute(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }
}


