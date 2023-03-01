use std::fmt;

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Todo {
    pub message: String,
    pub done: bool,
    pub id: Uuid,
}

impl Todo {
    pub fn new(message: String, id: Uuid) -> Self {
        Todo {
            message,
            done: false,
            id,
        }
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let marker = if self.done { 'X' } else { ' ' };
        write!(f, "[{marker}] - {} - id: {}", self.message, self.id)?;
        Ok(())
    }
}
