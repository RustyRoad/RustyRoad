//! Local catalog fixtures are not live PostgreSQL evidence.
mod additions;
mod blockers;
mod creation;
mod imports;
mod source_queries;
mod updates;

use super::types::{Catalog, Column, Table};
use crate::model_audit::{parse, types::Model};
use std::path::Path;

/// Parse an isolated declaration without a source-file import environment.
fn model(source: &str) -> Model {
    parse::parse(
        Path::new("/fixture/src/model.rs"),
        Path::new("/fixture"),
        source,
    )
    .unwrap()
    .remove(0)
}

/// Explicitly complete catalog fixture with no tables.
fn empty() -> Catalog {
    Catalog {
        version: 1,
        schema: "public".into(),
        complete: true,
        tables: vec![],
        reserved_names: vec![],
    }
}

/// Existing integer primary key with a catalog-provided type spelling.
fn rows(kind: &str) -> Catalog {
    let mut catalog = empty();
    catalog.tables.push(Table {
        name: "rows".into(),
        kind: kind.into(),
        primary_key: vec!["id".into()],
        primary_key_details: None,
        columns: vec![Column {
            name: "id".into(),
            sql_type: "integer".into(),
            nullable: false,
        }],
    });
    catalog
}
