use crate::terminal::Terminal;
use todocli::TodoCli;
use todos::Todos;
mod terminal;
mod todocli;
mod todos;
mod filestorage;
mod domain {
    pub mod todo;
}

#[tokio::main]
async fn main() {
    let user_interface = Box::new(Terminal::new());
    let todo_storage = Box::new(Todos::new());
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
