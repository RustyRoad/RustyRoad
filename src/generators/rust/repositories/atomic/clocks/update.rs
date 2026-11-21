//! Selective conflict updates with explicit, independently selected database clocks.
use super::policy;
use crate::database::introspection::Table;
use crate::generators::rust::repositories::patch;
use crate::generators::rust::{allocation::Names, naming::sql};

/// Preserve supplied Patch values and add time only for selected omitted fields.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(table: &Table, key: &[String], names: &Names) -> String {
    let patch::Patch {
        changed,
        mut assignments,
        ..
    } = patch::render(table, "patch", names);
    for column in table
        .columns
        .iter()
        .filter(|c| policy::eligible(table, c) && !key.contains(&c.name))
    {
        let field = names.field(table, &column.name);
        let name = &column.name;
        let clock = format!("{} = CURRENT_TIMESTAMP", sql(name));
        assignments.push_str(&format!(
            "\nif patch.{field}.is_none() && update_clocks.contains(&{name:?}) {{ set.push({clock:?}); }}"
        ));
    }
    format!(
        "if {changed} || !update_clocks.is_empty() {{\n\
             query.push(\" DO UPDATE SET \");\n\
             {{ let mut set = query.separated(\", \");\n{assignments}\n }}\n\
         }} else {{ query.push(\" DO NOTHING\"); }}\n"
    )
}
