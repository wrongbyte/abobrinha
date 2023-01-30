use crate::domain::{todo::Todo, todos::Todos};
use console::style;
use error::TerminalError;
use std::{
    fmt::Display,
    io::{Stdin, Stdout, Write},
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
    DoTodo(usize),
}

pub trait UserInterface {
    fn prompt_new_todo(&mut self) -> Result<Todo, TerminalError>;
    fn show_todo(&mut self, todo: &Todo) -> Result<(), TerminalError>;
    fn alert_unrecognized(&mut self) -> Result<(), TerminalError>;
    fn clear_todo_message(&mut self) -> Result<(), TerminalError>;
    fn remove_todo_message(&mut self) -> Result<(), TerminalError>;
    fn user_intention(&mut self) -> Result<UserOptions, TerminalError>;
    fn input(&mut self) -> Result<String, TerminalError>;
    fn write_interface(&mut self, string: &dyn Display) -> Result<(), TerminalError>;
    fn show_help(&mut self) -> Result<(), TerminalError>;
    fn show_todo_list(&mut self, todo_list: Todos) -> Result<(), TerminalError>;
    fn mark_done_message(&mut self) -> Result<(), TerminalError>;
    fn print_error(&mut self, error: &TerminalError);
}

impl UserInterface for Terminal {
    fn print_error(&mut self, error: &TerminalError) {
        self.write_interface(&style(error).red()).unwrap();
    }

    fn prompt_new_todo(&mut self) -> Result<Todo, TerminalError> {
        self.write_interface(&style("Write your new todo:").blue())?;
        let user_input = self.input()?;

        if user_input.is_empty() {
            self.write_interface(&style("Please input a valid todo.").red())?;
            self.prompt_new_todo()
        } else {
            Ok(Todo::new(user_input))
        }
    }

    fn mark_done_message(&mut self) -> Result<(), TerminalError> {
        self.write_interface(&style("Todo marked as done.").green())
    }

    fn show_todo(&mut self, todo: &Todo) -> Result<(), TerminalError> {
        self.write_interface(&style(todo).green())
    }

    fn alert_unrecognized(&mut self) -> Result<(), TerminalError> {
        self.write_interface(&style("Invalid option. Please type again").red())
    }

    fn clear_todo_message(&mut self) -> Result<(), TerminalError> {
        self.write_interface(&style("Successfully cleared all todos.").yellow())?;
        Ok(())
    }

    fn show_todo_list(&mut self, todo_list: Todos) -> Result<(), TerminalError> {
        if todo_list.is_empty() {
            self.write_interface(&style("Your current todo list is empty!").green())?;
        } else {
            self.write_interface(&style("Your current todo list is:").green())?;
            for todo in todo_list.iter() {
                self.show_todo(&todo)?;
            }
        }
        Ok(())
    }

    fn remove_todo_message(&mut self) -> Result<(), TerminalError> {
        self.write_interface(&style("Successfully removed todo.").yellow())
    }

    fn user_intention(&mut self) -> Result<UserOptions, TerminalError> {
        self.write_interface(&style("Do you want to input a new todo? Type \"y\" to add a new todo or \"help\" to see all commands.").blue())?;
        let user_input = self.input()?;

        if let Some(index) = user_input.strip_prefix("rm ") {
            let parsed_i = index
                .parse()
                .map_err(|_| TerminalError::ParseInt(index.to_string()))?;
            return Ok(UserOptions::RemoveTodo(parsed_i));
        }

        if let Some(index) = user_input.strip_prefix("done ") {
            let parsed_i = index
                .parse()
                .map_err(|_| TerminalError::ParseInt(index.to_string()))?;
            return Ok(UserOptions::DoTodo(parsed_i));
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

    fn write_interface(&mut self, string: &dyn Display) -> Result<(), TerminalError> {
        writeln!(self.stdout, "{}", string).map_err(TerminalError::Stdout)
    }

    fn show_help(&mut self) -> Result<(), TerminalError> {
        self.write_interface(&style("====== LIST OF COMMANDS =======").yellow())?;
        self.write_interface(&"")?;
        self.write_interface(
            &style("⭐️ To add a new todo, type y when asked, type your todo and press enter. ⭐️")
                .yellow(),
        )?;
        self.write_interface(&style("⭐️ To remove a todo, type \"rm n\", being \"n\" the index of the todo in the list. ⭐️").yellow())?;
        self.write_interface(&style("⭐️ To clear the list of todos, type \"clear\" ⭐️").yellow())?;
        self.write_interface(&style("⭐️ To see the list of todos, type \"list\" ⭐️").yellow())?;
        self.write_interface(&style("⭐️ To mark a todo as done, type \"done n\", being \"n\" the index of the todo in the list.  ⭐️").yellow())?;
        Ok(())
    }
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
        }
    }
}
