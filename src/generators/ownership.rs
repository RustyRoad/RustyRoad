//! File ownership shared by every code generator.
//!
//! Regenerating over a file a developer has edited loses work, so every emitted
//! file declares who owns it. Derived files are rewritten on every run; scaffold
//! files are written once and then left alone, because they are the intended place
//! to add hand-written code.
//!
//! This lives above the individual language generators because the rule is a
//! property of generation itself, not of the language being emitted: a Rust model
//! and a TypeScript router both need the same guarantee.

use std::path::Path;
use std::{fs, io};

pub use crate::generators::report::Outcome;

/// Who owns an emitted file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ownership {
    /// Derived from the database. Rewritten on every run.
    ///
    /// Editing one of these is always a mistake, since the next run reverts it.
    Generated,
    /// Written once as a starting point, then owned by the developer.
    ///
    /// Generation skips these when they already exist, so hand-written code that
    /// extends the generated surface survives regeneration.
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

/// Writes one file unless the developer owns it and it already exists.
pub fn write_owned(
    directory: &Path,
    name: &str,
    contents: &str,
    ownership: Ownership,
    force: bool,
) -> io::Result<Outcome> {
    let path = directory.join(name);

    if !ownership.should_write(&path, force) {
        return Ok(Outcome::Preserved(path));
    }

    fs::write(&path, contents)?;
    Ok(Outcome::Written(path))
}
