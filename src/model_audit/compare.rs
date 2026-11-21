//! Structural comparison is conservative: spelling equality is not type equality.

use super::types::{Differences, Model};

/// Compare independent dimensions; `differences(custom, generated)` never drops type changes.
pub(super) fn differences(custom: &Model, generated: &Model) -> Differences {
    let mut diff = Differences {
        attributes_differ: custom.attributes != generated.attributes,
        generics_differ: custom.generics != generated.generics,
        visibility_differs: custom.visibility != generated.visibility,
        variants_differ: custom.variants != generated.variants,
        ..Differences::default()
    };
    for (name, field) in &custom.fields {
        if let Some(other) = generated.fields.get(name) {
            if field.ty != other.ty {
                diff.type_changes
                    .insert(name.clone(), [field.ty.clone(), other.ty.clone()]);
            }
            if field.attributes != other.attributes || field.visibility != other.visibility {
                diff.field_contract_changes.push(name.clone());
            }
        } else {
            diff.custom_only.push(name.clone());
        }
    }
    diff.generated_only = generated
        .fields
        .keys()
        .filter(|key| !custom.fields.contains_key(*key))
        .cloned()
        .collect();
    diff
}

/// Names only suggest mappings. Preserve New/Patch/Update prefixes to avoid mixing row and inputs.
pub(super) fn name_key(name: &str) -> String {
    let lower = name.replace('_', "").to_lowercase();
    crate::generators::rust::singularize(&lower)
}

/// Classify syntax, not replacement safety; enums/aliases always need a separate review.
pub(super) fn classify(custom: &Model, target: &Model, diff: &Differences) -> String {
    if custom.kind != "struct" || target.kind != "struct" {
        return "non_row_review".into();
    }
    if !diff.custom_only.is_empty() {
        "fields_differ"
    } else if !diff.generated_only.is_empty() {
        "db_superset"
    } else if !diff.type_changes.is_empty() {
        "types_differ"
    } else {
        "field_syntax_equal"
    }
    .into()
}
