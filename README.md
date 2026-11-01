# bpm-vm-contract

BPMN 스타일의 업무 프로세스(워크플로우)를 블록체인 위에서 실행하는 **CosmWasm 스마트 컨트랙트**입니다.

프로세스 정의(설계도)를 코드가 아니라 **컨트랙트 상태 데이터**로 저장합니다. 그래서 프로세스를 바꿀 때 컨트랙트를 다시 배포하지 않고 메시지 하나(`UpdateProcess`)로 정의를 교체할 수 있습니다. 이 프로젝트는 이 방식의 유연성·비용·상호운용성을 살펴보는 연구용 프로토타입입니다.

프로젝트 “A Decentralized and Flexible BPM Framework Based on Blockchain VM Interpreter and Inter-Blockchain Communication.”, Telecom. 2026

## 주요 개념

| 개념 | 설명 |
| --- | --- |
| **Task** | 프로세스의 한 단계. `id`와 다음 단계 `next_task`(없으면 마지막 단계)를 가짐 |
| **Definition** | `Task` 목록으로 이루어진 프로세스 설계도. 컨트랙트당 하나 |
| **Instance** | 설계도를 따라 실제로 진행되는 개별 실행 건. `instance_id`로 구분 |

## 프로젝트 구조

```
.
├── src/
│   ├── contract.rs   # 엔트리 포인트(instantiate, execute)와 핵심 로직
│   ├── msg.rs        # InstantiateMsg / ExecuteMsg 메시지 규격
│   ├── state.rs      # Task, InstanceState 구조체와 저장소(DEFINITION, INSTANCES)
│   ├── error.rs      # ContractError 정의
│   └── lib.rs        # 모듈 선언
├── tests/
│   ├── gas_experiment.rs     # 실험 1: 태스크 수에 따른 가스 소모량
│   ├── flexibility_test.rs   # 실험 2: 재배포 vs 데이터 업데이트 비용 비교
│   └── interop_test.rs       # 실험 3: IBC 기반 교차 체인 동기화 시나리오 로그
├── artifacts/
│   ├── bpm_vm_contract.wasm  # 빌드된 WASM 바이너리
│   └── checksums.txt         # WASM 체크섬(SHA-256)
└── .cargo/config.toml        # `cargo wasm` 별칭
```

## 상태(State)

```rust
pub struct Task {
    pub id: String,
    pub next_task: Option<String>,
}

pub struct InstanceState {
    pub current_task_id: String,
    pub is_completed: bool,
}

pub const DEFINITION: Item<Vec<Task>> = Item::new("definition");
pub const INSTANCES: Map<&str, InstanceState> = Map::new("instances");
```

## 메시지

### Instantiate

프로세스 정의를 저장하며 컨트랙트를 초기화합니다.

```json
{
  "tasks": [
    { "id": "T1", "next_task": "T2" },
    { "id": "T2", "next_task": "T3" },
    { "id": "T3", "next_task": null }
  ]
}
```

### Execute

#### `move_next`

인스턴스를 다음 단계로 진행합니다.

```json
{ "move_next": { "instance_id": "order-001" } }
```

- 처음 호출된 `instance_id`는 정의의 **첫 번째 태스크**에서 시작한 뒤 바로 다음 태스크로 진행합니다.
- 현재 태스크의 `next_task`가 `null`이면 인스턴스가 완료(`is_completed = true`)됩니다.
- 이미 완료된 인스턴스에 호출하면 `ProcessCompleted` 에러가 납니다.
- 현재 태스크를 정의에서 찾을 수 없으면 `TaskNotFound` 에러가 납니다. 진행 중에 `update_process`로 정의가 바뀐 경우가 여기에 해당합니다.
- 응답 attribute: `action=move_next`, `current_task=<현재 태스크 id>`

#### `update_process`

프로세스 정의를 통째로 교체합니다. 재배포 없이 프로세스를 바꾸는 핵심 기능입니다.

