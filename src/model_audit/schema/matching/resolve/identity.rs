//! Record relation identity independently from column-level differences.
use super::super::super::types::{Table, TableAudit};
use crate::model_audit::types::Model;

/// Enums and aliases supply field types, not standalone PostgreSQL tables.
pub(super) fn non_table(model: &Model) -> TableAudit {
    let mut result = audit(model, String::new(), false, &[]);
    result.status = "non_table".into();
    result.mapping = "non_row_declaration".into();
    result.table = None;
    result
}

/// A unique catalog match, e.g. `Row` -> `rows`, permits a partial projection.
pub(super) fn audit(model: &Model, name: String, explicit: bool, tables: &[&Table]) -> TableAudit {
    let ambiguous = tables.len() > 1;
    let matched = tables.len() == 1;
    let status = if ambiguous {
        "ambiguous_table"
    } else if matched {
        "table_matched"
    } else {
        "missing_table"
    };
    let mapping = if explicit {
        "explicit_persistence_contract"
    } else if matched {
        "catalog_contract_match"
    } else {
        "unresolved_persistence_contract"
    };
    TableAudit {
        shared_models: vec![model.id.clone()],
        status: status.into(),
        table: (!ambiguous).then_some(name),
        mapping: mapping.into(),
        candidates: tables.iter().map(|table| table.name.clone()).collect(),
        missing_columns: Vec::new(),
        column_changes: Vec::new(),
        desired_columns: Vec::new(),
        database_only: Vec::new(),
        incompatible_columns: Default::default(),
        blockers: if ambiguous {
            vec!["Ambiguous table identity requires --table MODEL=TABLE".into()]
        } else {
            Vec::new()
        },
        proposal: None,
    }
}
