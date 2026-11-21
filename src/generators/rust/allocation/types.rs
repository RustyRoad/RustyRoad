//! Shared type namespace: rows take priority over enums and create/patch inputs.

use super::{namespace, reserved};
use crate::database::introspection::Schema;
use crate::generators::rust::naming::pascal;
use std::collections::BTreeMap;

/// Allocate row, enum and input names across the complete schema.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn allocate(schema: &Schema) -> BTreeMap<(u8, String), String> {
    // Role order establishes priority; original SQL names break catalog-order ties.
    let mut candidates = BTreeMap::new();
    for table in &schema.tables {
        let base = pascal(&table.name);
        candidates.insert((0, table.name.clone()), base.clone());
        candidates.insert((2, table.name.clone()), format!("New{base}"));
        candidates.insert((3, table.name.clone()), format!("Patch{base}"));
        candidates.insert((4, table.name.clone()), format!("Filter{base}"));
        candidates.insert((5, table.name.clone()), format!("ClockNew{base}"));
        candidates.insert((6, table.name.clone()), format!("Compare{base}"));
        for foreign in &table.foreign_keys {
            candidates.insert(
                (7, format!("{}\0{}", table.name, foreign.name)),
                format!("Joined{base}{}", pascal(&foreign.name)),
            );
        }
    }
    for item in &schema.enums {
        candidates.insert((1, item.name.clone()), pascal(&item.name));
    }
    namespace::allocate(candidates, reserved::TYPES, "")
}
