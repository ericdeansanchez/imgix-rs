//! Primary error structures for imgix.
use std::io;

/// Error types for imgix.
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

/// Custom result type for imgix.
pub type Result<T> = std::result::Result<T, Error>;
