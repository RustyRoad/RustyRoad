//! Shared SQL rendering for `migration generate` and catalog-backed model audits.

/// Render a create/drop pair from already parsed identifiers and column definitions.
/// Audits use strict DDL; the legacy CLI retains its conditional-existence behavior.
pub(crate) fn create_table(table: &str, columns: &[String], conditional: bool) -> (String, String) {
    let create_guard = if conditional { "IF NOT EXISTS " } else { "" };
    let drop_guard = if conditional { "IF EXISTS " } else { "" };
    (
        format!(
            "CREATE TABLE {create_guard}{table} (\n    {}\n);",
            columns.join(",\n    ")
        ),
        format!("DROP TABLE {drop_guard}{table};"),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn cli_and_audit_share_column_rendering() {
        let columns = vec!["id integer NOT NULL".into()];
        let (up, down) = super::create_table("rows", &columns, false);
        let (cli, _) = super::create_table("rows", &columns, true);
        assert_eq!(cli.replace("IF NOT EXISTS ", ""), up);
        assert_eq!(down, "DROP TABLE rows;");
    }
}
