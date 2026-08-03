//! Trigger installation and synchronization logic.

use crate::database::versions::ops::trigger::{Direction, Trigger, NEEDS_BACKFILL_COLUMN};

/// Forward trigger populating the shadow column from the original.
pub(super) fn up_trigger() -> Trigger<'static> {
    Trigger {
        table: "platform_trash_zone_mappings",
        physical_column: "_rustyroad_new_zone_id",
        direction: Direction::Up,
        expression: "NEW.zone_id::text::uuid",
        latest_schema: "public_02_rename",
    }
}

#[test]
fn install_adds_marker_function_and_trigger() {
    let statements = up_trigger().install();
    let sql = statements.join("\n");

    // The marker must exist before the function references it.
    assert!(statements[0].contains(NEEDS_BACKFILL_COLUMN));
    assert!(sql.contains("CREATE OR REPLACE FUNCTION"));
    assert!(sql.contains("BEFORE INSERT OR UPDATE ON"));
}

#[test]
fn trigger_fires_for_writes_from_the_other_version() {
    let sql = up_trigger().install().join("\n");

    // Forward direction rewrites writes that did NOT arrive via the new schema.
    assert!(sql.contains("IF search_path <> 'public_02_rename' THEN"));
    assert!(sql.contains(r#"NEW."_rustyroad_new_zone_id" = (NEW.zone_id::text::uuid)"#));
    // Processing a row clears its backfill marker.
    assert!(sql.contains(&format!(r#"NEW."{NEEDS_BACKFILL_COLUMN}" = false"#)));
}

#[test]
fn down_trigger_uses_the_opposite_comparison() {
    let down = Trigger {
        physical_column: "zone_id",
        direction: Direction::Down,
        expression: "NEW.trash_zone_id::text::int4",
        ..up_trigger()
    };
    let sql = down.install().join("\n");

    // Reverse direction rewrites writes that DID arrive via the new schema.
    assert!(sql.contains("IF search_path = 'public_02_rename' THEN"));
}
