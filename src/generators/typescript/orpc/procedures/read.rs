//! Read procedures: collection and single row.

use super::{key_schema, Procedure};
use crate::database::introspection::Table;
use crate::generators::typescript::casing::{binding, to_pascal, Casing};

/// Builds the collection read.
pub(super) fn list(table: &Table, path: String, casing: Casing) -> Procedure {
    let name = binding(&table.name, casing);

    Procedure {
        name: "list",
        method: "GET",
        path,
        input: "z.object({})".to_string(),
        output: format!("z.array({name}SelectSchema)"),
        body: format!("{name}Repository.list(context.db)"),
    }
}

/// Builds the single-row read.
pub(super) fn get(table: &Table, path: String, casing: Casing) -> Procedure {
    let name = binding(&table.name, casing);
    let entity = to_pascal(&table.name);

    Procedure {
        name: "get",
        method: "GET",
        path,
        input: format!("z.object({{ id: {} }})", key_schema(table, casing)),
        output: format!("{name}SelectSchema"),
        body: format!("found(await {name}Repository.find(context.db, input.id), \"{entity}\")"),
    }
}
