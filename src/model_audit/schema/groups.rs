//! Reconcile complete desired contracts before selection; never hide shared-table conflicts.
mod contracts;
use super::{proposals, types::Catalog};
use crate::model_audit::types::Candidate;
use std::collections::BTreeMap;

/// Merge compatible changes once, while preserving each model's observed differences.
pub(super) fn reconcile(
    candidates: &mut [Candidate],
    catalog: &Catalog,
    keys: &BTreeMap<String, Vec<String>>,
) {
    let mut groups: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (index, candidate) in candidates.iter().enumerate() {
        if let Some(table) = candidate
            .table_audit
            .as_ref()
            .and_then(|a| a.table.as_ref())
        {
            groups.entry(table.clone()).or_default().push(index);
        }
    }
    for (table, indices) in groups {
        if indices.len() < 2 {
            let candidate = &mut candidates[indices[0]];
            let audit = candidate.table_audit.as_mut().unwrap();
            if audit.status != "missing_table" {
                let table_keys = keys.get(&table).map(Vec::as_slice).unwrap_or(&[]);
                proposals::build(&candidate.model, catalog, audit, table_keys);
            }
            continue;
        }
        let ids: Vec<_> = indices
            .iter()
            .map(|i| candidates[*i].model.id.clone())
            .collect();
        let mut blockers = contracts::conflicts(candidates, &indices, &table);
        let first = indices[0];
        let mut combined = candidates[first].table_audit.clone().unwrap();
        if combined.status == "missing_table" {
            // New-table keys and full contracts must agree; never invent a merged key.
            for index in &indices[1..] {
                let other = candidates[*index].table_audit.as_ref().unwrap();
                if combined.proposal.as_ref().map(|p| &p.up)
                    != other.proposal.as_ref().map(|p| &p.up)
                {
                    blockers.push(format!(
                        "Shared new table {table}: conflicting contracts or primary keys"
                    ));
                }
            }
        } else if blockers.is_empty() {
            contracts::merge(candidates, &indices, &mut combined);
            combined.proposal = None;
            let table_keys = keys.get(&table).map(Vec::as_slice).unwrap_or(&[]);
            proposals::build(&candidates[first].model, catalog, &mut combined, table_keys);
            blockers.extend(combined.blockers.clone());
        }
        let proposal = if blockers.is_empty() {
            combined.proposal
        } else {
            None
        };
        for index in indices {
            let audit = candidates[index].table_audit.as_mut().unwrap();
            audit.shared_models = ids.clone();
            audit.blockers.extend(blockers.clone());
            audit.proposal = proposal.clone();
        }
    }
}
