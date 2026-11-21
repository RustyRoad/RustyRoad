//! Preserve every supported desired contract, including fields already matching the database.
use super::super::{
    rust_types,
    types::{ColumnChange, Table, TableAudit},
};
use crate::model_audit::types::Model;
use std::collections::{BTreeMap, BTreeSet};
mod numeric_storage;
mod source;
mod string_storage;

/// Record `i32 -> i64` and `Option<T>` changes as migration inputs, not blanket blockers.
/// # Examples
/// ```sh
/// rustyroad model audit --source src/models --report-only
/// ```
pub(super) fn compare(
    model: &Model,
    table: Option<&Table>,
    mappings: &BTreeMap<String, String>,
    audit: &mut TableAudit,
) {
    let resolver = rust_types::Resolver::for_model(model);
    let mut seen_columns = BTreeSet::new();
    for (name, field) in &model.fields {
        let mut column = match rust_types::model_column(name, field, mappings, &resolver) {
            Ok(column) => column,
            Err(error) => {
                audit.blockers.push(error);
                continue;
            }
        };
        if let Err(error) = super::identifier(&column.name) {
            audit.blockers.push(error);
        }
        if !seen_columns.insert(column.name.clone()) {
            audit
                .blockers
                .push(format!("Multiple fields map to column {}", column.name));
            continue;
        }
        if let Some(existing) = table.and_then(|t| t.columns.iter().find(|c| c.name == column.name))
        {
            string_storage::preserve(field, mappings, &resolver, existing, &mut column);
            numeric_storage::preserve(field, mappings, &resolver, existing, &mut column);
        }
        audit.desired_columns.push(column.clone());
        match table.and_then(|t| t.columns.iter().find(|c| c.name == column.name)) {
            None => audit.missing_columns.push(column),
            Some(existing)
                if rust_types::normalized(&existing.sql_type)
                    != rust_types::normalized(&column.sql_type)
                    || existing.nullable != column.nullable =>
            {
                audit.column_changes.push(ColumnChange {
                    before: existing.clone(),
                    after: column,
                });
            }
            Some(_) => (),
        }
    }
    if let Some(table) = table {
        audit.database_only = table
            .columns
            .iter()
            .filter(|c| {
                !audit
                    .desired_columns
                    .iter()
                    .any(|desired| desired.name == c.name)
            })
            .map(|c| c.name.clone())
            .collect();
        if !audit.column_changes.is_empty() {
            audit.status = "column_changes".into();
        } else if !audit.missing_columns.is_empty() {
            audit.status = "missing_columns".into();
        }
    }
}
