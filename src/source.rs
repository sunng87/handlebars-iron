use handlebars::{Handlebars, TemplateError, TemplateFileError};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct SourceError {
    pub cause: Box<Error + Send>,
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

impl From<TemplateError> for SourceError {
    fn from(err: TemplateError) -> SourceError {
        SourceError { cause: Box::new(err) }
    }
}

impl From<TemplateFileError> for SourceError {
    fn from(err: TemplateFileError) -> SourceError {
        SourceError { cause: Box::new(err) }
    }
}

pub trait Source {
    fn load(&self, reg: &mut Handlebars) -> Result<(), SourceError>;
}
