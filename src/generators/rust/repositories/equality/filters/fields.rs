//! Filter declarations and bound SQL fragments use the same supported columns.
use super::{sql, Names, Schema, Table};
use crate::database::introspection::Column;
use crate::generators::rust::types;

/// Derive typed optional predicates from canonical column codecs.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn declarations(
    table: &Table,
    schema: &Schema,
    columns: &[&Column],
    names: &Names,
) -> String {
    columns.iter().map(|column| {
        let mut required = (**column).clone();
        required.nullable = false;
        let base = types::map_column(table, &required, schema, names).rust;
        let field = names.field(table, &column.name);
        let value = if column.nullable { format!("Option<{base}>") } else { base };
        format!("    /// Equality predicate for {:?}; None leaves it unconstrained.\n    pub {field}: Option<{value}>,", column.name)
    }).collect::<Vec<_>>().join("\n")
}

/// Bind each present predicate in catalog order; NULL is never an equality bind.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(in crate::generators::rust) fn bindings(
    table: &Table,
    columns: &[&Column],
    names: &Names,
) -> String {
    bindings_for(table, columns, names, "filter", None)
}

/// Binds qualified predicates for joined reads without changing NULL semantics.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(in crate::generators::rust::repositories::equality) fn bindings_for(
    table: &Table,
    columns: &[&Column],
    names: &Names,
    input: &str,
    alias: Option<&str>,
) -> String {
    columns.iter().map(|column| {
        let field = names.field(table, &column.name);
        let qualifier = alias.map(|alias| format!("{}.", sql(alias))).unwrap_or_default();
        let predicate = format!(" AND {qualifier}{}", sql(&column.name));
        let value = if column.nullable {
            "match value { Some(value) => { query.push(\" = \").push_bind(value); }, None => { query.push(\" IS NULL\"); } }"
        } else {
            "query.push(\" = \").push_bind(value);"
        };
        format!("if let Some(value) = {input}.{field} {{ query.push({predicate:?}); {value} }}")
    }).collect::<Vec<_>>().join("\n")
}
