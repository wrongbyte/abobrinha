use error::TerminalError;
use std::io::{Stdin, Stdout, Write};
use crate::todo::Todo;
use console::{style};
pub(crate) mod error;

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

        self.write_stdout(&style("Write your new todo:").blue().to_string())?;
        let user_input = self.input()?;

        if user_input.is_empty() {
            self.write_stdout(&style("Please input a valid todo.").red().to_string())?;
            self.ask_new_todo()
        } else {
            Ok(Some(Todo::new(user_input)))
        }
    }

    pub fn show_todo(&mut self, todo: &Todo) -> Result<(), TerminalError> {
        let formatted_msg = format!("[ ] - {}", todo.message);
        self.write_stdout(&style(formatted_msg).green().to_string())
    }

    fn user_intention(&mut self) -> Result<bool, TerminalError> {
        self.write_stdout(&style("Do you want to input a new todo? Type \"y\" to add a new todo or \"help\" to see all commands.").blue().to_string())?;
        let user_input = self.input()?;
        if user_input == "help" {
            self.show_help()?;
        }
        Ok(user_input == "y")
    }

    fn input(&mut self) -> Result<String, TerminalError> {
        let mut buf = String::new();
        self.stdin
            .read_line(&mut buf)
            .map_err(TerminalError::Stdin)
            .map(|_| buf.trim().to_string())
    }

    pub fn write_stdout(&mut self, string: &str) -> Result<(), TerminalError> {
        writeln!(self.stdout, "{}", string)
            .map_err(TerminalError::Stdout)
    }
    
    pub fn show_help(&mut self) -> Result<(), TerminalError> {
        self.write_stdout(&style("====== WELCOME =======").yellow().to_string())?;
        self.write_stdout("")?;
        self.write_stdout(&style("⭐️ To add a new todo, type y when asked, type your todo and press enter. ⭐️").yellow().to_string())?;
        self.write_stdout(&style("⭐️ To remove a todo, type \"rm n\", being \"n\" the index of the todo in the list. ⭐️").yellow().to_string())?;
        self.write_stdout(&style("⭐️ To clear the list of todos, type \"clear\" ⭐️").yellow().to_string())?;
        self.user_intention()?;
        Ok(())
    }
}
