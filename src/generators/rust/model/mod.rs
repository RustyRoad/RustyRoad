//! The resolved shape of one generated model.
//!
//! Introspection describes a table in database terms; the renderers need it in Rust terms.
//! Resolving that once, here, keeps the CRUD renderers free of casing and type decisions
//! and guarantees they agree: the column list a struct declares is the same list the
//! `INSERT` binds, in the same order.

mod field;
mod resolve;

use crate::database::introspection::{Enum, Schema};
use crate::generators::rust::casing;

pub use field::Field;

/// A table resolved into everything needed to emit its model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Model {
    /// The table name, used verbatim in SQL.
    pub table: String,
    /// The module folder the files live in, singular and snake_case.
    pub module: String,
    /// The struct name, singular and PascalCase.
    pub name: String,
    pub fields: Vec<Field>,
    /// True for a view, which supports reads and nothing else.
    pub view: bool,
    /// True when actix handlers are being generated for this model.
    ///
    /// Carried on the model because the module root has to declare the `routes` submodule,
    /// and only the caller knows whether that file was emitted.
    pub actix: bool,
}

impl Model {
    /// Returns the single-column key used to address one row, if the model has one.
    ///
    /// Keyed lookups, updates, and deletes all address a row by one value, so a composite
    /// key means those methods cannot be generated. PostgreSQL does not expose primary-key
    /// metadata for a view, so a view projecting a conventional `id` column uses that as its
    /// read key. Views remain read-only, so the inferred key can never enable writes.
    pub fn key(&self) -> Option<&Field> {
        let mut keys = self.fields.iter().filter(|field| field.primary_key);
        let first = keys.next();

        if keys.next().is_some() {
            return None;
        }

        first.or_else(|| {
            self.view
                .then(|| self.fields.iter().find(|field| field.column == "id"))
                .flatten()
        })
    }

    /// Returns the generated type carrying optional filters for a view listing.
    pub fn all_options_name(&self) -> String {
        format!("{}AllOptions", self.name)
    }

    /// Returns the fields a caller supplies on insert.
    pub fn insertable(&self) -> Vec<&Field> {
        self.fields.iter().filter(|f| f.insertable()).collect()
    }

    /// Returns the fields an update writes.
    pub fn updatable(&self) -> Vec<&Field> {
        self.fields.iter().filter(|f| f.updatable()).collect()
    }

    /// Returns the enum types this model's columns reference, without duplicates.
    ///
    /// The module declares one Rust enum per referenced type, so this drives both the
    /// declarations and the parent's re-export style.
    pub fn enums<'a>(&self, schema: &'a Schema) -> Vec<&'a Enum> {
        let mut found: Vec<&Enum> = Vec::new();

        for field in &self.fields {
            // Matched on the resolved type name rather than the raw SQL type, so a
            // nullable or array-of-enum column is still recognised.
            let Some(item) = schema
                .enums
                .iter()
                .find(|item| field.mapping.names(&casing::to_pascal(&item.name)))
            else {
                continue;
            };

            if !found.iter().any(|seen| seen.name == item.name) {
                found.push(item);
            }
        }

        found
    }
}
