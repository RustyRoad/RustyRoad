//! The flat `models.rs` the API pull writes must put every type in the OpenAPI document.
//!
//! Handlers name the row, New and Patch structs in request and response bodies, so a
//! missing `ToSchema` is a compile error in the app's `ApiDoc` rather than anywhere here.

use super::support::{schema, scratch};
use crate::generators::rust::write_api;

#[test]
fn canonical_api_models_derive_openapi_schemas() {
    let out = scratch("canonical-api-schema");
    write_api(&out, &schema(), false).expect("write should succeed");
    let models = std::fs::read_to_string(out.join("models.rs")).unwrap();

    // This emitter keeps the table name as written, so `products` stays plural.
    for name in ["Products", "NewProducts", "PatchProducts"] {
        let declared = format!("pub struct {name} {{");
        let at = models.find(&declared).expect("struct should be declared");
        let derive = models[..at].rfind("#[derive(").unwrap();
        assert!(
            models[derive..at].contains("utoipa::ToSchema"),
            "{name} does not derive ToSchema"
        );
    }

    // PgInterval has no schema of its own, so the newtype implements it by hand.
    assert!(models.contains("impl utoipa::ToSchema for Interval {}"));

    let _ = std::fs::remove_dir_all(&out);
}
