use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;

// pub const ACCOUNTS_FACTORY_ADDR: Item<String> = Item::new("accounts_factory");
pub const ASSEMBLY_ADDR: Item<String> = Item::new("assembly");
pub const CONFIG: Item<Config> = Item::new("config");

#[cw_serde]
pub struct Config {
    /// The name of the contract.
    pub owner: String,
}
