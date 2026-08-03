//! Mutating procedures: create, update, delete.

use super::{key_schema, Procedure};
use crate::database::introspection::Table;
use crate::generators::typescript::casing::{binding, to_pascal, Casing};

/// Builds the create.
pub(super) fn create(table: &Table, path: String, casing: Casing) -> Procedure {
    let name = binding(&table.name, casing);

    Procedure {
        name: "create",
        method: "POST",
        path,
        input: format!("{name}InsertSchema"),
        output: format!("{name}SelectSchema"),
        body: format!("{name}Repository.create(context.db, input)"),
    }
}

/// Builds the partial update.
///
/// The input combines the key from the path with the columns to set, and `rest`
/// strips the key before it reaches the repository.
pub(super) fn update(table: &Table, path: String, casing: Casing) -> Procedure {
    let name = binding(&table.name, casing);
    let entity = to_pascal(&table.name);

    Procedure {
        name: "update",
        method: "PATCH",
        path,
        input: format!(
            "z.object({{ id: {} }}).and({name}UpdateSchema)",
            key_schema(table)
        ),
        output: format!("{name}SelectSchema"),
        body: format!(
            "found(await {name}Repository.update(context.db, input.id, rest(input)), \"{entity}\")"
        ),
    }
}

/// Builds the delete.
pub(super) fn delete(table: &Table, path: String, casing: Casing) -> Procedure {
    let name = binding(&table.name, casing);
    let entity = to_pascal(&table.name);

    Procedure {
        name: "delete",
        method: "DELETE",
        path,
        input: format!("z.object({{ id: {} }})", key_schema(table)),
        output: "z.object({ deleted: z.boolean() })".to_string(),
        // Parenthesized because an arrow body starting with `{` parses as a block
        // rather than an object literal, which would return void.
        body: format!(
            "({{ deleted: await removed({name}Repository.remove(context.db, input.id), \"{entity}\") }})"
        ),
    }
}
