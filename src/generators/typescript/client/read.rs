//! Read query rendering for the repository.

use super::super::casing::{identifier, Casing};
use super::rows::key_type;
use crate::database::introspection::Table;

/// Renders the list query.
pub(super) fn list(name: &str, type_name: &str) -> String {
    format!(
        "\tasync list(db: RepositoryDatabase): Promise<{type_name}Row[]> {{\n\
         \t\treturn db.select().from({name});\n\
         \t}},\n"
    )
}

/// Renders the single-row lookup.
pub(super) fn find(
    name: &str,
    type_name: &str,
    table: &Table,
    key: &str,
    casing: Casing,
) -> String {
    let field = identifier(key, casing);
    let key_type = key_type(table, key, casing);

    format!(
        "\tasync find(db: RepositoryDatabase, id: {key_type}): Promise<{type_name}Row | undefined> {{\n\
         \t\tconst rows = await db.select().from({name}).where(eq({name}.{field}, id)).limit(1);\n\
         \t\treturn rows[0];\n\
         \t}},\n"
    )
}
