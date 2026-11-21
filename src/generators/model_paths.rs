//! Shared model output locations and project configuration.
use serde::Deserialize;
use std::path::{Path, PathBuf};
mod output;
mod resolve;
mod settings;
mod source;
pub(crate) use output::rust_output;
pub(crate) use resolve::{resolve, resolve_with};
pub(crate) use settings::settings;
pub(crate) use source::model_file;
use source::source_directory;

/// Resolved model directories and their explicit-configuration markers.
/// # Examples
/// ```sh
/// rustyroad model audit --offline --report-only
/// ```
pub(crate) struct Layout {
    pub source: PathBuf,
    pub source_explicit: bool,
    pub generated: Option<PathBuf>,
    pub generated_explicit: bool,
}
