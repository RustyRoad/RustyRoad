//! Postgres types to TetherScript value kinds.
//!
//! TetherScript is dynamically typed, so there is no type to declare on a field. What a
//! generated model can still do is carry the *kind* each column decodes to, which is
//! what makes validation and defaults possible: the native client decodes rows by shape,
//! so a column whose text happens to look numeric arrives as a number whatever the
//! schema said. Naming the expected kind gives a model something to check against
//! before a bad value reaches SQL.

mod kind;

use crate::database::introspection::{Column, Schema};

pub use kind::Kind;

/// Maps a column to the kind its value takes.
pub fn map(column: &Column, schema: &Schema) -> Kind {
    let declared = column.sql_type.trim();

    // An enum arrives as its label, so it is a string rather than a distinct kind.
    if schema
        .enums
        .iter()
        .any(|item| item.name == declared.trim_matches('"'))
    {
        return Kind::Str;
    }

    let lowered = declared.to_lowercase();
    if is_array(&lowered) {
        return Kind::List;
    }

    scalar(&strip_modifier(&lowered))
}

/// Maps a non-array, modifier-free Postgres type name.
fn scalar(base: &str) -> Kind {
    match base {
        "smallint" | "int2" | "smallserial" | "integer" | "int4" | "serial" | "bigint" | "int8"
        | "bigserial" => Kind::Int,
        "real" | "float4" | "double precision" | "float8" => Kind::Float,
        // numeric decodes as a float by shape, not as an exact decimal: the wire format
        // is text and the client converts what looks like a decimal.
        "numeric" | "decimal" | "money" => Kind::Float,
        "boolean" | "bool" => Kind::Bool,
        "json" | "jsonb" => Kind::Json,
        // Everything else — text, uuid, timestamps, bytea — crosses the wire as text.
        _ => Kind::Str,
    }
}

/// Returns `true` when the type is an array.
///
/// `format_type` reports `integer[]`, while the raw catalog reports `_int4`.
fn is_array(lowered: &str) -> bool {
    lowered.ends_with("[]") || lowered.starts_with('_')
}

/// Removes a parenthesized modifier wherever it appears in the type name.
///
/// A precision modifier sits mid-type in `timestamp(3) with time zone`, so trimming at
/// the first parenthesis would leave the suffix behind and break matching.
fn strip_modifier(lowered: &str) -> String {
    let Some(start) = lowered.find('(') else {
        return lowered.trim().to_string();
    };
    let Some(end) = lowered.rfind(')') else {
        return lowered.trim().to_string();
    };

    format!("{}{}", &lowered[..start], &lowered[end + 1..])
        .trim()
        .to_string()
}
