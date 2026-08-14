//! Type mapping for timestamps and arrays.

use super::types_test::map;

#[test]
fn a_timestamp_keeps_its_zone_awareness() {
    assert_eq!(
        map("created_at", "timestamp without time zone", false),
        "chrono::NaiveDateTime"
    );
    // Collapsing timestamptz to NaiveDateTime would silently drop the offset.
    assert_eq!(
        map("created_at", "timestamp with time zone", false),
        "chrono::DateTime<chrono::Utc>"
    );
}

#[test]
fn a_precision_modifier_mid_type_still_resolves() {
    // `timestamp(3) with time zone` puts the modifier in the middle, so trimming at the
    // first parenthesis would leave ` with time zone` behind and miss the match.
    assert_eq!(
        map("at", "timestamp(3) with time zone", false),
        "chrono::DateTime<chrono::Utc>"
    );
}

#[test]
fn an_array_becomes_a_vec_of_its_element_type() {
    assert_eq!(map("attributes", "text[]", false), "Vec<String>");
    assert_eq!(map("scores", "integer[]", false), "Vec<i32>");
    // The catalog spelling must resolve the same way as the format_type spelling.
    assert_eq!(map("scores", "_int4", false), "Vec<i32>");
}

#[test]
fn a_nullable_array_is_an_option_of_vec() {
    assert_eq!(map("images", "text[]", true), "Option<Vec<String>>");
}
