use std::path::PathBuf;
use std::ffi::{OsString, OsStr};

use source::{Source, SourceError};

use handlebars::Handlebars;
use walkdir::{WalkDir, DirEntry};

pub struct DirectorySource {
    pub prefix: OsString,
    pub suffix: OsString,
}

impl DirectorySource {
    pub fn new<P>(prefix: P, suffix: P) -> DirectorySource
        where P: Into<PathBuf>
    {
        DirectorySource {
            prefix: prefix.into().into_os_string(),
            suffix: suffix.into().into_os_string(),
        }
    }
}

fn filter_file(entry: &DirEntry, suffix: &OsStr) -> bool {
    let path = entry.path();

    path.starts_with(".") || path.starts_with("#") || !path.ends_with(suffix)
}

impl Source for DirectorySource {
    fn load(&self, reg: &mut Handlebars) -> Result<(), SourceError> {
        let suffix_len = self.suffix.len();
        let prefix_len = self.prefix.len();

        info!("Loading templates from path {:?}", self.prefix);
        let walker = WalkDir::new(&self.prefix);
        for p in walker
                .min_depth(1)
                .into_iter()
                .filter(|e| e.is_ok() && !filter_file(e.as_ref().unwrap(), &self.suffix))
                .map(|e| e.unwrap()) {
            let tpl_path = p.path();
            let tpl_file_path = p.path().to_string_lossy();
            let tpl_name = &tpl_file_path[prefix_len..tpl_file_path.len() - suffix_len];
            let tpl_canonical_name = tpl_name.replace("\\", "/");
            debug!("getting file {:?}", tpl_file_path);
            debug!("register template {:?}", tpl_name);
            try!(reg.register_template_file(&tpl_canonical_name, &tpl_path))
        }
        Ok(())
    }
}
