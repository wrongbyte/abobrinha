use std::{fmt};

#[derive(Debug)]
pub enum StorageError {
    ReadError,
    WriteError,
    EmptyTodoError
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageError::EmptyTodoError => {
                write!(f, "Todo file storage is empty.")
            }
            StorageError::ReadError => {
                write!(f, "Error when reading file")
            }
            StorageError::WriteError => {
                write!(f, "Error when writing in file")
            }
        }
    }
}
