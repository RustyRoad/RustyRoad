//! Value-kind mapping for scalars and text-like types.

use super::support::{from_tables, required};
use crate::generators::tetherscript::types::{self, Kind};

/// Maps one column against a schema with no enums.
pub(super) fn map(sql_type: &str) -> Kind {
    types::map(&required("value", sql_type), &from_tables(Vec::new()))
}

#[test]
fn integers_and_floats_are_distinguished() {
    assert_eq!(map("integer"), Kind::Int);
    assert_eq!(map("bigint"), Kind::Int);
    assert_eq!(map("smallint"), Kind::Int);
    assert_eq!(map("double precision"), Kind::Float);
    assert_eq!(map("real"), Kind::Float);
}

#[test]
fn numeric_is_a_float_because_the_wire_format_is_text() {
    // The native client decodes by shape, so a decimal literal arrives as a float.
    // Claiming an exact decimal here would describe a type the runtime never produces.
    assert_eq!(map("numeric(12,2)"), Kind::Float);
}

#[test]
fn text_like_types_are_strings() {
    assert_eq!(map("text"), Kind::Str);
    assert_eq!(map("character varying(255)"), Kind::Str);
    assert_eq!(map("uuid"), Kind::Str);
    // A timestamp crosses the wire as text, so it is a string and not a number.
    assert_eq!(map("timestamp without time zone"), Kind::Str);
    assert_eq!(map("timestamp with time zone"), Kind::Str);
}

#[test]
fn booleans_are_their_own_kind() {
    assert_eq!(map("boolean"), Kind::Bool);
}

#[test]
fn a_precision_modifier_mid_type_still_resolves() {
    assert_eq!(map("timestamp(3) with time zone"), Kind::Str);
}
