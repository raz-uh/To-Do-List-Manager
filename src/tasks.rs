use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: usize, description: String) -> Self {
        Task {
            id,
            description,
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.completed { "✓" } else { " " };
        write!(f, "[{}] {}: {}", status, self.id, self.description)
    }
}