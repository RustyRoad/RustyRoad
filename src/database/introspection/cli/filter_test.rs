//! `--tables` and `--exclude` narrow the schema before anything is generated.

use super::filter_fixture::run;

#[test]
fn no_flags_keeps_everything() {
    assert_eq!(run(&[]).len(), 6);
}

#[test]
fn tables_keeps_only_what_is_named() {
    assert_eq!(
        run(&["--tables", "customers", "--tables", "orders"]),
        vec!["customers", "orders"]
    );
}

#[test]
fn a_trailing_star_matches_a_prefix() {
    assert_eq!(
        run(&["--exclude", "ab_test_*"]),
        vec!["customers", "orders", "user", "users"]
    );
}

#[test]
fn exclude_is_exact_without_a_star() {
    // Dropping `user` must not also drop `users`: an accidental prefix match here would
    // silently discard a real table.
    assert_eq!(
        run(&["--exclude", "user"]),
        vec![
            "customers",
            "orders",
            "ab_test_archive",
            "ab_test_queue",
            "users"
        ]
    );
}

#[test]
fn exclude_applies_after_tables() {
    assert_eq!(
        run(&["--tables", "ab_test_*", "--exclude", "ab_test_queue"]),
        vec!["ab_test_archive"]
    );
}
