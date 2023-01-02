use std::fmt;

#[derive(Debug, Clone)]
pub struct Todo {
    pub message: String,
}

impl Todo {
    pub fn new(message: String) -> Self {
        Todo { message }
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ ] - {}", self.message)
    }
}
