use std::io::Error as IoError;

pub enum Error {
    Io(std::io::Error),
    Args(ArgsError),
}

pub enum ArgsError {
    TooFew,
    TooMany,
}

impl From<IoError> for Error {
    fn from(io_error: IoError) -> Self {
        Error::Io(io_error)
    }
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::Io(io_error) => format!("IO error: {}", io_error),
            Error::Args(args_error) => format!("Invalid arguments: {}", args_error.to_string()),
        }
    }
}

impl ToString for ArgsError {
    fn to_string(&self) -> String {
        match self {
            ArgsError::TooFew => "Too few arguments",
            ArgsError::TooMany => "Too many arguments",
        }.to_string()
    }
}
