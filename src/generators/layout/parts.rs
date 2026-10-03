//! The pieces a layout is described by and reports on.

use crate::generators::ownership::{Outcome, Ownership};

/// What a run produced.
pub struct Layout {
    /// What happened to each file.
    pub outcomes: Vec<Outcome>,
    /// Models the preserved parent module does not declare.
    ///
    /// Their files exist but are unreachable, and nothing errors: an undeclared Rust
    /// module is never compiled, and an unimported TetherScript module is never loaded.
    pub undeclared: Vec<String>,
}

/// One model's folder: its name and the files inside it.
pub struct Module {
    /// Folder name, and the identifier the parent declares.
    pub name: String,
    pub files: Vec<File>,
    /// True when the module declares public types beyond the model itself.
    ///
    /// Drives whether the parent re-exports the module by name or by glob: a model with
    /// generated enum types has more than one name worth reaching from the parent, and
    /// listing them individually would mean the parent has to know what they are.
    pub has_extra_types: bool,
}

/// One file to emit.
pub struct File {
    pub name: &'static str,
    pub ownership: Ownership,
    pub contents: String,
}
