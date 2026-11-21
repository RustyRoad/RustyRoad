use super::*;

#[test]
fn table_level_descending_integer_primary_key_is_a_rowid_alias() {
    assert_eq!(
        fields(
            "CREATE TABLE t (id INTEGER, PRIMARY KEY (id DESC))",
            DatabaseType::Sqlite
        )[0]
        .1,
        "i64"
    );
}

#[test]
fn unrelated_desc_text_does_not_disable_integer_rowid_aliases() {
    for definition in [
        "id INTEGER PRIMARY KEY DEFAULT 'DESC'",
        "id INTEGER CONSTRAINT \"DESC\" PRIMARY KEY",
        "id INTEGER PRIMARY KEY CHECK ('DESC' <> '')",
    ] {
        assert_eq!(
            fields(
                &format!("CREATE TABLE t ({definition})"),
                DatabaseType::Sqlite,
            )[0]
            .1,
            "i64",
            "{definition}"
        );
    }
}
