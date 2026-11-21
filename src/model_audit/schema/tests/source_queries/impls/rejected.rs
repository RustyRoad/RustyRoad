//! Extracted fixtures retain their original assertions and inputs.
use super::audit;

/// `Self`, generics, traits, conditional code and shadowing remain unproven.
#[test]
fn rejects_unsafe_repository_evidence() {
    for implementation in [
        r#"impl Repository { fn load() {
            sqlx::query_as::<_, Self>("SELECT * FROM sessions"); } }"#,
        r#"impl<T> Repository<T> { fn load() {
            sqlx::query_as::<_, Row>("SELECT * FROM sessions"); } }"#,
        r#"impl Loader for Repository { fn load() {
            sqlx::query_as::<_, Row>("SELECT * FROM sessions"); } }"#,
        r#"#[cfg(feature = "db")] impl Repository { fn load() {
            sqlx::query_as::<_, Row>("SELECT * FROM sessions"); } }"#,
        r#"impl Repository { #[cfg(feature = "db")] fn load() {
            sqlx::query_as::<_, Row>("SELECT * FROM sessions"); } }"#,
        r#"impl Repository { fn load<Row>() {
            sqlx::query_as::<_, Row>("SELECT * FROM sessions"); } }"#,
        r#"impl Repository { fn load(sqlx: Other) {
            sqlx::query_as::<_, Row>("SELECT * FROM sessions"); } }"#,
        r#"impl Repository { fn load() { if enabled() {
            sqlx::query_as::<_, Row>("SELECT * FROM sessions"); } } }"#,
        r#"impl Repository { fn load() {
            sqlx::query_as::<_, Row>("SELECT id FROM sessions"); } }"#,
    ] {
        let source = format!("struct Row {{ id: i32 }} {implementation}");
        assert_eq!(
            audit(&source, None).mapping,
            "unresolved_persistence_contract",
            "{source}"
        );
    }
}
