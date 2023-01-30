use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

pub const ASSEMBLY_ADDR: Item<String> = Item::new("assembly");
// pub const DEAR_LEADER_ADDR: Item<String> = Item::new("dear_leader");
pub const BOSS_ADDR: Item<String> = Item::new("boss");

pub const ALLOWED_TOKENS: Map<String, CheckedTokenInfo> = Map::new("proposal_vote_history");

#[cw_serde]
pub enum CheckedTokenInfo {
    Native {
        denom: String,
        amount: Uint128,
    },
    Cw20 {
        contract_addr: Addr,
        amount: Uint128,
    },
}
