use models::{Todo, Priority};
use storage::{TodoStorage, memory_storage::MemoryStorage};
use service::TodoService;

mod models;
mod storage;
mod service;

fn main() {
    println!("=== TodoService 테스트 시작 ===\n");

    // MemoryStorage로 TodoService 테스트
    let storage = MemoryStorage::new();
    let mut service = TodoService::new(storage);

    // 1. Todo 생성 테스트
    println!("1. Todo 생성 테스트");
    let todo1 = service.create_todo("Rust 마스터하기".to_string()).unwrap();
    let todo2 = service.create_todo_with_priority("프로젝트 완성".to_string(), Priority::High).unwrap();
    
    println!("생성된 Todo 개수: {}", service.get_todos().unwrap().len());

    // 2. 완료 처리 테스트
    println!("\n2. 완료 처리 테스트");
    service.complete_todo(&todo1.id).unwrap();
    
    let updated_todo = service.get_todo_by_id(&todo1.id).unwrap().unwrap();
    println!("Todo 완료 상태: {}", updated_todo.completed);

    // 3. 토글 테스트
    println!("\n3. 토글 테스트");
    service.toggle_todo(&todo1.id).unwrap();
    let toggled_todo = service.get_todo_by_id(&todo1.id).unwrap().unwrap();
    println!("토글 후 완료 상태: {}", toggled_todo.completed);

    // 4. 제목 업데이트 테스트
    println!("\n4. 제목 업데이트 테스트");
    service.update_todo_title(&todo2.id, "Rust 전문가 되기".to_string()).unwrap();
    let updated_title_todo = service.get_todo_by_id(&todo2.id).unwrap().unwrap();
    println!("업데이트된 제목: {}", updated_title_todo.title);

    // 5. 삭제 테스트
    println!("\n5. 삭제 테스트");
    service.delete_todo(&todo1.id).unwrap();
    println!("삭제 후 Todo 개수: {}", service.get_todos().unwrap().len());

    // 6. 검증 에러 테스트
    println!("\n6. 검증 에러 테스트");
    match service.create_todo("".to_string()) {
        Err(e) => println!("예상된 에러: {:?}", e),
        Ok(_) => println!("에러가 발생해야 하는데 성공했습니다"),
    }
}