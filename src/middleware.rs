use std::str::FromStr;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::result::Result;
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

impl HandlebarsEngine {
    pub fn reload(&self) {
        let mut prefix_slash = self.prefix.clone();
        let suffix: &str = self.suffix.as_ref();
        let mut hbs = self.registry.write().unwrap();

        let normalized_prefix = if self.prefix.ends_with("/") {
            prefix_slash
        } else {
            prefix_slash.push('/');
            prefix_slash
        };
        let prefix_path = Path::new(&normalized_prefix);
        let walker = Walker::new(prefix_path);
        if !walker.is_ok() {
            panic!(format!("Failed to list directory: {}", normalized_prefix));
        }

        hbs.clear_templates();
        for p in walker.ok().unwrap().filter_map(Result::ok) {
            let path = p.path();
            let disp = path.to_str().unwrap();
            if disp.ends_with(suffix) {
                if let Ok(mut file) = File::open(&path) {
                    let mut buf = String::new();
                    if let Ok(_) = file.read_to_string(&mut buf) {
                        if let Err(e) = hbs.register_template_string(
                            &disp[normalized_prefix.len() .. disp.len()-suffix.len()], buf){
                            println!("Failed to parse template {}", e);
                        }
                    } else {
                        println!("Failed to read file {}, skipped", disp);
                    }
                } else {
                    println!("Failed to open file {}, skipped.", disp);
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
        let page = match resp.extensions.get::<HandlebarsEngine>() {
            Some(ref h) => {
                let name = &h.name;
                let value = &h.value;
                let hbs = self.registry.read().unwrap();
                let rendered = hbs.render(name.as_ref(), value);
                match rendered {
                    Ok(r) => Some(r),
                    Err(_) => None
                }
            },
            None => {
                None
            }
        };

        if page.is_some() {
            if !resp.headers.has::<ContentType>() {
                resp.headers.set(ContentType(FromStr::from_str("text/html;charset=utf-8").unwrap()));
            }
            resp.set_mut(page.unwrap());
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
        reg.register_helper("ignore", Box::new(|_: &Context, _: &Helper, _: &Handlebars, _: &mut RenderContext| -> Result<String, RenderError> {
            Ok("".to_string())
        }));
    }
}
