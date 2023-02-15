use crate::domain::todo::Todo;
use crate::domain::todos::Todos;
use crate::repository::file_storage::Storage;
use crate::terminal::{error::TerminalError, UserInterface, UserOptions};

pub(crate) struct TodoCli {
    pub user_interface: Box<dyn UserInterface>,
    pub todo_storage: Box<dyn Storage>,
}

impl TodoCli {
    pub async fn run(&mut self) -> Result<(), TerminalError> {
        loop {
            match self.user_interface.user_intention()? {
                UserOptions::Quit => break,
                UserOptions::NewTodo(todo) => self.add_todo(todo).await?,
                UserOptions::Help => self.user_interface.show_help()?,
                UserOptions::ClearList => self.clear_todo_list().await?,
                UserOptions::RemoveTodo(index) => self.remove_todo(index).await?,
                UserOptions::Unrecognized => self.user_interface.alert_unrecognized()?,
                UserOptions::ShowList => self.show_list().await?,
                UserOptions::DoTodo(index) => self.mark_todo_done(index).await?,
            }
        }
        self.user_interface.write_interface(&"Ok, quitting now.")?;
        Ok(())
    }

    async fn show_list(&mut self) -> Result<(), TerminalError> {
        let todo_list = self
            .todo_storage
            .get_todos_from_filestorage()
            .await
            .map_err(TerminalError::StorageError)?;
        self.user_interface.show_todo_list(todo_list)?;
        Ok(())
    }

    async fn add_todo(&mut self, todo: Todo) -> Result<(), TerminalError> {
        let mut todo_list = self
            .todo_storage
            .get_todos_from_filestorage()
            .await
            .map_err(TerminalError::StorageError)?;
        todo_list.push(todo);
        self.todo_storage
            .write_filestorage(&todo_list)
            .await
            .map_err(TerminalError::StorageError)?;
        self.user_interface.show_todo_list(todo_list)?;
        Ok(())
    }

    async fn clear_todo_list(&mut self) -> Result<(), TerminalError> {
        let todo_list = Todos::new(Vec::<Todo>::new());
        self.todo_storage
            .write_filestorage(&todo_list)
            .await
            .map_err(TerminalError::StorageError)?;
        self.user_interface.clear_todo_message()?;
        Ok(())
    }

    async fn remove_todo(&mut self, index_todo: usize) -> Result<(), TerminalError> {
        let mut todo_list = self
            .todo_storage
            .get_todos_from_filestorage()
            .await
            .map_err(TerminalError::StorageError)?;
        if index_todo > todo_list.len() {
            return Err(TerminalError::IndexError);
        }
        todo_list.remove(index_todo);
        self.todo_storage
            .write_filestorage(&todo_list)
            .await
            .map_err(TerminalError::StorageError)?;
        self.user_interface.remove_todo_message()?;
        Ok(())
    }

    async fn mark_todo_done(&mut self, index_todo: usize) -> Result<(), TerminalError> {
        let mut todo_list = self
            .todo_storage
            .get_todos_from_filestorage()
            .await
            .map_err(TerminalError::StorageError)?;
        if let Some(todo) = todo_list.get_mut(index_todo) {
            todo.done = true;
        } else {
            return Err(TerminalError::IndexError);
        }
        self.todo_storage
            .write_filestorage(&todo_list)
            .await
            .map_err(TerminalError::StorageError)?;
        self.user_interface.mark_done_message()?;
        self.show_list().await?;
        Ok(())
    }
}

#[cfg(test)]
mod mocks {
    use super::*;

    factori::factori!(Todos, {
        default {
            list = [
                Todo::new("first".to_string()),
                Todo::new("second".to_string()),
                Todo::new("third".to_string())
            ].to_vec()
        }
        mixin updated {
            list = [
                Todo::new("first".to_string()),
                Todo::new("second".to_string()),
                Todo::new("third".to_string()),
                Todo::new("fourth".to_string())
            ].to_vec()
        }
        mixin empty {
            list = [].to_vec()
        }
    });
}

#[cfg(test)]
mod tests {
    use super::{mocks::*, *};
    use crate::{repository::file_storage::MockStorage, terminal::MockUserInterface};

    #[tokio::test]
    async fn should_add_todo() {
        let mut mock_storage = MockStorage::new();
        let mut mock_user_interface = MockUserInterface::new();
        let todo_added = Todo::new("fourth".to_string());
        let original_todo_list = create!(Todos);
        let updated_todo_list = create!(Todos, :updated);

        mock_storage
            .expect_get_todos_from_filestorage()
            .return_once(|| Ok(original_todo_list));

        mock_storage
            .expect_write_filestorage()
            .return_once(|_| Ok(()));

        mock_user_interface
            .expect_show_todo_list()
            .withf(move |returned_list| *returned_list == updated_todo_list)
            .return_once(|_| Ok(()));

        let mut todo_cli_mock = TodoCli {
            user_interface: Box::new(mock_user_interface),
            todo_storage: Box::new(mock_storage),
        };

        todo_cli_mock
            .add_todo(todo_added)
            .await
            .expect("Should add a Todo successfully");
    }
}
