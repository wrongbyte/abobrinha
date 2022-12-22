use std::{io::Error};

#[derive(Debug)]
pub enum TerminalError {
    Stdout(Error),
    Stdin(Error),
    ParseInt(String),
    IndexError
}

impl TerminalError {
    pub fn format_error(&self) -> String {
        match self {
            TerminalError::Stdin(error) => format!("Input error: {}", error),
            TerminalError::Stdout(error) => format!("Input error: {}", error),
            TerminalError::ParseInt(i) => format!("Parse error: \"{}\" is an invalid index!", i),
            TerminalError::IndexError => "Index error: Index out of bounds!".to_string()
        }
    }
}
