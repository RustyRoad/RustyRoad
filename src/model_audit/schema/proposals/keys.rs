//! Explicit key redesigns require catalog evidence; never infer a replacement.
mod evidence;
mod rendering;

use super::super::types::{Catalog, TableAudit};
pub(super) use rendering::Change;

/// Plan a reviewed key change, retaining the original constraint for rollback.
pub(super) fn plan(catalog: &Catalog, audit: &mut TableAudit, keys: &[String]) -> Option<Change> {
    let table = catalog
        .tables
        .iter()
        .find(|table| Some(&table.name) == audit.table.as_ref())?;
    if keys.is_empty() {
        for change in &audit.column_changes {
            if change.after.nullable && table.primary_key.contains(&change.after.name) {
                audit.blockers.push(format!(
                    "Making primary key column {} nullable requires explicit key redesign",
                    change.after.name
                ));
            }
        }
        return None;
    }
    for key in keys {
        let column = audit
            .desired_columns
            .iter()
            .find(|column| &column.name == key)
            .or_else(|| table.columns.iter().find(|column| &column.name == key));
        match column {
            Some(column) if !column.nullable => {}
            _ => audit
                .blockers
                .push(format!("Primary key {key} must exist and be non-nullable")),
        }
    }
    if keys == table.primary_key || !audit.blockers.is_empty() {
        return None;
    }
    if table.kind != "table" {
        audit
            .blockers
            .push("Key redesign requires an ordinary, non-partitioned table".into());
        return None;
    }
    let before = match evidence::original(table) {
        Ok(before) => before,
        Err(error) => {
            audit.blockers.push(error);
            return None;
        }
    };
    let name = before
        .as_ref()
        .map(|key| key.name.clone())
        .unwrap_or_else(|| format!("{}_pkey", &table.name[..table.name.len().min(58)]));
    Some(Change {
        before,
        before_columns: table.primary_key.clone(),
        name,
        after: keys.into(),
    })
}
