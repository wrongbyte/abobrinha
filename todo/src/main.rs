use crate::terminal::Terminal;
use controllers::todo::TodoControllerImpl;
use db::connect::connect;
use repository::todo::PostgresTodoRepository;
use todocli::TodoCli;
mod controllers;
mod db;
mod domain;
mod repository;
mod terminal;
mod todocli;
extern crate factori;

#[tokio::main]
async fn main() {
    let client = connect()
        .await
        .expect("Database connection error. Quitting");
    let todo_repository = Box::new(PostgresTodoRepository { client });

    let todo_controller = Box::new(TodoControllerImpl {
        todo_repository,
        user_interface: Box::new(Terminal::new()),
    });

    let mut todo_cli = TodoCli {
        user_interface: Box::new(Terminal::new()),
        todo_controller,
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
