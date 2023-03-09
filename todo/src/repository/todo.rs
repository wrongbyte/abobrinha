use async_trait::async_trait;
use tokio_postgres::Client;
use uuid::Uuid;
pub(crate) mod error;
use crate::domain::todo::Todo;
use crate::domain::todos::Todos;
use crate::repository::todo::error::StorageError;
use std::sync::Arc;
use tokio_postgres::Row;

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
        let todo_uuid = todo.id;
        self.client
            .execute(
                "INSERT INTO todos(message, id) VALUES($1, $2)",
                &[&message, &todo_uuid],
            )
            .await?;
        Ok(())
    }
    async fn get_todo_list(&mut self) -> Result<Todos, StorageError> {
        let todos = self
            .client
            .query("SELECT * FROM todos;", &[])
            .await?
            .into_iter()
            .map(get_todo_from_sql)
            .collect();

        Ok(Todos::new(todos))
    }
    async fn clear_todo_list(&mut self) -> Result<(), StorageError> {
        self.client.execute("DELETE FROM todos", &[]).await?;
        Ok(())
    }
    async fn remove_todo(&mut self, todo_uuid: Uuid) -> Result<u64, StorageError> {
        let number_modified = self
            .client
            .execute("DELETE FROM todos WHERE id=$1", &[&todo_uuid])
            .await?;
        Ok(number_modified)
    }
    async fn mark_todo_done(&mut self, todo_uuid: Uuid) -> Result<u64, StorageError> {
        let number_modified = self
            .client
            .execute("UPDATE todos SET done='t' WHERE id=$1", &[&todo_uuid])
            .await?;
        Ok(number_modified)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::test_utils;
    use crate::todocli::mocks::*;
    use factori::create;

    #[tokio::test]
    async fn test_list_todos() {
        test_utils::with_client(|client| async move {
            self::test_utils::add_todo(client.clone()).await.unwrap();

            let mut todo_storage = PostgresTodoRepository { client: client };
            let mut todo_list = todo_storage.get_todo_list().await.unwrap();

            assert_eq!(todo_list.len(), 1);
        })
        .await;
    }

    #[tokio::test]
    async fn test_add_todos() {
        test_utils::with_client(|client| async move {
            let id = Uuid::new_v4();
            let todo = Todo::new("Test add todo".to_string(), id);
            let mut todo_storage = PostgresTodoRepository {
                client: client.clone(),
            };
            todo_storage.add_todo(todo.clone()).await.unwrap();
            let todo_created = self::test_utils::get_todo_by_id(client, id).await.unwrap();
            match todo_created {
                Some(todo_returned) => assert_eq!(todo_returned, todo),
                None => panic!("Could not find todo created"),
            }
        })
        .await;
    }

    #[tokio::test]
    async fn test_remove_todo() {
        test_utils::with_client(|client| async move {
            let todo_list = create!(Todos, number_todos: 3);
            let todo = todo_list.get(2).unwrap();
            let mut todo_storage = PostgresTodoRepository {
                client: client.clone(),
            };

            for todo in todo_list.iter() {
                let t = todo.clone();
                todo_storage.add_todo(t).await.unwrap();
            }
            todo_storage.remove_todo(todo.id).await.unwrap();
            let mut todo_list = todo_storage.get_todo_list().await.unwrap();

            assert_eq!(todo_list.len(), 2);
        })
        .await;
    }

    #[tokio::test]
    async fn test_clear_list() {
        test_utils::with_client(|client| async move {
            let mut todo_storage = PostgresTodoRepository {
                client: client.clone(),
            };

            todo_storage.clear_todo_list().await.unwrap();
            let mut todo_list = todo_storage.get_todo_list().await.unwrap();

            assert_eq!(todo_list.len(), 0);
        })
        .await;
    }
}

pub fn get_todo_from_sql(row: Row) -> Todo {
    Todo {
        done: row.get("done"),
        message: row.get("message"),
        id: row.get("id"),
    }
}
