use std::fs;
use std::path::{Path, PathBuf};

pub(super) const CREATE: &str = "CREATE TABLE example (id INTEGER);\n";

pub(super) fn migration(root: &Path, name: &str, source: &str) -> PathBuf {
    let directory = root.join(name);
    fs::create_dir(&directory).unwrap();
    fs::write(directory.join("up.sql"), source).unwrap();
    directory
}

pub(super) const NAME: &str = "20260101000000-example";
