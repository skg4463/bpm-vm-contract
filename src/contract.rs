// 핵심 로직

use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdResult, StdError};
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{DEFINITION, INSTANCES, InstanceState};

#[entry_point]
pub fn instantiate(deps: DepsMut, _env: Env, _info: MessageInfo, msg: InstantiateMsg) -> StdResult<Response> {
    DEFINITION.save(deps.storage, &msg.tasks)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(deps: DepsMut, _env: Env, _info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::MoveNext { instance_id } => move_next(deps, instance_id),
        ExecuteMsg::UpdateProcess { new_tasks } => {
            DEFINITION.save(deps.storage, &new_tasks)?;
            Ok(Response::new().add_attribute("action", "update_process"))
        }
    }
}

fn move_next(deps: DepsMut, instance_id: String) -> StdResult<Response> {
    let definition = DEFINITION.load(deps.storage)?;
    
    // 기존 인스턴스를 불러오거나, 없으면 첫 번째 태스크에서 시작
    let mut state = INSTANCES.may_load(deps.storage, &instance_id)?.unwrap_or(InstanceState {
        current_task_id: definition[0].id.clone(),
        is_completed: false,
    });

    if state.is_completed {
        return Err(StdError::generic_err("이미 완료된 프로세스입니다."));
    }

    // 현재 위치한 태스크 정보를 찾아 다음 단계 확인
    let current_task = definition.iter().find(|t| t.id == state.current_task_id)
        .ok_or(StdError::generic_err("태스크를 찾을 수 없습니다."))?;

    match &current_task.next_task {
        Some(next_id) => state.current_task_id = next_id.clone(),
        None => state.is_completed = true,
    }

    INSTANCES.save(deps.storage, &instance_id, &state)?;
    Ok(Response::new()
        .add_attribute("instance_id", instance_id)
        .add_attribute("new_task_id", &state.current_task_id))
}