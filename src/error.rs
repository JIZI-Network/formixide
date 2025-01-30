use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
pub struct ParseError {
    pub message: String,
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error: {}", self.message)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error: {}", self.message)
    }
}

impl Error for ParseError {}

pub struct StorageError {
    pub message: String,
}

impl Debug for StorageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "storage error: {}", self.message)
    }
}

impl Display for StorageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "storage error: {}", self.message)
    }
}

impl Error for StorageError {}

pub struct AuthorizationError {
    pub message: String,
}

impl Debug for AuthorizationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "authorization error: {}", self.message)
    }
}

impl Display for AuthorizationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "authorization error: {}", self.message)
    }
}

impl Error for AuthorizationError {}
