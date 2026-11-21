//! Generated insertion input that can delegate mandatory timestamps to PostgreSQL.
use super::policy;
use crate::database::introspection::{Schema, Table};
use crate::generators::rust::repositories::inputs;
use crate::generators::rust::{allocation::Names, types};

/// Declare a clock-aware input without changing ordinary New input contracts.
/// Optional/defaulted/null values retain existing insertion semantics.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(table: &Table, schema: &Schema, names: &Names) -> String {
    let fields = inputs::new_columns(table).iter().map(|column| {
        let field = names.field(table, &column.name);
        let mut required = (**column).clone();
        required.nullable = false;
        let base = types::map_column(table, &required, schema, names).rust;
        let value = if column.nullable && column.default.is_some() {
            format!("Option<Option<{base}>>")
        } else if inputs::optional(column) || policy::eligible(table, column) {
            format!("Option<{base}>")
        } else { base };
        format!("    /// Insertion value for {:?}; supplied values override requested clocks.\n    pub {field}: {value},", column.name)
    }).collect::<Vec<_>>().join("\n");
    let input = names.clock_input(table);
    format!(
        "/// Insert input for explicit database-clock upserts; ordinary New inputs are unchanged.\n\
         /// An omitted required timestamp must be selected in insert_clocks.\n\
         #[derive(Debug, Clone)]\n\
         pub struct {input} {{\n{fields}\n}}\n\n"
    )
}
