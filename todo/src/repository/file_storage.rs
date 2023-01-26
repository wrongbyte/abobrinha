use async_trait::async_trait;
use tokio::fs::{read_to_string, write};
pub(crate) mod error;
use std::path::PathBuf;
use crate::domain::todo::Todo;
use crate::domain::todos::Todos;
use crate::repository::file_storage::error::StorageError;

pub struct FileStorage {
    pub path: PathBuf,
}

#[async_trait]
pub trait Storage {
    async fn get_todos_from_filestorage(&self) -> Result<Todos, StorageError>;
    async fn write_filestorage(&self, todo_list: &mut Todos) -> Result<(), StorageError>;
}

#[async_trait]
impl Storage for FileStorage {
    async fn get_todos_from_filestorage(&self) -> Result<Todos, StorageError> {
        let todo_str = read_to_string(&self.path)
            .await
            .map_err(|_| StorageError::Read)?;

        let vec_todo = todo_str
            .lines()
            .map(|line| FileStorage::build_todo(line.to_string()))
            .collect::<Result<Vec<Todo>, _>>()? ;

        Ok(Todos { list: vec_todo })
    }

    async fn write_filestorage(&self, todo_list: &mut Todos) -> Result<(), StorageError> {
        let mut todo_list_str = String::new();

        for todo in &todo_list.list {
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
