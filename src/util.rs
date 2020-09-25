use std::io::{Result, Read};

pub trait StringRead {
    fn read_to_new_string(&mut self) -> Result<String>;
}

impl<T: Read> StringRead for T {
    fn read_to_new_string(&mut self) -> Result<String> {
        let mut string = String::new();
        self.read_to_string(&mut string).map(|_| string)
    }
}
