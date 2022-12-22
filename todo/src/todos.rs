use crate::{todo::Todo, terminal::error::TerminalError};

pub struct Todos {
    pub list: Vec<Todo>
}

impl Todos {
    pub fn new() -> Self {
        Todos {
            list: Vec::new()
        }
    }
    pub fn push_new_todo (&mut self, todo: Todo) {
        self.list.push(todo)
    }

    pub fn remove_todo (&mut self, index_todo: usize) -> Result<(), TerminalError> {
        if index_todo < self.list.len() {
            self.list.remove(index_todo);
            Ok(())
        } else {
            Err(TerminalError::IndexError)
        }
    }
}