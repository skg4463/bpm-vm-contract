use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError), // 기본 표준 에러

    #[error("Unauthorized")]
    Unauthorized {}, // 권한 없음

    #[error("Task not found")]
    TaskNotFound {}, // 존재하지 않는 태스크 요청 시

    #[error("Process already completed")]
    ProcessCompleted {}, // 이미 완료된 프로세스 진행 시
}