use async_trait::async_trait;
use tokio_postgres::Client;
use uuid::Uuid;
pub(crate) mod error;
use crate::domain::todo::Todo;
use crate::domain::todos::Todos;
use crate::repository::todo::error::StorageError;
use std::sync::Arc;

pub struct PostgresTodoRepository {
    pub client: Arc<Client>,
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait Storage {
    async fn add_todo(&mut self, todo: Todo) -> Result<(), StorageError>;
    async fn get_todo_list(&mut self) -> Result<Todos, StorageError>;
    async fn clear_todo_list(&mut self) -> Result<(), StorageError>;
    async fn remove_todo(&mut self, todo_uuid: Uuid) -> Result<u64, StorageError>;
    async fn mark_todo_done(&mut self, todo_uuid: Uuid) -> Result<u64, StorageError>;
}

#[async_trait]
impl Storage for PostgresTodoRepository {
    async fn add_todo(&mut self, todo: Todo) -> Result<(), StorageError> {
        let message = todo.message;
        let todo_uuid = Uuid::new_v4();
        self.client
            .execute(
                "INSERT INTO todos(message, id) VALUES($1, $2)",
                &[&message, &todo_uuid],
            )
            .await
            .map_err(|error| StorageError { error })?;
        Ok(())
    }
    async fn get_todo_list(&mut self) -> Result<Todos, StorageError> {
        let todos = self
            .client
            .query("SELECT * FROM todos;", &[])
            .await
            .map_err(|error| StorageError { error })?
            .into_iter()
            .map(|row| Todo {
                done: row.get("done"),
                message: row.get("message"),
                id: row.get("id"),
            })
            .collect();

        Ok(Todos::new(todos))
    }
    async fn clear_todo_list(&mut self) -> Result<(), StorageError> {
        self.client
            .execute("DELETE FROM todos", &[])
            .await
            .map_err(|error| StorageError { error })?;
        Ok(())
    }
    async fn remove_todo(&mut self, todo_uuid: Uuid) -> Result<u64, StorageError> {
        let number_modified = self
            .client
            .execute("DELETE FROM todos WHERE id=$1", &[&todo_uuid])
            .await
            .map_err(|error| StorageError { error })?;
        Ok(number_modified)
    }
    async fn mark_todo_done(&mut self, todo_uuid: Uuid) -> Result<u64, StorageError> {
        let number_modified = self
            .client
            .execute("UPDATE todos SET done='t' WHERE id=$1", &[&todo_uuid])
            .await
            .map_err(|error| StorageError { error })?;
        Ok(number_modified)
    }
}
