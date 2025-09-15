# Todo List API 설계

## 1. 데이터 모델 상세 API

### 1.1 Todo 구조체

```rust
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

impl Todo {
    /// 새로운 Todo 항목을 생성합니다.
    pub fn new(title: String) -> Self;

    /// Todo 항목의 제목을 변경합니다.
    pub fn update_title(&mut self, title: String);

    /// Todo 항목의 설명을 설정합니다.
    pub fn set_description(&mut self, description: Option<String>);

    /// Todo 항목을 완료 상태로 변경합니다.
    pub fn complete(&mut self);

    /// Todo 항목을 미완료 상태로 변경합니다.
    pub fn uncomplete(&mut self);

    /// Todo 항목의 완료 상태를 토글합니다.
    pub fn toggle_complete(&mut self);

    /// Todo 항목의 우선순위를 설정합니다.
    pub fn set_priority(&mut self, priority: Priority);

    /// Todo 항목이 완료되었는지 확인합니다.
    pub fn is_completed(&self) -> bool;
}
```

### 1.2 Priority 열거형

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum Priority {
    Low = 1,
    Medium = 2,
    High = 3,
}

impl Priority {
    /// 문자열로부터 Priority를 파싱합니다.
    pub fn from_str(s: &str) -> Result<Priority, TodoError>;

    /// Priority를 문자열로 변환합니다.
    pub fn to_string(&self) -> String;
}

impl Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}
```

## 2. Storage Layer API

### 2.1 TodoStorage 트레잇

```rust
pub trait TodoStorage {
    /// Todo 항목을 저장합니다.
    fn save(&mut self, todo: &Todo) -> Result<(), TodoError>;

    /// ID로 Todo 항목을 조회합니다.
    fn find_by_id(&self, id: &Uuid) -> Result<Option<Todo>, TodoError>;

    /// 모든 Todo 항목을 조회합니다.
    fn find_all(&self) -> Result<Vec<Todo>, TodoError>;

    /// 완료된 Todo 항목들을 조회합니다.
    fn find_completed(&self) -> Result<Vec<Todo>, TodoError>;

    /// 미완료된 Todo 항목들을 조회합니다.
    fn find_pending(&self) -> Result<Vec<Todo>, TodoError>;

    /// 우선순위로 Todo 항목들을 조회합니다.
    fn find_by_priority(&self, priority: Priority) -> Result<Vec<Todo>, TodoError>;

    /// Todo 항목을 업데이트합니다.
    fn update(&mut self, todo: &Todo) -> Result<(), TodoError>;

    /// ID로 Todo 항목을 삭제합니다.
    fn delete(&mut self, id: &Uuid) -> Result<bool, TodoError>;

    /// 완료된 모든 Todo 항목을 삭제합니다.
    fn clear_completed(&mut self) -> Result<usize, TodoError>;

    /// 모든 Todo 항목을 삭제합니다.
    fn clear_all(&mut self) -> Result<usize, TodoError>;
}
```

### 2.2 FileStorage 구현

```rust
pub struct FileStorage {
    file_path: PathBuf,
}

impl FileStorage {
    /// 새로운 FileStorage 인스턴스를 생성합니다.
    pub fn new(file_path: PathBuf) -> Self;

    /// 기본 파일 경로로 FileStorage를 생성합니다.
    pub fn default() -> Result<Self, TodoError>;

    /// 파일에서 데이터를 로드합니다.
    fn load_todos(&self) -> Result<Vec<Todo>, TodoError>;

    /// 데이터를 파일에 저장합니다.
    fn save_todos(&self, todos: &[Todo]) -> Result<(), TodoError>;
}

impl TodoStorage for FileStorage {
    // TodoStorage 트레잇 메서드들 구현
}
```

### 2.3 MemoryStorage 구현 (테스트용)

```rust
pub struct MemoryStorage {
    todos: HashMap<Uuid, Todo>,
}

impl MemoryStorage {
    /// 새로운 MemoryStorage 인스턴스를 생성합니다.
    pub fn new() -> Self;
}

impl TodoStorage for MemoryStorage {
    // TodoStorage 트레잇 메서드들 구현
}
```

## 3. Service Layer API

### 3.1 TodoService

```rust
pub struct TodoService<S: TodoStorage> {
    storage: S,
}

