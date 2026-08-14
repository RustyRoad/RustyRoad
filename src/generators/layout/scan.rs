//! Finding the modules a preserved parent does not declare.

use super::Module;
use std::fs;
use std::path::Path;

/// Returns the modules the parent does not declare.
///
/// An unreadable parent yields nothing rather than reporting everything as missing: the
/// file was just written, so a read failure is an environment problem and not evidence
/// about its contents.
pub(super) fn missing<D>(root: &Path, modules: &[Module], declares: D) -> Vec<String>
where
    D: Fn(&str, &str) -> bool,
{
    let Ok(contents) = fs::read_to_string(root) else {
        return Vec::new();
    };

    modules
        .iter()
        .filter(|module| !declares(&contents, &module.name))
        .map(|module| module.name.clone())
        .collect()
}
