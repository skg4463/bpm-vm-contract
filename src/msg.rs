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
    // 다음 단계로 전진
    MoveNext { instance_id: String },
    // 프로세스 설계도 동적 업데이트 (유연성 실험 핵심)
    UpdateProcess { new_tasks: Vec<Task> },
}