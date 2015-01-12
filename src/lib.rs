#![allow(unstable)]
#![unstable]

extern crate iron;
extern crate "rustc-serialize" as serialize;
extern crate handlebars;
extern crate modifier;
extern crate glob;

pub use self::middleware::Template;
pub use self::middleware::HandlebarsEngine;

mod middleware;
