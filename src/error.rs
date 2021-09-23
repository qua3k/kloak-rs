use std::{fmt, io, result};

/// A specialized [`Result`] type for `error::Error`
pub type Result<T> = result::Result<T, Error>;

/// The error type for kloak-rs operations
/// Implements from io::Error and a custom value for supported_keys errors
#[derive(Debug)]
pub enum Error {
    Io (io::Error),
    NoSupportedKeysError
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "{}", err),
            Error::NoSupportedKeysError => write!(f, "no supported keys")
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}