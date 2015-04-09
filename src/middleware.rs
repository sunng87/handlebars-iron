use std::str::FromStr;
use std::fs::{File, walk_dir};
use std::path::Path;
use std::io::prelude::*;
use std::result::Result;
use std::env;

use iron::prelude::*;
use iron::{AfterMiddleware, typemap};
use iron::modifier::Modifier;
use plugin::Plugin as PluginFor;
use iron::headers;

use hyper::header::ContentType;

use handlebars::Handlebars;
use serialize::json::{ToJson, Json};

pub struct HandlebarsEngine {
    registry: Box<Handlebars>
}

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
    pub fn new(prefix: &str, suffix: &str) -> HandlebarsEngine {
        let mut r = Handlebars::new();

        let mut prefix_slash = prefix.to_string();
        let normalized_prefix = if prefix.ends_with("/") {
            prefix_slash
        } else {
            prefix_slash.push('/');
            prefix_slash
        };
        let prefix_path = Path::new(&normalized_prefix);
        if ! prefix_path.exists() {
            let abs_prefix_path = if prefix_path.is_relative() {
                 let mut p = env::current_dir().ok().unwrap();
                 p.push(prefix_path);
                 p.iter().collect() // normalization
            } else { prefix_path.to_path_buf() };
            panic!("Prefix path '{}' doesn't exist.", abs_prefix_path.display());
        }

        let walker = walk_dir(prefix_path);
        if !walker.is_ok() {
            panic!("Failed to list directory.");
        }
        for p in walker.ok().unwrap().filter_map(Result::ok) {
            let path = p.path();
            let disp = path.to_str().unwrap();
            if disp.ends_with(suffix) {
                let mut file = File::open(&path).ok()
                    .expect(format!("Failed to open file {}", disp).as_ref());
                let mut buf = String::new();
                file.read_to_string(&mut buf).ok()
                    .expect(format!("Failed to read file {}", disp).as_ref());

                let t = r.register_template_string(
                    &disp[normalized_prefix.len() .. disp.len()-suffix.len()], buf);
                if t.is_err() {
                    panic!("Failed to create template.");
                }
            }
        }

        HandlebarsEngine {
            registry: Box::new(r)
        }
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
                let rendered = self.registry.render(name.as_ref(), value);
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
                resp.headers.set(headers::ContentType(FromStr::from_str("text/html;charset=utf-8").unwrap()));
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
}
