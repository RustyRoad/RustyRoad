//! Verify declaration ownership through the crate's actual module graph.
mod search;
use super::{root, Context};
use crate::model_audit::types::Model;

/// Construct source context only when exactly one declared module owns the file.
/// # Examples
/// ```sh
/// rustyroad model audit --source src/models --offline --report-only
/// ```
impl Context {
    /// Follow ordinary and literal-path module declarations, then inline scopes.
    /// Duplicate ownership and bounded-search failures remain unresolved.
    /// # Examples
    /// ```sh
    /// rustyroad model audit --source src/models --offline --report-only
    /// ```
    pub(in crate::model_audit::schema::rust_types::imports) fn for_model(
        model: &Model,
    ) -> Option<Self> {
        let root = root::for_file(&model.path)?.canonicalize().ok()?;
        let target = model.path.canonicalize().ok()?;
        let modules = search::find(&root, &target)?;
        let mut inline: Vec<_> = model.id.split("::").skip(1).map(str::to_owned).collect();
        inline.pop();
        let context = Self {
            root,
            modules,
            inline,
        };
        if context.node(&context.modules)?.file != target {
            return None;
        }
        Some(context)
    }
}
