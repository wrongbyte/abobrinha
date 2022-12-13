use std::io::{Error};

#[derive(Debug)]
pub enum TerminalError {
    Stdout(Error),
    Stdin(Error),
}

impl TerminalError {
    pub fn format_error(&self) -> String {
        match self {
            TerminalError::Stdin(error) => format!("Input error: {}", error),
            TerminalError::Stdout(error) => format!("Input error: {}", error),
        }
    }
}