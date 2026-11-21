use super::*;

#[test]
fn malformed_or_ambiguous_sql_is_rejected() {
    for sql in [
        "",
        "CREATE TABLE",
        "CREATE TABLE t (",
        "SELECT 1",
        "CREATE TABLE t (id INT); CREATE TABLE u (id INT)",
        "CREATE TABLE t AS SELECT 1 AS id",
        "CREATE TABLE t (id INT); ALTER TABLE t ADD COLUMN x TEXT",
        "CREATE TABLE t (id INT, PRIMARY KEY (missing))",
    ] {
        assert_invalid(sql, DatabaseType::Postgres);
    }
}

#[test]
fn unsupported_types_are_not_guessed() {
    for (sql, database) in [
        ("CREATE TABLE t (p polygon)", DatabaseType::Postgres),
        ("CREATE TABLE t (p custom_type)", DatabaseType::Postgres),
        ("CREATE TABLE t (p ENUM('one', 'two'))", DatabaseType::Mysql),
        ("CREATE TABLE t (p NUMERIC)", DatabaseType::Sqlite),
    ] {
        assert_invalid(sql, database);
    }
}

#[test]
fn mongodb_is_explicitly_unsupported() {
    assert_invalid("CREATE TABLE t (id INT)", DatabaseType::Mongo);
}

#[test]
fn duplicate_and_unusable_fields_are_rejected() {
    for sql in [
        "CREATE TABLE t (id INT, id INT)",
        "CREATE TABLE t (\"first-name\" TEXT, first_name TEXT)",
        "CREATE TABLE t (ID INT, id INT)",
        "CREATE TABLE t (\"\" TEXT)",
    ] {
        assert_invalid(sql, DatabaseType::Postgres);
    }
}
