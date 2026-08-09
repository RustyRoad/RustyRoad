//! File ownership: what `pull` may overwrite and what it must preserve.
//!
//! Regenerating over a file a developer has edited loses work, so every emitted
//! file declares who owns it. Derived files are rewritten on every run; scaffold
//! files are written once and then left alone, because they are the intended place
//! to add hand-written code.

use std::path::Path;

pub use crate::generators::report::Outcome;

/// Who owns an emitted file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ownership {
    /// Derived from the database. Rewritten on every run.
    ///
    /// Editing one of these is always a mistake, since the next `pull` reverts it.
    Generated,
    /// Written once as a starting point, then owned by the developer.
    ///
    /// `pull` skips these when they already exist, so composing generated
    /// procedures with hand-written ones survives regeneration.
    Scaffold,
}

impl Ownership {
    /// Returns `true` when the file should be written.
    ///
    /// A scaffold file is written only when absent, unless the caller forces it.
    pub fn should_write(self, path: &Path, force: bool) -> bool {
        match self {
            Self::Generated => true,
            Self::Scaffold => force || !path.exists(),
        }
    }
}
