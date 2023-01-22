pub(crate) mod error;
use crate::todo::Todo;
use crate::todos::Todos;
use async_trait::async_trait;
use error::StorageError;
use std::io::SeekFrom;
use tokio::fs::{OpenOptions, remove_file};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

#[async_trait]
pub trait Storage {
    async fn get_todos_from_storage() -> Result<Todos, StorageError>;
    async fn insert_todo(todo: String) -> Result<(), StorageError>;
    async fn remove_todo(todo_number: usize) -> Result<(), StorageError>;
    async fn clear_todos() -> Result<(), StorageError>;
    async fn mark_done(todo_list: &mut Todos) -> Result<(), StorageError>;
}

pub struct FileStorage {
    // pub file: tokio::fs::File,
    // pub todo_vec: Todos,
}

#[async_trait]
impl Storage for FileStorage {
    async fn get_todos_from_storage() -> Result<Todos, StorageError> {
        let mut buffer = Vec::new();
        let mut todo_vec = Vec::new();
        let mut file = FileStorage::open_file().await?;
        file.seek(SeekFrom::Start(0))
            .await
            .map_err(|_| StorageError::WriteError)?;

        file.read_to_end(&mut buffer)
            .await
            .map_err(|_| StorageError::ReadError)?;
        let s = String::from_utf8(buffer).map_err(|e| StorageError::InvalidBuffer(e))?;
        let mut vec_todo: Vec<&str> = s.split("\n").collect();
        vec_todo.truncate(vec_todo.len() - 1);
        for line in vec_todo.iter() {
            let todo = build_todo(line.to_string())?;
            todo_vec.push(todo);
        }
        Ok(Todos { list: todo_vec })
    }

    async fn insert_todo(todo: String) -> Result<(), StorageError> {
        let mut file = FileStorage::open_file().await?;
        let todo_newline = todo + "\n";
        file.write_all(todo_newline.as_bytes())
            .await
            .map_err(|_| StorageError::WriteError)?;
        file.flush().await.map_err(|_| StorageError::WriteError)?;
        Ok(())
    }

    async fn clear_todos() -> Result<(), StorageError> {
        remove_file("todo.txt").await.map_err(|_| StorageError::WriteError)?;
        FileStorage::open_file().await?;
        Ok(())
    }

    async fn remove_todo(todo_index: usize) -> Result<(), StorageError> {
        let mut todo_list = FileStorage::get_todos_from_storage().await?;

        FileStorage::clear_todos().await.map_err(|_| StorageError::WriteError)?;
        
        todo_list.list.remove(todo_index);

        for todo in todo_list.list.iter() {
            let formatted_todo = "[ ] - ".to_owned() + &todo.message.to_string();
            FileStorage::insert_todo(formatted_todo).await?;
        }

        Ok(())
    }

    async fn mark_done(todo_list: &mut Todos) -> Result<(), StorageError> {
        FileStorage::clear_todos().await.map_err(|_| StorageError::WriteError)?;
        
        for todo in todo_list.list.iter() {
            let formatted_todo = match todo.done {
                true => "[X] - ".to_owned() + &todo.message.to_string(),
                false => "[ ] - ".to_owned() + &todo.message.to_string()
            };
            FileStorage::insert_todo(formatted_todo).await?;
        }

        Ok(())
    }

}

impl FileStorage {
    pub async fn open_file() -> Result<tokio::fs::File, StorageError> {
        OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open("todo.txt")
        .await
        .map_err(|_| StorageError::OpenError)
    }
}

fn build_todo(line: String) -> Result<Todo, StorageError> {
    let done = line.starts_with("[X] - ");
    if let Some(message) = line.split("] - ").nth(1) {
        Ok(Todo {
            message: message.to_string(),
            done,
        })
    } else {
        Err(StorageError::EmptyTodoError)
    }
}
