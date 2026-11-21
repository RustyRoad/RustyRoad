use super::{expressions, JsonInsert, Names};
use crate::database::introspection::Schema;
use std::{collections::BTreeSet, io};

/// Requires declared columns, JSONB input, nullable integer targets and a real conflict key.
/// Rejects identity-column writes and omitted mandatory inputs before rendering.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn validate(
    schema: &Schema,
    names: &Names,
    name: &str,
    insert: &JsonInsert,
) -> io::Result<()> {
    let invalid = |reason: &str| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Invalid JSON insert {name:?}: {reason}"),
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
        .table(&insert.table)
        .ok_or_else(|| invalid("table outside pull scope"))?;
    let unique: BTreeSet<_> = insert.columns.iter().collect();
    if table.view
        || insert.columns.is_empty()
        || unique.len() != insert.columns.len()
        || insert.integers.is_empty()
        || !insert.columns.contains(&insert.json_column)
        || table.column(&insert.json_column).map(|c| c.sql_type.trim()) != Some("jsonb")
    {
        return Err(invalid(
            "requires distinct inputs and a JSONB input with integer derivations",
        ));
    }
    for input in &insert.columns {
        let field = table
            .column(input)
            .ok_or_else(|| invalid("unknown input column"))?;
        if field.auto_increment || expressions::cast(field).is_none() {
            return Err(invalid("unsupported or identity input column"));
        }
    }
    let mut assigned: BTreeSet<String> = insert.columns.iter().cloned().collect();
    for (target, rule) in &insert.integers {
        let field = table
            .column(target)
            .ok_or_else(|| invalid("unknown integer target"))?;
        if rule
            .key
            .chars()
            .any(|value| value == char::from(0u8) || value == char::from(92u8))
            || !field.nullable
            || field.auto_increment
            || !matches!(
                expressions::cast(field),
                Some("smallint" | "integer" | "bigint")
            )
            || rule.prefer_input != insert.columns.contains(target)
        {
            return Err(invalid(
                "integer derivation requires nullable storage and matching input precedence",
            ));
        }
        assigned.insert(target.clone());
    }
    for target in insert.booleans.keys() {
        let field = table
            .column(target)
            .ok_or_else(|| invalid("unknown boolean target"))?;
        if field.auto_increment
            || expressions::cast(field) != Some("boolean")
            || !assigned.insert(target.clone())
        {
            return Err(invalid("boolean target is unsupported or assigned twice"));
        }
    }
    if insert.conflict.is_empty()
        || (table.primary_key != insert.conflict
            && !table
                .uniques
                .iter()
                .any(|key| key.columns == insert.conflict)
            && !table
                .indexes
                .iter()
                .any(|key| key.unique && key.columns == insert.conflict))
    {
        return Err(invalid(
            "conflict columns need an unconditional catalog unique key",
        ));
    }
    if table.columns.iter().any(|field| {
        !field.nullable
            && !field.auto_increment
            && field.default.is_none()
            && !assigned.contains(&field.name)
    }) {
        return Err(invalid("required column omitted"));
    }
    Ok(())
}
