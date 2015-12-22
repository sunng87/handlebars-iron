handlebars-iron
===============

[Handlebars](https://github.com/sunng87/handlebars-rust) middleware
for the [Iron web framework](http://ironframework.io).

[![Build
Status](https://travis-ci.org/sunng87/handlebars-iron.svg?branch=master)](https://travis-ci.org/sunng87/handlebars-iron)
[![](http://meritbadge.herokuapp.com/handlebars-iron)](https://crates.io/crates/handlebars-iron)

The most recent version of handlebars-iron, like Hyper, Iron and
Handlebars-rust, now compiles on nightly, beta and latest stable (1.5.0+) channel. Our
travis task will track the compatibility on all these channels.

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
with `RUST_LOG=handlebars_iron=info cargo run --example server`
first, and [documentation](http://sunng.info/handlebars-iron/)
then.

0.10.0 of handlebars-iron introduces source API. Directory source and
Memory source are supported by default. Previous API
(`HandlebarsEngine::new()` and `HandlebarsEngine::from()`) is
deprecated in 0.10.0 and will be replaced by `new2` and `from2` in
future.

Since Rust and its ecosystem are still in early stage, this
project might been broken for various reasons. I will try my best to
keep this library compiles with latest Rust nightly before the 1.0
final release. If you find anything bad, pull requests and issue reporting
are always welcomed.

## Live reload

During development you may want to live-reload your templates without
having to restart your web server. Here comes the live-reload
feature.

Since live-reload may only be useful in development phase, we have
made it a optional feature. In order to enable it, you will need to
add feature `watch` in your cargo declaration:

```toml
[features]
## create a feature in your app
watch = ["handlebars-iron/watch"]

[dependencies]
handlebars-iron = ...
```

Check `examples/watch_server.rs` for further information. To test it:
`RUST_LOG=handlebars_iron=info cargo run --example watch_server
--features watch`.

## License

MIT, of course.
