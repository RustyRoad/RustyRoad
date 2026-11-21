use super::*;

#[test]
fn postgres_aliases_are_required_without_explicit_not_null() {
    let actual = fields(
        "CREATE TABLE t (a SERIAL2, b SERIAL4, c SERIAL8)",
        DatabaseType::Postgres,
    );
    assert_eq!(
        actual
            .iter()
            .map(|field| field.1.as_str())
            .collect::<Vec<_>>(),
        ["i16", "i32", "i64"]
    );
}
