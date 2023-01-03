use console::style;

use crate::{
    terminal::{error::TerminalError, UserInterface, UserOptions},
    todos::TodoStorage,
};

pub(crate) struct TodoCli {
    pub user_interface: Box<dyn UserInterface>,
    pub todo_storage: Box<dyn TodoStorage>,
}

impl TodoCli {
    pub fn run(&mut self) -> Result<(), TerminalError> {
        loop {
            match self.user_interface.user_intention()? {
                UserOptions::Quit => break,
                UserOptions::NewTodo(todo) => {
                    self.todo_storage.push_new_todo(todo);
                    self.user_interface.show_todo_list(&mut self.todo_storage)?
                }
                UserOptions::Help => self.user_interface.show_help()?,
                UserOptions::ClearList => {
                    self.todo_storage.get_list().clear();
                    self.user_interface.clear_todo_message()?
                }
                UserOptions::RemoveTodo(index) => {
                    self.todo_storage.remove_todo(index)?;
                    self.user_interface.remove_todo_message()?
                }
                UserOptions::Unrecognized => self.user_interface.alert_unrecognized()?,
                UserOptions::ShowList => {
                    self.user_interface.show_todo_list(&mut self.todo_storage)?
                }
            }
        }
        self.user_interface
            .write_stdout(&style("Ok, quitting now.").blue().to_string())?;
        Ok(())
    }
}
