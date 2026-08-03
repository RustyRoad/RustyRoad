//! Shared scratch-directory helper for writer tests.

use crate::database::introspection::Schema;
use crate::generators::typescript::writer::{Outcome, Report};
use crate::generators::typescript::{write, Casing, Outputs};
use std::path::PathBuf;

/// Creates a unique scratch directory.
///
/// Rooted in the OS temp directory rather than `target/`, which is not guaranteed
/// to exist: a fresh CI checkout runs tests from a `cargo` invocation that may not
/// have created it relative to the process's working directory.
///
/// The process id and label together keep concurrent test binaries from colliding.
pub(super) fn scratch(label: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "rustyroad-writer-test-{label}-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&path);
    std::fs::create_dir_all(&path).expect("failed to create scratch directory");
    path
}

/// Writes `schema` into `out`, returning the full report.
pub(super) fn pull(out: &PathBuf, schema: &Schema, force: bool) -> Report {
    write(out, schema, Casing::Camel, Outputs::all(), force).expect("write should succeed")
}

/// Returns the outcome for one file name.
pub(super) fn outcome_for<'a>(outcomes: &'a [Outcome], name: &str) -> &'a Outcome {
    outcomes
        .iter()
        .find(|outcome| outcome.path().ends_with(name))
        .unwrap_or_else(|| panic!("{name} should be emitted"))
}
