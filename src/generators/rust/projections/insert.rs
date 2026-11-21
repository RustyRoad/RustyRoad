//! Atomic projected inserts using catalog-derived New inputs and SQLx binds.
mod codecs;
pub(super) mod configuration;
mod fields;
pub(super) mod input;
pub(super) mod policy;
use super::{Names, Projection};
use crate::database::introspection::{Schema, Table};
use crate::generators::rust::{
    naming::sql,
    repositories::{create_clock, inputs},
};

/// Emits one INSERT RETURNING; defaults and explicit NULL inputs retain precedence.
/// Selected casts execute in PostgreSQL, sharing the same ordered tuple as read projections.
/// The supplied executor owns transaction control; database and codec errors propagate.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(
    schema: &Schema,
    names: &Names,
    function: &str,
    table: &Table,
    projection: &Projection,
) -> String {
    let (selected, tuple) = super::selection::render(schema, names, table, projection);
    let columns = inputs::new_columns(table);
    let required = columns.iter().any(|column| !inputs::optional(column));
    let present = inputs::present(table, &columns, names);
    let has_values = if required || !projection.database_timestamps.is_empty() {
        "true".to_string()
    } else if columns.is_empty() {
        "false".to_string()
    } else {
        present
    };
    let clocks = projection
        .database_timestamps
        .iter()
        .map(|name| format!("{name:?}"))
        .collect::<Vec<_>>()
        .join(", ");
    let (create, declaration, column_pushes, value_pushes) =
        if let Some(config) = &projection.insert_input {
            let (columns, values) = fields::render(table, names, config);
            (
                input::name(function),
                input::render(schema, names, function, table, config),
                columns,
                values,
            )
        } else {
            let (columns, values) = create_clock::fields::render(table, &columns, names);
            (
                names.create(table).to_string(),
                String::new(),
                columns,
                values,
            )
        };
    let prefix = format!("INSERT INTO {} ", sql(&table.name));
    let returning = format!(" RETURNING {selected}");
    format!(
        "{declaration}/// Inserts one row and returns selected columns in configured tuple order.\n\
         /// Configured omitted timestamps use database transaction time; explicit values win.\n\
         /// JSON text, numeric float and network host projections execute in PostgreSQL.\n\
         /// Tuple fields follow configured order and catalog nullability; cast errors propagate.\n\
         /// Defaults and explicit NULLs follow the selected input type; errors propagate.\n\
         /// The supplied executor controls the transaction; this operation never commits it.\n\
         /// # Examples\n\
         /// ```no_run\n\
         /// # async fn example(pool: &sqlx::PgPool, input: {create}) -> Result<(), sqlx::Error> {{\n\
         /// let row = {function}(pool, input).await?;\n\
         /// let _ = row;\n\
         /// # Ok(()) }}\n\
         /// ```\n\
         pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, input: {create}) -> Result<{tuple}, sqlx::Error> {{\n\
             let clocks: &[&str] = &[{clocks}];\n\
             let mut query = sqlx::QueryBuilder::<sqlx::Postgres>::new({prefix:?});\n\
             if {has_values} {{\n\
                 query.push(\"(\");\n\
                 {{ let mut columns = query.separated(\", \");\n\
         {column_pushes}\n\
                 }}\n\
                 query.push(\") VALUES (\");\n\
                 {{ let mut values = query.separated(\", \");\n\
         {value_pushes}\n\
                 }}\n\
                 query.push(\")\");\n\
             }} else {{ query.push(\"DEFAULT VALUES\"); }}\n\
             query.push({returning:?});\n\
             query.build_query_as::<{tuple}>().fetch_one(executor).await\n\
         }}\n\n"
    )
}
