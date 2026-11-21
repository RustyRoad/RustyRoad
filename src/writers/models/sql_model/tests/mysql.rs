use super::*;

#[test]
fn one_line_mysql_table_preserves_unsigned_types_and_nullability() {
    let actual = fields("CREATE TABLE `orders` (`id` BIGINT UNSIGNED PRIMARY KEY, qty SMALLINT UNSIGNED NOT NULL, note VARCHAR(100), cost DECIMAL(10,2), payload JSON, stamp DATETIME(6))", DatabaseType::Mysql);
    assert_eq!(
        actual,
        vec![
            ("id".into(), "u64".into()),
            ("qty".into(), "u16".into()),
            ("note".into(), "Option<String>".into()),
            ("cost".into(), "Option<sqlx::types::Decimal>".into()),
            ("payload".into(), "Option<serde_json::Value>".into()),
            ("stamp".into(), "Option<chrono::NaiveDateTime>".into()),
        ]
    );
}

#[test]
fn table_constraints_do_not_become_fields() {
    let actual = fields("CREATE TABLE child (id INT, parent_id INT, PRIMARY KEY (id), FOREIGN KEY (parent_id) REFERENCES parent(id), UNIQUE(parent_id))", DatabaseType::Mysql);
    assert_eq!(
        actual,
        vec![
            ("id".into(), "i32".into()),
            ("parent_id".into(), "Option<i32>".into()),
        ]
    );
}
