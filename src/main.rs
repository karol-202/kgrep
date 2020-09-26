use std::env;
use std::fs::File;
use std::io;

use crate::error::{Error, ArgsError};
use crate::util::{LinesRead};
use std::io::{Error as IoError};

mod util;
mod error;

struct Args {
    search_pattern: String,
    input_source: InputSource,
}

enum InputSource {
    File(String),
    Stdin,
}

struct LinesIterator {
    boxed_iterator: Box<dyn Iterator<Item = Result<String, Error>>>,
}

impl Iterator for LinesIterator {
    type Item = Result<String, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.boxed_iterator.next()
    }
}

impl LinesIterator {
    fn from(iterator: impl Iterator<Item = Result<String, Error>> + 'static) -> LinesIterator {
        LinesIterator { boxed_iterator: Box::new(iterator) }
    }

    fn from_io(iterator: impl Iterator<Item = Result<String, IoError>> + 'static) -> LinesIterator {
        LinesIterator::from(iterator.map(|line_result| Ok(line_result?)))
    }

    fn filter_lines(self, pattern: String) -> LinesIterator {
        LinesIterator::from(self.filter(move |line_result| match line_result {
            Result::Ok(line) => line.contains(&pattern),
            _ => true,
        }))
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
    read_source(&args.input_source)?
        .filter_lines(args.search_pattern)
        .try_for_each(|line_result| line_result.map(|line| println!("{}", line)))
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

fn read_source(source: &InputSource) -> Result<LinesIterator, Error> {
    match source {
        InputSource::File(path) => read_file(path),
        InputSource::Stdin => read_from_stdin(),
    }
}

fn read_file(path: &str) -> Result<LinesIterator, Error> {
    Ok(LinesIterator::from_io(File::open(path)?.read_lines()))
}

fn read_from_stdin() -> Result<LinesIterator, Error> {
    Ok(LinesIterator::from_io(io::stdin().read_lines()))
}
