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
    NewTodo(Todo),
    RemoveTodo(usize),
    ClearList,
    Quit,
    Help,
    ShowList,
    Unrecognized,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
        }
    }

    pub fn prompt_new_todo(&mut self) -> Result<Todo, TerminalError> {
        self.write_stdout(&style("Write your new todo:").blue().to_string())?;
        let user_input = self.input()?;

        if user_input.is_empty() {
            self.write_stdout(&style("Please input a valid todo.").red().to_string())?;
            self.prompt_new_todo()
        } else {
            Ok(Todo::new(user_input))
        }
    }

    pub fn show_todo(&mut self, todo: &Todo) -> Result<(), TerminalError> {
        let formatted_msg = format!("[ ] - {}", todo.message);
        self.write_stdout(&style(formatted_msg).green().to_string())
    }

    pub fn alert_unrecognized(&mut self) -> Result<(), TerminalError> {
        self.write_stdout(&style("Invalid option. Please type again").red().to_string())
    }

    pub fn clear_todo(&mut self) -> Result<(), TerminalError> {
        self.write_stdout(
            &style("Successfully cleared all todos.")
                .yellow()
                .to_string(),
        )?;
        Ok(())
    }

    pub fn show_todo_list(&mut self, todo_list: &Todos) -> Result<(), TerminalError> {
        if todo_list.list.is_empty() {
            self.write_stdout(&style("Your current todo list is empty!").green().to_string())?;
        } else {
            self.write_stdout(&style("Your current todo list is:").green().to_string())?;
            for todo in &todo_list.list {
                self.show_todo(todo)?;
            };
        }
        Ok(())
    }

    pub fn remove_todo(
        &mut self,
    ) -> Result<(), TerminalError> {
        self.write_stdout(&style("Successfully removed todo.").yellow().to_string())
    }

    pub fn user_intention(&mut self) -> Result<UserOptions, TerminalError> {
        self.write_stdout(&style("Do you want to input a new todo? Type \"y\" to add a new todo or \"help\" to see all commands.").blue().to_string())?;
        let user_input = self.input()?;

        if let Some(index) = user_input.strip_prefix("rm ") {
            let parsed_i = index
                .parse()
                .map_err(|_| TerminalError::ParseInt(index.to_string()))?;
            return Ok(UserOptions::RemoveTodo(parsed_i));
        }

        match user_input.as_str() {
            "y" => Ok(UserOptions::NewTodo(self.prompt_new_todo()?)),
            "help" => Ok(UserOptions::Help),
            "clear" => Ok(UserOptions::ClearList),
            "quit" => Ok(UserOptions::Quit),
            "list" => Ok(UserOptions::ShowList),
            _ => Ok(UserOptions::Unrecognized),
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
        self.write_stdout(
            &style("⭐️ To see the list of todos, type \"list\" ⭐️")
                .yellow()
                .to_string(),
        )?;
        Ok(())
    }
}
