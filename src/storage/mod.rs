use crate::models::{Todo, Priority};
use uuid::Uuid;

#[derive(Debug)]
pub enum TodoError {

  StorageError(String),
  ValidationError(String),
  NotFound(String),
  IoError(std::io::Error),
  SerializationError(serde_json::Error),
}

pub trait TodoStorage {

  fn save(&mut self, todo: &Todo) -> Result<(), TodoError>;
  fn find_by_id (&self, id: &Uuid) -> Result<Option<Todo>, TodoError>;
  fn find_all (&self) -> Result<Vec<Todo>, TodoError>;
  fn delete (&mut self, id: &Uuid) -> Result<bool, TodoError>;
  fn update (&mut self, todo: &Todo) -> Result<(), TodoError>;
}

pub mod memory_storage;
pub mod file_storage;