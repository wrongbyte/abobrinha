pub mod todo;

#[cfg(test)]
mod test_utils {
    use crate::domain::todo::Todo;
    use crate::repository::todo::error::StorageError;
    use crate::repository::todo::get_todo_from_sql;
    use http_problem::prelude::*;
    use std::{future::Future, panic::AssertUnwindSafe, sync::Arc};
    use tokio_postgres::Client;
    use uuid::Uuid;

    use futures_util::FutureExt;

    pub async fn connect() -> Result<Arc<Client>> {
        let (client, connection) = tokio_postgres::connect(
            "host=localhost user=postgres password=postgres dbname=onboarding_test",
            tokio_postgres::NoTls,
        )
        .await?;

        tokio::spawn(async move {
            if let Err(err) = connection.await {
                println!("connection error: {err}")
            }
        });
        println!("Successfully connected to database");
        Ok(Arc::new(client))
    }

    pub async fn add_todo(client: Arc<Client>) -> Result<(), StorageError> {
        let todo_uuid = Uuid::new_v4();
        client
            .execute(
                "INSERT INTO todos(message, id) VALUES('Test todo', $1);",
                &[&todo_uuid],
            )
            .await
            .map_err(|error| StorageError { error })?;
        Ok(())
    }

    pub async fn get_todo_by_id(
        client: Arc<Client>,
        id: Uuid,
    ) -> Result<Option<Todo>, StorageError> {
        println!("{id}");
        let todo = client
            .query_opt("SELECT id, message, done FROM todos WHERE id = $1;", &[&id])
            .await
            .map_err(|error| StorageError { error })?
            .map(get_todo_from_sql);
        Ok(todo)
    }

    pub async fn with_client<Fn, Fut>(f: Fn)
    where
        Fn: FnOnce(Arc<Client>) -> Fut,
        Fut: Future<Output = ()>,
    {
        let client = connect().await.unwrap();
        client.execute("BEGIN;", &[]).await.expect("tx begin");

        let res = AssertUnwindSafe(f(client.clone())).catch_unwind().await;

        client.execute("ROLLBACK;", &[]).await.expect("tx rollback");

        if let Err(panic) = res {
            std::panic::resume_unwind(panic);
        }
    }
}
