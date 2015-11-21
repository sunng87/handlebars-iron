use std::sync::RwLock;
use std::error::Error;

use iron::prelude::*;
use iron::{status};
use iron::{AfterMiddleware, typemap};
use iron::modifier::Modifier;
use plugin::Plugin as PluginFor;
use iron::headers::ContentType;

use handlebars::Handlebars;
use serialize::json::{ToJson, Json};

use ::source::{Source, SourceError};
use ::sources::directory::{DirectorySource};

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
    pub sources: Vec<Box<Source + Send + Sync>>,
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
    /// #[Deprecated], for backward compaitibility only
    pub fn new(prefix: &str, suffix: &str) -> HandlebarsEngine {
        let mut hbs = HandlebarsEngine::new2();
        hbs.add(Box::new(DirectorySource::new(prefix, suffix)));
        hbs.reload();
        hbs
    }

    pub fn from(prefix: &str, suffix: &str, custom: Handlebars) -> HandlebarsEngine {
        let mut hbs = HandlebarsEngine::of(custom);
        hbs.add(Box::new(DirectorySource::new(prefix, suffix)));
        hbs.reload();
        hbs
    }

    pub fn new2() -> HandlebarsEngine {
        HandlebarsEngine {
            sources: Vec::new(),
            registry: RwLock::new(Box::new(Handlebars::new()))
        }
    }

    pub fn of(reg: Handlebars) -> HandlebarsEngine {
        HandlebarsEngine {
            sources: Vec::new(),
            registry: RwLock::new(Box::new(reg))
        }
    }

    pub fn add(&mut self, source: Box<Source + Send + Sync>) {
        self.sources.push(source);
    }

    #[allow(unused_must_use)]
    pub fn reload(&self) -> Result<(), SourceError> {
        let mut hbs = self.registry.write().unwrap();
        hbs.clear_templates();
        for s in self.sources.iter() {
            try!(s.load(&mut hbs))
        }
        Ok(())
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
        let hbs = HandlebarsEngine::new2();
        let mut reg = hbs.registry.write().unwrap();
        reg.register_helper("ignore", Box::new(|_: &Context, _: &Helper, _: &Handlebars, _: &mut RenderContext| -> Result<(), RenderError> {
            Ok(())
        }));
    }
}
