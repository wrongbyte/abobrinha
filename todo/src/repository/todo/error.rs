use std::fmt;
use tokio_postgres::Error;

#[derive(Debug)]
pub struct StorageError {
    pub error: Error
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error in storage: {}", self.error)
    }
}
