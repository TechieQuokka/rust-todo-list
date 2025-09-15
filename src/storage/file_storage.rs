use std::fs;
use std::path::PathBuf;
use uuid::Uuid;
use crate::models::Todo;
use super::{TodoStorage, TodoError};

pub struct FileStorage {
  file_path: PathBuf,
}

impl FileStorage {

  pub fn new(file_path: PathBuf) -> Self {

    Self { file_path }
  }

  pub fn default() -> Result<Self, TodoError> {

    let file_path = PathBuf::from("todos.json");
    Ok(Self {file_path})
  }

  fn load_todos(&self) -> Result<Vec<Todo>, TodoError> {

    if !self.file_path.exists() {

      return Ok(Vec::new());
    }

    let content = fs::read_to_string(&self.file_path)
            .map_err(|e| TodoError::IoError(e))?;
    
    let todos: Vec<Todo> = serde_json::from_str(&content)
            .map_err(|e| TodoError::SerializationError(e))?;

    Ok(todos)
  }

  fn save_todos (&self, todos: &[Todo]) -> Result<(), TodoError> {

    let content = serde_json::to_string_pretty(todos)
          .map_err(|e| TodoError::SerializationError(e))?;
      
    fs::write(&self.file_path, content)
          .map_err(|e| TodoError::IoError(e))?;

    Ok(())
  }
}

impl TodoStorage for FileStorage {

  fn save(&mut self, todo: &Todo) -> Result<(), TodoError> {

    let mut todos = self.load_todos()?;

    if let Some(pos) = todos.iter().position(|t| t.id == todo.id) {

      todos[pos] = todo.clone();
    } else {

      todos.push(todo.clone());
    }

    self.save_todos(&todos)
  }

  fn find_by_id (&self, id: &Uuid) -> Result<Option<Todo>, TodoError> {

    let todos = self.load_todos()?;
    Ok(todos.into_iter().find(|t| t.id == *id))
  }

  fn find_all(&self) -> Result<Vec<Todo>, TodoError> {

    self.load_todos()
  }

  fn delete (&mut self, id: &Uuid) -> Result<bool, TodoError> {

    let mut todos = self.load_todos()?;

    if let Some(pos) = todos.iter().position(|t| t.id == *id) {

      todos.remove(pos);
      self.save_todos(&todos)?;
      Ok(true)
    } else {

      Ok(false)
    }
  }

  fn update (&mut self, todo: &Todo) -> Result<(), TodoError> {

    self.save(todo)
  }
}