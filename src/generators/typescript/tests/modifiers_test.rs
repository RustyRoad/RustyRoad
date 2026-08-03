//! Type modifiers carried into builder options.

use crate::generators::typescript::types::map;

#[test]
fn varchar_carries_its_length() {
    let builder = map("character varying(255)", false);

    assert_eq!(builder.import, "varchar");
    assert_eq!(builder.options.as_deref(), Some("{ length: 255 }"));
}

#[test]
fn unbounded_varchar_has_no_length() {
    let builder = map("character varying", false);

    assert_eq!(builder.import, "varchar");
    assert_eq!(builder.options, None);
}

#[test]
fn numeric_carries_precision_and_scale() {
    let builder = map("numeric(10,2)", false);

    assert_eq!(builder.import, "numeric");
    assert_eq!(
        builder.options.as_deref(),
        Some("{ precision: 10, scale: 2 }")
    );
}

#[test]
fn numeric_without_scale_states_precision_only() {
    assert_eq!(
        map("numeric(10)", false).options.as_deref(),
        Some("{ precision: 10 }")
    );
}

#[test]
fn timestamps_carry_mode_precision_and_timezone() {
    let plain = map("timestamp without time zone", false);
    assert_eq!(plain.import, "timestamp");
    assert_eq!(plain.options.as_deref(), Some("{ mode: 'string' }"));

    // The modifier sits mid-type here, so stripping must not lose the suffix.
    let zoned = map("timestamp(3) with time zone", false);
    assert_eq!(
        zoned.options.as_deref(),
        Some("{ mode: 'string', precision: 3, withTimezone: true }")
    );
}

#[test]
fn timetz_states_its_timezone() {
    assert_eq!(
        map("time with time zone", false).options.as_deref(),
        Some("{ withTimezone: true }")
    );
}
