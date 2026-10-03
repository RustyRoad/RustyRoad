//! Route registration: per-model `configure`, and the parent that collects them.

mod parent_file;

use crate::generators::rust::model::Model;

pub use parent_file::parent;

/// Renders the per-model `configure`.
///
/// Takes a `ServiceConfig` rather than mounting anything itself, so the caller chooses the
/// scope, the middleware, and the auth guard. Nothing is reachable until an application
/// deliberately registers it.
pub(super) fn configure(model: &Model) -> String {
    let path = &model.table;
    let keyed = model.key().is_some();
    let mut routes = route(path, "get", "list", false);

    if keyed {
        routes.push_str(&route(path, "get", "find", true));
    }
    if !model.view {
        routes.push_str(&route(path, "post", "create", false));

        if keyed {
            // Mirrors what `render` emitted: a route to a handler that was skipped would name
            // a function that does not exist.
            if !model.updatable().is_empty() {
                routes.push_str(&route(path, "put", "update", true));
            }
            routes.push_str(&route(path, "delete", "remove", true));
        }
    }

    format!(
        "/// Registers this model's routes on a caller-owned scope.\n\
         ///\n\
         /// Nothing is mounted until an application calls this, so authentication and\n\
         /// rate limiting stay the caller's decision rather than a generated default.\n\
         pub fn configure(cfg: &mut web::ServiceConfig) {{\n{routes}}}\n"
    )
}

/// Renders one route registration.
fn route(path: &str, method: &str, handler: &str, keyed: bool) -> String {
    let suffix = if keyed { "/{key}" } else { "" };

    format!("\x20   cfg.route(\"/{path}{suffix}\", web::{method}().to({handler}));\n")
}
