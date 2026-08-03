//! A column as exposed by a schema version.

/// A column exposed by a version, and the physical column backing it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Column {
    /// Logical name exposed to clients.
    pub name: String,
    /// Physical column in the underlying table.
    pub physical_name: String,
    /// Excluded from the view when true.
    pub deleted: bool,
    /// Default expression to restate on the view, if any.
    pub default: Option<String>,
}

impl Column {
    /// Builds a column whose logical and physical names match.
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            physical_name: name.clone(),
            name,
            deleted: false,
            default: None,
        }
    }

    /// Points the logical name at a different physical column.
    pub fn backed_by(mut self, physical_name: impl Into<String>) -> Self {
        self.physical_name = physical_name.into();
        self
    }

    /// Marks the column as removed from this version.
    pub fn deleted(mut self) -> Self {
        self.deleted = true;
        self
    }

    /// Sets a default expression restated on the view.
    pub fn with_default(mut self, default: impl Into<String>) -> Self {
        self.default = Some(default.into());
        self
    }
}
