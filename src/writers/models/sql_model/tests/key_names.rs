use super::*;

#[test]
fn mysql_and_sqlite_key_names_are_case_insensitive() {
    for database in [DatabaseType::Mysql, DatabaseType::Sqlite] {
        let actual = fields("CREATE TABLE t (ID INTEGER, PRIMARY KEY(id))", database);
        assert!(!actual[0].1.starts_with("Option<"));
    }
}

#[test]
fn sqlite_inline_primary_key_names_ignore_case() {
    let actual = fields(
        "CREATE TABLE t (ID INTEGER PRIMARY KEY)",
        DatabaseType::Sqlite,
    );
    assert_eq!(actual[0].1, "i64");
}
