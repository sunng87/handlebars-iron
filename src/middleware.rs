use std::str::FromStr;
use std::io::{File};
use std::collections::BTreeMap;

use iron::prelude::*;
use iron::{AfterMiddleware, ChainBuilder, typemap};
use iron::modifier::Modifier;
use iron::status;
use iron::headers;

use glob::glob;

use handlebars::Handlebars;
use serialize::json::{ToJson, Json};

struct HandlebarsRenderer {
    registry: Handlebars
}

struct Template {
    name: String,
    value: Json
}

impl Template {
    pub fn new(name: String, value: Json) -> Template{
        Template {
            name: name,
            value: value
        }
    }
}

impl Modifier<Response> for Template {
    fn modify(self, resp: &mut Response) {
        resp.extensions.insert::<HandlebarsRenderer>(self);
    }
}

impl typemap::Key for HandlebarsRenderer {
    type Value = Template;
}

impl HandlebarsRenderer {
    fn new(prefix: &str, suffix: &str) -> HandlebarsRenderer {
        let mut r = Handlebars::new();

        let mut pattern = String::new();
        pattern.push_str(prefix);
        pattern.push_str("**/*");
        pattern.push_str(suffix);

        for path in glob(pattern.as_slice()) {
            let disp = path.display();
            let t = r.register_template_string(
                disp.slice(prefix.len(), disp.len()-suffix.len()),
                File::open(&path).ok()
                    .expect(format!("Failed to open file {}", disp).as_slice())
                    .read_to_string().unwrap());

            if t.is_err() {
                panic!("Failed to create template.");
            }
        }

        HandlebarsRenderer {
            registry: r
        }
    }
}

impl AfterMiddleware for HandlebarsRenderer {
    fn after(&self, _: &mut Request, resp: &mut Response) -> IronResult<()> {
        let page = match resp.extensions.get::<HandlebarsRenderer>() {
            Some(h) => {
                let name = h.name;
                let value = &h.value;
                let page = self.registry.render(name.as_slice(), value).unwrap();
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

    resp.set(Template::new("index".to_string(), data.to_json()));
    Ok(resp)
}


/*
fn main() {
let mut chain = ChainBuilder::new(hello_world);
chain.link_after(HandlebarsRenderer::new());
Iron::new(chain).listen("localhost:3000").unwrap();
}
*/
