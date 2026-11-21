use super::super::{repair, Status};
use super::fixtures::{migration, CREATE, NAME};
use std::fs;

#[test]
fn preview_is_read_only_sorted_and_preserves_existing_bytes() {
    let root = tempfile::tempdir().unwrap();
    let existing = migration(root.path(), "z-existing", "not parseable SQL");
    let bytes = [0xff, 0x00, 0x0a];
    fs::write(existing.join("down.sql"), bytes).unwrap();
    let missing = migration(root.path(), "a-missing", CREATE);
    fs::write(root.path().join("marker"), "unchanged").unwrap();
    let entries = repair(root.path(), None, false).unwrap();
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].directory, missing);
    assert!(matches!(entries[0].status, Status::Ready));
    assert!(matches!(entries[1].status, Status::Existing));
    assert!(!missing.join("down.sql").exists());
    assert_eq!(fs::read(existing.join("down.sql")).unwrap(), bytes);
    assert_eq!(fs::read_to_string(missing.join("up.sql")).unwrap(), CREATE);
    assert_eq!(
        fs::read_to_string(root.path().join("marker")).unwrap(),
        "unchanged"
    );
}

#[test]
fn write_creates_only_missing_undo_and_rerun_skips_it() {
    let root = tempfile::tempdir().unwrap();
    let missing = migration(root.path(), NAME, CREATE);
    let existing = migration(root.path(), "z-existing", CREATE);
    fs::write(existing.join("down.sql"), "human-reviewed undo\n").unwrap();
    let entries = repair(root.path(), None, true).unwrap();
    assert!(matches!(entries[0].status, Status::Written));
    assert!(matches!(entries[1].status, Status::Existing));
    let undo = fs::read(missing.join("down.sql")).unwrap();
    assert_eq!(fs::read_to_string(missing.join("up.sql")).unwrap(), CREATE);
    assert_eq!(
        fs::read_to_string(existing.join("down.sql")).unwrap(),
        "human-reviewed undo\n"
    );
    assert_eq!(fs::read_dir(&missing).unwrap().count(), 2);
    let rerun = repair(root.path(), None, true).unwrap();
    assert!(rerun
        .iter()
        .all(|entry| matches!(entry.status, Status::Existing)));
    assert_eq!(fs::read(missing.join("down.sql")).unwrap(), undo);
}

#[test]
fn one_planning_blocker_prevents_every_write() {
    let root = tempfile::tempdir().unwrap();
    let ready = migration(root.path(), "a-ready", CREATE);
    let blocked = migration(root.path(), "b-blocked", "UPDATE example SET id = 1;");
    let entries = repair(root.path(), None, true).unwrap();
    assert!(matches!(entries[0].status, Status::Ready));
    assert!(matches!(entries[1].status, Status::Blocked));
    assert!(entries[1].sql.is_none());
    assert!(!ready.join("down.sql").exists());
    assert!(!blocked.join("down.sql").exists());
}

#[test]
fn single_directory_is_supported() {
    let root = tempfile::tempdir().unwrap();
    let directory = migration(root.path(), NAME, CREATE);
    assert_eq!(repair(&directory, None, false).unwrap().len(), 1);
}
