//! MySQL `enum(...)` column-type parsing.

/// Splits a MySQL `enum('a','b','c')` column type into its value list.
///
/// MySQL reports the full `enum(...)` expression as the column type, so the
/// values have to be recovered from that string.
pub(crate) fn parse_mysql_enum_values(column_type: &str) -> Vec<String> {
    let inner = column_type
        .trim()
        .strip_prefix("enum(")
        .and_then(|rest| rest.strip_suffix(')'))
        .unwrap_or("");

    inner
        .split("','")
        .map(|value| value.trim().trim_matches('\'').to_string())
        .filter(|value| !value.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::parse_mysql_enum_values;

    #[test]
    fn parses_mysql_enum_column_type() {
        assert_eq!(
            parse_mysql_enum_values("enum('pending','shipped','delivered')"),
            vec!["pending", "shipped", "delivered"]
        );
    }

    #[test]
    fn handles_empty_enum_and_non_enum_types() {
        assert_eq!(parse_mysql_enum_values("enum()"), Vec::<String>::new());
        assert_eq!(
            parse_mysql_enum_values("varchar(255)"),
            Vec::<String>::new()
        );
    }
}
