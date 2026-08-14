//! Locating and driving the real `tetherscript` binary.

use std::path::Path;
use std::process::Command;

/// Returns the `tetherscript` binary, or `None` when it is not installed.
///
/// A RustyRoad checkout cannot assume a separate project is installed, so the tests that
/// use this skip rather than fail when it is missing.
pub(super) fn binary() -> Option<String> {
    let candidate =
        std::env::var("TETHERSCRIPT_BIN").unwrap_or_else(|_| "tetherscript".to_string());

    Command::new(&candidate)
        .arg("--help")
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|_| candidate)
}

/// Runs one subcommand over `path`, returning success and combined output.
///
/// The working directory is set explicitly rather than inherited. Other tests in this
/// crate change and remove the process's directory, and a spawned child inherits it —
/// which surfaces as `can't read current directory` from a binary that never looked at
/// the path we passed.
pub(super) fn run(binary: &str, subcommand: &str, path: &Path, directory: &Path) -> (bool, String) {
    let output = Command::new(binary)
        .arg(subcommand)
        .arg(path)
        .current_dir(directory)
        .output()
        .expect("tetherscript should run");

    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    (output.status.success(), combined)
}
