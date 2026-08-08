//! Regressions found by running against a real Postgres server.
//!
//! Each case here failed live despite the SQL-shape tests passing, so each is
//! pinned to keep it from returning.

use crate::database::versions::ops::backfill::batch_sql;
use crate::database::versions::ops::model::Migration;
use crate::database::versions::ops::plan;
use crate::database::versions::ops::trigger::{Direction, Trigger};

/// The live migration: rename zone_id to trash_zone_id and retype to uuid.
fn rename() -> Migration {
    serde_json::from_str(
        r#"{"operations":[{"alter_column":{
            "table":"platform_trash_zone_mappings","column":"zone_id",
            "name":"trash_zone_id","type":"uuid","nullable":false,
            "up":"NEW.zone_id::text::uuid",
            "down":"(right(NEW.trash_zone_id::text, 12))::int4"}}]}"#,
    )
    .expect("should parse")
}

#[test]
fn cursor_keys_are_cast_in_sql() {
    // Live failure: "invalid input syntax for type integer" — an int4 key cannot be
    // read as text client-side, so the cast has to happen in the query.
    let sql = batch_sql("t", &["id".to_string()], 10, None);
    assert!(sql.contains(r#""id"::text FROM updated"#));
}

#[test]
fn search_path_is_normalized_before_comparison() {
    // Live failure: Postgres returns `"public_20251114211514-rename"` *with quotes*
    // when the name needs them, so a bare literal comparison never matched and the
    // down trigger silently never fired.
    let sql = Trigger {
        table: "t",
        physical_column: "c",
        direction: Direction::Down,
        expression: "NEW.x",
        latest_schema: "public_02-rename",
        rewrite: None,
    }
    .install()
    .join("\n");

    assert!(sql.contains("split_part(current_setting('search_path'), ',', 1)"));
    assert!(sql.contains("btrim("));
}

#[test]
fn down_expression_is_rewritten_to_the_physical_column() {
    // Live failure: `record "new" has no field "trash_zone_id"`. NEW carries physical
    // columns; the new logical name exists only in the view.
    let sql = plan(&rename(), "public_v").statements.join("\n");

    assert!(sql.contains(r#"right(NEW."_rustyroad_new_zone_id"::text, 12)"#));
    assert!(!sql.contains("NEW.trash_zone_id"));
}

#[test]
fn not_null_survives_a_rename() {
    // Live failure: the promoted column came back nullable, because promotion
    // hardcoded nullable and the constraint targeted the pre-rename name.
    let completion = plan(&rename(), "public_v")
        .completion_statements()
        .join("\n");

    assert!(completion.contains(r#"ALTER COLUMN "trash_zone_id" SET NOT NULL"#));

    // The constraint must be applied after the rename, or it targets a missing column.
    let rename_at = completion
        .find("RENAME COLUMN \"zone_id\"")
        .expect("renamed");
    let not_null_at = completion.find("SET NOT NULL").expect("constrained");
    assert!(rename_at < not_null_at);
}
