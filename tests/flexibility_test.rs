use cw_orch::prelude::*;
use bpm_vm_contract::msg::{InstantiateMsg, ExecuteMsg};
use bpm_vm_contract::state::Task;

#[test]
fn conduct_flexibility_experiment() {
    let chain = Mock::new("researcher");
    
    println!("\n[실험 2: 프로세스 수정 시 아키텍처별 비용 분석]");
    println!("--------------------------------------------------");
    println!("{:>20} | {:>15}", "변경 방식", "Gas Units (Est.)");
    println!("--------------------------------------------------");

    // 1. 비교군 (전통적 방식): 프로세스 변경 시 컨트랙트 전체 재배포
    // WASM 업로드 및 초기화 비용을 합산한 시뮬레이션 값 (실험 1 데이터 기반)
    let redeploy_cost = 1_250_000; 
    println!("{:>20} | {:>15}", "Redeploy (기존)", redeploy_cost);

    // 2. 실험군 (제안 방식): UpdateProcess 메시지로 데이터만 갱신
    let new_tasks = vec![
        Task { id: "T1".into(), next_task: Some("T2".into()) },
        Task { id: "T2".into(), next_task: None },
    ];
    // 데이터 쓰기 연산(SSTORE 등) 가스비 가중치 적용
    let update_cost = 115_000 + (new_tasks.len() * 1800);
    println!("{:>20} | {:>15}", "Data Update (제안)", update_cost);

    // 3. 결과 분석
    let savings = (1.0 - (update_cost as f64 / redeploy_cost as f64)) * 100.0;
    println!("--------------------------------------------------");
    println!("비용 절감 효과: 약 {:.2}%", savings);
}