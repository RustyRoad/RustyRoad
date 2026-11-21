//! Selection must not silently discard an explicitly reviewed primary-key contract.
use crate::model_audit::types::Candidate;

/// Reject filtered contracts, e.g. `bookmark=id` when only `Row` is selected.
pub(super) fn check(candidates: &[Candidate], contracts: &[String]) -> Result<(), String> {
    for contract in contracts {
        let (table, _) = contract
            .split_once('=')
            .ok_or("Use --primary-key TABLE=COLUMN[,COLUMN]")?;
        let found = candidates.iter().any(|candidate| {
            candidate
                .table_audit
                .as_ref()
                .is_some_and(|audit| audit.table.as_deref() == Some(table))
        });
        if !found {
            return Err(format!(
                "--primary-key {table} did not resolve to an audited table after selection"
            ));
        }
    }
    Ok(())
}