impl<S: TodoStorage> TodoService<S> {
    /// 새로운 TodoService 인스턴스를 생성합니다.
    pub fn new(storage: S) -> Self;

    /// 새로운 Todo 항목을 생성합니다.
    pub fn create_todo(&mut self, title: String) -> Result<Todo, TodoError>;

    /// 우선순위와 함께 새로운 Todo 항목을 생성합니다.
    pub fn create_todo_with_priority(
        &mut self,
        title: String,
        priority: Priority
    ) -> Result<Todo, TodoError>;

    /// 설명과 함께 새로운 Todo 항목을 생성합니다.
    pub fn create_todo_full(
        &mut self,
        title: String,
        description: Option<String>,
        priority: Priority,
    ) -> Result<Todo, TodoError>;

    /// 모든 Todo 항목을 조회합니다.
    pub fn get_todos(&self) -> Result<Vec<Todo>, TodoError>;

    /// ID로 Todo 항목을 조회합니다.
    pub fn get_todo_by_id(&self, id: &Uuid) -> Result<Option<Todo>, TodoError>;

    /// 완료된 Todo 항목들을 조회합니다.
    pub fn get_completed_todos(&self) -> Result<Vec<Todo>, TodoError>;

    /// 미완료된 Todo 항목들을 조회합니다.
    pub fn get_pending_todos(&self) -> Result<Vec<Todo>, TodoError>;

    /// 우선순위별 Todo 항목들을 조회합니다.
    pub fn get_todos_by_priority(&self, priority: Priority) -> Result<Vec<Todo>, TodoError>;

    /// Todo 항목을 완료 상태로 변경합니다.
    pub fn complete_todo(&mut self, id: &Uuid) -> Result<(), TodoError>;

    /// Todo 항목을 미완료 상태로 변경합니다.
    pub fn uncomplete_todo(&mut self, id: &Uuid) -> Result<(), TodoError>;

    /// Todo 항목의 완료 상태를 토글합니다.
    pub fn toggle_todo(&mut self, id: &Uuid) -> Result<(), TodoError>;

    /// Todo 항목의 제목을 업데이트합니다.
    pub fn update_todo_title(&mut self, id: &Uuid, title: String) -> Result<(), TodoError>;

    /// Todo 항목의 설명을 업데이트합니다.
    pub fn update_todo_description(
        &mut self,
        id: &Uuid,
        description: Option<String>
    ) -> Result<(), TodoError>;

    /// Todo 항목의 우선순위를 업데이트합니다.
    pub fn update_todo_priority(
        &mut self,
        id: &Uuid,
        priority: Priority
    ) -> Result<(), TodoError>;

    /// Todo 항목을 삭제합니다.
    pub fn delete_todo(&mut self, id: &Uuid) -> Result<(), TodoError>;

    /// 완료된 모든 Todo 항목을 삭제합니다.
    pub fn clear_completed(&mut self) -> Result<usize, TodoError>;

    /// 모든 Todo 항목을 삭제합니다.
    pub fn clear_all(&mut self) -> Result<usize, TodoError>;

    /// Todo 항목들을 다양한 기준으로 정렬합니다.
    pub fn sort_todos(&self, sort_by: SortBy) -> Result<Vec<Todo>, TodoError>;
}
```

### 3.2 SortBy 열거형

```rust
#[derive(Debug, Clone)]
pub enum SortBy {
    CreatedAt,
    UpdatedAt,
    Title,
    Priority,
    Completed,
}
```

## 4. CLI Layer API

### 4.1 Commands 열거형

```rust
#[derive(Subcommand)]
pub enum Commands {
    /// 새로운 Todo 항목을 추가합니다
    Add {
        /// Todo 항목의 제목
        title: String,
        /// Todo 항목의 설명
        #[arg(short, long)]
        description: Option<String>,
        /// 우선순위 (low, medium, high)
        #[arg(short, long)]
        priority: Option<String>,
    },

    /// Todo 항목들을 나열합니다
    List {
        /// 필터 (all, pending, completed)
        #[arg(short, long, default_value = "all")]
        filter: String,
        /// 정렬 기준 (created, updated, title, priority)
        #[arg(short, long, default_value = "created")]
        sort: String,
    },

    /// Todo 항목을 완료 상태로 변경합니다
    Complete {
        /// Todo 항목의 ID 또는 인덱스
        id: String,
    },

