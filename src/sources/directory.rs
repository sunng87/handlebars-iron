use ::source::{Source, SourceError};

use std::path::PathBuf;

use handlebars::Handlebars;
use walkdir::{WalkDir, DirEntry};

pub struct DirectorySource {
    pub prefix: PathBuf,
    pub suffix: String
}

impl DirectorySource {
    pub fn new(prefix: &str, suffix: &str) -> DirectorySource {
        DirectorySource {
            prefix: PathBuf::from(prefix),
            suffix: suffix.to_owned()
        }
    }
}

fn filter_file(entry: &DirEntry, suffix: &str) -> bool {
    entry.file_name().to_str()
        .map(|s| s.starts_with(".") || s.starts_with("#") || !s.ends_with(suffix))
        .unwrap_or(false)
}

impl Source for DirectorySource {
    fn load(&self, reg: &mut Handlebars) -> Result<(), SourceError> {
        let suffix_len = self.suffix.len();

        info!("Loading templates from path {}", self.prefix.display());
        let walker = WalkDir::new(&self.prefix);
        for p in walker.min_depth(1).into_iter().filter(|e| e.is_ok() && !filter_file(e.as_ref().unwrap(), &self.suffix)).map(|e| e.unwrap()) {
            let tpl_file = p.file_name().to_string_lossy();
            let tpl_name = &tpl_file[0 .. tpl_file.len() - suffix_len];
            debug!("getting file {}", tpl_file);
            let tpl_path = p.path();
            try!(reg.register_template_file(&tpl_name, &tpl_path))
        }
        Ok(())
    }
}
