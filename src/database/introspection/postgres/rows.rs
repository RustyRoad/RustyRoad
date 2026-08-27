//! Row readers for introspection queries.

use super::super::assemble;
use super::super::model::Column;
use sqlx::{postgres::PgRow, Row};

/// Reads a text column, defaulting to empty.
pub(super) fn text(row: &PgRow, name: &str) -> String {
    row.try_get::<String, _>(name).unwrap_or_default()
}

/// Converts a column row into `(table, column)`.
pub(super) fn column(row: &PgRow) -> Result<(String, Column), serde_json::Error> {
    let sql_type = text(row, "sql_type");
    let comment = row
        .try_get::<Option<String>, _>("column_comment")
        .ok()
        .flatten();

    Ok((
        text(row, "table_name"),
        Column {
            name: text(row, "column_name"),
            json_schema: if matches!(sql_type.as_str(), "json" | "jsonb") {
                json_schema_comment(comment.as_deref())?
            } else {
                None
            },
            sql_type,
            nullable: row.try_get("nullable").unwrap_or(true),
            default: row
                .try_get::<Option<String>, _>("default_value")
                .ok()
                .flatten(),
            auto_increment: row.try_get("auto_increment").unwrap_or(false),
        },
    ))
}

/// Parses an opt-in JSON Schema annotation from a database column comment.
///
/// Keeping the marker on one line lets a normal prose comment coexist with the
/// machine-readable contract while ensuring malformed metadata stops generation.
fn json_schema_comment(
    comment: Option<&str>,
) -> Result<Option<serde_json::Value>, serde_json::Error> {
    const MARKER: &str = "@rustyroad-json-schema";

    let Some(payload) = comment.and_then(|value| {
        value
            .lines()
            .find_map(|line| line.trim_start().strip_prefix(MARKER).map(str::trim))
    }) else {
        return Ok(None);
    };

    serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(payload)
        .map(|schema| Some(serde_json::Value::Object(schema)))
}

/// Reads a referential action code, which Postgres returns as a single char.
pub(super) fn action(row: &PgRow, name: &str) -> Option<String> {
    let code = row
        .try_get::<i8, _>(name)
        .map(|byte| (byte as u8 as char).to_string())
        .unwrap_or_default();
    assemble::action(&code)
}

#[cfg(test)]
mod tests {
    use super::json_schema_comment;

    #[test]
    fn json_schema_annotation_can_follow_prose() {
        let parsed = json_schema_comment(Some(
            "Platform payload.\n@rustyroad-json-schema {\"type\":\"object\",\"properties\":{\"city\":{\"type\":\"string\"}}}",
        ))
        .expect("valid annotation")
        .expect("schema present");

        assert_eq!(parsed["properties"]["city"]["type"], "string");
    }

    #[test]
    fn ordinary_comments_have_no_schema() {
        assert!(json_schema_comment(Some("ordinary documentation"))
            .expect("comment parses")
            .is_none());
    }

    #[test]
    fn malformed_annotations_are_rejected() {
        assert!(json_schema_comment(Some("@rustyroad-json-schema {broken")).is_err());
        assert!(
            json_schema_comment(Some("@rustyroad-json-schema [\"schemas must be objects\"]"))
                .is_err()
        );
    }
}
