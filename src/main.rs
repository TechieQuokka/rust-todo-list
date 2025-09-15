use models::todo::{Todo, Priority};
use storage::{TodoStorage, memory_storage::MemoryStorage, file_storage::FileStorage};
use std::path::PathBuf;

mod models;
mod storage;

fn main() {
    println!("=== Todo Storage 테스트 시작 ===\n");

    // 1. MemoryStorage 테스트
    println!("1. MemoryStorage 테스트");
    test_memory_storage();

    println!("\n{}\n", "=".repeat(50));

    // 2. FileStorage 테스트
    println!("2. FileStorage 테스트");
    test_file_storage();
}

fn test_memory_storage() {
    let mut storage = MemoryStorage::new();
    
    // Todo 생성 및 저장
    let todo1 = Todo::new("Rust 공부하기".to_string());
    let todo2 = Todo::new("프로젝트 완성하기".to_string());
    
    println!("Todo 저장 중...");
    storage.save(&todo1).unwrap();
    storage.save(&todo2).unwrap();
    
    // 전체 조회
    let all_todos = storage.find_all().unwrap();
    println!("저장된 Todo 개수: {}", all_todos.len());
    
    // ID로 조회
    let found = storage.find_by_id(&todo1.id).unwrap();
    println!("ID로 찾은 Todo: {:?}", found.is_some());
    
    // 삭제
    let deleted = storage.delete(&todo1.id).unwrap();
    println!("삭제 성공: {}", deleted);
    
    let remaining = storage.find_all().unwrap();
    println!("삭제 후 남은 Todo 개수: {}", remaining.len());
}

fn test_file_storage() {
    let file_path = PathBuf::from("test_todos.json");
    let mut storage = FileStorage::new(file_path);
    
    // Todo 생성 및 저장
    let todo1 = Todo::new("파일 저장 테스트".to_string());
    let mut todo2 = Todo::new("완료 테스트".to_string());
    todo2.complete();
    
    println!("파일에 Todo 저장 중...");
    storage.save(&todo1).unwrap();
    storage.save(&todo2).unwrap();
    
    // 파일에서 읽기
    let all_todos = storage.find_all().unwrap();
    println!("파일에서 읽은 Todo 개수: {}", all_todos.len());
    
    for (i, todo) in all_todos.iter().enumerate() {
        println!("  {}. {} (완료: {})", i+1, todo.title, todo.completed);
    }
    
    // 업데이트 테스트
    let mut updated_todo = todo1.clone();
    updated_todo.update_title("업데이트된 제목".to_string());
    storage.update(&updated_todo).unwrap();
    
    println!("업데이트 후 파일 확인...");
    let updated_todos = storage.find_all().unwrap();
    for todo in updated_todos.iter() {
        if todo.id == todo1.id {
            println!("업데이트된 제목: {}", todo.title);
        }
    }
}