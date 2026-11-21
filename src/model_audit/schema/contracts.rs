use super::{matching, types::Catalog};
use std::collections::{BTreeMap, BTreeSet};

/// Reject inconsistent snapshots before their metadata can become rollback SQL.
pub(super) fn validate(catalog: &Catalog) -> Result<(), String> {
    if catalog.version != 1 {
        return Err(format!("Unsupported catalog version {}", catalog.version));
    }
    matching::identifier(&catalog.schema)?;
    let mut tables = BTreeSet::new();
    for table in &catalog.tables {
        if !tables.insert(&table.name) {
            return Err(format!("Duplicate catalog table {}", table.name));
        }
        let mut columns = BTreeSet::new();
        for column in &table.columns {
            if !columns.insert(&column.name) {
                return Err(format!(
                    "Duplicate catalog column {}.{}",
                    table.name, column.name
                ));
            }
        }
        let mut keys = BTreeSet::new();
        for key in &table.primary_key {
            if !keys.insert(key) || !columns.contains(key) {
                return Err(format!("Invalid catalog primary key {}.{key}", table.name));
            }
        }
    }
    Ok(())
}

/// Parse explicit, reviewed key contracts; never infer a key from field names.
pub(super) fn keys(values: Vec<String>) -> Result<BTreeMap<String, Vec<String>>, String> {
    let mut result = BTreeMap::new();
    for value in values {
        let (table, columns) = value
            .split_once('=')
            .ok_or("Use --primary-key TABLE=COLUMN[,COLUMN]")?;
        let columns: Vec<String> = columns.split(',').map(str::to_owned).collect();
        if matching::identifier(table).is_err()
            || columns.is_empty()
            || columns.iter().any(|c| matching::identifier(c).is_err())
            || columns.iter().collect::<BTreeSet<_>>().len() != columns.len()
        {
            return Err(format!("Invalid primary key contract {value}"));
        }
        if result.insert(table.into(), columns).is_some() {
            return Err(format!("Duplicate primary key contract for {table}"));
        }
    }
    Ok(result)
}
