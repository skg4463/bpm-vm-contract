// 데이터 구조 선언 

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_storage_plus::{Item, Map};

// BPMN 태스크 구조체: 논문에서 "워크플로우 설계 도식"에 대응함
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Task {
    pub id: String,
    pub next_task: Option<String>,
}

// 개별 프로세스 실행 건의 상태
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstanceState {
    pub current_task_id: String,
    pub is_completed: bool,
}

// 고정된 프로세스 정의 (설계도) 저장
pub const DEFINITION: Item<Vec<Task>> = Item::new("definition");
// 실행 중인 인스턴스들의 진행 상황 관리
pub const INSTANCES: Map<&str, InstanceState> = Map::new("instances");