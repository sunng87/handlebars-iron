use std::sync::RwLock;
use std::error::Error;

use iron::prelude::*;
use iron::{status};
use iron::{AfterMiddleware, typemap};
use iron::modifier::Modifier;
use plugin::Plugin as PluginFor;
use iron::headers::ContentType;

use handlebars::{Handlebars, TemplateRenderError};
#[cfg(not(feature = "serde_type"))]
use serialize::json::{ToJson, Json};
#[cfg(feature = "serde_type")]
use serde::ser::Serialize as ToJson;
#[cfg(feature = "serde_type")]
use serde_json::value::{self, Value as Json};

use ::source::{Source, SourceError};

#[derive(Clone)]
pub struct Template {
    name: Option<String>,
    content: Option<String>,
    value: Json
}

#[cfg(not(feature = "serde_type"))]
impl Template {
    /// render some template from pre-registered templates
    pub fn new<T: ToJson>(name: &str, value: T) -> Template {
        Template {
            name: Some(name.to_string()),
            value: value.to_json(),
            content: None
        }
    }

    /// render some template with temporary template string
    pub fn with<T: ToJson>(content: &str, value: T) -> Template {
        Template {
            name: None,
            value: value.to_json(),
            content: Some(content.to_string())
        }
    }
}

#[cfg(feature = "serde_type")]
impl Template {
    /// render some template from pre-registered templates
    pub fn new<T: ToJson>(name: &str, value: T) -> Template {
        Template {
            name: Some(name.to_string()),
            value: value::to_value(&value),
            content: None
        }
    }

    /// render some template with temporary template string
    pub fn with<T: ToJson>(content: &str, value: T) -> Template {
        Template {
            name: None,
            value: value::to_value(&value),
            content: Some(content.to_string())
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
    pub fn new() -> HandlebarsEngine {
        HandlebarsEngine {
            sources: Vec::new(),
            registry: RwLock::new(Box::new(Handlebars::new()))
        }
    }

    pub fn from(reg: Handlebars) -> HandlebarsEngine {
        HandlebarsEngine {
            sources: Vec::new(),
            registry: RwLock::new(Box::new(reg))
        }
    }

    pub fn add(&mut self, source: Box<Source + Send + Sync>) {
        self.sources.push(source);
    }

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
    fn after(&self, _: &mut Request, mut resp: Response) -> IronResult<Response> {
        let page_wrapper = resp.extensions.remove::<HandlebarsEngine>()
            .and_then(|h| {
                let hbs = self.registry.read().unwrap();
                if let Some(ref name) = h.name {
                    return Some(hbs.render(name, &h.value).map_err(TemplateRenderError::from));
                } else if let Some(ref content) = h.content {
                    return Some(hbs.template_render(content, &h.value));
                } else {
                    unreachable!();
                }
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

    fn catch(&self, req: &mut Request, mut err: IronError) -> IronResult<Response> {
        err.response = try!(self.after(req, err.response));
        Err(err)
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;
    use iron::prelude::*;
    use middleware::*;
    use handlebars::{Handlebars, RenderError, RenderContext, Helper, Context};

    fn hello_world() -> IronResult<Response> {
        let resp = Response::new();

        let mut data = BTreeMap::new();
        data.insert("title".to_owned(), "Handlebars on Iron".to_owned());

        Ok(resp.set(Template::new("index", data)))
    }

    fn hello_world2() -> IronResult<Response> {
        let resp = Response::new();

        let mut data = BTreeMap::new();
        data.insert("title".to_owned(), "Handlebars on Iron".to_owned());

        Ok(resp.set(Template::with("{{title}}", data)))
    }

    #[test]
    fn test_resp_set() {
        let mut resp = hello_world().ok().expect("response expected");

        // use response plugin to retrieve a cloned template for testing
        match resp.get::<HandlebarsEngine>() {
            Ok(h) => {
                assert_eq!(h.name.unwrap(), "index".to_string());
                assert_eq!(h.value.as_object().unwrap()
                           .get(&"title".to_string()).unwrap()
                           .as_string().unwrap(),
                           "Handlebars on Iron");
            },
            _ => panic!("template expected")
        }
    }

    #[test]
    fn test_resp_set2() {
        let mut resp = hello_world2().ok().expect("response expected");

        // use response plugin to retrieve a cloned template for testing
        match resp.get::<HandlebarsEngine>() {
            Ok(h) => {
                assert_eq!(h.content.unwrap(), "{{title}}".to_string());
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
        let hbs = HandlebarsEngine::new();
        let mut reg = hbs.registry.write().unwrap();
        reg.register_helper("ignore", Box::new(|_: &Context, _: &Helper, _: &Handlebars, _: &mut RenderContext| -> Result<(), RenderError> {
            Ok(())
        }));
    }
}
