# Todo List 테스트 전략 및 구조

## 1. 테스트 전략 개요

### 1.1 테스트 피라미드
```
        ┌─────────────────┐
        │   E2E Tests     │  ← 적은 수의 통합 테스트
        │    (CLI)        │
        ├─────────────────┤
        │ Integration     │  ← 중간 수준의 통합 테스트
        │    Tests        │
        ├─────────────────┤
        │   Unit Tests    │  ← 많은 수의 단위 테스트
        │  (각 모듈별)     │
        └─────────────────┘
```

### 1.2 테스트 범위
- **단위 테스트**: 개별 함수와 메서드 검증
- **통합 테스트**: 모듈 간 상호작용 검증
- **End-to-End 테스트**: CLI 명령어 전체 플로우 검증

## 2. 테스트 디렉토리 구조

```
tests/
├── common/
│   ├── mod.rs              # 테스트 공통 유틸리티
│   ├── fixtures.rs         # 테스트 데이터 생성
│   └── test_storage.rs     # 테스트용 Storage 구현
├── unit/
│   ├── models/
│   │   └── todo_tests.rs   # Todo 모델 단위 테스트
│   ├── storage/
│   │   ├── file_storage_tests.rs
│   │   └── memory_storage_tests.rs
│   └── service/
│       └── todo_service_tests.rs
├── integration/
│   ├── storage_integration_tests.rs
│   ├── service_integration_tests.rs
│   └── cli_integration_tests.rs
└── e2e/
    └── cli_e2e_tests.rs    # End-to-End CLI 테스트
```

## 3. 단위 테스트 (Unit Tests)

