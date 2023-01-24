pub(crate) mod error;
use crate::todo::Todo;
use crate::todos::Todos;
use async_trait::async_trait;
use error::StorageError;
use tokio::fs::{read_to_string, remove_file, write, OpenOptions};
use tokio::io::{AsyncWriteExt};

#[async_trait]
pub trait Storage {
    async fn get_todos_from_filestorage() -> Result<Todos, StorageError>;
    async fn insert_todo(todo: String) -> Result<(), StorageError>;
    async fn remove_todo(todo_number: usize) -> Result<(), StorageError>;
    async fn clear_todos() -> Result<(), StorageError>;
    async fn mark_done(todo_list: &mut Todos) -> Result<(), StorageError>;
    async fn write_filestorage(todo_list: &mut Todos) -> Result<(), StorageError>;
}

pub struct FileStorage {
    pub path: String,
}

#[async_trait]
impl Storage for FileStorage {
    async fn get_todos_from_filestorage() -> Result<Todos, StorageError> {
        let mut todo_vec = Vec::new();
        let todo_str = read_to_string("todo.txt")
            .await
            .map_err(|_| StorageError::ReadError)?;

        let mut vec_todo: Vec<&str> = todo_str.split("\n").collect();
        vec_todo.truncate(vec_todo.len() - 1);

        for line in vec_todo.iter() {
            let todo = build_todo(line.to_string())?;
            todo_vec.push(todo);
        }
        Ok(Todos { list: todo_vec })
    }

    async fn write_filestorage(todo_list: &mut Todos) -> Result<(), StorageError> {
        let mut todo_list_str = String::new();
        for todo in todo_list.list.iter() {
            let item_list = match todo.done {
                true => "[X] - ".to_owned() + &todo.message.to_string() + "\n",
                false => "[ ] - ".to_owned() + &todo.message.to_string() + "\n",
            };
            todo_list_str.push_str(&item_list);
        }
        write("todo.txt", todo_list_str).await.map_err(|_| StorageError::WriteError)?;
        Ok(())
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
        remove_file("todo.txt")
            .await
            .map_err(|_| StorageError::DeleteError)?;
        FileStorage::open_file().await?;
        Ok(())
    }

    async fn remove_todo(todo_index: usize) -> Result<(), StorageError> {
        let mut todo_list = FileStorage::get_todos_from_filestorage().await?;

        FileStorage::clear_todos()
            .await
            .map_err(|_| StorageError::WriteError)?;

        todo_list.list.remove(todo_index);

        for todo in todo_list.list.iter() {
            let formatted_todo = "[ ] - ".to_owned() + &todo.message.to_string();
            FileStorage::insert_todo(formatted_todo).await?;
        }
        Ok(())
    }

    async fn mark_done(todo_list: &mut Todos) -> Result<(), StorageError> {
        FileStorage::clear_todos()
            .await
            .map_err(|_| StorageError::WriteError)?;

        for todo in todo_list.list.iter() {
            let formatted_todo = match todo.done {
                true => "[X] - ".to_owned() + &todo.message.to_string(),
                false => "[ ] - ".to_owned() + &todo.message.to_string(),
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
