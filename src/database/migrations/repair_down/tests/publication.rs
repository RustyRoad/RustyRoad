use super::super::{plan, publication, storage, Pending, Status};
use super::fixtures::{migration, CREATE, NAME};
use std::fs;

#[test]
fn changed_source_blocks_publication() {
    let root = tempfile::tempdir().unwrap();
    let directory = migration(root.path(), NAME, CREATE);
    fs::write(directory.join("up.sql"), "CREATE TABLE different (id INT);").unwrap();
    assert!(storage::publish(&directory, CREATE.as_bytes(), "DROP TABLE example;").is_err());
    assert!(!directory.join("down.sql").exists());
}

#[test]
fn existing_destination_is_never_replaced() {
    let root = tempfile::tempdir().unwrap();
    let directory = migration(root.path(), NAME, CREATE);
    fs::write(directory.join("down.sql"), "existing\n").unwrap();
    assert!(storage::publish(&directory, CREATE.as_bytes(), "DROP TABLE example;").is_err());
    assert_eq!(
        fs::read_to_string(directory.join("down.sql")).unwrap(),
        "existing\n"
    );
}

#[test]
fn all_sources_are_preflighted_before_any_publication() {
    let root = tempfile::tempdir().unwrap();
    let first = migration(root.path(), "a-first", CREATE);
    let second = migration(root.path(), "b-second", CREATE);
    let (a, source_a) = plan::entry(first.clone(), None);
    let (b, source_b) = plan::entry(second.clone(), None);
    let mut entries = vec![a, b];
    let pending = vec![
        Pending {
            entry: 0,
            source: source_a.unwrap(),
        },
        Pending {
            entry: 1,
            source: source_b.unwrap(),
        },
    ];
    fs::write(second.join("up.sql"), "changed since planning").unwrap();
    publication::write(&mut entries, &pending);
    assert!(matches!(entries[0].status, Status::Ready));
    assert!(matches!(entries[1].status, Status::Blocked));
    assert!(!first.join("down.sql").exists());
    assert!(!second.join("down.sql").exists());
}

#[test]
fn empty_undo_and_nonregular_source_are_rejected() {
    let root = tempfile::tempdir().unwrap();
    let directory = migration(root.path(), NAME, CREATE);
    assert!(storage::publish(&directory, CREATE.as_bytes(), "  \n").is_err());
    let other = root.path().join("source-directory");
    fs::create_dir(&other).unwrap();
    assert!(storage::read(&other).is_err());
    assert!(!directory.join("down.sql").exists());
}
