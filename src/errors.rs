pub enum Error {
    ScannerError(usize, String),
    ParseError(usize, String),
}

use Error::*;

impl Error {
    pub fn report(&self) {
        match &self {
            ScannerError(line, message) => {
                eprintln!("[line {}], ScannerError: {}", line, message);
            }
            ParseError(line, message) => {
                eprintln!("[line {}], ParseError: {}", line, message);
            }
        }
    }
}

pub type LoxResult<T> = Result<T, Error>;
