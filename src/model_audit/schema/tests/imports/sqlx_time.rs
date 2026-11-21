//! SQLx time re-exports have known PostgreSQL codecs; unrelated names do not.
use super::audits;

/// Example: an imported SQLx `PrimitiveDateTime` maps to a zone-free timestamp.
#[test]
fn sqlx_time_imports_preserve_known_storage_and_option_nullability() {
    let audit = audits(
        r#"
        use sqlx::types::time::{Date, Time, PrimitiveDateTime, OffsetDateTime};
        struct Event {
            date: Date,
            time: Time,
            local: Option<PrimitiveDateTime>,
            zoned: OffsetDateTime,
            dates: Vec<Date>,
        }
        "#,
    );
    assert!(audit[0].blockers.is_empty(), "{:?}", audit[0].blockers);
    for (name, sql, nullable) in [
        ("date", "date", false),
        ("time", "time without time zone", false),
        ("local", "timestamp without time zone", true),
        ("zoned", "timestamp with time zone", false),
        ("dates", "date[]", false),
    ] {
        let column = audit[0]
            .desired_columns
            .iter()
            .find(|column| column.name == name)
            .unwrap();
        assert_eq!(column.sql_type, sql);
        assert_eq!(column.nullable, nullable);
    }
    assert!(audit[0].proposal.is_some());
}

/// Example: `custom::PrimitiveDateTime` is not evidence of a SQLx time codec.
#[test]
fn custom_time_names_still_require_explicit_storage_mappings() {
    let audit = audits("use custom::PrimitiveDateTime; struct Event { at: PrimitiveDateTime }");
    assert!(audit[0].proposal.is_none());
    assert!(audit[0]
        .blockers
        .iter()
        .any(|error| error.contains("storage mapping")));
}

mod qualified;
