use crate::{terminal::error::TerminalError, todo::Todo};
use crate::filestorage::{Storage};
use crate::filestorage::FileStorage;
use async_trait::async_trait;

pub struct Todos {
    pub list: Vec<Todo>,
}

#[async_trait]
pub trait TodoStorage {
    async fn push_new_todo(&mut self, todo: Todo, storage: &mut FileStorage) -> Result<(), TerminalError>;
    async fn remove_todo(&mut self, index_todo: usize, storage: &mut FileStorage) -> Result<(), TerminalError>;
    fn is_empty(&mut self) -> bool;
    async fn get_list(&mut self, storage: &mut FileStorage) -> Result<Vec<Todo>, TerminalError>;
    async fn clear(&mut self, storage: &mut FileStorage) -> Result<(), TerminalError>;
    async fn mark_done(&mut self, index_todo: usize, storage: &mut FileStorage) -> Result<(), TerminalError>;
}

#[async_trait]
impl TodoStorage for Todos {
    async fn push_new_todo(&mut self, todo: Todo, storage: &mut FileStorage) -> Result<(), TerminalError> {
        let mut todo_list = storage.get_todos_from_filestorage().await.map_err(TerminalError::StorageError)?;
        todo_list.list.push(todo);
        self.list = todo_list.list;
        storage.write_filestorage(self).await.map_err(TerminalError::StorageError)?;
        Ok(())
    }

    async fn remove_todo(&mut self, index_todo: usize, storage: &mut FileStorage) -> Result<(), TerminalError> {
        let mut todo_list = storage.get_todos_from_filestorage().await.map_err(TerminalError::StorageError)?;
        if index_todo > todo_list.list.len() {
            return Err(TerminalError::IndexError);
        }
        todo_list.list.remove(index_todo);
        self.list = todo_list.list;
        storage.write_filestorage(self).await.map_err(TerminalError::StorageError)?;
        Ok(())
    }

    async fn mark_done(&mut self, index_todo: usize, storage: &mut FileStorage) -> Result<(), TerminalError> {
        let mut todo_list = storage.get_todos_from_filestorage().await.map_err(TerminalError::StorageError)?;
        if let Some(todo) = todo_list.list.get_mut(index_todo) {
            todo.done = true;
        } else {
            return Err(TerminalError::IndexError)
        }
        self.list = todo_list.list;
        storage.write_filestorage(self).await.map_err(TerminalError::StorageError)?;
        Ok(())
    }

    fn is_empty(&mut self) -> bool {
        self.list.is_empty()
    }

    async fn get_list(&mut self, storage: &mut FileStorage) -> Result<Vec<Todo>, TerminalError> {
        let todos = storage.get_todos_from_filestorage().await.map_err(TerminalError::StorageError)?;
        Ok(todos.list)
    }

    async fn clear(&mut self, storage: &mut FileStorage) -> Result<(), TerminalError> {
        self.list.clear();
        storage.write_filestorage(self).await.map_err(TerminalError::StorageError)?;
        Ok(())
    }
}

impl Todos {
    pub fn new() -> Self {
        Todos { list: Vec::new() }
    }
}
