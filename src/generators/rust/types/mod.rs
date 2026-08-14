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

mod mapping;
mod scalar;

use super::casing;
use crate::database::introspection::{Column, Schema};

pub use mapping::Mapping;

/// Maps a column to its Rust type, wrapping a nullable column in `Option`.
///
/// A nullable column decoded as a bare `T` errors on the first `NULL` row, so
/// nullability belongs in the type rather than in a comment.
pub fn map(column: &Column, schema: &Schema) -> Mapping {
    let mapped = base(column, schema);

    if column.nullable {
        return mapped.optional();
    }
    mapped
}

/// Maps the column's type, ignoring nullability.
fn base(column: &Column, schema: &Schema) -> Mapping {
    let declared = column.sql_type.trim();

    // A user-defined enum arrives as its bare type name, so it is matched before the
    // scalar table, where it would fall through to text and lose its variants.
    if schema
        .enums
        .iter()
        .any(|item| item.name == declared.trim_matches('"'))
    {
        return Mapping::plain(casing::to_pascal(declared.trim_matches('"')));
    }

    let lowered = declared.to_lowercase();

    if let Some(element) = scalar::array_element(&lowered) {
        return scalar::map(&scalar::strip_modifier(&element)).collected();
    }

    scalar::map(&scalar::strip_modifier(&lowered))
}
