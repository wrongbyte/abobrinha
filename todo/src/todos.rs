use crate::{terminal::error::TerminalError, todo::Todo};

pub struct Todos {
    pub list: Vec<Todo>,
}

pub trait TodoStorage {
    fn push_new_todo(&mut self, todo: Todo);
    fn remove_todo(&mut self, index_todo: usize) -> Result<(), TerminalError>;
    fn is_empty(&mut self) -> bool;
    fn get_list(&mut self) -> &[Todo];
    fn clear(&mut self);
    fn mark_done(&mut self, index_todo: usize) -> Result<(), TerminalError>;
}

impl TodoStorage for Todos {
    fn push_new_todo(&mut self, todo: Todo) {
        self.list.push(todo)
    }

    fn remove_todo(&mut self, index_todo: usize) -> Result<(), TerminalError> {
        if index_todo < self.list.len() {
            self.list.remove(index_todo);
            Ok(())
        } else {
            Err(TerminalError::IndexError)
        }
    }

    fn mark_done(&mut self, index_todo: usize) -> Result<(), TerminalError> {
        if let Some(todo) = self.list.get_mut(index_todo) {
            todo.done = true;
            Ok(())
        } else {
            Err(TerminalError::IndexError)
        }
    }

    fn is_empty(&mut self) -> bool {
        self.list.is_empty()
    }

    fn get_list(&mut self) -> &[Todo] {
        &self.list[..]
    }

    fn clear(&mut self) {
        self.list.clear()
    }
}

impl Todos {
    pub fn new() -> Self {
        Todos { list: Vec::new() }
    }
}
