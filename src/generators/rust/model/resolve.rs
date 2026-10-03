//! Resolving an introspected table into a model.

use super::{Field, Model};
use crate::database::introspection::{Schema, Table};
use crate::generators::naming::Names;
use crate::generators::rust::casing::{self, Field as Name};
use crate::generators::rust::types;

impl Model {
    /// Resolves a table into its model.
    ///
    /// `names` comes from the schema-wide naming pass rather than being derived here: two
    /// tables can singularize to the same word, and only a whole-schema view can tell.
    pub fn resolve(table: &Table, schema: &Schema, names: &Names) -> Self {
        let fields = table
            .columns
            .iter()
            .map(|column| {
                let Name { ident, rename } = casing::field(&column.name);

                Field {
                    column: column.name.clone(),
                    ident,
                    rename,
                    mapping: types::map(column, schema),
                    primary_key: table.primary_key.contains(&column.name),
                    generated: column.auto_increment,
                    nullable: column.nullable,
                }
            })
            .collect();

        Self {
            table: table.name.clone(),
            module: names.module.clone(),
            name: names.type_name.clone(),
            fields,
            view: table.view,
            actix: false,
        }
    }

    /// Returns the model with actix output enabled.
    ///
    /// Set after resolution rather than during it: whether handlers are emitted is a caller's
    /// flag, not something the database says.
    pub fn with_actix(mut self, actix: bool) -> Self {
        self.actix = actix;
        self
    }
}
