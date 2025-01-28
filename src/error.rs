use std::error::Error;
use std::fmt::{Display, Formatter};

struct ParseError {
    pub message: String,
}

struct StorageError {
    pub message: String,
}

struct AuthorizationError {
    pub message: String,
}

#[derive(Debug)]
pub enum Errors {
    ParseError(String),
    StorageError(String),
    AuthorizationError(String),
}

impl Display for Errors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Errors::ParseError(message) => format!("parse error: {}", message),
                Errors::StorageError(message) => format!("storage error: {}", message),
                Errors::AuthorizationError(message) => format!("authorization error: {}", message),
            }
        )
    }
}

impl Error for Errors {}
