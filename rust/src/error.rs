//! Error type.

use std::error::Error as StdError;
use std::fmt;

/// Error type.
#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    /// Type of error.
    kind: ErrorKind,
}

impl Error {
    /// Create new error.
    pub const fn new(kind: ErrorKind) -> Self {
        Self { kind }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&self.kind)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}
/// An error can occur during some data structures operations.
/// `ErrorKind` enumerates the various type of error.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// This error type occurs when deleting an item from an empty queue.
    QueueUnderflow,
    /// This error type occurs when adding an item to a full queue.
    QueueOverflow,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::QueueOverflow => "failed to enqueue a new element to already full queue.",
            Self::QueueUnderflow => "cannot dequeue element from an empty queue",
        };
        write!(f, "{}", s)
    }
}

impl StdError for ErrorKind {}
