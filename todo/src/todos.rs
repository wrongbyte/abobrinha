use crate::{todo::Todo, terminal::error::TerminalError};
use std::io::{Error, ErrorKind};

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
        if self.list.get(index_todo).is_some() {
            self.list.remove(index_todo);
            Ok(())
        } else {
            let custom_error = Error::new(ErrorKind::Other, "Index out of bounds!");
            Err(TerminalError::Stdin(custom_error))
        }
    }
}