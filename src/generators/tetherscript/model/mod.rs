//! The resolved shape of one generated TetherScript model.
//!
//! Resolved once so the renderers agree: the column list `new` seeds is the list `create`
//! binds, in the same order and with the same names.

mod field;
mod resolve;

pub use field::Field;

/// A table resolved into everything needed to emit its model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Model {
    /// The table name, used verbatim in SQL.
    pub table: String,
    /// The module folder, singular and snake_case.
    pub module: String,
    /// The model's display name, used in error messages.
    pub name: String,
    pub fields: Vec<Field>,
}

impl Model {
    /// Returns the single-column primary key, if the table has one.
    pub fn key(&self) -> Option<&Field> {
        let mut keys = self.fields.iter().filter(|field| field.primary_key);
        let first = keys.next()?;

        if keys.next().is_some() {
            return None;
        }
        Some(first)
    }

    /// Returns the fields a caller supplies on insert.
    pub fn insertable(&self) -> Vec<&Field> {
        self.fields.iter().filter(|f| f.insertable()).collect()
    }

    /// Returns the fields an update writes.
    pub fn updatable(&self) -> Vec<&Field> {
        self.fields.iter().filter(|f| f.updatable()).collect()
    }

    /// Returns the fields validation requires a value for.
    pub fn required(&self) -> Vec<&Field> {
        self.fields.iter().filter(|f| f.required()).collect()
    }
}
