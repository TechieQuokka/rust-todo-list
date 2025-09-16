use clap::Parser;
use crate::cli::{Cli, Commands};
use crate::service::TodoService;
use crate::storage::{TodoStorage, TodoError};
use crate::models::Priority;
use uuid::Uuid;

pub fn parse_and_execute<S: TodoStorage> (
  args: Vec<String>, service: &mut TodoService<S>
) -> Result<(), TodoError> {

  let cli = Cli::parse_from(args);

  match cli.command {
    Commands::Add { title, description, priority } => {
      handle_add(service, title, description, priority)
    }
    Commands::List { filter, sort } => {
      handle_list(service, filter, sort)
    }
    Commands::Complete { id } => {
      handle_complete(service, id)
    }
    Commands::Toggle { id } => {
      handle_toggle(service, id)
    }
    Commands::Update { id, title, description, priority } => {
      handle_update(service, id, title, description, priority)
    }
    Commands::Delete { id } => {
      handle_delete(service, id)
    }
    Commands::Clear { all } => {
      handle_clear(service, all)
    }
  }
}

fn handle_add<S: TodoStorage> (
  service: &mut TodoService<S>,
  title: String,
  description: Option<String>,
  priority: Option<String>
) -> Result<(), TodoError> {

  let priority_enum = parse_priority(priority)?;
  let todo = service.create_todo_with_priority(title, priority_enum)?;

  if let Some(_desc) = description {

    // TODO: SET_DESCRIPTION 메서드 필요
  }

  println!("Todo 추가 됨: {}, (ID: {})", todo.title, todo.id);
  Ok(())
}

fn handle_list<S: TodoStorage> (
  service: &TodoService<S>,
  filter: String,
  sort: String
) -> Result<(), TodoError> {

  let todos = service.get_todos()?;

  println! ("Todo 목록:");
  for (i, todo) in todos.iter().enumerate() {
    let status = if todo.completed { "O" } else { " " };
    println!("{}. [{}] {} ({})", i + 1, status, todo.title, todo.priority);
    println!("     ID: {}", todo.id);
  }

  Ok(())
}

fn handle_complete<S: TodoStorage> (

  service: &mut TodoService<S>,
  id: String
) -> Result<(), TodoError> {

  let uuid = parse_id(&id)?;
  service.complete_todo(&uuid)?;
  println! ("Todo 완료 됨");
  Ok(())
}

fn handle_toggle<S: TodoStorage> (

  service: &mut TodoService<S>,
  id: String
 ) -> Result<(), TodoError> {

  let uuid = parse_id(&id)?;
  service.toggle_todo(&uuid)?;
  println!("Todo 상태 토글 됨");
  Ok(())
 }

 fn handle_update<S: TodoStorage> (

  service: &mut TodoService<S>,
  id: String,
  title: Option<String>,
  desccription: Option<String>,
  priority: Option<String>
 ) -> Result<(), TodoError> {

  let uuid = parse_id(&id)?;

  if let Some(new_title) = title {

    service.update_todo_title (&uuid, new_title)?;
  }

  println! ("Todo 업데이트 됨");
  Ok(())
 }

 fn handle_delete<S: TodoStorage> (
  service: &mut TodoService<S>,
  id: String
 ) -> Result<(), TodoError> {

  let uuid = parse_id(&id)?;
  service.delete_todo(&uuid)?;

  println! ("Todo 삭제됨");
  Ok(())
 }

 fn handle_clear<S: TodoStorage> (
  _service: &mut TodoService<S>,
  all: bool
 ) -> Result<(), TodoError> {

  if all {
      println!("모든 Todo 삭제 기능은 아직 구현되지 않았습니다");
  } else {
      println!("완료된 Todo 삭제 기능은 아직 구현되지 않았습니다");
  }
  Ok(())
 }

 fn parse_priority(priority_str: Option<String>) -> Result<Priority, TodoError> {

  match priority_str.as_deref() {
    None | Some("medium") => Ok(Priority::Medium),
    Some("low") => Ok(Priority::Low),
    Some("high") => Ok(Priority::High),
    Some(other) => Err(TodoError::ValidationError(
      format!("잘못된 우선순위: {}. low, medium, high 중 하나를 사용하세요", other)
    ))
  }
 }

 fn parse_id(id_str: &str) -> Result<Uuid, TodoError> {

  Uuid::parse_str(id_str).map_err(|_| {
    TodoError::ValidationError(format!("잘못된 ID 형식: {}", id_str))
  })
 }