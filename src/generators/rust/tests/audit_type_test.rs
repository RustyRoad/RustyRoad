//! An audit-named column that is not a clock type must not be seeded like one.

use super::support::{required, scratch};
use crate::database::introspection::{Schema, Table};
use crate::generators::rust::write;

#[test]
fn a_date_column_is_not_treated_as_a_clock() {
    let built = Table {
        name: "reports".to_string(),
        columns: vec![required("id", "integer"), required("created_at", "date")],
        primary_key: vec!["id".to_string()],
        foreign_keys: Vec::new(),
        uniques: Vec::new(),
        indexes: Vec::new(),
        view: false,
    };
    let schema = Schema {
        tables: vec![built],
        enums: Vec::new(),
    };

    let out = scratch("date-audit");
    write(&out, &schema, false).expect("write should succeed");
    let root = std::fs::read_to_string(out.join("report").join("mod.rs")).unwrap();

    // A `date` named created_at is still a date: seeding it with a timestamp would not
    // type-check, so it takes its type's own default instead.
    assert!(
        !root.contains("created_at: Utc::now()"),
        "a date must not be seeded with a clock:\n{root}"
    );
    // And the import must not be emitted for a seed that never appears.
    assert!(
        !root.contains("use chrono::Utc;"),
        "unused Utc import:\n{root}"
    );

    let _ = std::fs::remove_dir_all(&out);
}
