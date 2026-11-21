use super::*;

#[test]
fn unquoted_names_fold_before_primary_key_matching() {
    let actual = fields(
        "CREATE TABLE T (ID INT, PRIMARY KEY(id))",
        DatabaseType::Postgres,
    );
    assert_eq!(actual, [("id".into(), "i32".into())]);
    let actual = fields(
        r#"CREATE TABLE t ("ID" INT, PRIMARY KEY("ID"))"#,
        DatabaseType::Postgres,
    );
    assert_eq!(actual, [("id".into(), "i32".into())]);
}

#[test]
fn quoted_names_remain_case_sensitive() {
    assert_invalid(
        r#"CREATE TABLE t ("ID" INT, PRIMARY KEY(id))"#,
        DatabaseType::Postgres,
    );
}
