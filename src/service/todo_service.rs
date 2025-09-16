use uuid::Uuid;
use crate::models::{Todo, Priority};
use crate::storage::{TodoStorage, TodoError};

pub struct TodoService<S: TodoStorage> {

  storage: S,
}

impl<S: TodoStorage> TodoService<S> {

  pub fn new (storage: S) -> Self {
    Self {storage}
  }

  pub fn create_todo (&mut self, title: String) -> Result<Todo, TodoError> {

    if title.trim().is_empty() {
      return Err(TodoError::ValidationError("제목은 비어있을 수 없습니다".to_string()));
    }

    let todo = Todo::new(title);
    self.storage.save(&todo)?;
    Ok(todo)
  }

  pub fn create_todo_with_priority (&mut self, title: String,
                                    priority: Priority) -> Result<Todo, TodoError> {
  
    if title.trim().is_empty() {

      return Err(TodoError::ValidationError("제목은 비어있을 수 없습니다".to_string()));
    }

    let mut todo = Todo::new(title);
    todo.set_priority(priority);
    self.storage.save(&todo)?;
    Ok(todo)
  }

  pub fn get_todos(&self) -> Result<Vec<Todo>, TodoError> {

    self.storage.find_all()
  }

  pub fn get_todo_by_id (&self, id: &Uuid) -> Result<Option<Todo>, TodoError> {

    self.storage.find_by_id(id)
  }

  pub fn complete_todo (&mut self, id: &Uuid) -> Result<(), TodoError> {

    match self.storage.find_by_id(id)? {
      Some(mut todo) => {
        todo.complete();
        self.storage.update(&todo)
      }
      None => Err(TodoError::NotFound(format!("ID {}인 Todo를 찾을 수 없습니다", id)))
    }
  }

  pub fn toggle_todo(&mut self, id: &Uuid) -> Result<(), TodoError> {

    match self.storage.find_by_id(id)? {
      Some(mut todo) => {

        todo.toggle_complete();
        self.storage.update(&todo)
      }
      None => Err(TodoError::NotFound(format!("ID {}인 Todo를 찾을 수 없습니다", id)))
    }
  }

  pub fn update_todo_title(&mut self, id: &Uuid, title: String) -> Result<(), TodoError> {

    if title.trim().is_empty() {

      return Err(TodoError::ValidationError("제목은 비어있을 수 없습니다.".to_string()));
    }

    match self.storage.find_by_id(id)? {
      Some(mut todo) => {
        todo.update_title(title);
        self.storage.update(&todo)
      }
      None => Err(TodoError::NotFound(format!("ID {}인 Todo를 찾을 수 없습니다", id)))
    }
  }

  pub fn delete_todo (&mut self, id: &Uuid) -> Result<(), TodoError> {

    if !self.storage.delete(id)? {

      return Err(TodoError::NotFound(format!("Id {}인 Todo를 찾을 수 없습니다", id)));
    }
    Ok(())
  }
}