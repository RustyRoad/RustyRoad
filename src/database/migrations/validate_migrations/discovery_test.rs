use super::discovery::discover;
use super::test_support::write_migration;
use std::fs;

#[test]
fn migrations_are_sorted() {
    let root = tempfile::tempdir().unwrap();
    write_migration(root.path(), "20240202-second", "SELECT 2;");
    write_migration(root.path(), "20240101-first", "SELECT 1;");
    let migrations = discover(root.path()).unwrap();
    let names: Vec<_> = migrations.iter().map(|item| item.name.as_str()).collect();
    assert_eq!(names, ["20240101-first", "20240202-second"]);
}

#[test]
fn missing_up_sql_is_rejected() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("20240101-incomplete");
    fs::create_dir_all(&path).unwrap();
    fs::write(path.join("down.sql"), "SELECT 1;").unwrap();
    let error = discover(root.path()).unwrap_err().to_string();
    assert!(error.contains("missing"));
    assert!(error.contains("up.sql"));
}
