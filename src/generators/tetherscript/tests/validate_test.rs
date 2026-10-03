//! The generated validation, and that writes go through it.

use super::rendered::file;

#[test]
fn validation_requires_the_columns_a_struct_would_have_made_non_optional() {
    let root = file("mod.tether");

    assert!(root.contains("let required = [\"name\", \"active\", \"attributes\"]"));
    // The generated key is not required: the database assigns it.
    assert!(!root.contains("\"id\", \"name\""));
}

#[test]
fn validation_checks_the_kind_each_column_decodes_to() {
    let root = file("mod.tether");

    assert!(root.contains("kinds[\"name\"] = \"str\""));
    assert!(root.contains("kinds[\"active\"] = \"bool\""));
    assert!(root.contains("kinds[\"price\"] = \"float\""));
    assert!(root.contains("kinds[\"metadata\"] = \"map\""));
    assert!(root.contains("kinds[\"attributes\"] = \"list\""));
    // A nil is absence, which the required check rules on, so it must be exempt here.
    assert!(root.contains("if value != nil && type_of(value) != kinds[column]"));
}

#[test]
fn writes_validate_before_reaching_sql() {
    let root = file("mod.tether");

    // Both mutating paths must validate; a create that skipped it would push a bad value
    // to Postgres and surface as an error about a parameter number.
    let create = root.split("fn create(row) {").nth(1).unwrap();
    assert!(
        create.trim_start().starts_with("validate(row)?"),
        "create did not validate first: {create}"
    );

    let update = root.split("fn update(key, row) {").nth(1).unwrap();
    assert!(
        update.trim_start().starts_with("validate(row)?"),
        "update did not validate first: {update}"
    );
}
