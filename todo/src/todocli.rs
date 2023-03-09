use crate::domain::todo::Todo;
use crate::repository::todo::Storage;
use crate::terminal::{error::TerminalError, UserInterface, UserOptions};
use uuid::Uuid;

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
        let todo_list = self.todo_storage.get_todo_list().await?;
        self.user_interface.show_todo_list(todo_list)?;
        Ok(())
    }

    async fn add_todo(&mut self, todo: Todo) -> Result<(), TerminalError> {
        self.todo_storage.add_todo(todo).await?;
        self.show_list().await?;
        Ok(())
    }

    async fn clear_todo_list(&mut self) -> Result<(), TerminalError> {
        self.todo_storage.clear_todo_list().await?;
        self.user_interface.clear_todo_message()?;
        Ok(())
    }

    async fn remove_todo(&mut self, uuid: Uuid) -> Result<(), TerminalError> {
        let todos_moodified = self.todo_storage.remove_todo(uuid).await?;
        match todos_moodified {
            0 => self.user_interface.report_not_found()?,
            _ => self.user_interface.remove_todo_message()?,
        }
        Ok(())
    }

    async fn mark_todo_done(&mut self, uuid: Uuid) -> Result<(), TerminalError> {
        let todos_modified = self.todo_storage.mark_todo_done(uuid).await?;
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
pub mod mocks {
    use crate::domain::todos::Todos;

    use super::*;

    pub fn builder(number_todos: usize, done_todo: Option<usize>) -> Todos {
        let list: Vec<Todo> = (0..number_todos)
            .map(|index| {
                let id = Uuid::new_v4();
                let message = format!("todo {}", index);
                let mut todo = Todo::new(message.to_string(), id);
                if let Some(done_index) = done_todo {
                    if index == done_index {
                        todo.done = true;
                    }
                }
                todo
            })
            .collect();
        Todos::new(list)
    }

    factori::factori!(Todos, {
        default {
            _list:Vec<Todo> = vec![],
            number_todos: usize = 0,
            done_todo: Option<usize> = None
        }
        builder {
            builder(number_todos, done_todo)
        }
    });
}

#[cfg(test)]
mod tests {
    use factori::create;

    use super::{mocks::*, *};
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

        let mut todo_cli_mock = TodoCli {
            user_interface: Box::new(mock_user_interface),
            todo_storage: Box::new(mock_storage),
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

        let mut todo_cli_mock = TodoCli {
            user_interface: Box::new(mock_user_interface),
            todo_storage: Box::new(mock_storage),
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

        let mut todo_cli_mock = TodoCli {
            user_interface: Box::new(mock_user_interface),
            todo_storage: Box::new(mock_storage),
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
            .expect_show_todo_list()
            .times(1)
            .returning(|_| Ok(()));

        mock_user_interface
            .expect_remove_todo_message()
            .times(1)
            .returning(|| Ok(()));

        let mut todo_cli_mock = TodoCli {
            user_interface: Box::new(mock_user_interface),
            todo_storage: Box::new(mock_storage),
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

        let mut todo_cli_mock = TodoCli {
            user_interface: Box::new(mock_user_interface),
            todo_storage: Box::new(mock_storage),
        };

        todo_cli_mock
            .mark_todo_done(todo_id)
            .await
            .expect("Should mark the last todo as done")
    }
}
