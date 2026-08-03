//! Procedure metadata for oRPC generation.
//!
//! Each procedure carries an explicit `.route({ method, path })`, so one
//! definition serves an RPC call, a REST request, and the OpenAPI document oRPC
//! generates.

mod read;
mod write;

use crate::database::introspection::Table;
use crate::generators::typescript::casing::Casing;

/// One generated oRPC procedure.
pub(super) struct Procedure {
    /// Procedure name within its router, e.g. `list`.
    pub name: &'static str,
    /// HTTP method the route is reachable by.
    pub method: &'static str,
    /// Route path, with `{id}` where the procedure takes a key.
    pub path: String,
    /// Zod schema expression for the input.
    pub input: String,
    /// Zod schema expression for the output.
    pub output: String,
    /// Expression the handler returns.
    pub body: String,
}

/// Builds every procedure for a table, in router order.
pub(super) fn procedures(table: &Table, prefix: &str, casing: Casing) -> Vec<Procedure> {
    let collection = format!("{prefix}/{}", table.name);
    let single = format!("{collection}/{{id}}");

    vec![
        read::list(table, collection.clone(), casing),
        read::get(table, single.clone(), casing),
        write::create(table, collection, casing),
        write::update(table, single.clone(), casing),
        write::delete(table, single, casing),
    ]
}

/// Returns the Zod schema for a table's primary key.
///
/// `coerce` is required because a REST path parameter arrives as a string, while a
/// direct RPC call may already pass the right type.
fn key_schema(table: &Table) -> &'static str {
    let key = &table.primary_key[0];
    match crate::generators::typescript::client::key_type(table, key) {
        "number" => "z.coerce.number().int()",
        _ => "z.string()",
    }
}
