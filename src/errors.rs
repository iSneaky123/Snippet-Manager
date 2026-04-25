//! Error types for the snip application

use std::fmt;

/// Represents all possible errors that can occur in snip.
#[derive(Debug)]
pub enum SnipError {
    /// User provided invalid input (e.g., invalid number, out of range)
    InvalidInput(String),

    /// A snippet with the given ID was not found
    SnippetNotFound(String),

    /// A shell is not supported or not installed
    ShellNotSupported(String),

    /// No shell was specified for a snippet that needs one
    ShellNotSpecified,

    /// Filesystem I/O error
    Io(std::io::Error),

    /// JSON serialization/deserialization error
    Serialization(serde_json::Error),

    /// Generic error with context
    Other(String),
}

impl fmt::Display for SnipError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SnipError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            SnipError::SnippetNotFound(id) => write!(f, "Snippet not found: {}", id),
            SnipError::ShellNotSupported(shell) => {
                write!(f, "'{}' is not a valid or supported shell", shell)
            }
            SnipError::ShellNotSpecified => {
                write!(f, "No shell specified for this snippet")
            }
            SnipError::Io(e) => write!(f, "IO error: {}", e),
            SnipError::Serialization(e) => write!(f, "Serialization error: {}", e),
            SnipError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for SnipError {}

impl From<std::io::Error> for SnipError {
    fn from(err: std::io::Error) -> Self {
        SnipError::Io(err)
    }
}

impl From<serde_json::Error> for SnipError {
    fn from(err: serde_json::Error) -> Self {
        SnipError::Serialization(err)
    }
}

impl From<String> for SnipError {
    fn from(s: String) -> Self {
        SnipError::Other(s)
    }
}

impl From<&str> for SnipError {
    fn from(s: &str) -> Self {
        SnipError::Other(s.to_string())
    }
}

pub type Result<T> = std::result::Result<T, SnipError>;
