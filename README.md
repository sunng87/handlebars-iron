handlebars-iron
===============

[Handlebars](https://github.com/sunng87/handlebars-rust) middleware
for the [Iron web framework](http://ironframework.io).

[![Build
Status](https://travis-ci.org/sunng87/handlebars-iron.svg?branch=master)](https://travis-ci.org/sunng87/handlebars-iron)
[![](http://meritbadge.herokuapp.com/handlebars-iron)](https://crates.io/crates/handlebars-iron)

The most recent version of handlebars-iron, like Iron and
Handlebars-rust, now compiles on nightly, beta and 1.0 channel. Our
travis task will track the compatibility on all three channels.

## Usage

Add HandlebarsEngine to your Iron middleware chain as an "after"
middleware.

```rust
  /// HandlebarsEngine will look up all files with "./examples/templates/**/*.hbs"
  chain.link_after(HandlebarsEngine::new("./examples/templates/", ".hbs"));
```

In your handler, set `Template` to response. As required by
Handlebars-rust, your data should impl `serialize::json::ToJson`. If
you are on nightly channel, it is highly recommended to use
[tojson_macros](https://github.com/sunng87/tojson_macros) to generate
default `ToJson` implementation without repeating yourself.

```rust
/// render data with "index" template
/// that is "./examples/templates/index.hbs"
fn hello_world(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let data = ...
    resp.set_mut(Template::new("index", data)).set_mut(status:Ok);
    Ok(resp)
}
```

Since this is simple library, you may run this
[example](https://github.com/sunng87/handlebars-iron/blob/master/examples/server.rs)
with `cargo run --example server`
first, and  [documentation](http://sunng.info/handlebars-iron/)
then.

Since Rust and its ecosystem are still in early stage, this
project might been broken for various reasons. I will try my best to
keep this library compiles with latest Rust nightly before the 1.0
final release. If you find anything bad, pull requests and issue reporting
are always welcomed.

## License

MIT, of course.
