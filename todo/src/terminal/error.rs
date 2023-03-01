use crate::repository::todo::error::StorageError;
use std::{fmt, io::Error};
use uuid::Error as UUIDError;

#[derive(Debug)]
pub enum TerminalError {
    Stdout(Error),
    Stdin(Error),
    StorageError(StorageError),
    UUIDParse(UUIDError),
}

impl fmt::Display for TerminalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TerminalError::Stdin(error) => write!(f, "Input error: {}", error),
            TerminalError::Stdout(error) => write!(f, "Output error: {}", error),
            TerminalError::StorageError(error) => write!(f, "Error in storage: {}", error),
            TerminalError::UUIDParse(error) => write!(f, "Error when parsing the uuid: {}", error),
        }
    }
}

impl TerminalError {
    pub fn is_fatal(&self) -> bool {
        matches!(
            self,
            TerminalError::Stdin(_) | TerminalError::Stdout(_) | TerminalError::StorageError(_)
        )
    }
}
