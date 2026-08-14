//! The read-only rule, carried from SQL up to the transport.

use super::support::{required, resolve, serial_key};
use crate::database::introspection::{Schema, Table};
use crate::generators::rust::render;

/// A read-only view.
fn view() -> Table {
    Table {
        name: "sales_summary".to_string(),
        columns: vec![serial_key("id"), required("total", "bigint")],
        view: true,
        ..Table::default()
    }
}

/// Renders the view's `routes.rs`.
fn routes() -> String {
    let schema = Schema {
        tables: vec![view()],
        enums: Vec::new(),
    };
    let model = resolve(&view(), &schema).with_actix(true);

    render::files(&model, &schema)
        .into_iter()
        .find(|file| file.name == "routes.rs")
        .expect("routes.rs should be emitted")
        .contents
}

#[test]
fn a_view_gets_read_handlers() {
    let routes = routes();

    assert!(routes.contains("pub async fn list()"), "{routes}");
    assert!(routes.contains("pub async fn find("), "{routes}");
    assert!(
        routes.contains("SalesSummary::find(path.into_inner())"),
        "{routes}"
    );
}

#[test]
fn a_view_gets_no_mutating_handler_or_route() {
    let routes = routes();

    // Postgres rejects writes to a view, so a mutating route could only ever fail. The rule is
    // now enforced three layers deep: SQL, the model, and the HTTP surface.
    for absent in [
        "fn create(",
        "fn update(",
        "fn remove(",
        "web::post()",
        "web::put()",
        "web::delete()",
    ] {
        assert!(!routes.contains(absent), "{absent} on a view:\n{routes}");
    }
}
