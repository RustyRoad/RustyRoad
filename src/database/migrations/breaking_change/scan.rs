use super::detector::detect;
use super::BreakingChangeFinding;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn scan_sql(path: &Path, sql: &str) -> Vec<BreakingChangeFinding> {
    detect(path, sql)
}

pub fn scan_migration(directory: &Path) -> Result<Vec<BreakingChangeFinding>, io::Error> {
    let path = directory.join("up.sql");
    let sql = fs::read_to_string(&path)?;
    Ok(scan_sql(&path, &sql))
}

pub fn scan_all_up_migrations(root: &Path) -> Result<Vec<BreakingChangeFinding>, io::Error> {
    let mut directories = migration_directories(root)?;
    directories.sort();
    let mut findings = Vec::new();
    for directory in directories {
        findings.extend(scan_migration(&directory)?);
    }
    Ok(findings)
}

pub fn migration_directories(root: &Path) -> Result<Vec<PathBuf>, io::Error> {
    fs::read_dir(root)?
        .filter_map(|entry| match entry {
            Ok(entry) if entry.path().is_dir() => Some(Ok(entry.path())),
            Ok(_) => None,
            Err(error) => Some(Err(error)),
        })
        .collect()
}
