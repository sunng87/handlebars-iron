use handlebars::Handlebars;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct SourceError {
    pub cause: Box<Error>
}

impl fmt::Display for SourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Display::fmt(&*self.cause, f)
    }
}

impl Error for SourceError {
    fn description(&self) -> &str {
        self.cause.description()
    }

    fn cause(&self) -> Option<&Error> {
        self.cause.cause()
    }
}

pub trait Source {
    fn load(&self, reg: &mut Handlebars) -> Result<(), SourceError>;
}
