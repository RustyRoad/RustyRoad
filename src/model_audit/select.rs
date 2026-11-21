//! Explicit user selection, never a heuristic approval to delete models.

mod key_contracts;
use super::{cli::values, types::Candidate};
use clap::ArgMatches;
use std::io::IsTerminal;

/// Filter before symbol lookup; e.g. `--status db_superset --name Customer`.
pub(super) fn apply(
    mut candidates: Vec<Candidate>,
    args: &ArgMatches,
) -> Result<Vec<Candidate>, String> {
    let names: Vec<_> = values(args, "name")
        .into_iter()
        .map(|n| n.to_lowercase())
        .collect();
    let statuses = values(args, "status");
    let selected = values(args, "select");
    for id in &selected {
        if !candidates.iter().any(|c| c.model.id == *id) {
            return Err(format!(
                "Unknown selection ID {id:?}; regenerate the audit instead of using stale IDs"
            ));
        }
    }
    candidates.retain(|c| {
        (names.is_empty() || names.iter().any(|n| c.model.id.to_lowercase().contains(n)))
            && (statuses.is_empty() || statuses.contains(&c.classification))
            && (selected.is_empty() || selected.contains(&c.model.id))
    });
    if !selected.is_empty()
        && candidates.len()
            != selected
                .iter()
                .collect::<std::collections::BTreeSet<_>>()
                .len()
    {
        return Err("An explicit --select ID was excluded by --name or --status; adjust conflicting filters".into());
    }
    key_contracts::check(&candidates, &values(args, "primary-key"))?;
    if !args.get_flag("interactive") || candidates.is_empty() {
        return Ok(candidates);
    }
    if !std::io::stdin().is_terminal() || !std::io::stderr().is_terminal() {
        return Err(
            "--interactive requires terminal stdin/stderr; use --select for scripts".into(),
        );
    }
    let labels: Vec<_> = candidates
        .iter()
        .map(|c| {
            format!(
                "{} [{}; +{} columns, {} type changes; table={}]",
                c.model.id,
                c.classification,
                c.differences.generated_only.len(),
                c.differences.type_changes.len(),
                c.table_audit
                    .as_ref()
                    .map(|a| a.status.as_str())
                    .unwrap_or("not audited")
            )
        })
        .collect();
    let chosen = dialoguer::MultiSelect::new()
        .defaults(&vec![true; labels.len()])
        .with_prompt("Select models to audit (Space toggles, Enter confirms; selecting is NOT migration approval)")
        .items(&labels).interact_opt().map_err(|e| e.to_string())?.unwrap_or_default();
    let candidates = candidates
        .into_iter()
        .enumerate()
        .filter_map(|(index, c)| chosen.contains(&index).then_some(c))
        .collect::<Vec<_>>();
    key_contracts::check(&candidates, &values(args, "primary-key"))?;
    Ok(candidates)
}
