use std::env;

use kgrep::Args;
use kgrep::error::Error;

mod error;

fn main() {
    if let Err(error) = execute() {
        println!("{}", error)
    }
}

fn execute() -> Result<(), Error> {
    let args = Args::new(env::args().collect())?;
    kgrep::run(args)?
        .try_for_each(|line_result| line_result.map(|line| println!("{}", line)))
}
