# Rust Todo List 아키텍처 설계

## 1. 프로젝트 개요

### 목적
- Rust 학습을 위한 실습용 Todo List 애플리케이션
- CLI 기반의 간단하지만 확장 가능한 구조
- Rust의 소유권, 타입 시스템, 에러 처리 등의 특징 활용

### 주요 기능
- Todo 항목 추가/삭제/수정
- 완료 상태 토글
- 목록 조회 및 필터링
- 영구 저장 (JSON 파일)

## 2. 프로젝트 구조

```
todo-list/
├── Cargo.toml
├── src/
│   ├── main.rs                 # CLI 애플리케이션 진입점
│   ├── lib.rs                  # 라이브러리 루트
│   ├── models/
│   │   ├── mod.rs              # 모델 모듈
│   │   └── todo.rs             # Todo 데이터 구조
│   ├── storage/
│   │   ├── mod.rs              # 저장소 모듈
│   │   ├── file_storage.rs     # 파일 기반 저장소
│   │   └── memory_storage.rs   # 메모리 저장소 (테스트용)
│   ├── cli/
│   │   ├── mod.rs              # CLI 모듈
│   │   ├── commands.rs         # 명령어 처리
│   │   └── parser.rs           # 인수 파싱
│   ├── service/
│   │   ├── mod.rs              # 서비스 모듈
│   │   └── todo_service.rs     # 비즈니스 로직
│   └── errors.rs               # 사용자 정의 에러 타입
├── tests/
│   ├── integration_tests.rs    # 통합 테스트
│   └── common/
│       └── mod.rs              # 테스트 공통 모듈
└── docs/
    ├── architecture.md         # 아키텍처 문서
    └── api.md                  # API 문서
```

## 3. 아키텍처 레이어

### 3.1 Presentation Layer (CLI)
- **역할**: 사용자 입력 처리 및 결과 출력
- **구성 요소**:
  - `cli::parser`: 명령줄 인수 파싱
  - `cli::commands`: 명령어별 처리 로직
- **의존성**: Service Layer

### 3.2 Service Layer (Business Logic)
- **역할**: 비즈니스 로직 및 데이터 검증
- **구성 요소**:
  - `service::todo_service`: Todo 관련 비즈니스 로직
- **의존성**: Storage Layer, Models

### 3.3 Storage Layer (Data Access)
- **역할**: 데이터 영속화 및 조회
- **구성 요소**:
  - `storage::file_storage`: JSON 파일 기반 저장소
  - `storage::memory_storage`: 인메모리 저장소 (테스트용)
- **의존성**: Models

### 3.4 Models Layer
- **역할**: 데이터 구조 정의
- **구성 요소**:
  - `models::todo`: Todo 데이터 모델

## 4. 핵심 컴포넌트 설계

### 4.1 데이터 모델

```rust
// models/todo.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub priority: Priority,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Todo {
    pub fn new(title: String) -> Self { /* 구현 */ }
    pub fn complete(&mut self) { /* 구현 */ }
    pub fn update_title(&mut self, title: String) { /* 구현 */ }
}
```

### 4.2 Storage Trait

```rust
// storage/mod.rs
use crate::models::Todo;
use crate::errors::TodoError;
use uuid::Uuid;

pub trait TodoStorage {
    fn save(&mut self, todo: &Todo) -> Result<(), TodoError>;
    fn find_by_id(&self, id: &Uuid) -> Result<Option<Todo>, TodoError>;
    fn find_all(&self) -> Result<Vec<Todo>, TodoError>;
    fn delete(&mut self, id: &Uuid) -> Result<bool, TodoError>;
    fn update(&mut self, todo: &Todo) -> Result<(), TodoError>;
}
```

### 4.3 Service Layer

```rust
// service/todo_service.rs
pub struct TodoService<S: TodoStorage> {
    storage: S,
}

impl<S: TodoStorage> TodoService<S> {
    pub fn new(storage: S) -> Self { /* 구현 */ }
    pub fn create_todo(&mut self, title: String) -> Result<Todo, TodoError> { /* 구현 */ }
    pub fn get_todos(&self) -> Result<Vec<Todo>, TodoError> { /* 구현 */ }
    pub fn complete_todo(&mut self, id: &Uuid) -> Result<(), TodoError> { /* 구현 */ }
    pub fn delete_todo(&mut self, id: &Uuid) -> Result<(), TodoError> { /* 구현 */ }
}
```

## 5. 에러 처리 전략

```rust
// errors.rs
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum TodoError {
    StorageError(String),
    ValidationError(String),
    NotFound(String),
    IoError(std::io::Error),
    SerializationError(serde_json::Error),
}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { /* 구현 */ }
}

impl Error for TodoError {}

// From 트레잇 구현으로 자동 변환 지원
impl From<std::io::Error> for TodoError { /* 구현 */ }
impl From<serde_json::Error> for TodoError { /* 구현 */ }
```

## 6. CLI 인터페이스 설계

### 6.1 명령어 구조

```
todo-list add "새로운 할 일" --priority high
todo-list list --filter completed
todo-list complete <id>
todo-list delete <id>
todo-list update <id> "수정된 제목"
```

### 6.2 CLI 구현

```rust
// cli/commands.rs
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo-list")]
#[command(about = "A simple todo list manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        title: String,
        #[arg(short, long)]
        priority: Option<String>,
    },
    List {
        #[arg(long)]
        filter: Option<String>,
    },
    Complete { id: String },
    Delete { id: String },
    Update { id: String, title: String },
}
```

## 7. 의존성 관리

### Cargo.toml
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
clap = { version = "4.0", features = ["derive"] }
anyhow = "1.0"

[dev-dependencies]
tempfile = "3.0"
```

## 8. 테스트 전략

### 8.1 단위 테스트
- 각 모듈별 개별 기능 테스트
- Mock Storage를 활용한 Service 테스트

### 8.2 통합 테스트
- CLI 명령어 End-to-End 테스트
- 파일 시스템과의 상호작용 테스트

## 9. 확장 가능성

### 9.1 향후 추가 가능한 기능
- 웹 인터페이스 (REST API 추가)
- 데이터베이스 백엔드 지원
- 사용자 인증 시스템
- Todo 카테고리/태그 시스템

### 9.2 아키텍처 확장점
- Storage Trait을 통한 다양한 백엔드 지원
- Service Layer의 비즈니스 로직 확장
- CLI 외 다른 인터페이스 추가 가능

## 10. Rust 학습 포인트

이 프로젝트를 통해 학습할 수 있는 Rust 개념들:

1. **소유권과 차용**: 데이터 전달 시 소유권 이전 vs 차용
2. **트레잇 시스템**: TodoStorage 트레잇을 통한 추상화
3. **에러 처리**: Result 타입과 ? 연산자 활용
4. **제네릭과 라이프타임**: Service에서 제네릭 Storage 사용
5. **패턴 매칭**: Command enum 처리
6. **직렬화/역직렬화**: serde를 활용한 JSON 처리
7. **모듈 시스템**: 체계적인 코드 구조화