use std::{io::Error};

#[derive(Debug)]
pub enum TerminalError {
    Stdout(Error),
    Stdin(Error),
    ParseInt(String),
}

impl TerminalError {
    pub fn format_error(&self) -> String {
        match self {
            TerminalError::Stdin(error) => format!("Input error: {}", error),
            TerminalError::Stdout(error) => format!("Input error: {}", error),
            TerminalError::ParseInt(i) => format!("Parse Error: \"{}\" is an invalid index!", i),
        }
    }
}
