//! Typed comparison predicates composed with the existing equality filters.
mod fields;
pub(in crate::generators::rust) use fields::bindings as comparison_bindings;
mod reads;
use super::{keys, sql, Names, Schema, Table};
use crate::database::introspection::Column;

/// Emits comparison inputs and bounded reads without assuming column uniqueness.
/// Values use the introspected scalar codec; only text columns allow LIKE/ILIKE.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(
    table: &Table,
    schema: &Schema,
    names: &Names,
    columns: &[&Column],
    equality: &str,
) -> String {
    let input = names.comparison(table);
    let declarations = fields::declarations(table, schema, names, columns);
    let bindings = fields::bindings(table, names, columns);
    let mut output = format!(
        "/// Typed AND comparisons; empty vectors add no predicate.\n\
         /// Non-null operands use the column codec; NULL matching belongs in Filter.\n\
         /// Example: `{input}::default()` leaves every column unconstrained.\n\
         #[derive(Debug, Clone, Default)]\n\
         pub struct {input} {{\n{declarations}\n}}\n\n"
    );
    output.push_str(&reads::render(table, names, equality, &bindings));
    output
}
