use crate::terminal::{Terminal};
use todocli::TodoCli;
use todos::Todos;
mod terminal;
mod todo;
mod todos;
mod todocli;

fn main() {
    let user_interface = Box::new(Terminal::new());
    let todo_storage = Box::new(Todos::new());
    let mut todo_cli = TodoCli { user_interface,  todo_storage} ;
    if let Err(error) = todo_cli.run() {
        println!("{}", error.format_error())
    }
}
