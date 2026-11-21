use super::*;

#[test]
fn ordinary_primary_keys_are_nullable_except_integer_rowid_aliases() {
    let actual = fields("CREATE TABLE t (id INTEGER PRIMARY KEY, title TEXT NOT NULL, active BOOLEAN, payload BLOB, score REAL)", DatabaseType::Sqlite);
    assert_eq!(
        actual,
        vec![
            ("id".into(), "i64".into()),
            ("title".into(), "String".into()),
            ("active".into(), "Option<bool>".into()),
            ("payload".into(), "Option<Vec<u8>>".into()),
            ("score".into(), "Option<f64>".into()),
        ]
    );
    assert_eq!(
        fields("CREATE TABLE t (id TEXT PRIMARY KEY)", DatabaseType::Sqlite)[0].1,
        "Option<String>"
    );
    assert_eq!(
        fields(
            "CREATE TABLE t (id INTEGER PRIMARY KEY DESC)",
            DatabaseType::Sqlite
        )[0]
        .1,
        "Option<i64>"
    );
}

#[test]
fn strict_and_without_rowid_primary_keys_are_required() {
    for suffix in ["STRICT", "WITHOUT ROWID"] {
        let sql = format!("CREATE TABLE t (id TEXT PRIMARY KEY) {suffix}");
        assert_eq!(fields(&sql, DatabaseType::Sqlite)[0].1, "String");
    }
}

#[test]
fn ordinary_composite_primary_keys_can_contain_null() {
    let actual = fields(
        "CREATE TABLE t (a INTEGER, b TEXT, PRIMARY KEY(a,b))",
        DatabaseType::Sqlite,
    );
    assert_eq!(actual[0].1, "Option<i64>");
    assert_eq!(actual[1].1, "Option<String>");
}
