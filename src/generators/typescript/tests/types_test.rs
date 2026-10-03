//! Scalar and serial type mapping.

use crate::generators::typescript::types::map;

#[test]
fn integers_map_to_their_builders() {
    assert_eq!(map("integer", false).import, "integer");
    assert_eq!(map("smallint", false).import, "smallint");
    let bigint = map("bigint", false);
    assert_eq!(bigint.import, "bigint");
    assert_eq!(bigint.options.as_deref(), Some("{ mode: 'number' }"));

    let int8 = map("int8", false);
    assert_eq!(int8.import, "bigint");
    assert_eq!(int8.options.as_deref(), Some("{ mode: 'number' }"));
}

#[test]
fn serial_columns_are_detected_by_their_sequence_default() {
    // Postgres reports a serial column as integer; the sequence default is what
    // distinguishes it, so the builder must come from auto_increment.
    assert_eq!(map("integer", true).import, "serial");
    assert_eq!(map("smallint", true).import, "smallserial");
}

#[test]
fn bigserial_states_its_mode() {
    // A bigint exceeds a JS number, so Drizzle requires an explicit mode.
    let builder = map("bigint", true);

    assert_eq!(builder.import, "bigserial");
    assert_eq!(builder.options.as_deref(), Some("{ mode: 'number' }"));
}

#[test]
fn common_scalars_map_directly() {
    assert_eq!(map("uuid", false).import, "uuid");
    assert_eq!(map("jsonb", false).import, "jsonb");
    assert_eq!(map("boolean", false).import, "boolean");
    assert_eq!(map("double precision", false).import, "doublePrecision");
    assert_eq!(map("text", false).import, "text");
}

#[test]
fn unknown_types_fall_back_to_text() {
    // An exotic type should not abort the pull.
    assert_eq!(map("tsvector", false).import, "text");
}
