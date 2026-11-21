use super::{Names, RelatedOperation, RelatedWrite};
use crate::database::introspection::{Schema, Table};
use crate::generators::rust::{repositories::equality, types};
use std::{collections::BTreeSet, io};

/// Rejects invalid names, ambiguous identities, and unrepresentable write columns.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn validate(
    schema: &Schema,
    names: &Names,
    name: &str,
    write: &RelatedWrite,
) -> io::Result<()> {
    let invalid = |reason: &str| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Invalid related write {name:?}: {reason}"),
        )
    };
    if name.is_empty()
        || !name.starts_with(|c: char| c.is_ascii_lowercase())
        || !name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        || syn::parse_str::<syn::Ident>(name).is_err()
        || names.function_reserved(name)
    {
        return Err(invalid("invalid or reserved function name"));
    }
    let table = schema
        .table(&write.table)
        .ok_or_else(|| invalid("missing source table"))?;
    let foreign = table
        .foreign_keys
        .iter()
        .find(|f| f.name == write.foreign_key)
        .ok_or_else(|| invalid("missing foreign key"))?;
    let related = schema
        .table(&foreign.foreign_table)
        .ok_or_else(|| invalid("related table outside pull scope"))?;
    if table.view
        || related.view
        || foreign.columns.len() != 1
        || foreign.foreign_columns.len() != 1
    {
        return Err(invalid("requires tables and a single-column foreign key"));
    }
    let lookup = related
        .column(&write.lookup)
        .ok_or_else(|| invalid("missing lookup column"))?;
    let key = vec![write.lookup.clone()];
    if related.primary_key != key && !related.uniques.iter().any(|u| u.columns == key) {
        return Err(invalid("related lookup must have unconditional uniqueness"));
    }
    if !equality::keys(related).contains(&key) {
        return Err(invalid("unsupported lookup codec"));
    }
    let local = table
        .column(&foreign.columns[0])
        .ok_or_else(|| invalid("missing FK column"))?;
    let remote = related
        .column(&foreign.foreign_columns[0])
        .ok_or_else(|| invalid("missing referenced column"))?;
    let required_type = |owner: &Table, column: &crate::database::introspection::Column| {
        let mut column = column.clone();
        column.nullable = false;
        types::map_column(owner, &column, schema, names).rust
    };
    if required_type(table, local) != required_type(related, remote) {
        return Err(invalid("FK codecs differ"));
    }
    let unique: BTreeSet<_> = write.columns.iter().collect();
    if write.columns.is_empty()
        || unique.len() != write.columns.len()
        || write.columns.contains(&foreign.columns[0])
    {
        return Err(invalid("value columns must be distinct and exclude the FK"));
    }
    for column in &write.columns {
        let field = table
            .column(column)
            .ok_or_else(|| invalid("unknown value column"))?;
        if field.auto_increment
            || (write.operation == RelatedOperation::Update && table.primary_key.contains(column))
        {
            return Err(invalid(
                "cannot assign generated columns or update primary keys",
            ));
        }
    }
    let predicates: BTreeSet<_> = write.predicates.iter().collect();
    if predicates.len() != write.predicates.len()
        || write
            .predicates
            .iter()
            .any(|c| !equality::keys(table).contains(&vec![c.clone()]))
    {
        return Err(invalid("invalid predicate columns"));
    }
    if write.operation == RelatedOperation::Create {
        if !write.predicates.is_empty()
            || write.clock.is_some()
            || table.columns.iter().any(|c| {
                !c.nullable
                    && !c.auto_increment
                    && c.default.is_none()
                    && c.name != foreign.columns[0]
                    && !write.columns.contains(&c.name)
            })
        {
            return Err(invalid(
                "creation omits required values or specifies update options",
            ));
        }
    } else if write.predicates.is_empty() {
        return Err(invalid("updates require source predicates"));
    }
    if let Some(clock) = &write.clock {
        let column = table
            .column(clock)
            .ok_or_else(|| invalid("unknown clock column"))?;
        let kind = column.sql_type.trim().to_ascii_lowercase();
        if !kind.starts_with("timestamp")
            || kind.ends_with("[]")
            || column.auto_increment
            || table.primary_key.contains(clock)
            || write.columns.contains(clock)
            || foreign.columns.contains(clock)
        {
            return Err(invalid("invalid database clock column"));
        }
    }
    let _ = lookup;
    Ok(())
}
