use super::super::{repair, storage, Status};
use super::fixtures::{migration, CREATE, NAME};
use std::fs;
use std::os::unix::fs::symlink;

#[test]
fn dangling_undo_symlink_is_existing_and_never_replaced() {
    let root = tempfile::tempdir().unwrap();
    let directory = migration(root.path(), NAME, CREATE);
    let missing = root.path().join("missing-target");
    let undo = directory.join("down.sql");
    symlink(&missing, &undo).unwrap();
    let entries = repair(&directory, None, true).unwrap();
    assert!(matches!(entries[0].status, Status::Existing));
    assert_eq!(fs::read_link(&undo).unwrap(), missing);
    assert!(!missing.exists());
    assert_eq!(
        fs::read_to_string(directory.join("up.sql")).unwrap(),
        CREATE
    );
}

#[test]
fn symlinked_source_is_blocked_without_touching_target() {
    let root = tempfile::tempdir().unwrap();
    let directory = root.path().join(NAME);
    fs::create_dir(&directory).unwrap();
    let source = root.path().join("source.sql");
    fs::write(&source, CREATE).unwrap();
    symlink(&source, directory.join("up.sql")).unwrap();
    let entries = repair(&directory, None, true).unwrap();
    assert!(matches!(entries[0].status, Status::Blocked));
    assert!(!directory.join("down.sql").exists());
    assert_eq!(fs::read_to_string(source).unwrap(), CREATE);
}

#[test]
fn symlinked_migration_and_review_directories_are_rejected() {
    let root = tempfile::tempdir().unwrap();
    let external = tempfile::tempdir().unwrap();
    let target = migration(external.path(), NAME, CREATE);
    let link = root.path().join(NAME);
    symlink(&target, &link).unwrap();
    assert!(repair(root.path(), None, true).is_err());
    assert!(repair(&link, None, true).is_err());
    assert!(!target.join("down.sql").exists());
    let source_root = tempfile::tempdir().unwrap();
    let directory = migration(source_root.path(), NAME, CREATE);
    let reviews = tempfile::tempdir().unwrap();
    symlink(&target, reviews.path().join(NAME)).unwrap();
    let entries = repair(&directory, Some(reviews.path()), true).unwrap();
    assert!(matches!(entries[0].status, Status::Blocked));
    assert!(!directory.join("down.sql").exists());
    assert!(repair(&directory, Some(&link), true).is_err());
}

#[test]
fn symlinked_review_file_is_not_used() {
    let root = tempfile::tempdir().unwrap();
    let reviews = tempfile::tempdir().unwrap();
    let directory = migration(root.path(), NAME, CREATE);
    let review = migration(reviews.path(), NAME, CREATE);
    let target = reviews.path().join("undo.sql");
    fs::write(&target, "DROP TABLE example;\n").unwrap();
    symlink(&target, review.join("down.sql")).unwrap();
    let entries = repair(&directory, Some(reviews.path()), true).unwrap();
    assert!(matches!(entries[0].status, Status::Blocked));
    assert!(!directory.join("down.sql").exists());
    assert_eq!(fs::read_to_string(target).unwrap(), "DROP TABLE example;\n");
}

#[test]
fn symlink_appearing_after_planning_blocks_publication() {
    let root = tempfile::tempdir().unwrap();
    let directory = migration(root.path(), NAME, CREATE);
    let target = root.path().join("missing-target");
    symlink(&target, directory.join("down.sql")).unwrap();
    assert!(storage::publish(&directory, CREATE.as_bytes(), "DROP TABLE example;").is_err());
    assert_eq!(fs::read_link(directory.join("down.sql")).unwrap(), target);
    assert!(!target.exists());
}
