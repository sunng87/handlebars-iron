extern crate handlebars_iron as hbsi;

use hbsi::{HandlebarsEngine, DirectorySource};

#[test]
fn test_template() {
    let mut hbse = HandlebarsEngine::new();
    let src = Box::new(DirectorySource::new("./examples/templates/", ".hbs"));

    hbse.add(src);
    hbse.reload();

    let hh = hbse.registry.read().unwrap();

    assert!(hh.get_template("index").is_some());
}
