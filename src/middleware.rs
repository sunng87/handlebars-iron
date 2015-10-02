use std::fs::File;
use std::env::current_dir;
use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::sync::RwLock;
use std::error::Error;

use iron::prelude::*;
use iron::{status};
use iron::{AfterMiddleware, typemap};
use iron::modifier::Modifier;
use plugin::Plugin as PluginFor;
use iron::headers::ContentType;
use walkdir::{WalkDir, DirEntry};

use handlebars::Handlebars;
use serialize::json::{ToJson, Json};

#[derive(Clone)]
pub struct Template {
    name: String,
    value: Json
}

impl Template {
    pub fn new<T: ToJson>(name: &str, value: T) -> Template {
        Template {
            name: name.to_string(),
            value: value.to_json()
        }
    }
}

pub struct HandlebarsEngine {
    pub prefix: String,
    pub suffix: String,
    pub registry: RwLock<Box<Handlebars>>
}

impl typemap::Key for HandlebarsEngine {
    type Value = Template;
}

impl Modifier<Response> for Template {
    fn modify(self, resp: &mut Response) {
        resp.extensions.insert::<HandlebarsEngine>(self);
    }
}

impl PluginFor<Response> for HandlebarsEngine {
    type Error = ();

    fn eval(resp: &mut Response) -> Result<Template, ()> {
        match resp.extensions.get::<HandlebarsEngine>(){
            Some(t) => Ok(t.clone()),
            None => Err(())
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

impl HandlebarsEngine {
    pub fn reload(&self) {
        let mut hbs = self.registry.write().unwrap();
        match current_dir()  {
            Ok(current_path) => {
                let mut prefix_path = PathBuf::from(current_path);
                prefix_path.push(self.prefix.clone());
                let template_path = prefix_path.as_path();

                info!("Loading templates from path {}", template_path.display());
                let walker = WalkDir::new(template_path);

                hbs.clear_templates();
                let suffix = &self.suffix;
                let prefix_len = template_path.as_os_str().to_str().unwrap().len();
                for p in walker.min_depth(1).into_iter().filter(|e| e.is_ok() && !filter_file(e.as_ref().unwrap(), suffix)).map(|e| e.unwrap()) {
                    let path = p.path();
                    let disp = path.to_str().unwrap();
                    debug!("getting file {}", disp);
                    let tpl_name = &disp[prefix_len .. disp.len()-suffix.len()];
                    if let Some(tpl) = read_file(&path) {
                        if let Err(e) = hbs.register_template_string(tpl_name, tpl){
                            warn!("Failed to parse template {}, {}", tpl_name, e);
                        } else {
                            info!("Added template {}", tpl_name);
                        }
                    }
                }
            },
            Err(e) => {
                error!("Failed to get current directory due to {}", e);
            }
        }
    }

    pub fn from(prefix: &str, suffix: &str, custom: Handlebars) -> HandlebarsEngine {
        let eng = HandlebarsEngine {
            prefix: prefix.to_string(),
            suffix: suffix.to_string(),
            registry: RwLock::new(Box::new(custom))
        };
        eng.reload();
        eng
    }

    pub fn new(prefix: &str, suffix: &str) -> HandlebarsEngine {
        HandlebarsEngine::from(prefix, suffix, Handlebars::new())
    }
}

impl AfterMiddleware for HandlebarsEngine {
    fn after(&self, _: &mut Request, r: Response) -> IronResult<Response> {
        let mut resp = r;
        let page_wrapper = resp.extensions.get::<HandlebarsEngine>().as_ref()
            .and_then(|h| {
                let hbs = self.registry.read().unwrap();
                Some(hbs.render(&h.name, &h.value))
            });

        match page_wrapper {
            Some(page_result) => {
                match page_result {
                    Ok(page) => {
                        if !resp.headers.has::<ContentType>() {
                            resp.headers.set(ContentType::html());
                        }
                        resp.set_mut(page);
                        Ok(resp)
                    }
                    Err(e) => {
                        info!("{}", e.description());
                        Err(IronError::new(e, status::InternalServerError))
                    }
                }
            }
            None => {
                Ok(resp)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use serialize::json::ToJson;
    use std::collections::BTreeMap;
    use iron::prelude::*;
    use middleware::*;
    use handlebars::{Handlebars, RenderError, RenderContext, Helper, Context};

    fn hello_world() -> IronResult<Response> {
        let resp = Response::new();

        let mut data = BTreeMap::new();
        data.insert("title".to_string(), "Handlebars on Iron".to_json());

        Ok(resp.set(Template::new("index", data)))
    }

    #[test]
    fn test_resp_set() {
        let mut resp = hello_world().ok().expect("response expected");

        // use response plugin to retrieve a cloned template for testing
        match resp.get::<HandlebarsEngine>() {
            Ok(h) => {
                assert_eq!(h.name, "index".to_string());
                assert_eq!(h.value.as_object().unwrap()
                           .get(&"title".to_string()).unwrap()
                           .as_string().unwrap(),
                           "Handlebars on Iron");
            },
            _ => panic!("template expected")
        }
    }

    #[test]
    fn test_register_helper() {
        let hbs = HandlebarsEngine::new("./examples/templates", ".hbs");
        let mut reg = hbs.registry.write().unwrap();
        reg.register_helper("ignore", Box::new(|_: &Context, _: &Helper, _: &Handlebars, _: &mut RenderContext| -> Result<(), RenderError> {
            Ok(())
        }));
    }
}
