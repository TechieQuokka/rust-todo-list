# 🦀 Rust Todo List

Rust 학습을 위한 CLI 기반 Todo List 애플리케이션입니다.

## 📋 프로젝트 개요

이 프로젝트는 Rust의 핵심 개념들을 학습하고 실습하기 위해 설계된 Todo List 애플리케이션입니다. 간단하지만 체계적인 아키텍처를 통해 Rust의 소유권, 타입 시스템, 에러 처리 등의 특징을 활용합니다.

## ✨ 주요 기능

- ✅ Todo 항목 추가/삭제/수정
- 🔄 완료 상태 토글
- 📋 목록 조회 및 필터링 (전체/완료/미완료)
- ⚡ 우선순위 설정 (High/Medium/Low)
- 💾 JSON 파일을 통한 영구 저장
- 🎯 CLI 인터페이스를 통한 직관적인 사용

## 🏗️ 프로젝트 구조

```
todo-list/
├── Cargo.toml                 # 프로젝트 설정 및 의존성
├── src/
│   ├── main.rs               # CLI 애플리케이션 진입점
│   ├── lib.rs                # 라이브러리 루트
│   ├── models/
│   │   ├── mod.rs            # 모델 모듈
│   │   └── todo.rs           # Todo 데이터 구조
│   ├── storage/
│   │   ├── mod.rs            # 저장소 모듈
│   │   ├── file_storage.rs   # 파일 기반 저장소
│   │   └── memory_storage.rs # 메모리 저장소 (테스트용)
│   ├── cli/
│   │   ├── mod.rs            # CLI 모듈
│   │   ├── commands.rs       # 명령어 처리
│   │   └── parser.rs         # 인수 파싱
│   ├── service/
│   │   ├── mod.rs            # 서비스 모듈
│   │   └── todo_service.rs   # 비즈니스 로직
│   └── errors.rs             # 사용자 정의 에러 타입
├── tests/
│   ├── integration_tests.rs  # 통합 테스트
│   └── common/
│       └── mod.rs            # 테스트 공통 모듈
└── docs/
    ├── architecture.md       # 아키텍처 문서
    ├── api.md               # API 문서
    └── testing.md           # 테스트 문서
```

## 🚀 시작하기

### 사전 요구사항

- [Rust](https://rustup.rs/) 1.70.0 이상
- Cargo (Rust와 함께 설치됨)

### 설치 및 실행

1. **저장소 클론**
   ```bash
   git clone <repository-url>
   cd ToDoList
   ```

2. **프로젝트 빌드**
   ```bash
   cargo build
   ```

3. **애플리케이션 실행**
   ```bash
   cargo run
   ```

### 개발 모드에서 실행

```bash
# 개발 빌드로 실행
cargo run

# 릴리즈 빌드로 실행
cargo run --release
```

## 💻 사용법

### 기본 명령어

```bash
# 새로운 Todo 항목 추가
todo-list add "새로운 할 일" --priority high

# Todo 목록 조회
todo-list list

# 완료된 항목만 조회
todo-list list --filter completed

# 미완료 항목만 조회
todo-list list --filter pending

# Todo 항목 완료 처리
todo-list complete <id>

# Todo 항목 삭제
todo-list delete <id>

# Todo 항목 수정
todo-list update <id> "수정된 제목"
```

### 고급 사용법

```bash
# 우선순위와 설명을 포함한 Todo 추가
todo-list add "중요한 업무" --priority high --description "자세한 설명"

# 우선순위별 정렬
todo-list list --sort priority

# 생성일순 정렬
todo-list list --sort created

# 완료된 모든 항목 삭제
todo-list clear

# 모든 항목 삭제
todo-list clear --all
```

## 🧪 테스트

### 단위 테스트 실행

```bash
cargo test
```

### 통합 테스트 실행

```bash
cargo test --test integration_tests
```

### 테스트 커버리지 확인

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

## 📊 아키텍처

이 프로젝트는 레이어드 아키텍처 패턴을 따릅니다:

- **Presentation Layer (CLI)**: 사용자 입력 처리 및 결과 출력
- **Service Layer**: 비즈니스 로직 및 데이터 검증
- **Storage Layer**: 데이터 영속화 및 조회
- **Models Layer**: 데이터 구조 정의

자세한 아키텍처 정보는 [docs/architecture.md](docs/architecture.md)를 참조하세요.

## 🎯 학습 목표

이 프로젝트를 통해 다음 Rust 개념들을 학습할 수 있습니다:

1. **소유권과 차용 (Ownership & Borrowing)**
   - 데이터 전달 시 소유권 이전 vs 차용
   - 참조자의 생명주기 관리

2. **트레잇 시스템 (Trait System)**
   - `TodoStorage` 트레잇을 통한 추상화
   - 제네릭과 트레잇 바운드

3. **에러 처리 (Error Handling)**
   - `Result` 타입과 `?` 연산자 활용
   - 사용자 정의 에러 타입

4. **패턴 매칭 (Pattern Matching)**
   - `match` 표현식과 `if let` 구문
   - 열거형(enum) 활용

5. **모듈 시스템 (Module System)**
   - 체계적인 코드 구조화
   - `pub`, `mod`, `use` 키워드

6. **직렬화/역직렬화**
   - `serde`를 활용한 JSON 처리
   - 데이터 영속화

## 🛠️ 사용된 크레이트 (Dependencies)

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

## 🔮 향후 계획

### Phase 1: 기본 기능 구현
- [x] 프로젝트 구조 설계
- [ ] Todo 모델 구현
- [ ] 파일 저장소 구현
- [ ] CLI 인터페이스 구현

### Phase 2: 고급 기능 추가
- [ ] 우선순위 시스템
- [ ] 카테고리/태그 시스템
- [ ] 검색 기능
- [ ] 데이터 내보내기/가져오기

## 🤝 기여하기

1. 이 저장소를 포크합니다
2. 새로운 기능 브랜치를 생성합니다 (`git checkout -b feature/amazing-feature`)
3. 변경사항을 커밋합니다 (`git commit -m 'Add some amazing feature'`)
4. 브랜치에 푸시합니다 (`git push origin feature/amazing-feature`)
5. Pull Request를 생성합니다

## 📝 라이선스

이 프로젝트는 MIT 라이선스 하에 배포됩니다. 자세한 내용은 [LICENSE](LICENSE) 파일을 참조하세요.


---

**Happy Coding with Rust! 🦀**