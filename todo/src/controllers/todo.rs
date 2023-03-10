use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::todo::Todo;
use crate::repository::todo::Storage;
use crate::terminal::error::TerminalError;
use crate::terminal::UserInterface;

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
}

#[async_trait]
impl TodoController for TodoControllerImpl {
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
        let todos_moodified = self.todo_repository.remove_todo(uuid).await?;
        match todos_moodified {
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
