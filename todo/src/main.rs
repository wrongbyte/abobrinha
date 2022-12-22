use crate::terminal::{error::TerminalError, Terminal};
use console::style;
use todos::Todos;
mod terminal;
mod todo;
mod todos;

fn run() -> Result<(), TerminalError> {
    let mut terminal = Terminal::new();
    let mut todo_list = Todos::new();
    while let Some(todo) = terminal.get_new_todo_or_quit(&mut todo_list)? {
        todo_list.push_new_todo(todo.clone());
        terminal.write_stdout(&style("Your current todo list is:").green().to_string())?;
        for todo in &todo_list.list {
            terminal.show_todo(todo)?;
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
