//! Extracted fixtures retain their original assertions and inputs.
use super::audits;

#[test]
fn unproven_contracts_remain_blocked() {
    for declaration in [
        r#"#[derive(sqlx::Type)] #[sqlx(type_name = "state", transparent)] enum State { A }"#,
        r#"#[derive(sqlx::Type)] #[sqlx(type_name = "state", rename_all = "invalid")] enum State { A }"#,
        r#"#[derive(sqlx::Type)] #[sqlx(type_name = "state", rename_all = "uppercase")] enum State { A }"#,
        r#"#[derive(sqlx::Type)] #[sqlx(type_name = "state", rename_all = "snake_case", rename_all = "lowercase")] enum State { A }"#,
        r#"#[cfg(feature = "codec")] #[derive(sqlx::Type)] #[sqlx(type_name = "state")] enum State { A }"#,
        r#"#[derive(sqlx::Type)] #[sqlx(type_name = "unsafe; DROP TABLE rows")] enum State { A }"#,
        r#"struct State; impl sqlx::Type<sqlx::Postgres> for State {
            fn type_info() -> sqlx::postgres::PgTypeInfo {
                if true { sqlx::postgres::PgTypeInfo::with_name("state") }
                else { sqlx::postgres::PgTypeInfo::with_name("other") }
            }
        }"#,
        r#"struct State; #[cfg(feature = "codec")]
        impl sqlx::Type<sqlx::Postgres> for State {
            fn type_info() -> sqlx::postgres::PgTypeInfo {
                sqlx::postgres::PgTypeInfo::with_name("state")
            }
        }"#,
    ] {
        let source = format!("{declaration} struct Row {{ id: i32, state: State }}");
        let result = audits(&source);
        let row = result.last().unwrap();
        assert!(row.proposal.is_none(), "{declaration}");
        assert!(row
            .blockers
            .iter()
            .any(|reason| reason.contains("no PostgreSQL storage mapping")));
    }
}
