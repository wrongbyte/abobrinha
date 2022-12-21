use crate::{todo::Todo, todos::Todos};
use console::style;
use error::TerminalError;
use std::{
    io::{Stdin, Stdout, Write},
    usize,
};
pub(crate) mod error;

pub struct Terminal {
    stdin: Stdin,
    stdout: Stdout,
}

pub enum UserOptions {
    NewTodo,
    RemoveTodo(usize),
    ClearList,
    Quit,
    Help,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
        }
    }

    pub fn ask_new_todo(&mut self, todo_list: &mut Todos) -> Result<Option<Todo>, TerminalError> {
        match self.user_intention()? {
            UserOptions::Quit => return Ok(None),
            UserOptions::RemoveTodo(index) => {
                todo_list.remove_todo(index)?;
                self.write_stdout(&style("Successfully removed todo.").yellow().to_string())?;
                self.user_intention()?;
            }
            UserOptions::ClearList => {
                todo_list.list.clear();
                self.write_stdout(
                    &style("Successfully cleared all todos.")
                        .yellow()
                        .to_string(),
                )?;
                self.user_intention()?;
            }
            UserOptions::Help => self.show_help()?,
            _ => (),
        }

        self.write_stdout(&style("Write your new todo:").blue().to_string())?;
        let user_input = self.input()?;

        if user_input.is_empty() {
            self.write_stdout(&style("Please input a valid todo.").red().to_string())?;
            self.ask_new_todo(todo_list)
        } else {
            Ok(Some(Todo::new(user_input)))
        }
    }

    pub fn show_todo(&mut self, todo: &Todo) -> Result<(), TerminalError> {
        let formatted_msg = format!("[ ] - {}", todo.message);
        self.write_stdout(&style(formatted_msg).green().to_string())
    }

    fn user_intention(&mut self) -> Result<UserOptions, TerminalError> {
        self.write_stdout(&style("Do you want to input a new todo? Type \"y\" to add a new todo or \"help\" to see all commands.").blue().to_string())?;
        let user_input = self.input()?;

        if user_input.starts_with("rm ") {
            let index = user_input.split(' ').collect::<Vec<&str>>()[1]
                .to_string()
                .parse::<usize>()
                .map_err(TerminalError::ParseInt)?;

            return Ok(UserOptions::RemoveTodo(index));
        }

        match user_input.as_str() {
            "help" => Ok(UserOptions::Help),
            "y" => Ok(UserOptions::NewTodo),
            "clear" => Ok(UserOptions::ClearList),
            _ => Ok(UserOptions::Quit),
        }
    }

    fn input(&mut self) -> Result<String, TerminalError> {
        let mut buf = String::new();
        self.stdin
            .read_line(&mut buf)
            .map_err(TerminalError::Stdin)
            .map(|_| buf.trim().to_string())
    }

    pub fn write_stdout(&mut self, string: &str) -> Result<(), TerminalError> {
        writeln!(self.stdout, "{}", string).map_err(TerminalError::Stdout)
    }

    pub fn show_help(&mut self) -> Result<(), TerminalError> {
        self.write_stdout(
            &style("====== LIST OF COMMANDS =======")
                .yellow()
                .to_string(),
        )?;
        self.write_stdout("")?;
        self.write_stdout(
            &style("⭐️ To add a new todo, type y when asked, type your todo and press enter. ⭐️")
                .yellow()
                .to_string(),
        )?;
        self.write_stdout(&style("⭐️ To remove a todo, type \"rm n\", being \"n\" the index of the todo in the list. ⭐️").yellow().to_string())?;
        self.write_stdout(
            &style("⭐️ To clear the list of todos, type \"clear\" ⭐️")
                .yellow()
                .to_string(),
        )?;
        self.user_intention()?;
        Ok(())
    }
}
