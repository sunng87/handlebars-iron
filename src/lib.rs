#![allow(unstable)]
#![unstable]

//! # Handlebars for Iron
//!
//! This library combines [Handlebars templating library](https://github.com/sunng87/handlebars-rust) and [Iron web framework](http://ironframework.io) together. It gives you a `HandlebarsEngine` as Iron `AfterMiddleware`, so you can render your data with specified handlebars template.
//!
//! ## Setup
//!
//! Given the template root directory (prefix) and template file extension (suffix), you can create `HandlebarsEngine` with `Handlebars::new("/prefix/path", ".hbs")` function. HandlebarsEngine will scan the directory and its sub-directories (with Unix glob **/*), and register these templates with `path/name` as name.
//!
//! ## Usage
//!
//! From any of your handler, you can set template name and data into our `Template` struct. Remember you need to make your data `ToJson`-able, which is required by handlebars-rust.
//!
//! We have implemented Modifier for `Template` on `Response`, so you can just use `response.set` to put set template into response and let it processed by our middleware.
//!

extern crate iron;
extern crate "rustc-serialize" as serialize;
extern crate handlebars;
extern crate modifier;
extern crate glob;

pub use self::middleware::Template;
pub use self::middleware::HandlebarsEngine;

mod middleware;
