use crate::terminal::{Terminal, error::TerminalError};
use console::{style};
mod terminal;
mod todo;

fn run() -> Result<(), TerminalError> {
    let mut terminal = Terminal::new();
    while let Ok(Some(todo)) = terminal.ask_new_todo() {
        terminal.show_todo(&todo)?;
    }
    terminal.write_stdout(&style("Ok, quitting now.").blue().to_string())?;
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        println!("{}", error.format_error())
    }
}
