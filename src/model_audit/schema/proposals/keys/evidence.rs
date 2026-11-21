//! Fail closed when the observed primary key cannot be restored safely.
use super::super::super::{
    matching,
    types::{PrimaryKey, Table},
};

/// Require observed constraint identity, key properties and inbound dependencies.
pub(super) fn original(table: &Table) -> Result<Option<PrimaryKey>, String> {
    if table.primary_key.is_empty() {
        return Ok(None);
    }
    let key = table
        .primary_key_details
        .as_ref()
        .ok_or("Key redesign requires catalog primary_key_details and dependency evidence")?;
    matching::identifier(&key.name)?;
    for column in &table.primary_key {
        matching::identifier(column)?;
        if !table
            .columns
            .iter()
            .any(|observed| &observed.name == column && !observed.nullable)
        {
            return Err("Observed primary key columns must exist and be non-nullable".into());
        }
    }
    if key.initially_deferred && !key.deferrable {
        return Err("Invalid observed primary-key deferral properties".into());
    }
    if !key.referenced_by.is_empty() {
        return Err(format!(
            "Primary key is referenced by foreign keys {:?}; coordinate their redesign first",
            key.referenced_by
        ));
    }
    if !key.standard_index {
        return Err(
            "Key redesign cannot restore a primary key with custom index properties".into(),
        );
    }
    Ok(Some(key.clone()))
}
