use rustyroad::database::{Database, DatabaseType};

#[test]
fn database_new_parses_sqlite_type() {
    let database = Database::new(
        "pwned".to_owned(),
        "test_user".to_owned(),
        "password".to_owned(),
        "localhost".to_owned(),
        3306,
        "sqlite",
    );

    assert_eq!(database.database_type, DatabaseType::Sqlite);
}
