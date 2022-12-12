pub(crate) mod error;
use error::TerminalError;
use std::io::{Stdin, Stdout, Write};
use crate::todo::Todo;
use console::{style};

pub enum TerminalColors {
    Red,
    Green,
    Yellow,
    White
}
pub struct Terminal {
    stdin: Stdin,
    stdout: Stdout,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
        }
    }

    pub fn ask_new_todo(&mut self) -> Result<Option<Todo>, TerminalError> {
        if !self.user_intention()? {
            return Ok(None);
        }

        self.write_stdout("Write your new todo:", TerminalColors::Yellow)?;
        let user_input = self.input()?;

        if user_input.is_empty() {
            self.write_stdout("Please input a valid todo.", TerminalColors::Red)?;
            self.ask_new_todo()
        } else {
            Ok(Some(Todo::new(user_input)))
        }
    }

    pub fn show_todo(&mut self, todo: &Todo) -> Result<(), TerminalError> {
        self.write_stdout(&format!("[ ] - {}", todo.message), TerminalColors::Green)
    }

    fn user_intention(&mut self) -> Result<bool, TerminalError> {
        self.write_stdout("Do you want to input a new todo? (y/n)", TerminalColors::Yellow)?;
        let user_input = self.input();
        Ok(matches!(user_input, Ok(input) if input == "y"))
    }

    fn input(&mut self) -> Result<String, TerminalError> {
        let mut buf = String::new();
        self.stdin
            .read_line(&mut buf)
            .map_err(TerminalError::Stdin)
            .map(|_| buf.trim().to_string())
    }

    pub fn write_stdout(&mut self, string: &str, color: TerminalColors) -> Result<(), TerminalError> {
        let coloful_str = match color {
            TerminalColors::Red => style(string).red(),
            TerminalColors::Green => style(string).green(),
            TerminalColors::Yellow => style(string).yellow(),
            TerminalColors::White => style(string).white()
        };
        writeln!(self.stdout, "{}", coloful_str).map_err(TerminalError::Stdout)
    }
}
