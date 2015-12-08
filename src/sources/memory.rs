use ::source::{Source, SourceError};
use std::collections::BTreeMap;
use handlebars::Handlebars;

pub struct MemorySource(BTreeMap<String, String>);

impl Source for MemorySource {
    fn load(&self, reg: &mut Handlebars) -> Result<(), SourceError> {
        for (name, tpl) in self.0.iter() {
            if let Err(e) = reg.register_template_string(name, tpl.clone()){
                warn!("Failed to parse template {}, {}", name, e);
            } else {
                info!("Added template {}", name);
            }
        }
        Ok(())
    }
}
