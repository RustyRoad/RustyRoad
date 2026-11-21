use super::super::{repair, Status};
use super::fixtures::{migration, CREATE, NAME};
use std::fs;

#[test]
fn matching_review_is_used_without_requiring_statement_parser_support() {
    let root = tempfile::tempdir().unwrap();
    let reviews = tempfile::tempdir().unwrap();
    let source = "CREATE FUNCTION example() RETURNS void LANGUAGE plpgsql AS $$ BEGIN END $$;";
    let directory = migration(root.path(), NAME, source);
    let review = migration(reviews.path(), NAME, source);
    let undo = "DO $$ BEGIN PERFORM 1; END $$;\nDROP FUNCTION example();\n";
    fs::write(review.join("down.sql"), undo).unwrap();
    let entries = repair(root.path(), Some(reviews.path()), true).unwrap();
    assert!(matches!(entries[0].status, Status::Written));
    assert_eq!(
        fs::read_to_string(directory.join("up.sql")).unwrap(),
        source
    );
    assert_eq!(
        fs::read_to_string(directory.join("down.sql")).unwrap(),
        undo
    );
    assert_eq!(fs::read_to_string(review.join("down.sql")).unwrap(), undo);
}

#[test]
fn stale_or_incomplete_review_blocks_instead_of_falling_back_to_inference() {
    for stale in [false, true] {
        let root = tempfile::tempdir().unwrap();
        let reviews = tempfile::tempdir().unwrap();
        let directory = migration(root.path(), NAME, CREATE);
        let review = migration(
            reviews.path(),
            NAME,
            if stale { "stale source" } else { CREATE },
        );
        if stale {
            fs::write(review.join("down.sql"), "DROP TABLE example;").unwrap();
        }
        let entries = repair(root.path(), Some(reviews.path()), true).unwrap();
        assert!(matches!(entries[0].status, Status::Blocked));
        assert!(!directory.join("down.sql").exists());
    }
}

#[test]
fn empty_comment_only_invalid_utf8_and_nul_review_are_blocked() {
    for undo in [
        b"".as_slice(),
        b"  -- only a comment\n; /* another comment */ ;".as_slice(),
        b"DROP TABLE example;\0".as_slice(),
        b"\xff".as_slice(),
    ] {
        let root = tempfile::tempdir().unwrap();
        let reviews = tempfile::tempdir().unwrap();
        let directory = migration(root.path(), NAME, CREATE);
        let review = migration(reviews.path(), NAME, CREATE);
        fs::write(review.join("down.sql"), undo).unwrap();
        let entries = repair(root.path(), Some(reviews.path()), true).unwrap();
        assert!(matches!(entries[0].status, Status::Blocked), "{undo:?}");
        assert!(!directory.join("down.sql").exists());
    }
}

#[test]
fn matching_review_takes_precedence_over_inference() {
    let root = tempfile::tempdir().unwrap();
    let reviews = tempfile::tempdir().unwrap();
    let directory = migration(root.path(), NAME, CREATE);
    let review = migration(reviews.path(), NAME, CREATE);
    let undo = "-- Explicit reviewed choice\nDROP TABLE example;\n";
    fs::write(review.join("down.sql"), undo).unwrap();
    let entries = repair(root.path(), Some(reviews.path()), false).unwrap();
    assert!(matches!(entries[0].status, Status::Ready));
    assert_eq!(entries[0].sql.as_deref(), Some(undo));
    assert!(!directory.join("down.sql").exists());
}
