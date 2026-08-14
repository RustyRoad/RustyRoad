//! Builder lookup tables for Postgres types.

use super::{options, Builder};

/// Maps types that take no modifier.
pub(super) fn scalar(base: &str) -> Option<Builder> {
    if matches!(base, "bigint" | "int8") {
        return Some(Builder::with_options(
            "bigint",
            "{ mode: 'number' }".to_string(),
        ));
    }

    let import = match base {
        "smallint" | "int2" => "smallint",
        "integer" | "int4" => "integer",
        "boolean" | "bool" => "boolean",
        "real" | "float4" => "real",
        "double precision" | "float8" => "doublePrecision",
        "uuid" => "uuid",
        "text" => "text",
        "json" => "json",
        "jsonb" => "jsonb",
        "date" => "date",
        "inet" => "inet",
        "cidr" => "cidr",
        "macaddr" => "macaddr",
        "interval" => "interval",
        "time without time zone" | "time" => "time",
        _ => return None,
    };

    Some(Builder::plain(import))
}

/// Maps types whose modifier becomes builder options.
pub(super) fn parameterized(base: &str, lowered: &str) -> Builder {
    match base {
        "character varying" | "varchar" => options::sized("varchar", lowered),
        "character" | "bpchar" | "char" => options::sized("char", lowered),
        "numeric" | "decimal" => options::numeric(lowered),
        "timestamp without time zone" | "timestamp" => options::timestamp(lowered, false),
        "timestamp with time zone" | "timestamptz" => options::timestamp(lowered, true),
        "time with time zone" | "timetz" => {
            Builder::with_options("time", "{ withTimezone: true }".to_string())
        }
        // An unrecognized type is still usable as text rather than failing the pull.
        _ => Builder::plain("text"),
    }
}

/// Maps a sequence-backed integer to its serial builder.
///
/// Postgres reports a serial column as an integer with a `nextval` default, so the
/// distinction comes from the default, not the type name.
pub(super) fn serial(base: &str) -> Builder {
    match base {
        // bigserial needs an explicit mode, since a bigint exceeds a JS number.
        "bigint" => Builder::with_options("bigserial", "{ mode: 'number' }".to_string()),
        "smallint" => Builder::plain("smallserial"),
        _ => Builder::plain("serial"),
    }
}
