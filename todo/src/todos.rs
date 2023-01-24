use crate::{terminal::error::TerminalError, todo::Todo};
use crate::filestorage::{Storage};
use crate::filestorage::FileStorage;
use async_trait::async_trait;

pub struct Todos {
    pub list: Vec<Todo>,
}

#[async_trait]
pub trait TodoStorage {
    async fn push_new_todo(&mut self, todo: Todo) -> Result<(), TerminalError>;
    async fn remove_todo(&mut self, index_todo: usize) -> Result<(), TerminalError>;
    fn is_empty(&mut self) -> bool;
    async fn get_list(&mut self) -> Result<Vec<Todo>, TerminalError>;
    async fn clear(&mut self) -> Result<(), TerminalError>;
    async fn mark_done(&mut self, index_todo: usize) -> Result<(), TerminalError>;
}

#[async_trait]
impl TodoStorage for Todos {
    async fn push_new_todo(&mut self, todo: Todo) -> Result<(), TerminalError> {
        let mut todo_list = FileStorage::get_todos_from_filestorage().await.map_err(|error| TerminalError::StorageError(error))?;
        todo_list.list.push(todo);
        self.list = todo_list.list;
        FileStorage::write_filestorage(&mut self).await.map_err(|error| TerminalError::StorageError(error))?;
        Ok(())
    }

    async fn remove_todo(&mut self, index_todo: usize) -> Result<(), TerminalError> {
        let mut todo_list = FileStorage::get_todos_from_filestorage().await.map_err(|error| TerminalError::StorageError(error))?;
        if index_todo > todo_list.list.len() {
            return Err(TerminalError::IndexError);
        }
        todo_list.list.remove(index_todo);
        self.list = todo_list.list;
        FileStorage::write_filestorage(&mut self).await.map_err(|error| TerminalError::StorageError(error))?;
        Ok(())
    }

    async fn mark_done(&mut self, index_todo: usize) -> Result<(), TerminalError> {
        let mut todo_list = FileStorage::get_todos_from_filestorage().await.map_err(|error| TerminalError::StorageError(error))?;
        if let Some(todo) = todo_list.list.get_mut(index_todo) {
            todo.done = true;
        } else {
            return Err(TerminalError::IndexError)
        }
        self.list = todo_list.list;
        FileStorage::write_filestorage(&mut self).await.map_err(|error| TerminalError::StorageError(error))?;
        Ok(())
    }

    fn is_empty(&mut self) -> bool {
        self.list.is_empty()
    }

    async fn get_list(&mut self) -> Result<Vec<Todo>, TerminalError> {
        let todos = FileStorage::get_todos_from_filestorage().await.map_err(|error| TerminalError::StorageError(error))?;
        Ok(todos.list)
    }

    async fn clear(&mut self) -> Result<(), TerminalError> {
        self.list.clear();
        FileStorage::write_filestorage(&mut self).await.map_err(|error| TerminalError::StorageError(error))?;
        Ok(())
    }
}

impl Todos {
    pub fn new() -> Self {
        Todos { list: Vec::new() }
    }
}
