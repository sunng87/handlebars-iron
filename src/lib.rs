//! # Handlebars for Iron
//!
//! This library combines [Handlebars templating library](https://github.com/sunng87/handlebars-rust) and [Iron web framework](http://ironframework.io) together. It gives you a `HandlebarsEngine` as Iron `AfterMiddleware`, so you can render your data with specified handlebars template.
//!
//! ## Setup
//!
//! Handlebars-iron provides two kinds of template source: `DirectorySource` and `MemorySource` by default. `DirectorySource` helps you to load template files from a directory, which `MemorySource` loads template as string in memory.
//!
//! To load files from file system, you need to specify template root and file suffix. Handlebars-iron will scan the directory and loads all templates that matches the suffix. The file's relative path name will be applied as template name.
//!
//! ```ignore
//! /// HandlebarsEngine will look up all files with "./examples/templates/**/*.hbs"
//! let mut hbse = HandlebarsEngine::new();
//! hbse.add(Box::new(DirectorySource::new("./examples/templates/", ".hbs")));
//!
//! // load templates from all registered sources
//! if let Err(r) = hbse.reload() {
//!   panic!("{}", r.description());
//! }
//!
//! chain.link_after(hbse);
//!
//! ```
//!
//! ## Usage
//!
//! From any of your handler, you can set template name and data into our `Template` struct. Remember you need to make your data implements `serde::Serialize`, which is required by handlebars-rust.
//!
//! We have implemented Modifier for `Template` on `Response`, so you can just use `response.set` to put set template into response and let it processed by our middleware.
//!
//! Also we made `Response` plugin for `Template` via `HandlebarsEngine`. So you can test your handler from a test case, and retrieve the `Template` you set into it by `response.get::<HandlebarsEngine>`.
//!

pub extern crate handlebars;

extern crate iron;

extern crate serde;
extern crate serde_json;

extern crate plugin;
extern crate walkdir;
#[cfg(feature = "watch")]
extern crate notify;

#[macro_use]
extern crate log;

pub use self::middleware::Template;
pub use self::middleware::HandlebarsEngine;
pub use self::source::{Source, SourceError};
pub use self::sources::directory::DirectorySource;
pub use self::sources::memory::MemorySource;
#[cfg(feature = "watch")]
pub use self::watch::Watchable;

mod middleware;
#[cfg(feature = "watch")]
mod watch;
mod source;
mod sources;