```json
{
  "update_process": {
    "new_tasks": [
      { "id": "T1", "next_task": "T2" },
      { "id": "T2", "next_task": null }
    ]
  }
}
```

- 응답 attribute: `action=update_process`

### 에러

| 에러 | 메시지 |
| --- | --- |
| `Std` | CosmWasm 표준 에러 |
| `Unauthorized` | `Unauthorized` |
| `TaskNotFound` | `Task not found` |
| `ProcessCompleted` | `Process already completed` |

## 빌드

### 요구 사항

- Rust (stable)
- `wasm32-unknown-unknown` 타깃

```bash
rustup target add wasm32-unknown-unknown
```

### WASM 빌드

`.cargo/config.toml`에 별칭이 설정되어 있습니다.

```bash
cargo wasm
```

이 명령은 `cargo build --release --target wasm32-unknown-unknown`과 같습니다. 결과물은 `target/wasm32-unknown-unknown/release/bpm_vm_contract.wasm`에 생깁니다.

체인 배포용으로 최적화된 바이너리가 필요하면 [cosmwasm/optimizer](https://github.com/CosmWasm/optimizer)를 사용하세요.

```bash
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/optimizer:0.15.0
```

## 테스트 / 실험

```bash
cargo test -- --nocapture
```

`--nocapture`를 붙여야 각 실험의 결과표가 출력됩니다. 실험 하나만 실행하려면 `--test`로 지정합니다.

```bash
cargo test --test gas_experiment -- --nocapture
```

| 테스트 | 내용 |
| --- | --- |
| `gas_experiment` | 태스크 수(N = 5, 10, 20, 50, 100)별 가스 소모량 표 |
| `flexibility_test` | 전체 재배포 방식과 `UpdateProcess` 방식의 비용 비교 및 절감률 |
| `interop_test` | Chain A → Relayer → Chain B로 이어지는 IBC 워크플로우 동기화 시나리오 로그 |

> **참고:** 현재 실험 코드의 수치는 컨트랙트를 실제로 실행해 측정한 값이 아닙니다. 가스 값은 코드에 고정된 계수(기본값 + 태스크당 가중치)로 계산하고, IBC 로그는 미리 작성한 시나리오 문자열을 출력합니다. `cw-orch`의 `Mock` 체인을 만들기는 하지만 컨트랙트를 업로드하거나 호출하지는 않습니다. 논문 등에 실측 데이터로 쓰려면 Mock/테스트넷에서 컨트랙트를 실제로 실행해 가스를 측정하도록 바꿔야 합니다.

## 알려진 제한 사항 / TODO

- `update_process`에 권한 검사가 없어 누구나 프로세스 정의를 바꿀 수 있습니다. (`Unauthorized` 에러는 정의만 되어 있음)
- 빈 태스크 목록으로 초기화한 뒤 `move_next`를 호출하면 패닉이 납니다. (`definition[0]` 접근)
- 처음 호출된 인스턴스는 첫 태스크를 건너뛰고 바로 두 번째 태스크로 진행합니다.
- 조회(`query`) 엔트리 포인트가 없어 인스턴스 상태를 읽을 수 없습니다.
- 분기(gateway), 병렬 흐름 등 BPMN의 다른 요소는 지원하지 않고 단일 선형 흐름만 지원합니다.
- IBC 연동은 아직 구현되지 않았습니다. (시나리오 로그만 있음)
- `src/lib.rs`에 템플릿 코드(`add` 함수와 테스트)가 남아 있습니다.

## 의존성

- [cosmwasm-std](https://crates.io/crates/cosmwasm-std) 1.5
- [cw-storage-plus](https://crates.io/crates/cw-storage-plus) 1.2
- [schemars](https://crates.io/crates/schemars), [serde](https://crates.io/crates/serde), [thiserror](https://crates.io/crates/thiserror)
- 개발용: [cw-orch](https://crates.io/crates/cw-orch) 0.22, [tokio](https://crates.io/crates/tokio)
