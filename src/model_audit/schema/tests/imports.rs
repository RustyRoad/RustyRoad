//! Import inference uses source fixtures, not Cargo or language-server execution.
use crate::model_audit::{
    parse,
    schema::{matching, proposals},
};

mod ambiguity;
mod binding_shapes;
mod namespace_shapes;
mod scalar_shapes;
mod source_modules;
mod sqlx_contracts;
mod sqlx_time;
mod wildcard;

/// Return audited declarations while their temporary source still exists.
fn audits(source: &str) -> Vec<super::super::types::TableAudit> {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("models.rs");
    std::fs::write(&path, source).unwrap();
    parse::parse(&path, directory.path(), source)
        .unwrap()
        .iter()
        .map(|model| {
            let catalog = super::empty();
            // Codec fixtures explicitly declare persistence; production discovery
            // must still reject unmatched helper/request structs without ownership.
            let table = model.name.to_lowercase();
            let mut audit = matching::audit(model, &catalog, Some(&table));
            proposals::build(model, &catalog, &mut audit, &[]);
            audit
        })
        .collect()
}

mod known;
