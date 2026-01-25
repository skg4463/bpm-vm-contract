// 핵심 로직

use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{DEFINITION, INSTANCES, InstanceState};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    DEFINITION.save(deps.storage, &msg.tasks)?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("task_count", msg.tasks.len().to_string()))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::MoveNext { instance_id } => move_next(deps, instance_id),
        ExecuteMsg::UpdateProcess { new_tasks } => {
            DEFINITION.save(deps.storage, &new_tasks)?;
            Ok(Response::new().add_attribute("action", "update_process"))
        }
    }
}

fn move_next(deps: DepsMut, instance_id: String) -> Result<Response, ContractError> {
    let definition = DEFINITION.load(deps.storage)?;
    
    let mut state = INSTANCES.may_load(deps.storage, &instance_id)?.unwrap_or(InstanceState {
        current_task_id: definition[0].id.clone(),
        is_completed: false,
    });

    if state.is_completed {
        return Err(ContractError::ProcessCompleted {});
    }

    let current_task = definition.iter().find(|t| t.id == state.current_task_id)
        .ok_or(ContractError::TaskNotFound {})?;

    match &current_task.next_task {
        Some(next_id) => state.current_task_id = next_id.clone(),
        None => state.is_completed = true,
    }

    INSTANCES.save(deps.storage, &instance_id, &state)?;
    Ok(Response::new()
        .add_attribute("action", "move_next")
        .add_attribute("current_task", &state.current_task_id))
}