//! Dedicated insertion inputs retain NULL separately from omitted defaults.
use super::{codecs, configuration::InsertInput};
use crate::database::introspection::{Schema, Table};
use crate::generators::rust::{allocation::Names, naming::pascal, repositories::inputs, types};

/// Derive a configured type name; the dispatcher rejects namespace collisions.
/// # Examples
/// ```toml
/// [models.rust_projections.insert_consent]
/// table = "sms_consent_audit"
/// columns = ["id"]
/// insert = true
/// insert_input = {} # emits InsertConsentInput
/// ```
pub(in crate::generators::rust::projections) fn name(function: &str) -> String {
    format!("{}Input", pascal(function))
}

/// Emit a native input; every nullable field has independent presence and value.
/// The generated example explicitly binds NULL for every writable nullable column.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(
    schema: &Schema,
    names: &Names,
    function: &str,
    table: &Table,
    input: &InsertInput,
) -> String {
    let mut fields = Vec::new();
    let mut assignments = Vec::new();
    for column in inputs::new_columns(table) {
        let field = names.field(table, &column.name);
        let mut required = column.clone();
        required.nullable = false;
        let base = codecs::cast(column, input)
            .map(|(ty, _)| ty.to_string())
            .unwrap_or_else(|| types::map_column(table, &required, schema, names).rust);
        let (ty, doc) = if column.nullable {
            assignments.push(format!(
                "/// input.{field} = Some(None); // explicit SQL NULL\n"
            ));
            (
                format!("Option<Option<{base}>>"),
                "None omits; Some(None) binds NULL; Some(Some(v)) binds v.",
            )
        } else if inputs::optional(column) {
            assignments.push(format!(
                "/// input.{field} = None; // omitted default/configured clock\n"
            ));
            (
                format!("Option<{base}>"),
                "None omits for a default/configured clock; Some(v) binds v.",
            )
        } else {
            (
                base,
                "Required bound value; SQL NULL and omission are not accepted.",
            )
        };
        fields.push(format!("    /// {doc}\n    pub {field}: {ty},"));
    }
    let input = name(function);
    let fields = fields.join("\n");
    let assignments = assignments.join("");
    format!(
        "/// Native input for `{function}`; ordinary New inputs are unchanged.\n\
         /// Input casts are independent of RETURNING casts; PostgreSQL performs conversion.\n\
         /// Omitted fields use defaults, except explicitly configured database clocks.\n\
         /// # Examples\n\
         /// ```no_run\n\
         /// # async fn example(connection: &mut sqlx::PgConnection, mut input: {input}) -> Result<(), sqlx::Error> {{\n\
         {assignments}\
         /// let _row = {function}(connection, input).await?;\n\
         /// # Ok(()) }}\n\
         /// ```\n\
         #[derive(Debug, Clone)]\n\
         pub struct {input} {{\n{fields}\n}}\n\n"
    )
}
