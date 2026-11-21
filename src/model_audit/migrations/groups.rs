//! Reconcile table-local blockers before inspecting or writing migration files.

use super::Prepared;
use crate::model_audit::{confirmation::Scope, schema::types::TableAudit};
use std::collections::{BTreeMap, BTreeSet};

/// Collect eligible table groups; e.g. conflicting `users` drafts do not block `posts`.
pub(super) fn collect(audits: Vec<&TableAudit>, scope: Scope) -> Prepared {
    let mut groups: BTreeMap<_, Vec<_>> = BTreeMap::new();
    for audit in audits {
        if audit.status == "non_table" {
            continue;
        }
        if scope.includes(
            audit.mapping == "proposed_new_table_contract" || audit.status == "missing_table",
        ) {
            groups
                .entry(audit.table.as_deref().unwrap_or("unresolved"))
                .or_default()
                .push(audit);
        }
    }
    let mut prepared = Prepared {
        proposals: vec![],
        diagnostics: vec![],
        skipped: vec![],
    };
    for (table, audits) in groups {
        let mut blockers: BTreeSet<_> = audits
            .iter()
            .flat_map(|audit| audit.blockers.iter().cloned())
            .collect();
        let proposals: Vec<_> = audits
            .iter()
            .filter_map(|audit| audit.proposal.as_ref())
            .collect();
        if let Some(first) = proposals.first() {
            if proposals
                .iter()
                .any(|p| p.up != first.up || p.down != first.down)
            {
                blockers.insert(format!("Conflicting proposals for shared table {table}"));
            }
        }
        if !blockers.is_empty() {
            let reason = blockers.into_iter().collect::<Vec<_>>().join("; ");
            prepared.diagnostics.push(format!(
                "Migration skipped for table {table}: {reason}. Other eligible tables are processed independently"
            ));
            prepared.skipped.push((table.into(), reason));
        } else if let Some(proposal) = proposals.first() {
            prepared.proposals.push((*proposal).clone());
        }
    }
    prepared
}
