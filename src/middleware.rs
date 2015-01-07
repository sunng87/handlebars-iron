#![feature(globs)]
extern crate iron;
extern crate handlebars;
extern crate "rustc-serialize" as serialize;

use std::str::FromStr;
use std::io::{File};
use std::collections::BTreeMap;

use iron::prelude::*;
use iron::{AfterMiddleware, ChainBuilder, typemap, Response};
use iron::status;
use iron::headers;

use handlebars::Handlebars;
use serialize::json::{ToJson, Json};
use modifier::Modifier;

struct HandlebarsRenderer {
	  registry: Handlebars
}

struct Template<'a> {
    name: &'a str,
    value: Json
}

impl<'a> Template<'a> {
    pub fn new(name: &'a str, value: Json) -> Template<'a>{
        Template<'a> {
            name: name,
            value: value
        }
    }
}

impl<'a> Modifier<Response> for Template<'a> {
    fn modify(self, resp: &mut Response) {
        resp.extensions.insert::<HandlebarsRenderer, Template<'a>>(self);
    }
}

impl typemap::Assoc<Template<'a>> for HandlebarsRenderer {}

impl HandlebarsRenderer {
	  fn new(dir: &'static str, suffix: &'static str) -> HandlebarsRenderer {
		    let mut r = Handlebars::new();

		    let t = r.register_template_string("index", File::open(&Path::new("./examples/index.hbs")).unwrap().read_to_string().unwrap());

        if t.is_err() {
            panic!("Failed to create template.");
        }

		    HandlebarsRenderer {
			      registry: r
		    }
	  }
}

impl AfterMiddleware for HandlebarsRenderer {
	  fn after(&self, _: &mut Request, resp: &mut Response) -> IronResult<()> {
        let page = match resp.extensions.get::<HandlebarsRenderer, Template<'a>>() {
            Some(h) => {
                let name = h.name;
                let value = &h.value;
		            let page = self.registry.render(name, value).unwrap();
                Some(page)
            },
            None => {
                None
            }
        };

        if page.is_some() {
            resp.headers.set(headers::ContentType(FromStr::from_str("text/html;charset=utf-8").unwrap()));
            resp.set_mut(status::Ok).set_mut(page.unwrap());
        }

        Ok(())
    }
}

fn hello_world(_: &mut Request) -> IronResult<Response> {
	  let mut resp = Response::new();

	  let mut data = BTreeMap::new();
	  data.insert("title".to_string(), "Handlebars on Iron".to_json());

    resp.set(Template::new("index", data.to_json()));
    Ok(resp)
}


/*
fn main() {
	  let mut chain = ChainBuilder::new(hello_world);
    chain.link_after(HandlebarsRenderer::new());
    Iron::new(chain).listen("localhost:3000").unwrap();
}
*/
