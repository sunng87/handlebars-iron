extern crate iron;
extern crate handlebars_iron as hbs;
extern crate rustc_serialize;

use std::error::Error;
use iron::prelude::*;
use iron::{status, AfterMiddleware};
use hbs::{Template, HandlebarsEngine};

/// the handler
fn hello_world(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let data = "".to_owned();
    resp.set_mut(Template::new("not-exist", data)).set_mut(status::Ok);
    Ok(resp)
}

struct ErrorReporter;

impl AfterMiddleware for ErrorReporter {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        println!("{}", err.description());
        Err(err)
    }
}

fn main() {
    let mut chain = Chain::new(hello_world);
    chain.link_after(HandlebarsEngine::new("./examples/templates/", ".hbs"));
    chain.link_after(ErrorReporter);
    println!("Server running at http://localhost:3000/");
    Iron::new(chain).http("localhost:3000").unwrap();
}
