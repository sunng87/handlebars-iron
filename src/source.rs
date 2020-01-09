use handlebars::{Handlebars, TemplateError, TemplateFileError};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct SourceError {
    pub cause: Box<dyn Error + Send>,
}

impl fmt::Display for SourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Display::fmt(&*self.cause, f)
    }
}

impl Error for SourceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause.source()
    }
}

impl From<TemplateError> for SourceError {
    fn from(err: TemplateError) -> SourceError {
        SourceError {
            cause: Box::new(err),
        }
    }
}

impl From<TemplateFileError> for SourceError {
    fn from(err: TemplateFileError) -> SourceError {
        SourceError {
            cause: Box::new(err),
        }
    }
}

pub trait Source {
    fn load(&self, reg: &mut Handlebars) -> Result<(), SourceError>;
}
