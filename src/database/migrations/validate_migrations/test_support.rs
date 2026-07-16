use crate::database::Database;
use std::fs;
use std::path::Path;

pub(super) fn write_migration(root: &Path, directory: &str, up_sql: &str) {
    let path = root.join(directory);
    fs::create_dir_all(&path).unwrap();
    fs::write(path.join("up.sql"), up_sql).unwrap();
    fs::write(path.join("down.sql"), "-- not used by validation").unwrap();
}

pub(super) fn sqlite_database(path: &Path) -> Database {
    Database::new(
        path.to_string_lossy().into_owned(),
        String::new(),
        String::new(),
        String::new(),
        0,
        "sqlite",
    )
}
