//! Preserve declaration, namespace, and catalog blockers before proposing DDL.
use super::super::super::types::{Catalog, Table, TableAudit};
use crate::model_audit::types::Model;

/// Reject unsafe evidence, e.g. a view named `rows` cannot receive table DDL.
pub(super) fn check(
    model: &Model,
    catalog: &Catalog,
    table: Option<&Table>,
    audit: &mut TableAudit,
) {
    audit.blockers.extend(super::declaration::blockers(model));
    if audit.mapping == "unresolved_persistence_contract" {
        audit.blockers.push(format!(
            "{} has no catalog match or explicit persistence contract; a struct name alone does not establish a table. Review ownership and use --table '{}=TABLE' only for a persisted row",
            model.id, model.id
        ));
    }
    if !catalog.complete {
        audit
            .blockers
            .push("Catalog is incomplete; schema absence is not established".into());
    }
    if let Some(table) = table {
        if table.kind != "table" {
            audit
                .blockers
                .push(format!("{} is a {}, not a table", table.name, table.kind));
        }
    } else if let Some(name) = &audit.table {
        if catalog.reserved_names.contains(name) {
            audit
                .blockers
                .push(format!("PostgreSQL namespace name {name} is occupied"));
        }
    }
    if let Some(name) = &audit.table {
        if let Err(error) = super::super::identifier(name) {
            audit.blockers.push(error);
        }
    }
}
