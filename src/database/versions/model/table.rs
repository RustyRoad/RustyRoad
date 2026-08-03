//! Tables and schema snapshots exposed by a version.

use super::Column;

/// A table as exposed by a schema version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    /// Logical name exposed to clients.
    pub name: String,
    /// Physical table backing the view.
    pub physical_name: String,
    /// Excluded from the version when true.
    pub deleted: bool,
    pub columns: Vec<Column>,
}

impl Table {
    /// Builds a table whose logical and physical names match.
    pub fn new(name: impl Into<String>, columns: Vec<Column>) -> Self {
        let name = name.into();
        Self {
            physical_name: name.clone(),
            name,
            deleted: false,
            columns,
        }
    }

    /// Returns the columns exposed by the view, in stable order.
    pub fn visible_columns(&self) -> Vec<&Column> {
        let mut visible: Vec<&Column> = self.columns.iter().filter(|c| !c.deleted).collect();
        visible.sort_by(|a, b| a.name.cmp(&b.name));
        visible
    }
}

/// A full schema snapshot for one version.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Schema {
    pub tables: Vec<Table>,
}

impl Schema {
    /// Returns the tables exposed by the version, in stable order.
    pub fn visible_tables(&self) -> Vec<&Table> {
        let mut visible: Vec<&Table> = self.tables.iter().filter(|t| !t.deleted).collect();
        visible.sort_by(|a, b| a.name.cmp(&b.name));
        visible
    }
}
