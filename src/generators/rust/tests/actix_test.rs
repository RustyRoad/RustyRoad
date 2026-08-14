//! Generated actix handlers for a writable table.

use super::support::{products, resolve, schema};
use crate::generators::rust::render;

/// Renders one file for the `products` fixture with actix output on.
pub(super) fn file(name: &str) -> String {
    let schema = schema();
    let model = resolve(&products(), &schema).with_actix(true);

    render::files(&model, &schema)
        .into_iter()
        .find(|file| file.name == name)
        .unwrap_or_else(|| panic!("{name} should be emitted"))
        .contents
}

#[test]
fn every_verb_gets_a_handler() {
    let routes = file("routes.rs");

    for handler in ["list()", "find(", "create(", "update(", "remove("] {
        let signature = format!("pub async fn {handler}");
        assert!(
            routes.contains(&signature),
            "missing {signature}:\n{routes}"
        );
    }
}

#[test]
fn every_handler_is_routed() {
    let routes = file("routes.rs");

    // A handler nothing routes to is dead code.
    for registration in [
        "web::get().to(list)",
        "web::get().to(find)",
        "web::post().to(create)",
        "web::put().to(update)",
        "web::delete().to(remove)",
    ] {
        assert!(routes.contains(registration), "missing {registration}");
    }
}

#[test]
fn each_handler_is_annotated_for_the_published_schema() {
    let routes = file("routes.rs");

    assert_eq!(routes.matches("#[utoipa::path(").count(), 5, "{routes}");
    // The path must be the table's and the body the model's, or the document describes an
    // endpoint that does not exist.
    assert!(routes.contains("path = \"/products\""));
    assert!(routes.contains("body = [Product]"));
    assert!(routes.contains("request_body = Product"));
}
