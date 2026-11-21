use crate::database::introspection::{Column, Schema, Table};
use crate::generators::rust::naming::sql;

/// Supplies the database enum cast for a bound text parameter, or no cast for text.
/// Arrays and unsupported scalar types are rejected before rendering.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn cast(schema: &Schema, column: &Column) -> Option<String> {
    let declared = column.sql_type.trim().trim_matches('"');
    if let Some(item) = schema.enum_type(declared) {
        return Some(format!("::{}", sql(&item.name)));
    }
    let kind = declared.to_ascii_lowercase();
    if !kind.ends_with("[]")
        && (kind == "text"
            || kind == "varchar"
            || kind == "character varying"
            || kind.starts_with("varchar(")
            || kind.starts_with("character varying("))
    {
        Some(String::new())
    } else {
        None
    }
}

/// Rejects configured labels absent from the catalog enum; arbitrary text stays valid.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn valid_label(schema: &Schema, column: &Column, label: &str) -> bool {
    schema
        .enum_type(column.sql_type.trim().trim_matches('"'))
        .map_or(true, |item| item.values.iter().any(|value| value == label))
}

/// Accepts scalar PostgreSQL timestamp codecs for nullable MAX projections.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn timestamp(column: &Column) -> bool {
    let kind = column.sql_type.trim().to_ascii_lowercase();
    let normalized = match (kind.find('('), kind.rfind(')')) {
        (Some(start), Some(end)) => format!("{}{}", &kind[..start], &kind[end + 1..]),
        _ => kind,
    };
    matches!(
        normalized.trim(),
        "timestamp" | "timestamptz" | "timestamp without time zone" | "timestamp with time zone"
    )
}

/// Emits one optional runtime text predicate with a schema-derived enum cast.
/// The supplied value remains bound; PostgreSQL validates runtime enum labels.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn runtime(schema: &Schema, table: &Table, column: Option<&str>) -> (String, String) {
    match column {
        None => (String::new(), String::new()),
        Some(name) => {
            let start = format!(" AND {} = ", sql(name));
            let cast = cast(schema, table.column(name).unwrap()).unwrap();
            (
                ", value: String".into(),
                format!("query.push({start:?}).push_bind(value).push({cast:?});\n"),
            )
        }
    }
}
