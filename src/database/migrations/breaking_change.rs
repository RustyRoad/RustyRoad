mod detector;
mod finding;
mod patterns;
mod scan;

#[cfg(test)]
mod detector_test;

pub use finding::{BreakingChangeFinding, BreakingChangeKind};
pub use scan::{scan_all_up_migrations, scan_migration, scan_sql};

pub fn has_breaking_changes(findings: &[BreakingChangeFinding]) -> bool {
    !findings.is_empty()
}
