use super::super::Projection;
use crate::database::introspection::{Schema, Table};
use crate::generators::rust::repositories::equality;
use std::{collections::BTreeSet, io};

/// Requires a catalog foreign key, known distinct projections and sortable related fields.
/// At most sixteen projected columns fit SQLx tuple decoding; no table identity is inferred.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn validate(schema: &Schema, table: &Table, projection: &Projection) -> io::Result<()> {
    let invalid = |reason: &str| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Invalid joined projection for {:?}: {reason}", table.name),
        )
    };
    let join = projection
        .join
        .as_ref()
        .ok_or_else(|| invalid("missing join configuration"))?;
    let foreign = table
        .foreign_keys
        .iter()
        .find(|key| key.name == join.foreign_key)
        .ok_or_else(|| invalid("missing foreign key"))?;
    let related = schema
        .table(&foreign.foreign_table)
        .ok_or_else(|| invalid("related table outside pull scope"))?;
    if foreign.columns.is_empty()
        || foreign.columns.len() != foreign.foreign_columns.len()
        || foreign
            .columns
            .iter()
            .any(|name| table.column(name).is_none())
        || foreign
            .foreign_columns
            .iter()
            .any(|name| related.column(name).is_none())
    {
        return Err(invalid("foreign key columns are inconsistent"));
    }
    let count = projection.columns.len() + join.columns.len();
    if count == 0 || count > 16 || equality::keys(table).is_empty() {
        return Err(invalid(
            "requires 1-16 result columns and supported source filters",
        ));
    }
    for (owner, columns) in [(table, &projection.columns), (related, &join.columns)] {
        let unique: BTreeSet<_> = columns.iter().collect();
        if unique.len() != columns.len() || columns.iter().any(|name| owner.column(name).is_none())
        {
            return Err(invalid(
                "projection columns must be distinct known catalog columns",
            ));
        }
    }
    let ordering = equality::keys(related);
    if join
        .order
        .iter()
        .any(|order| !ordering.contains(&vec![order.column.clone()]))
    {
        return Err(invalid(
            "ordering requires a supported scalar related column",
        ));
    }
    Ok(())
}
