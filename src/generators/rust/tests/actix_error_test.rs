//! How a generated handler answers when the query does not go as planned.

use super::actix_test::file;
use super::support::{schema, scratch};
use crate::generators::rust::{write_with, Options};

#[test]
fn a_missing_row_is_a_404_rather_than_an_error() {
    let routes = file("routes.rs");

    // `find` returning None is an ordinary outcome; a 500 would blame the server for a
    // well-formed request whose answer is simply "no".
    assert!(routes.contains("Ok(None) => HttpResponse::NotFound().finish()"));
    assert!(routes.contains("(status = 404"));
}

#[test]
fn an_absent_delete_target_is_a_404() {
    let routes = file("routes.rs");

    // The model returns `bool`; the caller asked for a specific row to be gone and it was
    // never there, which is not a success.
    assert!(routes.contains("Ok(false) => HttpResponse::NotFound().finish()"));
    assert!(routes.contains("Ok(true) => HttpResponse::NoContent().finish()"));
}

#[test]
fn a_query_failure_never_leaks_its_message() {
    let routes = file("routes.rs");

    // A sqlx error names columns, constraints, and SQL text. It is logged, not returned.
    assert!(routes.contains("api::server_error(error)"), "{routes}");
    assert!(!routes.contains("error.to_string()"), "{routes}");
}

#[test]
fn the_shared_responder_logs_rather_than_returning_the_error() {
    let out = scratch("actix-error");
    write_with(&out, &schema(), Options { actix: true }, false).expect("write should succeed");

    let api = std::fs::read_to_string(out.join("api").join("mod.rs")).unwrap();

    assert!(
        api.contains("log::error!(\"query failed: {error}\")"),
        "{api}"
    );
    assert!(
        api.contains("HttpResponse::InternalServerError().finish()"),
        "{api}"
    );

    let _ = std::fs::remove_dir_all(&out);
}

#[test]
fn nothing_is_written_unless_asked_for() {
    let out = scratch("no-actix");
    write_with(&out, &schema(), Options::default(), false).expect("write should succeed");

    assert!(!out.join("api").exists(), "api module without --actix");
    assert!(!out.join("product").join("routes.rs").exists());

    let _ = std::fs::remove_dir_all(&out);
}
