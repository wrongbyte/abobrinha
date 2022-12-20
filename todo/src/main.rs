use crate::terminal::{error::TerminalError, Terminal};
use console::style;
use todos::Todos;
mod terminal;
mod todo;
mod todos;

fn run() -> Result<(), TerminalError> {
    let mut terminal = Terminal::new();
    let mut todo_list = Todos::new();
    terminal.write_stdout(
        &style("Welcome to your new todo manager. Type \"help\" to see the list of commands.")
            .blue()
            .to_string(),
    )?;

    while let Ok(Some(todo)) = terminal.ask_new_todo() {
        todo_list.push_new_todo(todo.clone());
        for todo in &todo_list.list {
            terminal.show_todo(&todo)?;
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
