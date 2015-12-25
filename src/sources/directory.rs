use ::source::{Source, SourceError};

use std::io::prelude::*;
use std::fs::File;
use std::path::{Path, PathBuf};

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

fn read_file(path: &Path) -> Option<String> {
    if let Ok(mut file) = File::open(path) {
        let mut buf = String::new();
        if file.read_to_string(&mut buf).is_ok() {
            Some(buf)
        } else {
            info!("Failed to read file {}, skipped", path.display());
            None
        }
    } else {
        info!("Failed to open file {}, skipped.", path.display());
        None
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
            if let Some(tpl) = read_file(tpl_path) {
                if let Err(e) = reg.register_template_string(&tpl_name, tpl){
                    warn!("Failed to parse template {}, {}", tpl_name, e);
                } else {
                    info!("Added template {}", tpl_name);
                }
            }
        }
        Ok(())
    }
}
