//! CRUD repository rendering.

use super::super::casing::{binding, to_pascal, Casing};
use super::{read, write};
use crate::database::introspection::Table;

/// Renders CRUD helpers for a table with a usable primary key.
pub(super) fn repository(table: &Table, casing: Casing) -> String {
    let name = binding(&table.name, casing);
    let type_name = to_pascal(&table.name);
    let key = &table.primary_key[0];

    format!(
        "/** Queries for the `{table_name}` table. */\n\
         export const {name}Repository = {{\n\
         {list}\n{find}\n{create}\n{update}\n{remove}\
         }};\n",
        table_name = table.name,
        list = read::list(&name, &type_name),
        find = read::find(&name, &type_name, table, key, casing),
        create = write::create(&name, &type_name),
        update = write::update(&name, &type_name, table, key, casing),
        remove = write::remove(&name, table, key, casing),
    )
}
