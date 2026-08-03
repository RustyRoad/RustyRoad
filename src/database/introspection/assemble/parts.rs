//! Grouped constraint parts collected during introspection.

/// Grouped constraint parts keyed by name, preserving column order.
pub(crate) type Grouped = std::collections::BTreeMap<String, (String, Vec<String>)>;

/// Accumulated parts of one index.
#[derive(Default)]
pub(crate) struct IndexParts {
    pub table: String,
    pub columns: Vec<String>,
    pub unique: bool,
}

/// Accumulated parts of one foreign key.
#[derive(Default)]
pub(crate) struct FkParts {
    pub table: String,
    pub foreign_table: String,
    pub columns: Vec<String>,
    pub foreign_columns: Vec<String>,
    pub on_delete: Option<String>,
    pub on_update: Option<String>,
}

/// Maps a Postgres referential action code to its SQL keyword.
///
/// `a` is NO ACTION, the default, which is left unstated.
pub(crate) fn action(code: &str) -> Option<String> {
    match code {
        "c" => Some("cascade".to_string()),
        "n" => Some("set null".to_string()),
        "d" => Some("set default".to_string()),
        "r" => Some("restrict".to_string()),
        _ => None,
    }
}
