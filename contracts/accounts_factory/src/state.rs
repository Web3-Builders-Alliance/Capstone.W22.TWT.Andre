use cw_storage_plus::{Item, Map};

// contract admin.
pub const ADMIN_ADDR: Item<String> = Item::new("admin_addr");

// owner/user_account
pub const USER_ACCOUNTS_UNDER_MANAGEMENT: Map<String, String> =
    Map::new("user_accounts_under_management");

pub const DEAR_LEADER_ACCOUNTS_UNDER_MANAGEMENT: Map<String, String> =
    Map::new("dear_leader_accounts_under_management");

pub const STATE_MANAGEMENT_CACHE_WITHIN_TX: Item<String> =
    Item::new("state_management_cache_within_tx");

pub const USER_ACCOUNTS_CODE_ID: Item<u64> = Item::new("user_accounts_code_id");

pub const DEAR_LEADER_ACCOUNTS_CODE_ID: Item<u64> = Item::new("dear_leader_accounts_code_id");

pub const ASSEMBLY_ADDR: Item<String> = Item::new("assembly_addr");
