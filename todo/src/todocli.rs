use uuid::Uuid;

use crate::domain::todo::Todo;
use crate::repository::todo::Storage;
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
            .get_todo_list()
            .await
            .map_err(TerminalError::StorageError)?;
        self.user_interface.show_todo_list(todo_list)?;
        Ok(())
    }

    async fn add_todo(&mut self, todo: Todo) -> Result<(), TerminalError> {
        self.todo_storage
            .add_todo(todo)
            .await
            .map_err(TerminalError::StorageError)?;
        self.show_list().await?;
        Ok(())
    }

    async fn clear_todo_list(&mut self) -> Result<(), TerminalError> {
        self.todo_storage
            .clear_todo_list()
            .await
            .map_err(TerminalError::StorageError)?;
        self.user_interface.clear_todo_message()?;
        Ok(())
    }

    async fn remove_todo(&mut self, uuid: Uuid) -> Result<(), TerminalError> {
        let todos_moodified = self
            .todo_storage
            .remove_todo(uuid)
            .await
            .map_err(TerminalError::StorageError)?;
        match todos_moodified {
            0 => self.user_interface.report_not_found()?,
            _ => self.user_interface.remove_todo_message()?,
        }
        Ok(())
    }

    async fn mark_todo_done(&mut self, uuid: Uuid) -> Result<(), TerminalError> {
        let todos_modified = self
            .todo_storage
            .mark_todo_done(uuid)
            .await
            .map_err(TerminalError::StorageError)?;
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
mod mocks {
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
