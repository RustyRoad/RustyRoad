//! Resolving an introspected table into a model.

use super::{Field, Model};
use crate::database::introspection::{Schema, Table};
use crate::generators::naming::Names;
use crate::generators::tetherscript::types;

impl Model {
    /// Resolves a table into its model.
    ///
    /// `names` comes from the schema-wide naming pass rather than being derived here: two
    /// tables can singularize to the same word, and only a whole-schema view can tell.
    pub fn resolve(table: &Table, schema: &Schema, names: &Names) -> Self {
        let fields = table
            .columns
            .iter()
            .map(|column| Field {
                column: column.name.clone(),
                kind: types::map(column, schema),
                primary_key: table.primary_key.contains(&column.name),
                generated: column.auto_increment,
                nullable: column.nullable,
            })
            .collect();

        Self {
            table: table.name.clone(),
            module: names.module.clone(),
            name: names.type_name.clone(),
            fields,
        }
    }
}
