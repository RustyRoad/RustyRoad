//! Common reporting types for database code generators.

use std::path::{Path, PathBuf};

/// What happened to one generated file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// The file was written.
    Written(PathBuf),
    /// The file already existed and is owned by the developer.
    Preserved(PathBuf),
}

impl Outcome {
    /// Returns the path this outcome refers to.
    pub fn path(&self) -> &Path {
        match self {
            Self::Written(path) | Self::Preserved(path) => path,
        }
    }

    /// Returns `true` when the file was left untouched.
    pub fn is_preserved(&self) -> bool {
        matches!(self, Self::Preserved(_))
    }
}

/// The result of writing a generated database folder.
pub struct Report {
    /// What happened to each file.
    pub outcomes: Vec<Outcome>,
    /// Tables generated but not exposed by a developer-owned composition file.
    pub unwired: Vec<String>,
}
