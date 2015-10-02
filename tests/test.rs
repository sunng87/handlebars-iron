extern crate handlebars_iron as hbsi;

use hbsi::HandlebarsEngine;

#[test]
fn test_template() {
    let hbse = HandlebarsEngine::new("./examples/templates/", ".hbs");
    let hh = hbse.registry.read().unwrap();

    assert!(hh.get_template("index").is_some());
}
