//! Default-value rendering for column declarations.

use crate::database::introspection::Column;

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
        return Some(default.to_string());
    }
    if default.parse::<f64>().is_ok() {
        let lowered = column.sql_type.to_lowercase();
        let is_decimal = lowered.starts_with("numeric") || lowered.starts_with("decimal");
        return Some(if is_decimal {
            format!("'{default}'")
        } else {
            default.to_string()
        });
    }

    text_literal(default)
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
