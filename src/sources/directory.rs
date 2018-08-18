use std::path::PathBuf;

use source::{Source, SourceError};

use handlebars::Handlebars;

pub struct DirectorySource {
    pub prefix: PathBuf,
    pub suffix: &'static str,
}

impl DirectorySource {
    pub fn new<P>(prefix: P, suffix: &'static str) -> DirectorySource
    where
        P: Into<PathBuf>,
    {
        DirectorySource {
            prefix: prefix.into(),
            suffix: suffix,
        }
    }
}

impl Source for DirectorySource {
    fn load(&self, reg: &mut Handlebars) -> Result<(), SourceError> {
        reg.register_templates_directory(self.suffix, &self.prefix)
            .map_err(SourceError::from)
    }
}
