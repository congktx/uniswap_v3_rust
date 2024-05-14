use cosmwasm_schema::{cw_serde, QueryResponses};
// use uniswap_sdk_core::{token};
use crate::prelude::*;
#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Example { a: u64, b: u64 },
    make_pool { token0: Token, token1: Token}
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
