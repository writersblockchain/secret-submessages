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
        SubMsgResult::Ok(_s) => {
            todo!()
        }

        SubMsgResult::Err(e) => Err(ContractError::CustomError { val: e }),
    }
}

pub fn try_increment(_deps: DepsMut, _env: Env, contract: String) -> StdResult<Response> {
    let exec_msg = CounterExecuteMsg::Increment { contract };

    let submsg = SubMsg::reply_always(
        exec_msg.to_cosmos_msg(
            "cf6c359e936ded4e18716aafdef4d880cc42e4d87c29ca88205ff38c1ddf6531".to_string(),
            "secret1edd6prk0w55c27dkcxzuau8mvlwa2rghgwelqk".to_string(),
            None,
        )?,
        EXECUTE_INCREMENT_REPLY_ID,
    );

    Ok(Response::new().add_submessage(submsg))
}
