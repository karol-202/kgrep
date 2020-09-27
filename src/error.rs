use std::error::Error as StdError;
use std::io::Error as IoError;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum Error {
    Io(IoError),
    Args(ArgsErrorType),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Error::Io(io_error) => write!(f, "IO error: {}", io_error),
            Error::Args(args_error) => write!(f, "Invalid arguments: {}", args_error),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Io(io_error) => Some(io_error),
            Error::Args(_) => None,
        }
    }
}

impl From<IoError> for Error {
    fn from(io_error: IoError) -> Self {
        Error::Io(io_error)
    }
}

#[derive(Debug)]
pub enum ArgsErrorType {
    TooFew,
    TooMany,
}

impl Display for ArgsErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ArgsErrorType::TooFew => write!(f, "Too few arguments"),
            ArgsErrorType::TooMany => write!(f, "Too many arguments"),
        }
    }
}