### 3.1 모델 테스트 (models/todo_tests.rs)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_todo_creation() {
        let todo = Todo::new("Test Todo".to_string());

        assert_eq!(todo.title, "Test Todo");
        assert!(!todo.completed);
        assert_eq!(todo.priority, Priority::Medium);
        assert!(todo.description.is_none());
        assert!(todo.created_at <= Utc::now());
        assert_eq!(todo.created_at, todo.updated_at);
    }

    #[test]
    fn test_todo_completion() {
        let mut todo = Todo::new("Test Todo".to_string());
        let initial_updated_at = todo.updated_at;

        // 잠시 대기 (updated_at 변경 확인용)
        std::thread::sleep(std::time::Duration::from_millis(1));

        todo.complete();

        assert!(todo.completed);
        assert!(todo.updated_at > initial_updated_at);
    }

    #[test]
    fn test_todo_toggle() {
        let mut todo = Todo::new("Test Todo".to_string());

        assert!(!todo.completed);
        todo.toggle_complete();
        assert!(todo.completed);
        todo.toggle_complete();
        assert!(!todo.completed);
    }

    #[test]
    fn test_priority_ordering() {
        assert!(Priority::High > Priority::Medium);
        assert!(Priority::Medium > Priority::Low);
    }

    #[test]
    fn test_priority_from_string() {
        assert_eq!(Priority::from_str("high").unwrap(), Priority::High);
        assert_eq!(Priority::from_str("medium").unwrap(), Priority::Medium);
        assert_eq!(Priority::from_str("low").unwrap(), Priority::Low);
        assert!(Priority::from_str("invalid").is_err());
    }
}
```

### 3.2 메모리 저장소 테스트 (storage/memory_storage_tests.rs)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Todo, Priority};

    fn create_test_todo(title: &str) -> Todo {
        let mut todo = Todo::new(title.to_string());
        todo.set_priority(Priority::High);
        todo
    }

    #[test]
    fn test_save_and_find_todo() {
        let mut storage = MemoryStorage::new();
        let todo = create_test_todo("Test Todo");
        let todo_id = todo.id;

        storage.save(&todo).unwrap();

        let found_todo = storage.find_by_id(&todo_id).unwrap();
        assert!(found_todo.is_some());
        assert_eq!(found_todo.unwrap().title, "Test Todo");
    }

    #[test]
    fn test_find_all_todos() {
        let mut storage = MemoryStorage::new();
        let todo1 = create_test_todo("Todo 1");
        let todo2 = create_test_todo("Todo 2");

        storage.save(&todo1).unwrap();
        storage.save(&todo2).unwrap();

        let all_todos = storage.find_all().unwrap();
        assert_eq!(all_todos.len(), 2);
    }

    #[test]
    fn test_update_todo() {
        let mut storage = MemoryStorage::new();
        let mut todo = create_test_todo("Original Title");
        let todo_id = todo.id;

        storage.save(&todo).unwrap();

        todo.update_title("Updated Title".to_string());
        storage.update(&todo).unwrap();

        let updated_todo = storage.find_by_id(&todo_id).unwrap().unwrap();
        assert_eq!(updated_todo.title, "Updated Title");
    }

    #[test]
    fn test_delete_todo() {
        let mut storage = MemoryStorage::new();
        let todo = create_test_todo("To Delete");
        let todo_id = todo.id;

        storage.save(&todo).unwrap();
        assert!(storage.find_by_id(&todo_id).unwrap().is_some());

        let deleted = storage.delete(&todo_id).unwrap();
        assert!(deleted);
        assert!(storage.find_by_id(&todo_id).unwrap().is_none());
    }

    #[test]
    fn test_find_by_priority() {
        let mut storage = MemoryStorage::new();

        let mut high_todo = create_test_todo("High Priority");
        high_todo.set_priority(Priority::High);

        let mut low_todo = create_test_todo("Low Priority");
        low_todo.set_priority(Priority::Low);

        storage.save(&high_todo).unwrap();
        storage.save(&low_todo).unwrap();

        let high_todos = storage.find_by_priority(Priority::High).unwrap();
        assert_eq!(high_todos.len(), 1);
        assert_eq!(high_todos[0].title, "High Priority");
    }

    #[test]
    fn test_clear_completed() {
        let mut storage = MemoryStorage::new();

        let mut completed_todo = create_test_todo("Completed");
        completed_todo.complete();

        let pending_todo = create_test_todo("Pending");

        storage.save(&completed_todo).unwrap();
        storage.save(&pending_todo).unwrap();

        let cleared_count = storage.clear_completed().unwrap();
        assert_eq!(cleared_count, 1);

        let remaining_todos = storage.find_all().unwrap();
        assert_eq!(remaining_todos.len(), 1);
        assert_eq!(remaining_todos[0].title, "Pending");
    }
}
```

### 3.3 서비스 레이어 테스트 (service/todo_service_tests.rs)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::MemoryStorage;
    use crate::models::Priority;

    fn create_test_service() -> TodoService<MemoryStorage> {
        TodoService::new(MemoryStorage::new())
    }

    #[test]
    fn test_create_todo() {
        let mut service = create_test_service();

        let todo = service.create_todo("New Task".to_string()).unwrap();
        assert_eq!(todo.title, "New Task");
        assert!(!todo.completed);
    }

    #[test]
    fn test_create_todo_with_priority() {
        let mut service = create_test_service();

        let todo = service.create_todo_with_priority(
            "High Priority Task".to_string(),
            Priority::High
        ).unwrap();

        assert_eq!(todo.priority, Priority::High);
    }

    #[test]
    fn test_complete_todo() {
        let mut service = create_test_service();

        let todo = service.create_todo("Task to Complete".to_string()).unwrap();
        let todo_id = todo.id;

        service.complete_todo(&todo_id).unwrap();

        let updated_todo = service.get_todo_by_id(&todo_id).unwrap().unwrap();
        assert!(updated_todo.completed);
    }

    #[test]
    fn test_delete_nonexistent_todo() {
        let mut service = create_test_service();
        let random_id = uuid::Uuid::new_v4();

        let result = service.delete_todo(&random_id);
        assert!(matches!(result, Err(TodoError::NotFound(_))));
    }

    #[test]
    fn test_get_todos_by_status() {
        let mut service = create_test_service();

        // 완료된 할 일 생성
        let todo1 = service.create_todo("Completed Task".to_string()).unwrap();
        service.complete_todo(&todo1.id).unwrap();

        // 미완료 할 일 생성
        service.create_todo("Pending Task".to_string()).unwrap();

        let completed_todos = service.get_completed_todos().unwrap();
        let pending_todos = service.get_pending_todos().unwrap();

        assert_eq!(completed_todos.len(), 1);
        assert_eq!(pending_todos.len(), 1);
        assert_eq!(completed_todos[0].title, "Completed Task");
        assert_eq!(pending_todos[0].title, "Pending Task");
    }
}
```

## 4. 통합 테스트 (Integration Tests)

### 4.1 파일 저장소 통합 테스트 (integration/storage_integration_tests.rs)

```rust
use tempfile::TempDir;
use std::path::PathBuf;

