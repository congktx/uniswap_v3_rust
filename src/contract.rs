#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, WasmMsg};
use cw20::Cw20ExecuteMsg;
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:rust-project";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    if !(info.funds.contains(&Coin{
        denom: "orai".to_string(),
        amount: Uint128::one()
    })) {
        return Err(ContractError::Unauthorized {  })
    }
    let contract_addr = info.sender.clone();
    let transfer_msg = WasmMsg::Execute {
        contract_addr: contract_addr.to_string(), 
        msg: to_json_binary(&Cw20ExecuteMsg::Transfer { recipient: info.sender.to_string(), amount: Uint128::one() })?,
        funds: vec![]
    };

    Ok(Response::default().add_message(transfer_msg))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
