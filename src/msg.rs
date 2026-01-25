// 메시지 규격 선언 

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::Task;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // 프로세스 엔진을 한 단계 전진시킴
    MoveNext { instance_id: String },
    // 논문의 핵심: 코드 수정 없이 프로세스 정의만 트랜잭션으로 업데이트
    UpdateProcess { new_tasks: Vec<Task> },
}