#[test]
fn test_file_storage_persistence() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test_todos.json");

    // 첫 번째 저장소 인스턴스에서 데이터 저장
    {
        let mut storage = FileStorage::new(file_path.clone());
        let todo = Todo::new("Persistent Todo".to_string());
        storage.save(&todo).unwrap();
    }

    // 두 번째 저장소 인스턴스에서 데이터 로드
    {
        let storage = FileStorage::new(file_path);
        let todos = storage.find_all().unwrap();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].title, "Persistent Todo");
    }
}

#[test]
fn test_file_storage_concurrent_access() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("concurrent_test.json");

    let storage = Arc::new(Mutex::new(FileStorage::new(file_path)));
    let mut handles = vec![];

    // 여러 스레드에서 동시에 할 일 추가
    for i in 0..5 {
        let storage_clone = Arc::clone(&storage);
        let handle = thread::spawn(move || {
            let todo = Todo::new(format!("Todo {}", i));
            let mut storage = storage_clone.lock().unwrap();
            storage.save(&todo).unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let storage = storage.lock().unwrap();
    let todos = storage.find_all().unwrap();
    assert_eq!(todos.len(), 5);
}
```

### 4.2 CLI 통합 테스트 (integration/cli_integration_tests.rs)

```rust
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn setup_test_cli() -> (Command, TempDir) {
    let temp_dir = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("todo-list").unwrap();
    cmd.env("TODO_DATA_PATH", temp_dir.path().join("todos.json"));
    (cmd, temp_dir)
}

#[test]
fn test_add_and_list_todo() {
    let (mut cmd, _temp_dir) = setup_test_cli();

    // 할 일 추가
    cmd.args(&["add", "Test Todo"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Todo added successfully"));

    // 목록 조회
    let (mut cmd, _) = setup_test_cli();
    cmd.args(&["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Test Todo"));
}

#[test]
fn test_complete_todo() {
    let (mut cmd, _temp_dir) = setup_test_cli();

    // 할 일 추가
    cmd.args(&["add", "Todo to Complete"])
        .assert()
        .success();

    // 첫 번째 항목 완료 (인덱스 1 사용)
    let (mut cmd, _) = setup_test_cli();
    cmd.args(&["complete", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Todo completed"));

    // 완료된 항목 확인
    let (mut cmd, _) = setup_test_cli();
    cmd.args(&["list", "--filter", "completed"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Todo to Complete"));
}

#[test]
fn test_invalid_command() {
    let (mut cmd, _temp_dir) = setup_test_cli();

    cmd.args(&["invalid-command"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("error"));
}
```

## 5. End-to-End 테스트

### 5.1 CLI E2E 테스트 (e2e/cli_e2e_tests.rs)

```rust
#[test]
fn test_complete_todo_workflow() {
    let (mut cmd, _temp_dir) = setup_test_cli();

    // 1. 여러 할 일 추가
    cmd.args(&["add", "Buy groceries", "--priority", "high"])
        .assert().success();

    let (mut cmd, _) = setup_test_cli();
    cmd.args(&["add", "Walk the dog", "--priority", "medium"])
        .assert().success();

    // 2. 목록 확인
    let (mut cmd, _) = setup_test_cli();
    cmd.args(&["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Buy groceries"))
        .stdout(predicate::str::contains("Walk the dog"));

    // 3. 첫 번째 항목 완료
    let (mut cmd, _) = setup_test_cli();
    cmd.args(&["complete", "1"])
        .assert().success();

    // 4. 미완료 항목만 조회
    let (mut cmd, _) = setup_test_cli();
    cmd.args(&["list", "--filter", "pending"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Walk the dog"))
        .stdout(predicate::str::contains("Buy groceries").not());

    // 5. 완료된 항목 삭제
    let (mut cmd, _) = setup_test_cli();
    cmd.args(&["clear"])
        .assert().success();

    // 6. 최종 목록 확인
    let (mut cmd, _) = setup_test_cli();
    cmd.args(&["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Walk the dog"))
        .stdout(predicate::str::contains("Buy groceries").not());
}
```

## 6. 테스트 공통 유틸리티

### 6.1 Test Fixtures (common/fixtures.rs)

```rust
use crate::models::{Todo, Priority};
use uuid::Uuid;

pub struct TodoBuilder {
    title: String,
    description: Option<String>,
    priority: Priority,
    completed: bool,
}

impl TodoBuilder {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            description: None,
            priority: Priority::Medium,
            completed: false,
        }
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }

    pub fn completed(mut self) -> Self {
        self.completed = true;
        self
    }

    pub fn build(self) -> Todo {
        let mut todo = Todo::new(self.title);
        if let Some(desc) = self.description {
            todo.set_description(Some(desc));
        }
        todo.set_priority(self.priority);
        if self.completed {
            todo.complete();
        }
        todo
    }
}

pub fn create_sample_todos() -> Vec<Todo> {
    vec![
        TodoBuilder::new("Buy groceries")
            .with_priority(Priority::High)
            .build(),
        TodoBuilder::new("Write documentation")
            .with_description("Complete API documentation")
            .with_priority(Priority::Medium)
            .build(),
        TodoBuilder::new("Review code")
            .completed()
            .build(),
    ]
}
```

## 7. 성능 테스트

### 7.1 벤치마크 테스트

```rust
#[cfg(test)]
mod bench_tests {
    use test::Bencher;
    use crate::storage::MemoryStorage;
    use crate::service::TodoService;

    #[bench]
    fn bench_create_1000_todos(b: &mut Bencher) {
        let mut service = TodoService::new(MemoryStorage::new());

        b.iter(|| {
            for i in 0..1000 {
                service.create_todo(format!("Todo {}", i)).unwrap();
            }
        });
    }

    #[bench]
    fn bench_search_in_large_dataset(b: &mut Bencher) {
        let mut service = TodoService::new(MemoryStorage::new());

        // 테스트 데이터 준비
        for i in 0..10000 {
            service.create_todo(format!("Todo {}", i)).unwrap();
        }

        b.iter(|| {
            service.get_pending_todos().unwrap()
        });
    }
}
```

## 8. 테스트 실행 명령어

```bash
# 모든 테스트 실행
cargo test

# 특정 모듈 테스트
cargo test storage

# 통합 테스트만 실행
cargo test --test integration_tests

# 테스트 커버리지 (tarpaulin 사용)
cargo tarpaulin --out Html

# 벤치마크 테스트 (nightly 필요)
cargo +nightly test --benches
```

## 9. 테스트 모범 사례

### 9.1 테스트 작성 원칙
- **AAA 패턴**: Arrange (준비), Act (실행), Assert (검증)
- **독립성**: 각 테스트는 다른 테스트에 의존하지 않음
- **반복성**: 동일한 결과를 보장
- **명확성**: 테스트 이름으로 의도를 파악 가능

### 9.2 Mock과 Stub 활용
- 외부 의존성 격리
- 예측 가능한 테스트 환경 구성
- 에러 시나리오 테스트

이 테스트 구조는 코드의 품질을 보장하고 리팩토링 시 안전성을 제공합니다.