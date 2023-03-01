use std::fmt;

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Todo {
    pub message: String,
    pub done: bool,
    pub id: Option<Uuid>
}

impl Todo {
    pub fn new(message: String) -> Self {
        Todo {
            message,
            done: false,
            id: None
        }
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.done {
            write!(f, "[ ] - {} - id: {}", self.message, self.id.unwrap())?
        } else {
            write!(f, "[X] - {} - id: {}", self.message, self.id.unwrap())?
        }
        Ok(())
    }
}
