pub(crate) mod error;
use std::path::PathBuf;

use crate::todo::Todo;
use crate::todos::Todos;
use async_trait::async_trait;
use error::StorageError;
use tokio::fs::{read_to_string, write};

pub struct FileStorage {
    pub path: PathBuf,
}

#[async_trait]
pub trait Storage {
    async fn get_todos_from_filestorage(&mut self) -> Result<Todos, StorageError>;
    async fn write_filestorage(&mut self, todo_list: &mut Todos) -> Result<(), StorageError>;
}

#[async_trait]
impl Storage for FileStorage {
    async fn get_todos_from_filestorage(&mut self) -> Result<Todos, StorageError> {
        let mut todo_vec = Vec::new();
        let todo_str = read_to_string(&self.path)
            .await
            .map_err(|_| StorageError::Read)?;

        let mut vec_todo: Vec<&str> = todo_str.split('\n').collect();
        vec_todo.truncate(vec_todo.len() - 1);

        for line in vec_todo.iter() {
            let todo = FileStorage::build_todo(line.to_string())?;
            todo_vec.push(todo);
        }
        Ok(Todos { list: todo_vec })
    }

    async fn write_filestorage(&mut self, todo_list: &mut Todos) -> Result<(), StorageError> {
        let mut todo_list_str = String::new();
        
        for todo in todo_list.list.iter() {
            let item_list = match todo.done {
                true => format!("[X] - {}\n", &todo.message.to_string()),
                false => format!("[ ] - {}\n", &todo.message.to_string()),
            };
            todo_list_str.push_str(&item_list);
        }
        write(&self.path, todo_list_str)
            .await
            .map_err(|_| StorageError::Write)?;
        Ok(())
    }
}

impl FileStorage {
    pub fn build_todo(line: String) -> Result<Todo, StorageError> {
        let done = line.starts_with("[X] - ");
        if let Some(message) = line.split("] - ").nth(1) {
            Ok(Todo {
                message: message.to_string(),
                done,
            })
        } else {
            Err(StorageError::EmptyTodo)
        }
    }
}
