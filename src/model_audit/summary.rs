//! Table counts are distinct from generated-Rust name matches and migration counts.
use super::types::Candidate;
use serde::Serialize;
use std::collections::BTreeSet;

#[derive(Debug, Default, Serialize)]
pub(super) struct Summary {
    pub existing_table: usize,
    pub missing_table: usize,
    pub unresolved: usize,
    pub non_table: usize,
    pub blocked: usize,
    pub update_migrations: usize,
    pub create_migrations: usize,
}

impl Summary {
    /// Count selected declarations; duplicates share a single SQL proposal.
    pub(super) fn from_candidates(candidates: &[Candidate]) -> Self {
        let mut summary = Self::default();
        let mut updates = BTreeSet::new();
        let mut creates = BTreeSet::new();
        for candidate in candidates {
            let Some(audit) = &candidate.table_audit else {
                summary.unresolved += 1;
                continue;
            };
            match audit.status.as_str() {
                "table_matched" | "missing_columns" | "column_changes" => {
                    summary.existing_table += 1
                }
                "missing_table" => summary.missing_table += 1,
                "non_table" => summary.non_table += 1,
                _ => summary.unresolved += 1,
            }
            if !audit.blockers.is_empty() {
                summary.blocked += 1;
            }
            if let Some(proposal) = &audit.proposal {
                if proposal.operation == "create_table" {
                    creates.insert(&proposal.table);
                } else {
                    updates.insert(&proposal.table);
                }
            }
        }
        summary.update_migrations = updates.len();
        summary.create_migrations = creates.len();
        summary
    }

    pub(super) fn text(&self, discovered: usize, selected: usize) -> String {
        format!(
            "Found {discovered} declarations; {selected} selected: {} match an existing table, {} have no table, {} non-table declarations, {} unresolved.\n{} update migrations; {} create-table migrations proposed; {} models blocked.\nUnmatched structs require explicit persistence mapping before table creation. SQL is NEVER applied.\n",
            self.existing_table, self.missing_table, self.non_table, self.unresolved,
            self.update_migrations, self.create_migrations, self.blocked,
        )
    }
}
