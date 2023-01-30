use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[cw_serde]
pub struct CwTemplateContract(pub Addr);
