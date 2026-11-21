use super::{Names, RelationInsert};
use crate::database::introspection::{Schema, Table};
use crate::generators::rust::{repositories::equality, types};
use std::io;

/// Requires explicit identities, nullable optional linkage and compatible scalar codecs.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn validate(
    schema: &Schema,
    names: &Names,
    name: &str,
    report: &RelationInsert,
) -> io::Result<()> {
    let invalid = |reason: &str| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Invalid relation insert {name:?}: {reason}"),
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
        return Err(invalid("invalid or reserved Rust function name"));
    }
    let table = |name: &str| {
        schema
            .tables
            .iter()
            .find(|t| t.name == name)
            .ok_or_else(|| invalid("table absent from pull scope"))
    };
    let target = table(&report.table)?;
    let source = table(&report.source_table)?;
    let optional = table(&report.optional_table)?;
    if target.view
        || source.primary_key != vec![report.source_key.clone()]
        || optional.primary_key != vec![report.optional_key.clone()]
        || std::collections::BTreeSet::from([
            &report.input_column,
            &report.source_target,
            &report.optional_target,
        ])
        .len()
            != 3
    {
        return Err(invalid("invalid write target or ambiguous row identity"));
    }
    let scalar = |table: &Table, column: &str| -> io::Result<String> {
        if !equality::keys(table)
            .iter()
            .any(|key| key == &vec![column.to_owned()])
        {
            return Err(invalid("missing or unsupported scalar column"));
        }
        let mut field = table.column(column).unwrap().clone();
        field.nullable = false;
        Ok(types::map_column(table, &field, schema, names).rust)
    };
    scalar(target, &report.input_column)?;
    let source_type = scalar(source, &report.source_key)?;
    if scalar(target, &report.source_target)? != source_type
        || scalar(optional, &report.optional_source)? != source_type
        || scalar(target, &report.optional_target)? != scalar(optional, &report.optional_key)?
        || !target.column(&report.optional_target).unwrap().nullable
    {
        return Err(invalid(
            "relationship codecs or optional target do not match",
        ));
    }
    let eligible = source
        .column(&report.eligible_column)
        .ok_or_else(|| invalid("eligibility column absent"))?;
    if !matches!(
        eligible.sql_type.trim().to_ascii_lowercase().as_str(),
        "bool" | "boolean"
    ) {
        return Err(invalid("eligibility column must be boolean"));
    }
    Ok(())
}
