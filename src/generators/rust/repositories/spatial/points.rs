//! Point coordinate reads keep computation, filtering and ordering in PostgreSQL.
use super::{Column, Names, Table};
use crate::generators::rust::{naming::sql, repositories::equality};

/// Selects only declared point geometries, never lines, polygons or unknown subtypes.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(in crate::generators::rust) fn columns(table: &Table) -> Vec<&Column> {
    super::columns(table)
        .into_iter()
        .filter(|column| {
            let kind = column.sql_type.trim().to_ascii_lowercase();
            let subtype = kind
                .strip_prefix("geometry(")
                .unwrap_or("")
                .split([',', ')'])
                .next()
                .unwrap_or("");
            matches!(subtype, "point" | "pointz" | "pointm" | "pointzm")
        })
        .collect()
}

/// Emits bounded (Y, X) reads with explicit direction and NULL ordering.
/// Coordinates retain the stored CRS; this operation does not transform them.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(table: &Table, names: &Names) -> String {
    let keys = equality::keys(table);
    if keys.is_empty() {
        return String::new();
    }
    let order_columns = keys
        .iter()
        .map(|key| format!("{:?} => {:?},", key[0], sql(&key[0])))
        .collect::<Vec<_>>()
        .join("\n");
    let filter = names.filter(table);
    let bindings = equality::aggregate_bindings(table, names);
    let mut output = String::new();
    for column in columns(table) {
        let function = names.selector(
            table,
            "point_coordinates",
            std::slice::from_ref(&column.name),
        );
        let geometry = sql(&column.name);
        let prefix = format!(
            "SELECT ST_Y({geometry}), ST_X({geometry}) FROM {} WHERE {geometry} IS NOT NULL",
            sql(&table.name)
        );
        output.push_str(&format!(
            "/// Returns (Y, X) coordinates of matching non-null point geometries in their stored CRS.\n\
             /// Example: `{function}(pool, {filter}::default(), {:?}, false, false, Some(1), None).await?`.\n\
             pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, filter: {filter}, order_by: &str, ascending: bool, nulls_first: bool, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<(Option<f64>, Option<f64>)>, sqlx::Error> {{\n\
                 let order = match order_by {{\n{order_columns}\n\
                     _ => return Err(sqlx::Error::Protocol(format!(\"Unsupported order column: {{order_by}}\"))),\n\
                 }};\n\
                 let mut query = QueryBuilder::<Postgres>::new({prefix:?});\n\
                 {bindings}\n\
                 query.push(\" ORDER BY \").push(order);\n\
                 query.push(if ascending {{ \" ASC\" }} else {{ \" DESC\" }});\n\
                 query.push(if nulls_first {{ \" NULLS FIRST\" }} else {{ \" NULLS LAST\" }});\n\
                 if let Some(limit) = limit {{ query.push(\" LIMIT \").push_bind(limit); }}\n\
                 if let Some(offset) = offset {{ query.push(\" OFFSET \").push_bind(offset); }}\n\
                 query.build_query_as::<(Option<f64>, Option<f64>)>().fetch_all(executor).await\n\
             }}\n\n", keys[0][0]
        ));
    }
    output
}
