//! Extracted fixtures retain their original assertions and inputs.
use super::empty;
use crate::model_audit::{
    parse,
    schema::{matching, proposals, types::TableAudit},
};

/// Plan a fixture row against a missing or occupied table, e.g. `sessions`.
pub(super) fn planned(source: &str, explicit: bool, keys: &[String], occupied: bool) -> TableAudit {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("models.rs");
    std::fs::write(&path, source).unwrap();
    let model = parse::parse(&path, directory.path(), source)
        .unwrap()
        .remove(0);
    let mut catalog = empty();
    if occupied {
        catalog.reserved_names.push("sessions".into());
    }
    let target = explicit.then(|| "sessions".to_owned());
    let mut result = matching::audit(&model, &catalog, target.as_ref());
    proposals::build(&model, &catalog, &mut result, keys);
    result
}
