//! Column declarations in `schema.ts`.

use super::support::schema;
use crate::generators::typescript::schema::render;
use crate::generators::typescript::Casing;

#[test]
fn columns_carry_database_name_under_camel_casing() {
    let ts = render(&schema(), Casing::Camel);

    // The identifier is camelCase, but the database name must still be stated.
    assert!(ts.contains("emailAddress: varchar(\"email_address\", { length: 255 })"));
}

#[test]
fn database_name_is_omitted_under_preserve_casing() {
    let ts = render(&schema(), Casing::Preserve);

    assert!(ts.contains("email_address: varchar({ length: 255 })"));
}

#[test]
fn single_column_primary_key_is_inline() {
    let ts = render(&schema(), Casing::Camel);

    assert!(ts.contains("id: serial(\"id\").primaryKey().notNull()"));
}

#[test]
fn not_null_and_defaults_are_chained() {
    let ts = render(&schema(), Casing::Camel);

    assert!(ts.contains("isActive: boolean(\"is_active\").notNull().default(true)"));
    // numeric is a string, so its default is quoted despite looking numeric.
    assert!(ts.contains(".default('0')"));
}

#[test]
fn jsonb_columns_are_type_annotated() {
    let ts = render(&schema(), Casing::Camel);

    // Drizzle would infer `unknown` while drizzle-zod infers `Json`; the two are
    // not assignable, so the type is stated.
    assert!(ts.contains("jsonb(\"metadata\").$type<Record<string, unknown>>()"));
}

#[test]
fn annotated_jsonb_columns_use_the_database_shape() {
    let ts = render(&super::support::annotated_json_schema(), Casing::Camel);

    assert!(
        ts.contains("jsonb(\"metadata\").$type<{ \"city\": string; \"population\"?: number }>()")
    );
}

#[test]
fn serial_sequence_default_is_not_emitted() {
    let ts = render(&schema(), Casing::Camel);

    // `serial` already implies the sequence, so restating it would be wrong.
    assert!(!ts.contains("nextval"));
}
