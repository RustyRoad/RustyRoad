//! Tests for migration folder naming.

use super::folder::folder_for;
use std::fs;

/// Creates a unique scratch directory under the target folder.
fn scratch(label: &str) -> String {
    let path = format!(
        "target/folder-test-{label}-{}",
        std::process::id()
    );
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).expect("failed to create scratch directory");
    path
}

#[test]
fn folder_carries_timestamp_and_name() {
    let dir = scratch("basic");
    let folder = folder_for(&dir, "create_users");

    assert!(folder.starts_with(&format!("{dir}/")));
    assert!(folder.ends_with("-create_users"));

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn repeated_names_in_one_second_get_distinct_folders() {
    let dir = scratch("burst");

    // Agents commonly retry a generate, landing inside the same second. The
    // timestamp alone cannot separate those, so a suffix must.
    let first = folder_for(&dir, "same_name");
    fs::create_dir_all(&first).unwrap();

    let second = folder_for(&dir, "same_name");
    assert_ne!(first, second);
    fs::create_dir_all(&second).unwrap();

    let third = folder_for(&dir, "same_name");
    assert_ne!(third, first);
    assert_ne!(third, second);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn suffixed_folders_keep_the_timestamp_prefix_shape() {
    let dir = scratch("shape");

    let first = folder_for(&dir, "orders");
    fs::create_dir_all(&first).unwrap();
    let second = folder_for(&dir, "orders");

    // Directory parsing splits on the first hyphen, so the prefix must survive.
    let name = second.rsplit('/').next().unwrap();
    let (timestamp, rest) = name.split_once('-').expect("expected <timestamp>-<name>");
    assert_eq!(timestamp.len(), 14);
    assert!(timestamp.chars().all(|c| c.is_ascii_digit()));
    assert!(rest.starts_with("orders"));

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn missing_parent_directories_do_not_block_naming() {
    // Naming must not depend on the tree existing; creation builds it.
    let folder = folder_for("target/does/not/exist/yet", "fresh");
    assert!(folder.contains("-fresh"));
}
