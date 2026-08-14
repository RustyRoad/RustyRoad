//! The `#[utoipa::path]` annotation above each handler.
//!
//! Written as an attribute rather than a hand-maintained list, so the documented status codes
//! and body types come from the same model the handler queries. A response the handler can
//! return but the annotation omits is a published schema that lies.

mod keyed;
mod unkeyed;

use crate::generators::rust::model::Field;

pub(super) use keyed::{find, remove, update};
pub(super) use unkeyed::{create, create_empty, list};

/// Renders the path parameter block for a keyed route.
fn parameter(key: &Field) -> String {
    format!(
        "\x20   params(\n\
         \x20       (\"{ident}\" = {ty}, Path, description = \"The row's key\"),\n\
         \x20   ),\n",
        ident = key.ident,
        ty = owned_key(key.key_type())
    )
}

/// Returns the owned type an extracted path segment deserializes into.
///
/// `Option` is stripped because a primary key is never null, whatever the column's declared
/// nullability says — a path segment either parses or the request is a 400.
pub(super) fn owned_key(rust: &str) -> String {
    rust.strip_prefix("Option<")
        .and_then(|rest| rest.strip_suffix('>'))
        .unwrap_or(rust)
        .to_string()
}

/// The status lines every keyed route shares.
const NOT_FOUND: &str = "\x20       (status = 404, description = \"No such row\"),\n";
