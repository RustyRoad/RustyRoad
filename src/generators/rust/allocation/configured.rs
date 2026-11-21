//! Collision guards for opt-in configured input declarations.
use super::{reserved, Names};

/// Keep configured input types from shadowing allocated models and codec paths.
/// Existing canonical names are never renamed to accommodate new configuration.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
impl Names {
    /// Reject a configured input name already used by a model, helper, or codec.
    /// # Examples
    /// ```sh
    /// rustyroad pull --language rust --out crates/db/src
    /// ```
    pub(in crate::generators::rust) fn type_reserved(&self, name: &str) -> bool {
        reserved::TYPES.contains(&name)
            || self.types.values().any(|value| value == name)
            || self
                .codecs
                .values()
                .chain(self.column_codecs.values())
                .any(|codec| codec.rust.split("::").next() == Some(name))
    }
}
