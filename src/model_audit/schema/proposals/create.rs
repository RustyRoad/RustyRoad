//! Missing tables use RustyRoad's canonical CREATE/DROP renderer.
use super::{super::types::TableAudit, quote};

/// Empty contracts and nullable/unknown keys are not executable table definitions.
pub(super) fn validate(audit: &mut TableAudit, keys: &[String]) {
    if audit.missing_columns.is_empty() {
        audit
            .blockers
            .push("A new table requires at least one supported column".into());
    }
    let mut seen = std::collections::BTreeSet::new();
    for key in keys {
        if !seen.insert(key) {
            audit
                .blockers
                .push(format!("Duplicate primary-key column {key}"));
        }
        if !audit
            .missing_columns
            .iter()
            .any(|column| column.name == *key && !column.nullable)
        {
            audit.blockers.push(format!(
                "Primary-key column {key} must exist and be non-nullable"
            ));
        }
    }
}

/// Preserve declared nullability and only explicitly supplied primary keys.
pub(super) fn sql(table: &str, audit: &TableAudit, keys: &[String]) -> (String, String) {
    let mut columns: Vec<_> = audit
        .missing_columns
        .iter()
        .map(|column| {
            format!(
                "{} {}{}",
                quote(&column.name),
                column.sql_type,
                if column.nullable { "" } else { " NOT NULL" }
            )
        })
        .collect();
    if !keys.is_empty() {
        columns.push(format!(
            "PRIMARY KEY ({})",
            keys.iter()
                .map(|key| quote(key))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    let (up, down) = crate::database::migrations::ddl::create_table(table, &columns, false);
    (format!("{up}\n"), format!("{down}\n"))
}
