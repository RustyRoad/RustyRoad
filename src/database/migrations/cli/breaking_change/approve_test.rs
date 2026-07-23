use super::approve::changes;
use crate::database::migrations::{BreakingChangeFinding, BreakingChangeKind};
use std::path::PathBuf;

#[test]
fn explicit_override_bypasses_confirmation() {
    let findings = vec![BreakingChangeFinding {
        path: PathBuf::from("up.sql"),
        line: 1,
        kind: BreakingChangeKind::ColumnTypeChange,
        sql: "ALTER COLUMN customer_id TYPE BIGINT".to_string(),
    }];
    assert!(changes(&findings, true).is_ok());
}
