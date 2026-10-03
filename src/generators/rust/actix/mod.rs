//! Actix-web handlers and utoipa annotations for the generated models.
//!
//! The model layer already knows every column, its type, and the table's primary key —
//! everything a REST endpoint needs. Deriving the handlers from the same source keeps the
//! route, the SQL, and the OpenAPI document in step: a column added to the database reaches
//! the HTTP surface and the published schema without being described a third time.
//!
//! One `routes.rs` per model folder, plus a parent `api` module collecting them into a single
//! `configure` and one `ApiDoc`. A view gets `GET` only — the same read-only rule the model
//! layer enforces, carried up to the transport.
//!
//! Routes are never mounted for you. `configure` is a function you call inside whatever
//! `web::scope` and auth middleware your application already uses: generating 800 open CRUD
//! endpoints against a live business database would be a security hole, not a feature.

mod api;
mod handlers;
mod imports;
mod paths;

use crate::database::introspection::Schema;
use crate::generators::rust::model::Model;

pub use api::parent;

/// Renders the `routes.rs` for one model.
///
/// Each handler is emitted only when the model actually has the method behind it: a table whose
/// every column is the key or database-assigned gets no `update`, and a route calling it would
/// not compile.
pub fn render(model: &Model, schema: &Schema) -> String {
    let mut file = imports::render(model, schema);

    file.push_str(&handlers::list(model));

    // `create` always exists for a table: the model emits a `DEFAULT VALUES` insert when every
    // column is database-assigned. A view has none.
    if !model.view {
        file.push_str(&handlers::create(model));
    }

    // Every keyed route addresses a row by one value, so a composite or absent key means none
    // of them can be generated — the listing stands alone.
    if let Some(key) = model.key() {
        file.push_str(&handlers::find(model, key));

        if !model.view {
            // A table whose every column is the key or database-assigned gets no `update`, so
            // a handler calling it would not compile.
            if !model.updatable().is_empty() {
                file.push_str(&handlers::update(model, key));
            }
            file.push_str(&handlers::remove(model, key));
        }
    }

    file.push_str(&api::configure(model));
    file
}
