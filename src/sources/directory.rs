use std::path::PathBuf;

use source::{Source, SourceError};

use handlebars::Handlebars;

pub struct DirectorySource {
    pub prefix: PathBuf,
    pub suffix: &'static str,
}

impl DirectorySource {
    pub fn new<P>(prefix: P, suffix: &'static str) -> DirectorySource
        where P: Into<PathBuf>
    {
        DirectorySource {
            prefix: prefix.into(),
            suffix: suffix,
        }
    }
}

// /// return false when file name not satisfied
// fn filter_file(entry: &DirEntry, suffix: &OsStr) -> bool {
//     let path = entry.path();

//     // ignore hidden files, emacs buffers and files with wrong suffix
//     !path.is_file() ||
//     path.file_name()
//         .map(|s| {
//                  let ds = s.to_string_lossy();
//                  ds.starts_with(".") || ds.starts_with("#") ||
//                  !ds.ends_with(suffix.to_string_lossy().as_ref())
//              })
//         .unwrap_or(true)
// }

impl Source for DirectorySource {
    fn load(&self, reg: &mut Handlebars) -> Result<(), SourceError> {
        reg.register_templates_directory(self.suffix, &self.prefix)
            .map_err(SourceError::from)
    }
}
