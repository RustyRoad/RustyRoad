//! Clause construction for the backfill batch statement.

use super::super::super::quote::{quote_ident, quote_literal};

/// Returns the primary key columns as a quoted, comma-separated list.
pub(super) fn key_list(primary_key: &[String]) -> String {
    primary_key
        .iter()
        .map(|key| quote_ident(key))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Returns the tuple comparison resuming after `last_value`.
pub(super) fn cursor(primary_key: &[String], last_value: &[String]) -> String {
    let values = last_value
        .iter()
        .map(|value| quote_literal(value))
        .collect::<Vec<_>>()
        .join(", ");
    format!(" AND ({}) > ({values})", key_list(primary_key))
}

/// Returns the join condition matching a table row to its batch entry.
pub(super) fn join(table: &str, primary_key: &[String]) -> String {
    let table = quote_ident(table);
    primary_key
        .iter()
        .map(|key| {
            let key = quote_ident(key);
            format!("{table}.{key} = batch.{key}")
        })
        .collect::<Vec<_>>()
        .join(" AND ")
}

/// Returns the qualified column list for the UPDATE's RETURNING clause.
pub(super) fn returning(table: &str, primary_key: &[String]) -> String {
    let table = quote_ident(table);
    primary_key
        .iter()
        .map(|key| format!("{table}.{}", quote_ident(key)))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Returns the cursor projection, casting each key to text.
///
/// The cursor is carried between batches as a string, so keys are cast in SQL
/// rather than relying on the client to coerce an integer column into text.
pub(super) fn cursor_projection(primary_key: &[String]) -> String {
    primary_key
        .iter()
        .map(|key| format!("{}::text", quote_ident(key)))
        .collect::<Vec<_>>()
        .join(", ")
}
