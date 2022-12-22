use crate::terminal::{error::TerminalError, Terminal};
use console::style;
use terminal::UserOptions;
use todos::Todos;
mod terminal;
mod todo;
mod todos;

fn run() -> Result<(), TerminalError> {
    let mut terminal = Terminal::new();
    let mut todo_list = Todos::new();
    loop {
        match terminal.user_intention()? {
            UserOptions::Quit => break,
            UserOptions::NewTodo(todo) => {
                todo_list.push_new_todo(todo.clone());
                terminal.show_todo_list(&todo_list)?
            }
            UserOptions::Help => terminal.show_help()?,
            UserOptions::ClearList => {
                todo_list.list.clear();
                terminal.clear_todo()?
            }
            UserOptions::RemoveTodo(index) => {
                todo_list.remove_todo(index)?;
                terminal.remove_todo()?
            }
            UserOptions::Unrecognized => terminal.alert_unrecognized()?,
            UserOptions::ShowList => terminal.show_todo_list(&todo_list)?,
        }
    }
    terminal.write_stdout(&style("Ok, quitting now.").blue().to_string())?;
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        println!("{}", error.format_error())
    }
}
