//! The generated routes are inert until an application mounts them.

use super::support::{schema, scratch};
use crate::generators::rust::{write_with, Options};

#[test]
fn configure_takes_a_caller_owned_service_config() {
    let out = scratch("actix-mount");
    write_with(&out, &schema(), Options { actix: true }, false).expect("write should succeed");

    let api = std::fs::read_to_string(out.join("api").join("mod.rs")).unwrap();

    // Mounting 750 open CRUD endpoints against a live business database would be a security
    // hole, not a feature. The scope, the auth guard, and the rate limiting stay the
    // application's decision.
    assert!(api.contains("pub fn configure(cfg: &mut web::ServiceConfig)"));
    assert!(api.contains("super::product::routes::configure(cfg);"));
    assert!(api.contains("pub struct ApiDoc;"));

    // Nothing here binds a port, wraps middleware, or names a scope. Checked past the header,
    // whose example deliberately shows the caller doing exactly that.
    let code = api.split("use actix_web::").nth(1).unwrap();
    assert!(!code.contains("HttpServer"), "{code}");
    assert!(!code.contains("web::scope("), "{code}");
    assert!(!code.contains(".wrap("), "{code}");

    let _ = std::fs::remove_dir_all(&out);
}

#[test]
fn the_parent_declares_the_api_module() {
    let out = scratch("actix-declare");
    write_with(&out, &schema(), Options { actix: true }, false).expect("write should succeed");

    let parent = std::fs::read_to_string(out.join("mod.rs")).unwrap();

    assert!(parent.contains("pub mod api;"), "{parent}");
    // `configure` and `ApiDoc` are both named by the application wiring them up.
    assert!(parent.contains("pub use api::*;"), "{parent}");

    let _ = std::fs::remove_dir_all(&out);
}

#[test]
fn a_model_declares_its_routes_submodule() {
    let out = scratch("actix-submodule");
    write_with(&out, &schema(), Options { actix: true }, false).expect("write should succeed");

    let root = std::fs::read_to_string(out.join("product").join("mod.rs")).unwrap();

    // Public, unlike the CRUD submodules: the parent's `configure` and utoipa's `paths(...)`
    // both name the handlers directly.
    assert!(root.contains("pub mod routes;"), "{root}");

    let _ = std::fs::remove_dir_all(&out);
}
