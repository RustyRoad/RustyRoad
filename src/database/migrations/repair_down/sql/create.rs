//! Reverse newly created tables and explicitly named, nonconcurrent indexes.

use sqlparser::ast::{CreateIndex, CreateTable};

pub(super) fn table(table: &CreateTable) -> Result<Vec<String>, String> {
    if table.if_not_exists {
        return Err(super::conditional());
    }
    if table.or_replace
        || table.temporary
        || table.external
        || table.dynamic
        || table.transient
        || table.volatile
        || table.iceberg
        || table.snapshot
        || table.global.is_some()
        || table.query.is_some()
        || table.like.is_some()
        || table.clone.is_some()
        || table.inherits.is_some()
        || table.partition_of.is_some()
        || table.on_cluster.is_some()
        || table.on_commit.is_some()
    {
        return Err("Only ordinary, unconditional CREATE TABLE is inferred".into());
    }
    super::names::check(&table.name, 2)?;
    Ok(vec![format!("DROP TABLE {} RESTRICT;", table.name)])
}

pub(super) fn index(index: &CreateIndex) -> Result<Vec<String>, String> {
    if index.if_not_exists {
        return Err(super::conditional());
    }
    if index.concurrently {
        return Err("Concurrent index operations require reviewed rollback semantics".into());
    }
    if !index.alter_options.is_empty() {
        return Err("Index alteration options require reviewed undo".into());
    }
    let name = index
        .name
        .as_ref()
        .ok_or("Unnamed index requires reviewed undo")?;
    super::names::check(name, 1)?;
    super::names::check(&index.table_name, 2)?;
    if name.0.len() != 1 || index.table_name.0.is_empty() || index.table_name.0.len() > 2 {
        return Err("Index/table qualification requires reviewed undo".into());
    }
    // PostgreSQL creates indexes in their table's schema, not the search-path default.
    let qualified = if index.table_name.0.len() == 2 {
        format!("{}.{}", index.table_name.0[0], name)
    } else {
        name.to_string()
    };
    Ok(vec![format!("DROP INDEX {qualified} RESTRICT;")])
}
