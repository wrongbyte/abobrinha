use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::todo::Todo;
use crate::repository::todo::Storage;
use crate::terminal::error::TerminalError;
use crate::terminal::{UserInterface, UserOptions};

pub struct TodoControllerImpl {
    pub todo_repository: Box<dyn Storage + Send + Sync>,
    pub user_interface: Box<dyn UserInterface + Send + Sync>,
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait TodoController {
    async fn show_list(&mut self) -> Result<(), TerminalError>;
    async fn add_todo(&mut self, todo: Todo) -> Result<(), TerminalError>;
    async fn clear_todo_list(&mut self) -> Result<(), TerminalError>;
    async fn remove_todo(&mut self, uuid: Uuid) -> Result<(), TerminalError>;
    async fn mark_todo_done(&mut self, uuid: Uuid) -> Result<(), TerminalError>;
    async fn get_user_intention(&mut self) -> Result<(), TerminalError>;
}

#[async_trait]
impl TodoController for TodoControllerImpl {
    async fn get_user_intention(&mut self) -> Result<(), TerminalError> {
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
        let todo_list = self.todo_repository.get_todo_list().await?;
        self.user_interface.show_todo_list(todo_list)?;
        Ok(())
    }

    async fn add_todo(&mut self, todo: Todo) -> Result<(), TerminalError> {
        self.todo_repository.add_todo(todo).await?;
        self.show_list().await?;
        Ok(())
    }

    async fn clear_todo_list(&mut self) -> Result<(), TerminalError> {
        self.todo_repository.clear_todo_list().await?;
        self.user_interface.clear_todo_message()?;
        Ok(())
    }

    async fn remove_todo(&mut self, uuid: Uuid) -> Result<(), TerminalError> {
        let todos_modified = self.todo_repository.remove_todo(uuid).await?;
        match todos_modified {
            0 => self.user_interface.report_not_found()?,
            _ => self.user_interface.remove_todo_message()?,
        }
        Ok(())
    }

    async fn mark_todo_done(&mut self, uuid: Uuid) -> Result<(), TerminalError> {
        let todos_modified = self.todo_repository.mark_todo_done(uuid).await?;
        match todos_modified {
            0 => self.user_interface.report_not_found()?,
            _ => {
                self.user_interface.mark_done_message()?;
                self.show_list().await?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use factori::create;
    use uuid::Uuid;
    use crate::domain::todo::{Todo, mocks::*};

    use super::*;
    use crate::{repository::todo::MockStorage, terminal::MockUserInterface};

    #[tokio::test]
    async fn should_add_todo() {
        let mut mock_storage = MockStorage::new();
        let mut mock_user_interface = MockUserInterface::new();
        let id = Uuid::new_v4();
        let todo_added = Todo::new("todo 3".to_string(), id);
        let original_todo_list = create!(Todos, number_todos: 3);
        let mut updated_todo_list = original_todo_list.clone();
        updated_todo_list.push(todo_added.clone());

        mock_storage
            .expect_add_todo()
            .times(1)
            .returning(|_| Ok(()));

        mock_storage.expect_get_todo_list().times(1).returning({
            let updated_list = updated_todo_list.clone();
            move || Ok(updated_list.clone())
        });

        mock_user_interface
            .expect_show_todo_list()
            .withf(move |returned_list| *returned_list == updated_todo_list)
            .times(1)
            .returning(|_| Ok(()));

        let mut todo_cli_mock = TodoControllerImpl {
            user_interface: Box::new(mock_user_interface),
            todo_repository: Box::new(mock_storage),
        };

        todo_cli_mock
            .add_todo(todo_added)
            .await
            .expect("Should add a Todo successfully");
    }

    #[tokio::test]
    async fn should_show_todo_list() {
        let mut mock_storage = MockStorage::new();
        let mut mock_user_interface = MockUserInterface::new();
        let todo_list = create!(Todos, number_todos: 3);

        mock_user_interface
            .expect_show_todo_list()
            .withf({
                let list = todo_list.clone();
                move |returned_list| *returned_list == list
            })
            .times(1)
            .returning(|_| Ok(()));

        mock_storage
            .expect_get_todo_list()
            .times(1)
            .returning(move || Ok(todo_list.clone()));

        let mut todo_cli_mock = TodoControllerImpl {
            user_interface: Box::new(mock_user_interface),
            todo_repository: Box::new(mock_storage),
        };

        todo_cli_mock
            .show_list()
            .await
            .expect("Should list all todos")
    }

    #[tokio::test]
    async fn should_clear_list() {
        let mut mock_storage = MockStorage::new();
        let mut mock_user_interface = MockUserInterface::new();

        mock_storage
            .expect_clear_todo_list()
            .times(1)
            .returning(|| Ok(()));

        mock_user_interface
            .expect_clear_todo_message()
            .times(1)
            .returning(|| Ok(()));

        let mut todo_cli_mock = TodoControllerImpl {
            user_interface: Box::new(mock_user_interface),
            todo_repository: Box::new(mock_storage),
        };

        todo_cli_mock
            .clear_todo_list()
            .await
            .expect("Should clear the list")
    }

    #[tokio::test]
    async fn should_remove_todo() {
        let mut mock_storage = MockStorage::new();
        let mut mock_user_interface = MockUserInterface::new();
        let mut original_todo_list = create!(Todos, number_todos: 4);
        let todo = original_todo_list.get_mut(3).unwrap();
        let todo_id = todo.id;

        mock_storage
            .expect_get_todo_list()
            .return_once(|| Ok(original_todo_list));

        mock_storage
            .expect_remove_todo()
            .times(1)
            .returning(|_| Ok(1));

        mock_user_interface
            .expect_remove_todo_message()
            .times(1)
            .returning(|| Ok(()));

        let mut todo_cli_mock = TodoControllerImpl {
            user_interface: Box::new(mock_user_interface),
            todo_repository: Box::new(mock_storage),
        };

        todo_cli_mock
            .remove_todo(todo_id)
            .await
            .expect("Should remove the fourth todo")
    }

    #[tokio::test]
    async fn should_mark_todo_as_done() {
        let mut mock_storage = MockStorage::new();
        let mut mock_user_interface = MockUserInterface::new();
        let mut todo_list = create!(Todos, number_todos: 4);
        let todo = todo_list.get_mut(3).unwrap();
        let todo_id = todo.id;
        todo.done = true;

        mock_storage
            .expect_mark_todo_done()
            .times(1)
            .returning(|_| Ok(1));

        mock_user_interface
            .expect_mark_done_message()
            .times(1)
            .returning(|| Ok(()));

        mock_user_interface
            .expect_show_todo_list()
            .times(1)
            .returning(|_| Ok(()));

        mock_storage
            .expect_get_todo_list()
            .times(1)
            .returning(move || Ok(todo_list.clone()));

        mock_user_interface
            .expect_mark_done_message()
            .times(1)
            .returning(|| Ok(()));

        let mut todo_cli_mock = TodoControllerImpl {
            user_interface: Box::new(mock_user_interface),
            todo_repository: Box::new(mock_storage),
        };

        todo_cli_mock
            .mark_todo_done(todo_id)
            .await
            .expect("Should mark the last todo as done")
    }
}
