//! Primary error structures for imgix.
use std::fmt;
use std::io;

/// Error types for imgix.
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    DomainError(String),
    JoinError(String),
    ParamError(String),
    PathError(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: there may be a macro opportunity here...
        match self {
            Error::Io(e) => write!(f, "{error}: {msg}", error = stringify!(Error::Io), msg = e),
            Error::DomainError(msg) => {
                write!(f, "{error}: {msg}", error = "DomainError", msg = msg)
            }
            Error::JoinError(msg) => write!(f, "{error}: {msg}", error = "JoinError", msg = msg),
            Error::PathError(msg) => write!(f, "{error}: {msg}", error = "PathError", msg = msg),
            Error::ParamError(msg) => write!(f, "{error}: {msg}", error = "ParamError", msg = msg),
        }
    }
}

/// Custom result type for imgix.
pub type Result<T> = std::result::Result<T, Error>;
