use crate::database::migrations::{migration_directories, scan_migration, BreakingChangeFinding};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const MIGRATIONS: &str = "./config/database/migrations";

/// Scans every migration that is not already recorded as applied.
///
/// Migrations the ledger already records are skipped: they will be no-ops when the
/// apply command reaches them, so warning about them (and blocking on the warning)
/// would stop work that was never going to run.
pub(super) async fn all() -> io::Result<Vec<BreakingChangeFinding>> {
    let mut directories = migration_directories(Path::new(MIGRATIONS))?;
    directories.sort();

    let mut findings = Vec::new();
    for directory in super::pending::unapplied(directories).await {
        findings.extend(scan_migration(&directory)?);
    }
    Ok(findings)
}

/// Scans a single named migration, skipping it when already applied.
pub(super) async fn named(name: &str) -> io::Result<Vec<BreakingChangeFinding>> {
    let matched = directories_for(name)?;

    let mut findings = Vec::new();
    for directory in super::pending::unapplied(matched).await {
        findings.extend(scan_migration(&directory)?);
    }
    Ok(findings)
}

/// Scans a single named migration without consulting the ledger.
pub(super) fn named_on_disk(name: &str) -> io::Result<Vec<BreakingChangeFinding>> {
    let mut findings = Vec::new();
    for directory in directories_for(name)? {
        findings.extend(scan_migration(&directory)?);
    }
    Ok(findings)
}

/// Returns the migration directories matching `name`, in stable order.
///
/// Accepts both the bare name and the full `<timestamp>-<name>` directory name.
fn directories_for(name: &str) -> io::Result<Vec<PathBuf>> {
    let mut matched: Vec<PathBuf> = fs::read_dir(MIGRATIONS)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir() && matches_name(path, name))
        .collect();
    matched.sort();
    Ok(matched)
}

/// Returns `true` when `path` identifies the migration `name`.
fn matches_name(path: &Path, name: &str) -> bool {
    let Some(dir_name) = path.file_name().and_then(|value| value.to_str()) else {
        return false;
    };
    dir_name == name || dir_name.split_once('-').map(|(_, rest)| rest) == Some(name)
}
