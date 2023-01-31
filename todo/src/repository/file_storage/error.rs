use std::fmt;

#[derive(Debug)]
pub enum StorageError {
    Read,
    Write,
    EmptyTodo,
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageError::EmptyTodo => {
                write!(f, "Todo file storage is empty.")
            }
            StorageError::Read => {
                write!(f, "Error when reading file")
            }
            StorageError::Write => {
                write!(f, "Error when writing in file")
            }
        }
    }
}
