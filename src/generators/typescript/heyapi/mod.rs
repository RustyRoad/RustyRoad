//! OpenAPI document generation for Hey API client generation.
//!
//! Rather than emitting a Hey API SDK directly — which would mean vendoring the
//! ~2000-line `client/` and `core/` runtime it ships — this emits the OpenAPI
//! document Hey API consumes, plus a config file. That is the pipeline a Hey API
//! project already uses: produce a document, then run `@hey-api/openapi-ts`
//! against it.
//!
//! Generating the document rather than scraping it from a running server removes
//! the bootstrap problem: a client can be generated in CI without booting the API.

mod document;
mod operations;
mod paths;
mod schemas;

use super::casing::Casing;
use super::client::key_is_numeric;
use crate::database::introspection::{Schema, Table};
use operations::Operation;

/// One generated file: its name and contents.
pub struct File {
    pub name: &'static str,
    pub contents: String,
}

/// Renders the OpenAPI document and the Hey API config.
///
/// `prefix` is the path the routes are mounted under, so the document's URLs match
/// the server. Tables without a single-column primary key are skipped, since their
/// routes are not generated either.
pub fn render(schema: &Schema, casing: Casing, prefix: &str) -> Vec<File> {
    let operations = collect(schema, prefix);

    vec![
        File {
            name: "openapi.json",
            contents: document::document(schema, casing, &operations),
        },
        File {
            name: "openapi-ts.config.ts",
            contents: document::config(),
        },
    ]
}

/// Builds the operation list for every routable table.
fn collect(schema: &Schema, prefix: &str) -> Vec<Operation> {
    schema
        .tables
        .iter()
        .filter(|table| table.has_simple_key())
        .flat_map(|table| operations::operations(table, prefix, primary_key_type(table)))
        .collect()
}

/// Returns the TypeScript type of a table's primary key.
fn primary_key_type(table: &Table) -> &'static str {
    if key_is_numeric(table, &table.primary_key[0]) {
        "number"
    } else {
        "string"
    }
}
