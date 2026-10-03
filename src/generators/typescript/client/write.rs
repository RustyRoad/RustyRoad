//! Mutating query rendering for the repository.

use super::super::casing::{identifier, Casing};
use super::rows::key_type;
use crate::database::introspection::Table;

/// Renders the insert.
pub(super) fn create(name: &str, type_name: &str) -> String {
    format!(
        "\tasync create(db: RepositoryDatabase, values: New{type_name}Row): Promise<{type_name}Row> {{\n\
         \t\tconst rows = await db.insert({name}).values(values).returning();\n\
         \t\treturn rows[0];\n\
         \t}},\n"
    )
}

/// Renders the partial update.
pub(super) fn update(
    name: &str,
    type_name: &str,
    table: &Table,
    key: &str,
    casing: Casing,
) -> String {
    let field = identifier(key, casing);
    let key_type = key_type(table, key, casing);

    format!(
        "\tasync update(db: RepositoryDatabase, id: {key_type}, values: Partial<New{type_name}Row>): Promise<{type_name}Row | undefined> {{\n\
         \t\tconst rows = await db.update({name}).set(values).where(eq({name}.{field}, id)).returning();\n\
         \t\treturn rows[0];\n\
         \t}},\n"
    )
}

/// Renders the delete.
pub(super) fn remove(name: &str, table: &Table, key: &str, casing: Casing) -> String {
    let field = identifier(key, casing);
    let key_type = key_type(table, key, casing);

    format!(
        "\tasync remove(db: RepositoryDatabase, id: {key_type}): Promise<boolean> {{\n\
         \t\tconst rows = await db.delete({name}).where(eq({name}.{field}, id)).returning();\n\
         \t\treturn rows.length > 0;\n\
         \t}},\n"
    )
}
