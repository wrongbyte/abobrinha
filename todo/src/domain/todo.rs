use std::fmt;

#[derive(Debug, Clone)]
pub struct Todo {
    pub message: String,
    pub done: bool,
}

impl Todo {
    pub fn new(message: String) -> Self {
        Todo {
            message,
            done: false,
        }
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.done {
            write!(f, "[ ] - {}", self.message)?
        } else {
            write!(f, "[X] - {}", self.message)?
        }
        Ok(())
    }
}
