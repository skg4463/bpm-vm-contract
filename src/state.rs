// 데이터 구조 선언 

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_storage_plus::{Item, Map};

// BPMN 태스크 구조체: 논문에서 "프로세스 설계도"로 지칭함
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Task {
    pub id: String,
    pub task_type: String, // 예: "UserTask", "ServiceTask"
    pub next_task: Option<String>,
}

// 인스턴스 상태: 특정 비즈니스 건의 현재 진행 위치
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstanceState {
    pub current_task_id: String,
    pub is_completed: bool,
}

// 프로세스 정의 (설계도) 저장소
pub const DEFINITION: Item<Vec<Task>> = Item::new("definition");
// 개별 실행 인스턴스 저장소 (ID별 매핑)
pub const INSTANCES: Map<&str, InstanceState> = Map::new("instances");