use std::path::PathBuf;

use crate::terminal::Terminal;
use repository::file_storage::FileStorage;
use todocli::TodoCli;
mod terminal;
mod todocli;
mod repository;
mod domain;
extern crate factori;

#[tokio::main]
async fn main() {
    let user_interface = Box::new(Terminal::new());
    let todo_storage = Box::new(FileStorage {
        path: PathBuf::from("todo.txt"),
    });
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
