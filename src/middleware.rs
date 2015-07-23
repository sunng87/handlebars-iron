use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::sync::RwLock;

use iron::prelude::*;
use iron::{AfterMiddleware, typemap};
use iron::modifier::Modifier;
use plugin::Plugin as PluginFor;
use iron::headers::ContentType;
use walker::Walker;

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

fn is_temp_file(tpl_name: &str) -> bool {
    tpl_name.starts_with(".") || tpl_name.starts_with("#")
}

fn read_file(path: &Path) -> Option<String> {
    if let Ok(mut file) = File::open(path) {
        let mut buf = String::new();
        if file.read_to_string(&mut buf).is_ok() {
            Some(buf)
        } else {
            println!("Failed to read file {}, skipped", path.display());
            None
        }
    } else {
        println!("Failed to open file {}, skipped.", path.display());
        None
    }
}

impl HandlebarsEngine {
    pub fn reload(&self) {
        let mut hbs = self.registry.write().unwrap();

        let mut prefix = self.prefix.clone();
        if !prefix.ends_with('/') {
            prefix.push('/');
        }
        let normalized_prefix = prefix;

        let prefix_path = Path::new(&normalized_prefix);
        let walker = Walker::new(prefix_path).ok().expect(
            &format!("Failed to list directory: {}", normalized_prefix));

        hbs.clear_templates();
        let suffix = &self.suffix;
        for p in walker.filter_map(Result::ok) {
            let path = p.path();
            let disp = path.to_str().unwrap();
            if disp.ends_with(suffix) {
                let tpl_name = &disp[normalized_prefix.len() .. disp.len()-suffix.len()];
                if !is_temp_file(tpl_name) {
                    if let Some(tpl) = read_file(&path) {
                        if let Err(e) = hbs.register_template_string(tpl_name, tpl){
                            println!("Failed to parse template {}, {}", tpl_name, e);
                        }
                    }
                }
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
        HandlebarsEngine::from(prefix,suffix, Handlebars::new())
    }
}

impl AfterMiddleware for HandlebarsEngine {
    fn after(&self, _: &mut Request, r: Response) -> IronResult<Response> {
        let mut resp = r;
        // internally we still extensions.get to avoid clone
        let page = resp.extensions.get::<HandlebarsEngine>().as_ref()
                    .and_then(|h| {
                        let hbs = self.registry.read().unwrap();
                        hbs.render(&h.name, &h.value).ok()
                    });

        if let Some(page) = page {
            if !resp.headers.has::<ContentType>() {
                resp.headers.set(ContentType::html());
            }
            resp.set_mut(page);
        }

        Ok(resp)
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
