use crate::contentstack::ContentstackError;
use std::io;

#[derive(Debug)]
pub enum Error {
    Contentstack(Box<ContentstackError>),
    Io(io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Contentstack(e) => write!(f, "Contentstack error: {e}"),
            Error::Io(e) => write!(f, "IO error: {e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Contentstack(e) => Some(e.as_ref()),
            Error::Io(e) => Some(e),
        }
    }
}

impl From<ContentstackError> for Error {
    fn from(error: ContentstackError) -> Self {
        Error::Contentstack(Box::new(error))
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
