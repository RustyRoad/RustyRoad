//! SQL generation for versioned schema views.
//!
//! Each schema version gets its own Postgres schema containing one view per table.
//! Clients select a version by setting `search_path`, so old and new schemas are
//! served simultaneously from the same physical tables.

mod schema;
mod table;

use super::model::Schema;

pub use schema::{create_schema, drop_schema};
pub use table::{create_view, SECURITY_INVOKER_MIN_MAJOR};

/// Returns every statement needed to serve `snapshot` as `version`.
pub fn create_version(
    schema: &str,
    version: &str,
    snapshot: &Schema,
    major_version: u32,
) -> Vec<String> {
    let mut statements = vec![create_schema(schema, version)];
    for table in snapshot.visible_tables() {
        statements.extend(create_view(schema, version, table, major_version));
    }
    statements
}
