//! Timestamp seeding must match the column's own type.
//!
//! `created_at`/`updated_at` are seeded from the clock, but the seed was hardcoded to
//! `Utc::now().naive_utc()` regardless of what the column actually is. A `timestamptz`
//! column resolves to `DateTime<Utc>`, so the seed was the wrong type and both `Default` and
//! `new` failed to compile — 1638 errors across the live schema.

use super::timestamp_fixture::root;

#[test]
fn a_timestamptz_column_is_seeded_with_an_offset_aware_value() {
    let root = root("timestamp with time zone", false);

    // The column is DateTime<Utc>, so a NaiveDateTime seed does not type-check.
    assert!(
        root.contains("created_at: Utc::now(),"),
        "expected an offset-aware seed:\n{root}"
    );
    assert!(!root.contains("Utc::now().naive_utc()"), "got:\n{root}");
}

#[test]
fn a_naive_timestamp_column_is_seeded_with_a_naive_value() {
    let root = root("timestamp without time zone", false);

    assert!(
        root.contains("created_at: Utc::now().naive_utc(),"),
        "expected a naive seed:\n{root}"
    );
}

#[test]
fn a_nullable_timestamptz_is_wrapped_in_some() {
    let root = root("timestamp with time zone", true);

    assert!(
        root.contains("created_at: Some(Utc::now()),"),
        "expected a wrapped offset-aware seed:\n{root}"
    );
}

#[test]
fn the_import_matches_the_seed_the_file_uses() {
    // `Utc::now()` needs Utc in scope either way, so the import must not depend on which seed
    // was chosen.
    for sql_type in ["timestamp with time zone", "timestamp without time zone"] {
        let root = root(sql_type, false);
        assert!(
            root.contains("use chrono::Utc;"),
            "{sql_type} lost its import:\n{root}"
        );
    }
}
