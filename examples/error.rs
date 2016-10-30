extern crate iron;
extern crate handlebars_iron as hbs;

use iron::prelude::*;
use iron::{status, AfterMiddleware};
use hbs::{Template, HandlebarsEngine, DirectorySource};

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
        println!("{}", err);
        Err(err)
    }
}

fn main() {
    let mut chain = Chain::new(hello_world);
    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new("./examples/templates/", ".hbs")));
    // success of panic
    if let Err(r) = hbse.reload() {
        panic!("{}", r);
    }


    chain.link_after(hbse);
    chain.link_after(ErrorReporter);
    println!("Server running at http://localhost:3000/");
    Iron::new(chain).http("localhost:3000").unwrap();
}
