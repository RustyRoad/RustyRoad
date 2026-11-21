//! Restrict ownership targets to one unqualified physical relation.
use sqlparser::ast::{ObjectName, TableFactor, TableWithJoins};

/// Preserve quoted case but fold unquoted PostgreSQL names, e.g. `Rows` -> `rows`.
pub(super) fn name(name: &ObjectName) -> Option<String> {
    let [part] = name.0.as_slice() else {
        return None;
    };
    let ident = part.as_ident()?;
    let name = if ident.quote_style.is_some() {
        ident.value.clone()
    } else {
        ident.value.to_lowercase()
    };
    Some(name)
}

/// Reject joins and function relations, e.g. `FROM generate_series(1, 2)`.
pub(super) fn table(table: &TableWithJoins) -> Option<String> {
    if !table.joins.is_empty() {
        return None;
    }
    let TableFactor::Table {
        name: target,
        args: None,
        alias,
        with_hints,
        version: None,
        with_ordinality: false,
        partitions,
        json_path: None,
        sample: None,
        index_hints,
    } = &table.relation
    else {
        return None;
    };
    if !with_hints.is_empty() || !partitions.is_empty() || !index_hints.is_empty() {
        return None;
    }
    if alias
        .as_ref()
        .is_some_and(|alias| !alias.columns.is_empty())
    {
        return None;
    }
    name(target)
}
