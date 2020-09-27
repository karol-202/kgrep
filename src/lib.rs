use std::fs::File;
use std::io;
use std::io::Error as IoError;

use crate::error::{ArgsErrorType, Error};
use crate::util::LinesRead;

pub mod util;
pub mod error;

pub struct Args {
    search_pattern: String,
    input_source: InputSource,
}

impl Args {
    pub fn new(mut vec: Vec<String>) -> Result<Args, Error> {
        vec.remove(0);
        match vec.len() {
            0 => Err(Error::Args(ArgsErrorType::TooFew)),
            1 => Ok(Args {
                search_pattern: vec.remove(0),
                input_source: InputSource::Stdin,
            }),
            2 => Ok(Args {
                search_pattern: vec.remove(0),
                input_source: InputSource::File(vec.remove(0))
            }),
            _ => Err(Error::Args(ArgsErrorType::TooMany)),
        }
    }
}

pub enum InputSource {
    File(String),
    Stdin,
}

pub struct LinesIterator {
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

pub fn run(args: Args) -> Result<LinesIterator, Error> {
    Ok(read_source(&args.input_source)?
        .filter_lines(args.search_pattern)
    )
}

pub fn read_source(source: &InputSource) -> Result<LinesIterator, Error> {
    Ok(match source {
        InputSource::File(path) => LinesIterator::from_io(File::open(path)?.read_lines()),
        InputSource::Stdin => LinesIterator::from_io(io::stdin().read_lines()),
    })
}
