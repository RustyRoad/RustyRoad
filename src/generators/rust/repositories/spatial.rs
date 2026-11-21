//! PostGIS containment uses the same configured codec as the geometry column.
mod points;
use super::super::{allocation::Names, types};
use crate::database::introspection::{Column, Schema, Table};
pub(in crate::generators::rust) use points::columns as point_columns;

/// Finds scalar geometry columns without treating arrays or similarly named types as geometry.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(in crate::generators::rust) fn columns(table: &Table) -> Vec<&Column> {
    table
        .columns
        .iter()
        .filter(|column| {
            let kind = column.sql_type.trim().to_ascii_lowercase();
            kind == "geometry" || (kind.starts_with("geometry(") && kind.ends_with(')'))
        })
        .collect()
}

/// Emits parameterized containment calls with an explicit coordinate SRID.
/// The supplied geometry stays in its native SQLx codec; no text/JSON conversion is added.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(table: &Table, schema: &Schema, names: &Names) -> String {
    let mut output = String::new();
    for column in columns(table) {
        let mut required = column.clone();
        required.nullable = false;
        let geometry_type = types::map_column(table, &required, schema, names).rust;
        let function = names.selector(table, "contains_point", std::slice::from_ref(&column.name));
        output.push_str(&format!(
            "/// Checks geometry containment of an x/y point in the supplied SRID.\n\
             /// Geometry/database errors propagate; coordinates and geometry are bound values.\n\
             /// Example: `{function}(pool, &polygon, longitude, latitude, 4326).await?`.\n\
             pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, geometry: &'e {geometry_type}, x: f64, y: f64, srid: i32) -> Result<bool, sqlx::Error> {{\n\
                 sqlx::query_scalar::<_, bool>(\"SELECT ST_Contains($1::geometry, ST_SetSRID(ST_MakePoint($2, $3), $4))\")\n\
                     .bind(geometry).bind(x).bind(y).bind(srid).fetch_one(executor).await\n\
             }}\n\n"
        ));
    }
    output.push_str(&points::render(table, names));
    output
}
