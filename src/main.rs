use std::env;
use std::fs::File;
use std::io;
use std::io::Error as IoError;

use crate::util::StringRead;

mod util;

struct Args {
    search_pattern: String,
    input_source: InputSource,
}

enum InputSource {
    File(String),
    Stdin,
}

enum Error {
    Io(std::io::Error),
    Args(ArgsError),
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

enum ArgsError {
    TooFew,
    TooMany,
}

impl ToString for ArgsError {
    fn to_string(&self) -> String {
        match self {
            ArgsError::TooFew => "Too few arguments",
            ArgsError::TooMany => "Too many arguments",
        }.to_string()
    }
}

fn main() {
    match execute() {
        Err(error) => println!("{}", error.to_string()),
        _ => (),
    }
}

fn execute() -> Result<(), Error> {
    let args = read_args()?;
    let data = read_source(&args.input_source)?;
    let found_lines = process_data(&data, &args.search_pattern);
    found_lines.iter().for_each(|line| println!("{}", line));
    Ok(())
}

fn read_args() -> Result<Args, Error> {
    parse_args(env::args().collect())
}

fn parse_args(mut vec: Vec<String>) -> Result<Args, Error> {
    vec.remove(0);
    match vec.len() {
        0 => Err(Error::Args(ArgsError::TooFew)),
        1 => Ok(Args {
            search_pattern: vec.remove(0),
            input_source: InputSource::Stdin,
        }),
        2 => Ok(Args {
            search_pattern: vec.remove(0),
            input_source: InputSource::File(vec.remove(0))
        }),
        _ => Err(Error::Args(ArgsError::TooMany)),
    }
}

fn read_source(source: &InputSource) -> Result<String, Error> {
    match source {
        InputSource::File(path) => read_file(path),
        InputSource::Stdin => read_from_stdin(),
    }
}

fn read_file(path: &str) -> Result<String, Error> {
    Ok(File::open(path)?.read_to_new_string()?)
}

fn read_from_stdin() -> Result<String, Error> {
    Ok(io::stdin().read_to_new_string()?)
}

fn process_data<'a>(data: &'a str, pattern: &str) -> Vec<&'a str> {
    data.split("\n").filter(|line| line.contains(pattern)).collect()
}
