#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;

    use cosmwasm_std::{coins, Addr, Coin, Empty, Uint128};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};

    use crate::{
        contract::{execute, instantiate, query},
        msg::{ExecuteMsg, GetConfigResponse, InstantiateMsg},
    };

    const ADMIN: &str = "admin";
    const ACCOUNT_FACTORY: &str = "account_factory";
    const USED_DENOM: &str = "Juno";

    fn assembly_contract() -> Box<dyn Contract<Empty>> {
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
        let assembly_code_id = app.store_code(assembly_contract());
        let assembly = app
            .instantiate_contract(
                assembly_code_id,
                Addr::unchecked(ADMIN),
                &InstantiateMsg {},
                &[],
                "assembly",
                None,
            )
            .unwrap();

        // check that admin is set properly and the remaining configs are empty
        let config: GetConfigResponse = app
            .wrap()
            .query_wasm_smart(&assembly, &&crate::msg::QueryMsg::GetConfig {})
            .unwrap();

        assert_eq!(config.admin_addr, ADMIN);

        // update account factory address
        app.execute_contract(
            Addr::unchecked(ADMIN),
            assembly.clone(),
            &ExecuteMsg::SetAccountFactoryAddr {
                account_factory_addr: ACCOUNT_FACTORY.to_string(),
            },
            &[],
        )
        .unwrap();

        // check that account_factory is set properly
        let config: GetConfigResponse = app
            .wrap()
            .query_wasm_smart(assembly, &&crate::msg::QueryMsg::GetConfig {})
            .unwrap();

        assert_eq!(config.admin_addr, ADMIN);
        assert_eq!(config.accounts_factory_addr, ACCOUNT_FACTORY);
    }
}
