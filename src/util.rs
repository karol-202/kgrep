use std::io::{BufRead, BufReader, Lines, Read};

pub trait LinesRead<R: Read> {
    fn read_lines(self) -> Lines<BufReader<R>>;
}

impl<R: Read> LinesRead<R> for R {
    fn read_lines(self) -> Lines<BufReader<R>> {
        BufReader::new(self).lines()
    }
}
