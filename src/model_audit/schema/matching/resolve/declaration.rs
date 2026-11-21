//! Decide whether a declaration has a concrete, unconditional storage shape.
use crate::model_audit::types::Model;
use syn::parse::Parser;

/// Recognize a plain fieldless namespace before matching its name to a table.
/// Explicit table mappings and annotated or generic declarations are still reviewed.
/// # Examples
/// ```sh
/// rustyroad model audit --source src/models --offline --report-only
/// ```
pub(super) fn service_marker(model: &Model) -> bool {
    model.kind == "tuple_or_unit"
        && model.fields.is_empty()
        && model.attributes.is_empty()
        && model.generics.is_empty()
}

/// Return blockers for declarations such as `struct Row<T> { value: T }`.
pub(super) fn blockers(model: &Model) -> Vec<String> {
    let mut blockers = Vec::new();
    if model.kind != "struct" || !model.generics.is_empty() {
        blockers.push("Only concrete structs have an inferable table contract".into());
    }
    for source in &model.attributes {
        let supported = syn::Attribute::parse_outer
            .parse_str(source)
            .is_ok_and(|attrs| {
                !attrs.is_empty()
                    && attrs.iter().all(|attr| {
                        [
                            "derive", "doc", "serde", "schema", "allow", "deny", "warn", "forbid",
                        ]
                        .iter()
                        .any(|name| attr.path().is_ident(name))
                    })
            });
        if !supported {
            blockers.push(
                "Conditional or storage-affecting container attributes require review".into(),
            );
            break;
        }
    }
    blockers
}
