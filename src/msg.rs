use cosmwasm_schema::{cw_serde, QueryResponses};
use crate::prelude::*;
#[cw_serde]
pub struct  InstantiateMsg {
}

#[cw_serde]
pub enum ExecuteMsg {
    Example { a: u64, b: u64 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
