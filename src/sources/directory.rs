use ::source::{Source, SourceError};

use std::io::prelude::*;
use std::fs::File;
use std::env::current_dir;
use std::path::{Path, PathBuf};

use handlebars::Handlebars;
use walkdir::{WalkDir, DirEntry};

pub struct DirectorySource {
    pub prefix: String,
    pub suffix: String
}

impl DirectorySource {
    pub fn new(prefix: &str, suffix: &str) -> DirectorySource {
        DirectorySource {
            prefix: prefix.to_owned(),
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
        .and_then(|s| Some(s.starts_with(".") || s.starts_with("#") || !s.ends_with(suffix)))
        .unwrap_or(false)
}

impl Source for DirectorySource {
    fn load(&self, reg: &mut Handlebars) -> Result<(), SourceError> {
        match current_dir() {
            Ok(current_path) => {
                let mut prefix_path = PathBuf::from(current_path);
                prefix_path.push(self.prefix.clone());
                let template_path = prefix_path.as_path();

                info!("Loading templates from path {}", template_path.display());
                let walker = WalkDir::new(template_path);

                let suffix = &self.suffix;
                let prefix_len = template_path.as_os_str().to_str().unwrap().len();
                for p in walker.min_depth(1).into_iter().filter(|e| e.is_ok() && !filter_file(e.as_ref().unwrap(), suffix)).map(|e| e.unwrap()) {
                    let path = p.path();
                    let disp = path.to_str().unwrap();
                    debug!("getting file {}", disp);
                    let tpl_name = &disp[prefix_len .. disp.len()-suffix.len()];
                    if let Some(tpl) = read_file(&path) {
                        if let Err(e) = reg.register_template_string(tpl_name, tpl){
                            warn!("Failed to parse template {}, {}", tpl_name, e);
                        } else {
                            info!("Added template {}", tpl_name);
                        }
                    }
                }
                Ok(())
            },
            Err(e) => {
                Err(SourceError{ cause: Box::new(e) })
            }
        }
    }
}
