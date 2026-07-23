use crate::database::migrations::{scan_all_up_migrations, scan_migration, BreakingChangeFinding};
use std::fs;
use std::io;
use std::path::Path;

const MIGRATIONS: &str = "./config/database/migrations";

pub(super) fn all() -> io::Result<Vec<BreakingChangeFinding>> {
    scan_all_up_migrations(Path::new(MIGRATIONS))
}

pub(super) fn named(name: &str) -> io::Result<Vec<BreakingChangeFinding>> {
    let mut findings = Vec::new();
    for entry in fs::read_dir(MIGRATIONS)? {
        let entry = entry?;
        if entry.path().is_dir() && migration_name(&entry.path()) == Some(name) {
            findings.extend(scan_migration(&entry.path())?);
        }
    }
    Ok(findings)
}

fn migration_name(path: &Path) -> Option<&str> {
    path.file_name()?
        .to_str()?
        .split_once('-')
        .map(|(_, name)| name)
}
