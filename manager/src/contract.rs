use crate::error::ContractError;
use cosmwasm_std::{
    entry_point, DepsMut, Env, MessageInfo, Reply, Response, StdResult, SubMsg, SubMsgResult,
};
use secret_toolkit::utils::HandleCallback;

use crate::counter::CounterExecuteMsg;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::EXECUTE_INCREMENT_REPLY_ID;

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Increment { contract } => try_increment(deps, env, contract),
    }
}

#[entry_point]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        EXECUTE_INCREMENT_REPLY_ID => handle_increment_reply(deps, msg),
        id => Err(ContractError::UnexpectedReplyId { id }),
    }
}

fn handle_increment_reply(_deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
    match msg.result {
        SubMsgResult::Ok(_) => Ok(Response::new().add_attribute("action", "increment")),

        SubMsgResult::Err(e) => Err(ContractError::CustomError { val: e }),
    }
}

pub fn try_increment(_deps: DepsMut, _env: Env, contract: String) -> StdResult<Response> {
    let exec_msg = CounterExecuteMsg::Increment { contract };

    let submsg = SubMsg::reply_always(
        exec_msg.to_cosmos_msg(
            "d3474b3c15ce262c78746f3536cd5f50657f0bc0b4020963947005134583e593".to_string(),
            "secret14q0jeyflxsd43zq3j82vkp08vp47r5ftt3glfr".to_string(),
            None,
        )?,
        EXECUTE_INCREMENT_REPLY_ID,
    );

    Ok(Response::new().add_submessage(submsg))
}
