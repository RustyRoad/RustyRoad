use super::*;

#[test]
fn postgres_identity_columns_are_required_without_a_primary_key() {
    for generation in ["ALWAYS", "BY DEFAULT"] {
        let sql = format!("CREATE TABLE t (id BIGINT GENERATED {generation} AS IDENTITY)");
        assert_eq!(fields(&sql, DatabaseType::Postgres)[0].1, "i64");
    }
}

#[test]
fn mysql_auto_increment_columns_are_required_without_explicit_not_null() {
    let sql = "CREATE TABLE t (id BIGINT AUTO_INCREMENT UNIQUE)";
    assert_eq!(fields(sql, DatabaseType::Mysql)[0].1, "i64");
}

#[test]
fn generated_expressions_do_not_implicitly_require_columns() {
    let sql = "CREATE TABLE t (a INT, b INT GENERATED ALWAYS AS (a + 1) STORED)";
    assert_eq!(fields(sql, DatabaseType::Postgres)[1].1, "Option<i32>");
}
