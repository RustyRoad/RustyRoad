//! Scratch-directory generation checks; no migration is executed against a database.

use super::{history, storage};
use crate::model_audit::schema::types::Proposal;

/// A known safe additive proposal for deterministic writer tests.
fn proposal() -> Proposal {
    Proposal {
        model_id: "src/model.rs::Row".into(),
        table: "rows".into(),
        operation: "add_columns".into(),
        columns: vec!["note".into()],
        up: "ALTER TABLE \"public\".\"rows\" ADD COLUMN \"note\" text;\n".into(),
        down: "ALTER TABLE \"public\".\"rows\" DROP COLUMN \"note\";\n".into(),
        warnings: vec![],
    }
}

#[test]
fn writes_canonical_files_once_and_refuses_edited_retry() {
    let root = tempfile::tempdir().unwrap();
    let proposal = proposal();
    let first = storage::write(root.path(), "public", &proposal).unwrap();
    assert_eq!(first.state, "generated");
    assert_eq!(
        first.directory.parent().unwrap(),
        root.path().join("config/database/migrations")
    );
    assert_eq!(
        std::fs::read_to_string(first.directory.join("up.sql")).unwrap(),
        proposal.up
    );
    let retry = storage::write(root.path(), "public", &proposal).unwrap();
    assert_eq!(retry.state, "already_generated");
    assert_eq!(retry.directory, first.directory);
    std::fs::write(first.directory.join("down.sql"), "-- user edit\n").unwrap();
    assert!(storage::write(root.path(), "public", &proposal)
        .unwrap_err()
        .contains("edited or incomplete"));
    assert_eq!(
        std::fs::read_to_string(first.directory.join("down.sql")).unwrap(),
        "-- user edit\n"
    );
    assert!(!root
        .path()
        .join("config/database/migrations/.model-audit.lock")
        .exists());
}

#[test]
fn pending_canonical_addition_is_not_duplicated() {
    let root = tempfile::tempdir().unwrap();
    let dir = root
        .path()
        .join("config/database/migrations/20260101000000-add_note");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("up.sql"),
        "ALTER TABLE public.rows ADD COLUMN note TEXT;",
    )
    .unwrap();
    let result = history::check(root.path(), "public", &proposal());
    assert!(
        result.is_err(),
        "proposal parse: {:?}",
        crate::database::migrations::parse_sql_operations(&proposal().up)
    );
    let error = result.unwrap_err();
    assert!(error.contains("overlapping DDL"), "{error}");
}

#[test]
fn existing_unowned_identical_sql_is_not_silently_duplicated() {
    let root = tempfile::tempdir().unwrap();
    let dir = root
        .path()
        .join("config/database/migrations/20260101000000-add_note");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("up.sql"), proposal().up).unwrap();
    assert!(history::check(root.path(), "public", &proposal()).is_err());
}

#[test]
fn stale_generation_lock_fails_closed() {
    let root = tempfile::tempdir().unwrap();
    let dir = root.path().join("config/database/migrations");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join(".model-audit.lock"), "held elsewhere").unwrap();
    assert!(storage::write(root.path(), "public", &proposal())
        .unwrap_err()
        .contains("lock"));
    assert_eq!(
        std::fs::read_to_string(dir.join(".model-audit.lock")).unwrap(),
        "held elsewhere"
    );
}

#[cfg(unix)]
#[test]
fn symlinked_migration_destination_is_not_followed() {
    let root = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    std::os::unix::fs::symlink(outside.path(), root.path().join("config")).unwrap();
    assert!(storage::write(root.path(), "public", &proposal()).is_err());
    assert_eq!(std::fs::read_dir(outside.path()).unwrap().count(), 0);
}
