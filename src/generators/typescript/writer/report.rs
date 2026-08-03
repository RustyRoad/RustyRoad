//! What a `pull` run produced.

use super::ownership::Outcome;

/// The result of writing a folder.
pub struct Report {
    /// What happened to each file.
    pub outcomes: Vec<Outcome>,
    /// Tables whose procedures the preserved composition file does not reference.
    ///
    /// Their procedures exist but are unreachable until wired up by hand. Reporting
    /// this matters because nothing errors: every file looks current.
    pub unwired: Vec<String>,
}
