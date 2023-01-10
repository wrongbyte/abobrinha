use crate::terminal::Terminal;
use todocli::TodoCli;
use todos::Todos;
mod terminal;
mod todo;
mod todocli;
mod todos;

fn main() {
    let user_interface = Box::new(Terminal::new());
    let todo_storage = Box::new(Todos::new());
    let mut todo_cli = TodoCli {
        user_interface,
        todo_storage,
    };
    loop {
        if let Err(mut error) = todo_cli.run() {
            println!("{}", error);
            if error.is_fatal() {
                break;
            }
        }
    }
}
