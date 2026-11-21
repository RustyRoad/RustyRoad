//! Extracted fixtures retain their original assertions and inputs.
use super::audits;

/// Example: the fully qualified onboarding expiry type retains its nullability.
#[test]
fn qualified_sqlx_time_and_chrono_paths_preserve_known_codecs() {
    let audit = audits(
        r#"struct Session {
            expires_at: Option<sqlx::types::time::PrimitiveDateTime>,
            created_at: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
            date: sqlx::types::chrono::NaiveDate,
            time: sqlx::types::chrono::NaiveTime,
            local: sqlx::types::chrono::NaiveDateTime,
        }"#,
    );
    assert!(audit[0].blockers.is_empty(), "{:?}", audit[0].blockers);
    for (name, sql, nullable) in [
        ("expires_at", "timestamp without time zone", true),
        ("created_at", "timestamp with time zone", false),
        ("date", "date", false),
        ("time", "time without time zone", false),
        ("local", "timestamp without time zone", false),
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
