use std::{io::Error, num::ParseIntError};

#[derive(Debug)]
pub enum TerminalError {
    Stdout(Error),
    Stdin(Error),
    ParseInt(ParseIntError),
}

impl TerminalError {
    pub fn format_error(&self) -> String {
        match self {
            TerminalError::Stdin(error) => format!("Input error: {}", error),
            TerminalError::Stdout(error) => format!("Input error: {}", error),
            TerminalError::ParseInt(error) => format!("ParseInt error: {}", error),
        }
    }
}
