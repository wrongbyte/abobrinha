use crate::domain::todos::TodoStorage;
use crate::{
    filestorage::FileStorage,
    terminal::{error::TerminalError, UserInterface, UserOptions},
};
use std::path::PathBuf;

pub(crate) struct TodoCli {
    pub user_interface: Box<dyn UserInterface>,
    pub todo_storage: Box<dyn TodoStorage>,
}

impl TodoCli {
    pub async fn run(&mut self) -> Result<(), TerminalError> {
        let mut file = FileStorage {
            path: PathBuf::from("todo.txt"),
        };
        loop {
            match self.user_interface.user_intention()? {
                UserOptions::Quit => break,
                UserOptions::NewTodo(todo) => {
                    self.todo_storage.push_new_todo(todo, &mut file).await?;
                    let todo_list = self.todo_storage.get_list(&mut file).await?;
                    self.user_interface.show_todo_list(&todo_list)?
                }
                UserOptions::Help => self.user_interface.show_help()?,
                UserOptions::ClearList => {
                    self.todo_storage.clear(&mut file).await?;
                    self.user_interface.clear_todo_message()?
                }
                UserOptions::RemoveTodo(index) => {
                    self.todo_storage.remove_todo(index, &mut file).await?;
                    self.user_interface.remove_todo_message()?
                }
                UserOptions::Unrecognized => self.user_interface.alert_unrecognized()?,
                UserOptions::ShowList => self
                    .user_interface
                    .show_todo_list(&self.todo_storage.get_list(&mut file).await?)?,
                UserOptions::DoTodo(index) => {
                    self.todo_storage.mark_done(index, &mut file).await?;
                    self.user_interface.mark_done_message()?;
                    self.user_interface
                        .show_todo_list(&self.todo_storage.get_list(&mut file).await?)?
                }
            }
        }
        self.user_interface.write_interface(&"Ok, quitting now.")?;
        Ok(())
    }
}
