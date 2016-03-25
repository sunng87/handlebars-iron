#![allow(dead_code, unused_imports)]
extern crate iron;
extern crate handlebars_iron as hbs;
#[cfg(not(feature = "serde_type"))]
extern crate rustc_serialize;
extern crate env_logger;

use iron::prelude::*;
use hbs::{HandlebarsEngine, DirectorySource};
#[cfg(feature = "watch")]
use hbs::Watchable;

use std::sync::Arc;
use std::error::Error;

#[cfg(not(feature = "serde_type"))]
mod data {
    use std::collections::BTreeMap;
    use rustc_serialize::json::{ToJson, Json};
    use iron::prelude::*;
    use iron::status;
    use hbs::Template;

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
        data.insert("engine".to_string(), "rustc_serialize".to_json());
        data
    }

    /// the handler
    pub fn hello_world(_: &mut Request) -> IronResult<Response> {
        let mut resp = Response::new();

        let data = make_data();
        resp.set_mut(Template::new("index", data)).set_mut(status::Ok);
        Ok(resp)
    }
}


#[cfg(feature = "watch")]
fn main() {
    use data::*;
    env_logger::init().unwrap();

    let mut chain = Chain::new(hello_world);

    let mut hbse = HandlebarsEngine::new();
    let source = Box::new(DirectorySource::new("./examples/templates/", ".hbs"));
    hbse.add(source);
    if let Err(r) = hbse.reload() {
        panic!("{}", r.description());
    }

    let hbse_ref = Arc::new(hbse);
    hbse_ref.watch("./examples/templates/");

    chain.link_after(hbse_ref);

    println!("Server running at http://localhost:3000/");
    Iron::new(chain).http("localhost:3000").unwrap();
}

#[cfg(not(feature = "watch"))]
fn main() {
    println!("Watch only enabled via --features watch option");
}
