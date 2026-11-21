//! Catalog-only fixture: no database or application-owned generated files needed.

mod enums;
mod selectors;
mod tables;

use crate::database::introspection::Schema;

pub(super) fn schema() -> Schema {
    // Prefixes, punctuation, reserved helpers and suffix-like names share namespaces.
    let tables = [
        "proposal",
        "new_proposal",
        "patch_proposal",
        "service_areas",
        "service-areas",
        "service_areas_by_type",
        "generated",
        "interval",
        "interval_wire",
        "option",
        "string",
        "self",
        "api_error",
        "foo",
        "foo_by_a_and_b",
        "foo2",
        "from",
        "some",
        "none",
        "ok",
        "err",
        "enum_key",
    ]
    .into_iter()
    .map(tables::build)
    .collect();
    Schema {
        tables,
        enums: enums::build(),
    }
}
