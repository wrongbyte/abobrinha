use std::{
    fmt,
    io::Error,
};
use crate::filestorage::error::StorageError;

#[derive(Debug)]
pub enum TerminalError {
    Stdout(Error),
    Stdin(Error),
    ParseInt(String),
    IndexError,
    StorageError(StorageError)
}

impl fmt::Display for TerminalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TerminalError::Stdin(error) => write!(f, "Input error: {}", error),
            TerminalError::Stdout(error) => write!(f, "Output error: {}", error),
            TerminalError::ParseInt(i) => write!(f, "Parse error: \"{}\" is an invalid index!", i),
            TerminalError::IndexError => write!(f, "Index error: Index out of bounds!"),
            TerminalError::StorageError(error) => write!(f, "Error in storage: {}", error),
        }
    }
}


impl TerminalError {
    pub fn is_fatal(self) -> bool {
        match self {
            TerminalError::Stdin(_) => true,
            TerminalError::Stdout(_) => true,
            TerminalError::StorageError(_) => true,
            _ => false
        }
    }
}