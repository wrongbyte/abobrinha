use std::{fmt, string::FromUtf8Error};

#[derive(Debug)]
pub enum StorageError {
    InvalidBuffer(FromUtf8Error),
    ReadError,
    WriteError,
    EmptyTodoError,
    OpenError,
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageError::EmptyTodoError => {
                write!(f, "Todo file storage is empty.")
            }
            StorageError::InvalidBuffer(error) => {
                write!(f, "Error when reading file: {}", error)
            },
            StorageError::ReadError => {
                write!(f, "Error when reading file")
            },
            StorageError::WriteError => {
                write!(f, "Error when writing in file")
            },
            StorageError::OpenError => {
                write!(f, "Error when opening the file")
            }
        }
    }
}
