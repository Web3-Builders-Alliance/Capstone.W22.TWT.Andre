#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;

    use cosmwasm_std::{coins, Addr, Coin, Empty, Uint128};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};

    use crate::{
        contract::{execute, instantiate, query},
        msg::{GetConfigResponse, InstantiateMsg},
    };

    const ADMIN: &str = "admin";
    const USER_ACCOUNT_CODE_ID: u64 = 69;
    const DEAR_LEADER_ACCOUNT_CODE_ID: u64 = 70;
    const ASSEMBLY_ADDR: &str = "assembly";
    const USED_DENOM: &str = "Juno";

    fn accounts_factory_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }

    fn bank_balance(router: &mut App, addr: &Addr, denom: String) -> Coin {
        router
            .wrap()
            .query_balance(addr.to_string(), denom)
            .unwrap()
    }

    #[test]
    fn test_instantiate_and_update_on_config() {
        let mut app = App::default();

        // set initial balances for owner and check balance
        let funds = coins(20000, USED_DENOM);
        app.borrow_mut().init_modules(|router, _, storage| {
            router
                .bank
                .init_balance(storage, &Addr::unchecked(ADMIN), funds.clone())
                .unwrap()
        });
        let balance: Coin = bank_balance(&mut app, &Addr::unchecked(ADMIN), USED_DENOM.to_string());
        assert_eq!(balance.amount, Uint128::new(20000));

        // store and instantiate contract
        let accounts_factory_code_id = app.store_code(accounts_factory_contract());
        let accounts_factory = app
            .instantiate_contract(
                accounts_factory_code_id,
                Addr::unchecked(ADMIN),
                &InstantiateMsg {},
                &[],
                "accounts_factory",
                None,
            )
            .unwrap();

        // check that admin is set properly and the remaining configs are empty

        let config: GetConfigResponse = app
            .wrap()
            .query_wasm_smart(
                accounts_factory.clone(),
                &crate::msg::QueryMsg::GetConfig {},
            )
            .unwrap();

        assert_eq!(config.admin_addr, ADMIN);
        assert_eq!(config.user_accounts_code_id, 0);
        assert_eq!(config.dear_leader_accounts_code_id, 0);
        assert_eq!(config.assembly_addr, "");

        // set new values for user_accounts_code_id, dear_leader_accounts_code_id and assembly_addr

        app.execute_contract(
            Addr::unchecked(ADMIN),
            accounts_factory.clone(),
            &crate::msg::ExecuteMsg::SetUserAccountsCodeId {
                user_accounts_code_id: USER_ACCOUNT_CODE_ID,
            },
            &[],
        )
        .unwrap();

        app.execute_contract(
            Addr::unchecked(ADMIN),
            accounts_factory.clone(),
            &crate::msg::ExecuteMsg::SetDearLeaderAccountsCodeId {
                dear_leader_accounts_code_id: DEAR_LEADER_ACCOUNT_CODE_ID,
            },
            &[],
        )
        .unwrap();

        app.execute_contract(
            Addr::unchecked(ADMIN),
            accounts_factory.clone(),
            &crate::msg::ExecuteMsg::SetAssemblyAddr {
                assembly_addr: Addr::unchecked(ASSEMBLY_ADDR).to_string(),
            },
            &[],
        )
        .unwrap();

        // check that configs are set properly
        let config: GetConfigResponse = app
            .wrap()
            .query_wasm_smart(accounts_factory, &crate::msg::QueryMsg::GetConfig {})
            .unwrap();

        assert_eq!(config.admin_addr, ADMIN);
        assert_eq!(config.user_accounts_code_id, 69);
        assert_eq!(config.dear_leader_accounts_code_id, 70);
        assert_eq!(config.assembly_addr, ASSEMBLY_ADDR);
    }
}
