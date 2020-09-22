use std::env;
use std::error::Error;

struct Args {
    search_pattern: String,
    input_source: InputSource,
}

enum InputSource {
    File(String),
    Stdin,
}

fn main() {
    let args = read_args();
}

fn read_args() -> Result<Args, &str> {
    parse_args(env::args().collect())
}

fn parse_args(mut vec: Vec<String>) -> Result<Args, &str> {
    match vec.len() {
        0 => Err("Too few arguments"),
        1 => Args {
            search_pattern: vec.remove(0),
            input_source: InputSource::Stdin,
        },
        2 => Args {
            search_pattern: vec.remove(0),
            input_source: InputSource::File(vec.remove(0))
        },
        _ => Err("Too many arguments"),
    }
}
