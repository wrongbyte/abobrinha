use crate::terminal::Terminal;
use db::connect::connect;
use repository::todo::PostgresTodoRepository;
use todocli::TodoCli;
mod db;
mod domain;
mod repository;
mod terminal;
mod todocli;
extern crate factori;

#[tokio::main]
async fn main() {
    let client = connect().await.expect("Database connection error. Quitting");
    let user_interface = Box::new(Terminal::new());
    let todo_storage = Box::new(PostgresTodoRepository { client });
    let mut todo_cli = TodoCli {
        user_interface,
        todo_storage,
    };
    loop {
        if let Err(error) = todo_cli.run().await {
            todo_cli.user_interface.print_error(&error);
            if error.is_fatal() {
                break;
            }
        }
    }
}
