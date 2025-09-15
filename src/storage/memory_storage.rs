use std::collections::HashMap;
use uuid::Uuid;
use crate::models::Todo;
use super::{TodoStorage, TodoError};

pub struct MemoryStorage {

  todos: HashMap<Uuid, Todo>,
}

impl MemoryStorage {

  pub fn new() -> Self {
    Self {
      todos: HashMap::new(),
    }
  }
}

impl TodoStorage for MemoryStorage {

  fn save (&mut self, todo: &Todo) -> Result<(), TodoError> {

    self.todos.insert(todo.id, todo.clone());
    Ok(())
  }

  fn find_by_id (&self, id: &Uuid) -> Result<Option<Todo>, TodoError> {

    Ok(self.todos.get(id).cloned())
  }

  fn find_all (&self) -> Result<Vec<Todo>, TodoError> {

    Ok(self.todos.values().cloned().collect())
  }

  fn delete (&mut self, id: &Uuid) -> Result<bool, TodoError> {

    Ok(self.todos.remove(id).is_some())
  }

  fn update (&mut self, todo: &Todo) -> Result<(), TodoError> {

    self.todos.insert(todo.id, todo.clone());
    Ok(())
  }
}