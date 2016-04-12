use ::source::{Source, SourceError};

use handlebars::Handlebars;
use walkdir::{WalkDir, DirEntry};

pub struct DirectorySource {
    pub prefix: String,
    pub suffix: String
}

impl DirectorySource {
    pub fn new(prefix: &str, suffix: &str) -> DirectorySource {
        let mut prefix_owned = prefix.to_owned();
        // append tailing slash
        if ! prefix_owned.ends_with("/") {
            prefix_owned.push_str("/");
        }
        DirectorySource {
            prefix: prefix_owned,
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
        let prefix_len = self.prefix.len();

        info!("Loading templates from path {}", self.prefix);
        let walker = WalkDir::new(&self.prefix);
        for p in walker.min_depth(1).into_iter().filter(|e| e.is_ok() && !filter_file(e.as_ref().unwrap(), &self.suffix)).map(|e| e.unwrap()) {
            let tpl_path = p.path();
            let tpl_file_path = p.path().to_string_lossy();
            let tpl_name = &tpl_file_path[prefix_len .. tpl_file_path.len() - suffix_len];
            let tpl_canonical_name = tpl_name.replace("\\", "/");
            debug!("getting file {}", tpl_file_path);
            debug!("register template {}", tpl_name);
            try!(reg.register_template_file(&tpl_canonical_name, &tpl_path))
        }
        Ok(())
    }
}
