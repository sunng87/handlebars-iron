#![allow(dead_code, unused_imports)]
extern crate iron;
extern crate handlebars_iron as hbs;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::{status};
use hbs::{Template, HandlebarsEngine};
#[cfg(feature = "watch")]
use hbs::Watchable;
use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use std::sync::Arc;

struct Team {
    name: String,
    pts: u16
}

impl ToJson for Team {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("name".to_string(), self.name.to_json());
        m.insert("pts".to_string(), self.pts.to_json());
        m.to_json()
    }
}

fn make_data () -> BTreeMap<String, Json> {
    let mut data = BTreeMap::new();

    data.insert("year".to_string(), "2015".to_json());

    let teams = vec![ Team { name: "Jiangsu Sainty".to_string(),
                             pts: 43u16 },
                      Team { name: "Beijing Guoan".to_string(),
                             pts: 27u16 },
                      Team { name: "Guangzhou Evergrand".to_string(),
                             pts: 22u16 },
                      Team { name: "Shandong Luneng".to_string(),
                             pts: 12u16 } ];

    data.insert("teams".to_string(), teams.to_json());
    data
}

/// the handler
fn hello_world(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let data = make_data();
    resp.set_mut(Template::new("index", data)).set_mut(status::Ok);
    Ok(resp)
}

#[cfg(feature = "watch")]
fn main() {
    let mut chain = Chain::new(hello_world);
    let template_engine_ref = Arc::new(HandlebarsEngine::new("./examples/templates/", ".hbs"));
    template_engine_ref.watch();

    chain.link_after(template_engine_ref);

    println!("Server running at http://localhost:3000/");
    Iron::new(chain).http("localhost:3000").unwrap();
}

#[cfg(not(feature = "watch"))]
fn main() {
    println!("Watch only enabled via --features watch option");
}
