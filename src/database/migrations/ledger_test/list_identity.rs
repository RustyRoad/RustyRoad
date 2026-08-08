use crate::database::migrations::ledger::{display_name, identities_match};

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
