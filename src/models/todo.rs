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

  Low, Medium, High
}

impl Todo {
  pub fn new(title: String) -> Self {

    let now = Utc::now();
    Self {

      id: Uuid::new_v4(),
      title,
      description: None,
      completed: false,
      created_at: now,
      updated_at: now,
      priority: Priority::Medium,
    }
  }

  pub fn complete (&mut self) {

    self.completed = true;
    self.updated_at = Utc::now();
  }

  pub fn update_title (&mut self, title: String) {

    self.title = title;
    self.updated_at = Utc::now();
  }

  pub fn set_priority(&mut self, priority: Priority) {

    self.priority = priority;
    self.updated_at = Utc::now();
  }

  pub fn toggle_complete(&mut self) {

    self.completed = !self.completed;
    self.updated_at = Utc::now();
  }
}