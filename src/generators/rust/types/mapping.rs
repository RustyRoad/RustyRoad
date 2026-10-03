//! The resolved type for one column.

/// A resolved Rust type for one column.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mapping {
    /// The type as written in the struct, `Option<_>` included when nullable.
    pub rust: String,
    /// `ts_rs` override for a type it cannot derive, if one is needed.
    ///
    /// `ts_rs` cannot see through `chrono` or `serde_json`, so those fields declare what
    /// they serialize as; without it the exported TypeScript fails to compile.
    pub ts: Option<&'static str>,
}

impl Mapping {
    /// Builds a mapping `ts_rs` understands unaided.
    pub(super) fn plain(rust: impl Into<String>) -> Self {
        Self {
            rust: rust.into(),
            ts: None,
        }
    }

    /// Builds a mapping carrying a `ts_rs` type override.
    pub(super) fn overridden(rust: impl Into<String>, ts: &'static str) -> Self {
        Self {
            rust: rust.into(),
            ts: Some(ts),
        }
    }

    /// Wraps the type in `Option`, preserving any override.
    pub(super) fn optional(self) -> Self {
        Self {
            rust: format!("Option<{}>", self.rust),
            ts: self.ts,
        }
    }

    /// Wraps the type in `Vec`, preserving any override.
    pub(super) fn collected(self) -> Self {
        Self {
            rust: format!("Vec<{}>", self.rust),
            ts: self.ts,
        }
    }

    /// Returns `true` when this mapping resolves to the type named `name`.
    ///
    /// Compared against the innermost type so an `Option<T>` or `Vec<T>` still reports the
    /// `T` it wraps: an enum column that happens to be nullable still needs its
    /// declaration emitted.
    pub fn names(&self, name: &str) -> bool {
        self.inner() == name
    }

    /// Returns the type with any `Option`/`Vec` wrappers stripped.
    fn inner(&self) -> &str {
        let mut current = self.rust.as_str();

        loop {
            let stripped = current
                .strip_prefix("Option<")
                .or_else(|| current.strip_prefix("Vec<"));

            match stripped.and_then(|rest| rest.strip_suffix('>')) {
                Some(rest) => current = rest,
                None => return current,
            }
        }
    }
}
