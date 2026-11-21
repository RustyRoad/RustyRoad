//! Observed key fixtures for mocked local CLI checks; no SQL is executed.
use super::fixture::{catalog, project};
use serde_json::{json, Value};

/// Existing key facts, e.g. `table()["primary_key_details"]["deferrable"]`.
pub(super) fn table() -> Value {
    json!({
        "name":"rows", "kind":"table", "primary_key":["id"],
        "primary_key_details": {
            "name":"original_rows_key", "deferrable":true,
            "initially_deferred":true, "referenced_by":[], "standard_index":true
        },
        "columns":[
            {"name":"id", "sql_type":"integer", "nullable":false},
            {"name":"replacement", "sql_type":"bigint", "nullable":false}
        ]
    })
}

/// A reviewed key replacement, e.g. `replacement(table())`.
pub(super) fn replacement(table: Value) -> tempfile::TempDir {
    let dir = project("struct Rows { id: Option<i64>, replacement: i64 }");
    catalog(dir.path(), json!([table]));
    dir
}
