extern crate handlebars_iron as hbsi;

use hbsi::{DirectorySource, HandlebarsEngine};

#[test]
fn test_template() {
    let mut hbse = HandlebarsEngine::new();
    let src = Box::new(DirectorySource::new("./examples/templates/", ".hbs"));

    hbse.add(src);

    // success of panic
    if let Err(r) = hbse.reload() {
        panic!("{}", r);
    }

    let hh = hbse.registry.read().unwrap();

    assert!(hh.get_template("index").is_some());
    assert!(hh.get_template("some/path/hello").is_some());
}

#[test]
fn test_template_without_slash() {
    let mut hbse = HandlebarsEngine::new();
    let src = Box::new(DirectorySource::new("./examples/templates", ".hbs"));

    hbse.add(src);

    // success of panic
    if let Err(r) = hbse.reload() {
        panic!("{}", r);
    }

    let hh = hbse.registry.read().unwrap();

    assert!(hh.get_template("index").is_some());
    assert!(hh.get_template("some/path/hello").is_some());
}
