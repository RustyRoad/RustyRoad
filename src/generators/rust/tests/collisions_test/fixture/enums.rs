//! Enum labels and type names intentionally collide after Rust casing.

use crate::database::introspection::Enum;

pub(super) fn build() -> Vec<Enum> {
    vec![
        Enum {
            name: "proposal".into(),
            values: vec!["active", "active_", "self", "active_variant2"]
                .into_iter()
                .map(str::to_string)
                .collect(),
        },
        Enum {
            name: "service areas".into(),
            values: vec!["ok".into()],
        },
    ]
}
