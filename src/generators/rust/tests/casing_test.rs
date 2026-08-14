//! Casing, keyword handling, and singularization.

use crate::generators::rust::casing::{field, singularize, to_pascal, to_snake};

#[test]
fn snake_case_passes_through_and_camel_case_is_split() {
    assert_eq!(to_snake("stripe_product_id"), "stripe_product_id");
    assert_eq!(to_snake("stripeProductId"), "stripe_product_id");
    assert_eq!(to_snake("HTTPStatus"), "httpstatus");
}

#[test]
fn a_digit_leading_name_is_prefixed_rather_than_truncated() {
    // A bare `2fa_enabled` is not a legal identifier, and dropping the digit would
    // collide with a sibling column named `fa_enabled`.
    assert_eq!(to_snake("2fa_enabled"), "f_2fa_enabled");
    assert_eq!(to_pascal("2fa_enabled"), "T2faEnabled");
}

#[test]
fn a_table_name_becomes_a_singular_type_name() {
    assert_eq!(to_pascal(&singularize("products")), "Product");
    assert_eq!(to_pascal(&singularize("categories")), "Category");
    assert_eq!(to_pascal(&singularize("addresses")), "Address");
    assert_eq!(to_pascal(&singularize("order_items")), "OrderItem");
}

#[test]
fn an_already_singular_name_survives_singularization() {
    // `status` ends in `s` but is not plural; stripping it would give `statu`.
    assert_eq!(singularize("status"), "status");
    assert_eq!(singularize("address"), "address");
    assert_eq!(singularize("s"), "s");
}

#[test]
fn a_keyword_column_becomes_a_raw_identifier_and_keeps_its_column_name() {
    let resolved = field("type");

    assert_eq!(resolved.ident, "r#type");
    // The column name matches the bare identifier, so no rename is needed: sqlx
    // strips the `r#` prefix when it derives the column name.
    assert_eq!(resolved.rename, None);
}

#[test]
fn a_keyword_that_cannot_be_raw_is_renamed_instead() {
    let resolved = field("super");

    assert_eq!(resolved.ident, "super_");
    // `r#super` is rejected by the compiler, so the field name has to diverge and
    // the attribute is what carries the mapping back to the column.
    assert_eq!(resolved.rename.as_deref(), Some("super"));
}

#[test]
fn a_converted_column_carries_a_rename() {
    let resolved = field("stripeProductId");

    assert_eq!(resolved.ident, "stripe_product_id");
    assert_eq!(resolved.rename.as_deref(), Some("stripeProductId"));
}

#[test]
fn an_unconverted_column_carries_no_rename() {
    let resolved = field("stripe_product_id");

    assert_eq!(resolved.ident, "stripe_product_id");
    assert_eq!(resolved.rename, None);
}
