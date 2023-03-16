use crate::terminal::Terminal;
use controllers::todo::{TodoController, TodoControllerImpl};
use db::connect::connect;
use repository::todo::PostgresTodoRepository;
mod controllers;
mod db;
mod domain;
mod repository;
mod terminal;
extern crate factori;

#[tokio::main]
async fn main() {
    let client = connect()
        .await
        .expect("Database connection error. Quitting");
    let todo_repository = Box::new(PostgresTodoRepository { client });
    let user_interface = Box::new(Terminal::new());

    let mut todo_controller = Box::new(TodoControllerImpl {
        todo_repository,
        user_interface,
    });

    loop {
        if let Err(error) = todo_controller.get_user_intention().await {
            todo_controller.user_interface.print_error(&error);
            if error.is_fatal() {
                break;
            }
        }
    }
}
