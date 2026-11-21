//! Scalar operands and validated SQL comparison operators share column metadata.
use super::{sql, Column, Names, Schema, Table};
use crate::generators::rust::types;

/// Emits per-column comparison vectors with the canonical non-null operand type.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn declarations(
    table: &Table,
    schema: &Schema,
    names: &Names,
    columns: &[&Column],
) -> String {
    columns.iter().map(|column| {
        let mut required = (**column).clone();
        required.nullable = false;
        let value = types::map_column(table, &required, schema, names).rust;
        let field = names.field(table, &column.name);
        format!("    /// AND comparisons for {:?}; empty means unconstrained.\n    pub {field}: Vec<(String, {value})>,", column.name)
    }).collect::<Vec<_>>().join("\n")
}

/// Emits bound comparisons, rejecting operators outside the column allowlist.
/// NULL operands are not represented; equality filters own explicit NULL checks.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(in crate::generators::rust) fn bindings(
    table: &Table,
    names: &Names,
    columns: &[&Column],
) -> String {
    columns.iter().map(|column| {
        let field = names.field(table, &column.name);
        let prefix = format!(" AND {} ", sql(&column.name));
        let declared = column.sql_type.to_lowercase();
        let base = declared.split('(').next().unwrap_or("").trim();
        let text = matches!(base, "text" | "varchar" | "character varying" | "char" | "character" | "bpchar");
        let patterns = if text { " | \"LIKE\" | \"ILIKE\"" } else { "" };
        format!(
            "for (operator, value) in comparisons.{field} {{\n\
                 let operator = match operator.as_str() {{\n\
                     \"=\" | \"<>\" | \"<\" | \"<=\" | \">\" | \">=\"{patterns} => operator.as_str(),\n\
                     _ => return Err(sqlx::Error::Protocol(format!(\"Unsupported comparison for {field}: {{operator}}\"))),\n\
                 }};\n\
                 query.push({prefix:?}).push(operator).push(\" \").push_bind(value);\n\
             }}"
        )
    }).collect::<Vec<_>>().join("\n")
}
