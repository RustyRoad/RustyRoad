//! Default-value rendering for column declarations.

use crate::database::introspection::Column;
use crate::generators::typescript::types;

/// Renders the default-value modifier, if the column has one worth emitting.
///
/// A serial column's `nextval` default is implied by the builder, so it is skipped.
pub(super) fn call(column: &Column) -> Option<String> {
    if column.auto_increment {
        return None;
    }
    let default = column.default.as_deref()?.trim();
    if default.is_empty() {
        return None;
    }

    // A function call or expression must stay SQL; a literal can be a TS value.
    Some(match literal(default, column) {
        Some(value) => format!(".default({value})"),
        None => format!(".default(sql`{default}`)"),
    })
}

/// Converts a Postgres literal default into a TypeScript literal.
///
/// `numeric` is carried as a string by Drizzle to avoid float precision loss, so a
/// numeric default must be quoted even though it looks like a number.
fn literal(default: &str, column: &Column) -> Option<String> {
    if default == "true" || default == "false" {
        return match types::map(&column.sql_type, false).import {
            "boolean" => Some(default.to_string()),
            builder if string_builder(builder) => Some(quote(default)),
            _ => None,
        };
    }
    if default.parse::<f64>().is_ok() {
        return match types::map(&column.sql_type, false).import {
            // Drizzle carries arbitrary-precision decimals as strings.
            "numeric" => Some(quote(default)),
            // PostgreSQL enum types fall back to `text` in the scalar mapper;
            // their labels are strings too, even when a label looks numeric.
            builder if string_builder(builder) => Some(quote(default)),
            "smallint" | "integer" | "bigint" | "real" | "doublePrecision" => {
                Some(default.to_string())
            }
            _ => None,
        };
    }

    text_literal(default)
}

/// Returns whether a Drizzle builder carries scalar values as strings.
fn string_builder(builder: &str) -> bool {
    matches!(
        builder,
        "text"
            | "varchar"
            | "char"
            | "uuid"
            | "inet"
            | "cidr"
            | "macaddr"
            | "date"
            | "time"
            | "interval"
            | "timestamp"
    )
}

fn quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "\\'"))
}

/// Extracts a quoted text default, ignoring casts that would change meaning.
///
/// Postgres renders string defaults as `'value'::type`.
fn text_literal(default: &str) -> Option<String> {
    let (quoted, cast) = default.split_once("::")?;
    let text = quoted.strip_prefix('\'')?.strip_suffix('\'')?;
    let cast = cast.trim();

    let is_text = cast.starts_with("text")
        || cast.starts_with("character")
        || cast.starts_with("varchar")
        || cast.starts_with("bpchar");

    is_text.then(|| format!("'{}'", text.replace('\'', "\\'")))
}

#[cfg(test)]
mod tests {
    use super::call;
    use crate::database::introspection::Column;

    fn column(sql_type: &str, default: &str) -> Column {
        Column {
            name: "value".to_string(),
            sql_type: sql_type.to_string(),
            json_schema: None,
            nullable: false,
            default: Some(default.to_string()),
            auto_increment: false,
        }
    }

    #[test]
    fn numeric_looking_varchar_defaults_remain_strings() {
        assert_eq!(
            call(&column("character varying(40)", "0")).as_deref(),
            Some(".default('0')")
        );
    }

    #[test]
    fn boolean_looking_text_defaults_remain_strings() {
        assert_eq!(
            call(&column("text", "false")).as_deref(),
            Some(".default('false')")
        );
    }

    #[test]
    fn numeric_and_boolean_columns_keep_native_literals() {
        assert_eq!(
            call(&column("integer", "0")).as_deref(),
            Some(".default(0)")
        );
        assert_eq!(
            call(&column("boolean", "false")).as_deref(),
            Some(".default(false)")
        );
    }

    #[test]
    fn arbitrary_precision_numeric_defaults_remain_strings() {
        assert_eq!(
            call(&column("numeric(10, 2)", "0")).as_deref(),
            Some(".default('0')")
        );
    }

    #[test]
    fn unsupported_literal_shapes_stay_as_sql() {
        assert_eq!(
            call(&column("jsonb", "0")).as_deref(),
            Some(".default(sql`0`)")
        );
    }
}
