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
        match terminal.user_intention(&mut todo_list)? {
            UserOptions::Quit => break,
            UserOptions::NewTodo(todo) => {
                todo_list.push_new_todo(todo.clone());
                terminal.show_todo_list(&mut todo_list)?
            }
            UserOptions::Help => terminal.show_help(&mut todo_list)?,
            UserOptions::ClearList => terminal.clear_todo(&mut todo_list)?,
            UserOptions::RemoveTodo(index) => terminal.remove_todo(&mut todo_list, index)?,
            UserOptions::Unrecognized => terminal.alert_unrecognized()?,
            UserOptions::ShowList => terminal.show_todo_list(&mut todo_list)?
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