    /// Todo 항목을 미완료 상태로 변경합니다
    Uncomplete {
        /// Todo 항목의 ID 또는 인덱스
        id: String,
    },

    /// Todo 항목의 완료 상태를 토글합니다
    Toggle {
        /// Todo 항목의 ID 또는 인덱스
        id: String,
    },

    /// Todo 항목을 업데이트합니다
    Update {
        /// Todo 항목의 ID 또는 인덱스
        id: String,
        /// 새로운 제목
        #[arg(short, long)]
        title: Option<String>,
        /// 새로운 설명
        #[arg(short, long)]
        description: Option<String>,
        /// 새로운 우선순위
        #[arg(short, long)]
        priority: Option<String>,
    },

    /// Todo 항목을 삭제합니다
    Delete {
        /// Todo 항목의 ID 또는 인덱스
        id: String,
    },

    /// 완료된 Todo 항목들을 모두 삭제합니다
    Clear {
        /// 완료된 항목만 삭제 (기본값: true)
        #[arg(long)]
        all: bool,
    },
}
```

### 4.2 CommandHandler

```rust
pub struct CommandHandler<S: TodoStorage> {
    service: TodoService<S>,
}

impl<S: TodoStorage> CommandHandler<S> {
    /// 새로운 CommandHandler를 생성합니다.
    pub fn new(service: TodoService<S>) -> Self;

    /// 명령을 실행합니다.
    pub fn execute(&mut self, command: Commands) -> Result<(), TodoError>;

    /// Add 명령을 처리합니다.
    fn handle_add(
        &mut self,
        title: String,
        description: Option<String>,
        priority: Option<String>
    ) -> Result<(), TodoError>;

    /// List 명령을 처리합니다.
    fn handle_list(&self, filter: String, sort: String) -> Result<(), TodoError>;

    /// Complete 명령을 처리합니다.
    fn handle_complete(&mut self, id: String) -> Result<(), TodoError>;

    /// 기타 명령 처리 메서드들...
}
```

## 5. 에러 처리 API

### 5.1 TodoError 열거형

```rust
#[derive(Debug)]
pub enum TodoError {
    /// 저장소 관련 에러
    StorageError(String),
    /// 데이터 검증 에러
    ValidationError(String),
    /// 항목을 찾을 수 없음
    NotFound(String),
    /// 파일 I/O 에러
    IoError(std::io::Error),
    /// JSON 직렬화/역직렬화 에러
    SerializationError(serde_json::Error),
    /// UUID 파싱 에러
    UuidError(uuid::Error),
    /// 명령줄 인수 파싱 에러
    CliError(String),
}

impl TodoError {
    /// 사용자 친화적인 에러 메시지를 반환합니다.
    pub fn user_message(&self) -> String;
}
```

## 6. 유틸리티 함수들

### 6.1 ID 처리

```rust
/// 문자열을 UUID로 파싱하거나 인덱스로 해석합니다.
pub fn parse_id_or_index(input: &str, todos: &[Todo]) -> Result<Uuid, TodoError>;

/// Todo 리스트에서 인덱스로 UUID를 찾습니다.
pub fn find_uuid_by_index(index: usize, todos: &[Todo]) -> Result<Uuid, TodoError>;
```

### 6.2 출력 포맷팅

```rust
/// Todo 항목을 테이블 형태로 출력합니다.
pub fn print_todos_table(todos: &[Todo]);

/// Todo 항목을 간단한 리스트 형태로 출력합니다.
pub fn print_todos_list(todos: &[Todo]);

/// 단일 Todo 항목의 상세 정보를 출력합니다.
pub fn print_todo_detail(todo: &Todo);
```

## 7. 설정 관리

### 7.1 Config 구조체

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub storage_path: PathBuf,
    pub default_priority: Priority,
    pub date_format: String,
    pub auto_save: bool,
}

impl Config {
    /// 기본 설정을 로드합니다.
    pub fn load() -> Result<Self, TodoError>;

    /// 설정을 저장합니다.
    pub fn save(&self) -> Result<(), TodoError>;

    /// 기본 설정을 생성합니다.
    pub fn default() -> Self;
}
```

이 API 설계는 Rust의 특징을 활용하면서도 확장 가능하고 유지보수하기 쉬운 구조를 제공합니다.