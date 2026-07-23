use super::BreakingChangeKind;

pub(super) fn classify(line: &str) -> Vec<BreakingChangeKind> {
    let sql = normalize(line);
    if sql.is_empty() || sql.starts_with("--") {
        return Vec::new();
    }
    let mut kinds = Vec::new();
    if is_type_change(&sql) {
        kinds.push(BreakingChangeKind::ColumnTypeChange);
    }
    if sql.contains(" USING ") || sql.contains("CAST(") || sql.contains("::") {
        kinds.push(BreakingChangeKind::ExplicitCast);
    }
    if sql.contains("DROP CONSTRAINT") || sql.contains("DROP FOREIGN KEY") {
        kinds.push(BreakingChangeKind::ConstraintDrop);
    }
    kinds
}

fn normalize(line: &str) -> String {
    line.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_uppercase()
}

fn is_type_change(sql: &str) -> bool {
    sql.contains("ALTER COLUMN") && (sql.contains(" TYPE ") || sql.ends_with(" TYPE"))
        || sql.contains(" MODIFY COLUMN ")
        || sql.starts_with("MODIFY COLUMN ")
        || sql.contains(" CHANGE COLUMN ")
        || sql.starts_with("CHANGE COLUMN ")
}
