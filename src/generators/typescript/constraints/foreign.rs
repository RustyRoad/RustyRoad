//! Foreign key rendering for `schema.ts`.

use super::super::casing::{binding, identifier, Casing};
use super::super::ordering;
use crate::database::introspection::{ForeignKey, Table};

/// Renders foreign key declarations, including referential actions.
pub(super) fn render(table: &Table, names: &[String], casing: Casing) -> Vec<String> {
    table
        .foreign_keys
        .iter()
        .map(|key| {
            let mut entry = format!(
                "foreignKey({{\n\t\tcolumns: [{}],\n\t\tforeignColumns: [{}],\n\t\tname: \"{}\"\n\t}})",
                super::columns(&key.columns, casing),
                remote(table, key, names, casing),
                key.name
            );
            if let Some(action) = &key.on_delete {
                entry.push_str(&format!(".onDelete(\"{action}\")"));
            }
            if let Some(action) = &key.on_update {
                entry.push_str(&format!(".onUpdate(\"{action}\")"));
            }
            entry
        })
        .collect()
}

/// Renders the referenced column list for a foreign key.
///
/// A self-reference or a forward reference cannot name the binding directly,
/// because the `const` is not yet initialized; `table` is used instead.
fn remote(table: &Table, key: &ForeignKey, names: &[String], casing: Casing) -> String {
    let deferred = key.foreign_table == table.name
        || ordering::is_forward_reference(table, &key.foreign_table, names);

    let target = if deferred {
        "table".to_string()
    } else {
        binding(&key.foreign_table, casing)
    };

    key.foreign_columns
        .iter()
        .map(|column| format!("{target}.{}", identifier(column, casing)))
        .collect::<Vec<_>>()
        .join(", ")
}
