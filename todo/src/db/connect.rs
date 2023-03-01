use std::sync::Arc;

use http_problem::prelude::*;
use tokio_postgres::Client;

pub async fn connect() -> Result<Arc<Client>> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=postgres dbname=onboarding",
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
