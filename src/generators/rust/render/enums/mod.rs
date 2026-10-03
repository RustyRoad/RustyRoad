//! Rust enum declarations for Postgres enum types.
//!
//! Declared once for the whole schema rather than once per table that references them: seven
//! tables share `campaign_workflow_node_kind`, and per-module copies made seven distinct Rust
//! types that could not be assigned to one another, plus eighteen ambiguous glob re-exports
//! in the parent. One declaration means one type, so a value read through one model can be
//! written through another.

mod variants;

use crate::database::introspection::Enum;
use crate::generators::rust::casing;

/// Derives every generated enum carries.
///
/// `Type` is what lets `sqlx` bind and decode the value; the rest match the struct's derives so
/// an enum field does not weaken the model's API surface.
///
/// `Default` is required rather than optional: a struct's own `Default` seeds a non-nullable
/// enum field with `Default::default()`, so without it the generated model compiles everywhere
/// except its own `Default` impl.
const DERIVES: &str = "#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, \
                       Deserialize, sqlx::Type, ToSchema, TS)]";

/// Renders the shared `enums` module: every referenced enum, declared once.
pub fn module(referenced: &[&Enum]) -> String {
    let mut file = String::from(
        "//! Postgres enum types, declared once and shared by every model.\n\n\
         use serde::{Deserialize, Serialize};\n\
         use ts_rs::TS;\n\
         use utoipa::ToSchema;\n\n",
    );

    let declarations = referenced
        .iter()
        .map(|item| declaration(item))
        .collect::<Vec<_>>()
        .join("\n");
    file.push_str(&declarations);

    file
}

/// Renders one enum declaration.
///
/// `type_name` is what ties the Rust type back to the Postgres type, so `sqlx` binds the right
/// OID rather than guessing from the identifier.
fn declaration(item: &Enum) -> String {
    format!(
        "/// The `{}` Postgres enum.\n\
         {DERIVES}\n\
         #[sqlx(type_name = \"{}\")]\n\
         pub enum {} {{\n{}}}\n",
        item.name,
        item.name,
        casing::to_pascal(&item.name),
        variants::render(item)
    )
}
