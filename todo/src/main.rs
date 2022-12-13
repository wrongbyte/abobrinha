mod terminal;
mod todo;
use terminal::{Terminal};
use terminal::error::TerminalError;

fn run() -> Result<(), TerminalError> {
    let mut terminal = Terminal::new();
    while let Ok(Some(todo)) = terminal.ask_new_todo() {
        terminal.show_todo(&todo)?;
    }
    terminal.write_stdout("Ok, quitting now.")?;
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        println!("{}", error.format_error())
    }
}
