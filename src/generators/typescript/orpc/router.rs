//! Per-table oRPC router rendering.

use super::procedures::{procedures, Procedure};
use crate::database::introspection::Table;
use crate::generators::typescript::casing::{binding, Casing};

/// Renders one table's router.
pub(super) fn router(table: &Table, prefix: &str, casing: Casing) -> String {
    let name = binding(&table.name, casing);
    let procedures = procedures(table, prefix, casing);

    let definitions = procedures
        .iter()
        .map(|procedure| definition(&name, procedure))
        .collect::<Vec<_>>()
        .join("\n");

    let members = procedures
        .iter()
        .map(|procedure| format!("\t{}: {name}_{},", procedure.name, procedure.name))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "{definitions}\n\
         /** Procedures for the `{table_name}` table. */\n\
         export const {name}Router = {{\n{members}\n}};\n",
        table_name = table.name
    )
}

/// Renders one procedure.
///
/// The explicit `.route()` is what lets a single definition serve both an RPC call
/// and a REST request, and is what oRPC reads when generating OpenAPI.
fn definition(name: &str, procedure: &Procedure) -> String {
    format!(
        "const {name}_{} = base\n\
         \t.route({{ method: \"{}\", path: \"{}\" }})\n\
         \t.input({})\n\
         \t.output({})\n\
         \t.handler(async ({{ input, context }}) => {});\n\n",
        procedure.name,
        procedure.method,
        procedure.path,
        procedure.input,
        procedure.output,
        procedure.body,
    )
}
