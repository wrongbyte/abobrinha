use async_trait::async_trait;
use tokio::fs::{read_to_string, write};
pub(crate) mod error;
use crate::domain::todo::Todo;
use crate::domain::todos::Todos;
use crate::repository::file_storage::error::StorageError;
use std::path::PathBuf;


pub struct FileStorage {
    pub path: PathBuf,
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait Storage {
    async fn get_todos_from_filestorage(&self) -> Result<Todos, StorageError>;
    async fn write_filestorage(&self, todo_list: &Todos) -> Result<(), StorageError>;
}

#[async_trait]
impl Storage for FileStorage {
    async fn get_todos_from_filestorage(&self) -> Result<Todos, StorageError> {
        let todo_str = read_to_string(&self.path)
            .await
            .map_err(|_| StorageError::Read)?;

        let vec_todo = todo_str
            .lines()
            .map(FileStorage::build_todo)
            .collect::<Result<Vec<Todo>, _>>()?;

        Ok(Todos::new(vec_todo))
    }

    async fn write_filestorage(&self, todo_list: &Todos) -> Result<(), StorageError> {
        let mut todo_list_str = String::new();

        for todo in todo_list {
            let item_list = match todo.done {
                true => format!("[X] - {}\n", &todo.message),
                false => format!("[ ] - {}\n", &todo.message),
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
    fn build_todo(line: &str) -> Result<Todo, StorageError> {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn should_read_from_storage() {
        let mut mock = MockStorage::new();
        let todo_list = Todos::new([].to_vec());
        mock.expect_get_todos_from_filestorage().return_once(|| Ok(todo_list));
    }

    #[tokio::test]
    async fn should_write_to_storage() {
        let mut mock = MockStorage::new();
        mock.expect_write_filestorage().return_once(|_| Ok(()));
    }
}
