use std::{
    fmt::{self},
    io::Error,
};

#[derive(Debug)]
pub enum TerminalError {
    Stdout(Error),
    Stdin(Error),
    ParseInt(String),
    IndexError,
}

impl fmt::Display for TerminalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TerminalError::Stdin(error) => write!(f, "Input error: {}", error),
            TerminalError::Stdout(error) => write!(f, "Output error: {}", error),
            TerminalError::ParseInt(i) => write!(f, "Parse error: \"{}\" is an invalid index!", i),
            TerminalError::IndexError => write!(f, "Index error: Index out of bounds!"),
        }
    }
}
