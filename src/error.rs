//! Errors

use std::io;
use std::error;
use std::fmt;
use std::result;

/// A shorthand for a rusted result.
pub type Result<T> = result::Result<T, Error>;

/// Any error producable by rusted.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    InvalidAddress,
    InvalidCommand,
    NoCommand,
    NoFilename,
    Io,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", error::Error::description(self))
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidAddress => "Invalid address",
            Error::InvalidCommand => "Invalid command",
            Error::NoCommand => "No command provided",
            Error::NoFilename => "No current filename",
            Error::Io => "I/O error",
        }
    }
}

impl From<io::Error> for Error {
    fn from(_err: io::Error) -> Error {
        Error::Io
    }
}
