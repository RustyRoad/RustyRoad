//! Per-table Fastify route plugin using Zod schemas.

use super::super::casing::{binding, to_pascal, Casing};
use super::super::client;
use super::{read, write};
use crate::database::introspection::Table;

/// Renders one table's Fastify plugin.
///
/// Handlers take no explicit generics: the Zod type provider infers request and
/// reply types from the schemas, so `request.body` and `request.params` are typed
/// from the same definitions that validate them.
pub(super) fn plugin(table: &Table, casing: Casing) -> String {
    let name = binding(&table.name, casing);
    let type_name = to_pascal(&table.name);
    let key = &table.primary_key[0];

    format!(
        "/** Routes for `{table_name}`. */\n\
         export const {name}Routes: FastifyPluginAsyncZod<RouteOptions> = async (\n\
         \tapp,\n\
         \toptions,\n\
         ) => {{\n\
         \tconst {{ db }} = options;\n\
         \tconst params = z.object({{ id: {key_schema} }});\n\n\
         {list}\n{find}\n{create}\n{update}\n{remove}\
         }};\n",
        table_name = table.name,
        key_schema = key_schema(table, key),
        list = read::list(&name, &type_name),
        find = read::find(&name, &type_name),
        create = write::create(&name, &type_name),
        update = write::update(&name, &type_name),
        remove = write::remove(&name, &type_name),
    )
}

/// Renders the path-parameter schema for a table's key.
///
/// `coerce` is required because a path parameter always arrives as a string.
fn key_schema(table: &Table, key: &str) -> &'static str {
    match client::key_type(table, key) {
        "number" => "z.coerce.number().int()",
        _ => "z.string()",
    }
}
