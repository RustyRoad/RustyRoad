//! Malformed generic paths cannot establish known scalar codec identities.
use super::audits;

/// Example: `uuid::Uuid<i32>` is not the concrete PostgreSQL UUID codec.
#[test]
fn parameterized_scalars_and_namespaces_remain_unresolved() {
    for ty in [
        "uuid::Uuid<i32>",
        "sqlx::types::Uuid<i32>",
        "serde_json::Value<i32>",
        "chrono::NaiveDate<i32>",
        "chrono::DateTime<chrono::Utc<i32>>",
        "chrono::DateTime<chrono::FixedOffset<i32>>",
        "sqlx::types::time::PrimitiveDateTime<i32>",
        "rust_decimal::Decimal<i32>",
        "i32<i64>",
        "String<i32>",
        "uuid<i32>::Uuid",
        "sqlx::types<i32>::Uuid",
        "std<i32>::string::String",
        "std<i32>::option::Option<i32>",
        "std<i32>::vec::Vec<i32>",
        "Vec<u8<i32>>",
        "Vec<::u8>",
        "Vec<uuid::Uuid<i32>>",
        "Option<chrono::NaiveDate<i32>>",
        "Option<i32<i64>>",
        "::String",
        "::Vec<i32>",
        "<Other as Trait>::Uuid",
    ] {
        let source = format!("struct Row {{ id: i32, value: {ty} }}");
        syn::parse_file(&source).unwrap_or_else(|error| panic!("{ty}: {error}"));
        let result = audits(&source);
        let row = &result[0];
        assert!(row.proposal.is_none(), "{ty}");
        assert!(
            row.blockers
                .iter()
                .any(|reason| reason.contains("storage mapping")),
            "{ty}: {:?}",
            row.blockers
        );
    }
}

mod concrete;
