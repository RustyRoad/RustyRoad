//! Postgres types to Rust types, as `sqlx` decodes them.
//!
//! The mapping is driven by the column's own reported type, so a table mixing `text`,
//! `timestamp` and `bigserial` produces three different Rust types. Every choice
//! matches what `sqlx`'s Postgres decoder actually implements: a type that merely looks
//! right but has no `Decode` impl produces a struct that compiles and then fails on the
//! first query, which is harder to diagnose than a plain string.
//!
//! Types are written fully qualified, following the hand-written models: a single
//! `chrono::NaiveDateTime` in a field is clearer than an import that has to be kept in
//! step with the columns that need it.

mod base;
mod column_override;
pub(super) mod column_validation;
mod mapping;
mod scalar;
pub use column_override::ColumnOverride;
pub(super) mod overrides;
pub use overrides::Override;
pub(super) use scalar::array_element;

use super::allocation::Names;
use crate::database::introspection::{Column, Schema, Table};

pub use mapping::Mapping;

/// Maps a column to its Rust type, wrapping a nullable column in `Option`.
///
/// A nullable column decoded as a bare `T` errors on the first `NULL` row, so
/// nullability belongs in the type rather than in a comment.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub fn map(column: &Column, schema: &Schema) -> Mapping {
    let mapped = base::map(column, schema, &Names::new(schema));
    if column.nullable {
        mapped.optional()
    } else {
        mapped
    }
}

/// Reuses the API allocation and configured codecs, retaining column nullability.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn map_column(
    table: &Table,
    column: &Column,
    schema: &Schema,
    names: &Names,
) -> Mapping {
    let mapped = names
        .column_codec(table, column)
        .map(|codec| Mapping::plain(&codec.rust))
        .unwrap_or_else(|| base::map(column, schema, names));

    if column.nullable {
        return mapped.optional();
    }
    mapped
}
