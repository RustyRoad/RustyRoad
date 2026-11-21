//! Source-query contracts use temporary Rust files, never a database or Cargo.
mod attributes;
mod impls;
mod rejections;
mod resolution;
mod safeguards;
use super::empty;
use crate::model_audit::{
    parse,
    schema::{
        matching,
        types::{Catalog, TableAudit},
    },
};

/// Reconcile a real source fixture while retaining its file during discovery.
fn audit(source: &str, explicit: Option<&str>) -> TableAudit {
    audit_catalog(source, &empty(), explicit)
}

/// Reconcile ownership against controlled catalog facts without connecting.
fn audit_catalog(source: &str, catalog: &Catalog, explicit: Option<&str>) -> TableAudit {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("row.rs");
    std::fs::write(&path, source).unwrap();
    let model = parse::parse(&path, root.path(), source).unwrap().remove(0);
    matching::audit(&model, catalog, explicit.map(str::to_owned).as_ref())
}

mod contracts;
