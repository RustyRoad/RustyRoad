//! Identifier casing and escaping.

use crate::generators::typescript::casing::{
    db_name_argument, escape, identifier, to_camel, to_pascal, Casing,
};

#[test]
fn snake_case_becomes_camel_case() {
    assert_eq!(to_camel("email_address"), "emailAddress");
    assert_eq!(to_camel("created_at_utc"), "createdAtUtc");
    assert_eq!(to_camel("id"), "id");
}

#[test]
fn leading_underscores_do_not_capitalize() {
    // A leading separator has nothing before it to join, so it is dropped.
    assert_eq!(to_camel("_internal"), "internal");
}

#[test]
fn pascal_case_is_used_for_type_names() {
    assert_eq!(to_pascal("users"), "Users");
    assert_eq!(to_pascal("trash_zone_mappings"), "TrashZoneMappings");
}

#[test]
fn preserve_casing_keeps_database_names() {
    assert_eq!(identifier("email_address", Casing::Preserve), "email_address");
    assert_eq!(identifier("email_address", Casing::Camel), "emailAddress");
}

#[test]
fn invalid_identifiers_are_quoted() {
    // These cannot appear unquoted as object keys.
    assert_eq!(escape("2fa_enabled"), "\"2fa_enabled\"");
    assert_eq!(escape("odd-name"), "\"odd-name\"");
    assert_eq!(escape("order"), "order");
}

#[test]
fn database_name_is_stated_only_when_casing_changes() {
    // Under preserve the identifier already equals the column name.
    assert_eq!(db_name_argument("email_address", Casing::Preserve), "");
    assert_eq!(
        db_name_argument("email_address", Casing::Camel),
        "\"email_address\""
    );
}

#[test]
fn casing_defaults_to_camel() {
    assert_eq!(Casing::parse(None), Casing::Camel);
    assert_eq!(Casing::parse(Some("preserve")), Casing::Preserve);
    assert_eq!(Casing::parse(Some("camel")), Casing::Camel);
}
