//! Errors

use std::error;
use std::fmt;
use std::result;

/// A shorthand for a rusted result.
pub type Result<T> = result::Result<T, Error>;

/// Any error producable by rusted.
#[derive(Copy, Debug, Eq, PartialEq)]
pub enum Error {
    InvalidAddress,
    InvalidCommand,
    NoCommand,
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
        }
    }
}
