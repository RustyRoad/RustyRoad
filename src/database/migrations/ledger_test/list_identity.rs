use crate::database::migrations::ledger::{display_name, identities_match, latest_status};

#[test]
fn full_identity_matches_its_display_suffix() {
    let full = "20260723120000-change_customer_id";
    assert_eq!(display_name(full), "change_customer_id");
    assert!(identities_match(full, "change_customer_id"));
    assert!(identities_match("change_customer_id", full));
}

#[test]
fn full_identities_with_the_same_suffix_stay_distinct() {
    assert!(!identities_match(
        "20260723120000-change_customer_id",
        "20260724120000-change_customer_id"
    ));
}

#[test]
fn hyphenated_bare_name_is_not_treated_as_timestamped() {
    let bare = "change-customer-id";
    assert_eq!(display_name(bare), bare);
    assert!(!identities_match(bare, "customer-id"));
}

#[test]
fn full_ledger_identities_do_not_mark_a_sibling_timestamp_applied() {
    let rows = vec![(
        "20260101000000-add_columns".to_string(),
        "2026-01-01 00:00:00".to_string(),
        "up".to_string(),
    )];

    assert_eq!(
        latest_status("20260101000000-add_columns", &rows),
        Some(("2026-01-01 00:00:00", "up"))
    );
    assert_eq!(latest_status("20260202000000-add_columns", &rows), None);
}

#[test]
fn bare_legacy_status_applies_to_each_matching_directory() {
    let rows = vec![(
        "add_columns".to_string(),
        "2026-01-01 00:00:00".to_string(),
        "up".to_string(),
    )];

    assert_eq!(
        latest_status("20260101000000-add_columns", &rows),
        Some(("2026-01-01 00:00:00", "up"))
    );
    assert_eq!(
        latest_status("20260202000000-add_columns", &rows),
        Some(("2026-01-01 00:00:00", "up"))
    );
}

#[test]
fn exact_full_status_wins_over_a_newer_legacy_row() {
    let rows = vec![
        (
            "20260101000000-add_columns".to_string(),
            "2026-01-01 00:00:00".to_string(),
            "down".to_string(),
        ),
        (
            "add_columns".to_string(),
            "2026-02-01 00:00:00".to_string(),
            "up".to_string(),
        ),
    ];

    assert_eq!(
        latest_status("20260101000000-add_columns", &rows),
        Some(("2026-01-01 00:00:00", "down"))
    );
}
