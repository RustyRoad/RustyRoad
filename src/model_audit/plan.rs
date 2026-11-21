//! Adoption plans require review even when the field syntax matches.

use super::{
    compare,
    types::{Candidate, Differences, Model},
};
use std::collections::BTreeMap;

/// Resolve explicit IDs or unique names; `--map path::Type=GeneratedType` overrides heuristics.
pub(super) fn mappings(
    values: &[String],
    models: &[Model],
    generated: &[Model],
) -> Result<BTreeMap<String, String>, String> {
    let mut result = BTreeMap::new();
    for value in values {
        let (left, right) = value
            .split_once('=')
            .ok_or("--map expects SOURCE=GENERATED")?;
        let source = unique(left, models)?;
        let target = unique(right, generated)?;
        if result
            .insert(source.id.clone(), target.id.clone())
            .is_some()
        {
            return Err(format!("Duplicate mapping for {}", source.id));
        }
    }
    Ok(result)
}

/// A short name is only accepted when unambiguous across scanned declarations.
fn unique<'a>(key: &str, models: &'a [Model]) -> Result<&'a Model, String> {
    let found: Vec<_> = models
        .iter()
        .filter(|m| m.id == key || m.name == key)
        .collect();
    match found.as_slice() {
        [model] => Ok(model),
        _ => Err(format!(
            "Mapping {key:?} must identify exactly one declaration; use its full ID"
        )),
    }
}

/// Build a candidate with independently reported blockers and migration actions.
pub(super) fn candidate(
    model: Model,
    generated: &[Model],
    mappings: &BTreeMap<String, String>,
) -> Candidate {
    let explicit = mappings.get(&model.id);
    let targets: Vec<_> = generated
        .iter()
        .filter(|g| match explicit {
            Some(id) => g.id == *id,
            None => compare::name_key(&g.name) == compare::name_key(&model.name),
        })
        .cloned()
        .collect();
    let target = if targets.len() == 1 {
        targets.first().cloned()
    } else {
        None
    };
    let differences = target
        .as_ref()
        .map(|g| compare::differences(&model, g))
        .unwrap_or_default();
    let classification = match &target {
        Some(g) => compare::classify(&model, g, &differences),
        None if targets.len() > 1 => "ambiguous_mapping".into(),
        None => "unmatched".into(),
    };
    let mut blockers = vec!["Review implementations, trait/derive behavior, cfg/features and API contracts before replacement".into()];
    if target.is_none() {
        blockers.push(
            "No unique generated target; supply --map or decide domain/projection/schema ownership"
                .into(),
        );
    }
    if model.kind != "struct" {
        blockers.push(
            "Enum, alias, tuple or unit declaration needs a dedicated contract review".into(),
        );
    }
    blockers.extend(field_blockers(&differences));
    Candidate {
        model, targets: targets.iter().map(|m| m.id.clone()).collect(), target,
        match_basis: if explicit.is_some() { "explicit" } else { "name_hint_only" }.into(),
        classification, differences, blockers,
        actions: vec!["Review callers separately before replacing any application type".into(),
            "Use generated New/Patch inputs for writes; do not fabricate missing row values".into(),
            "Move business behavior into domain functions or local extension traits, not external inherent impls".into()],
        usage: super::types::UsageAudit::NotRun,
        table_audit: None,
    }
}

/// Emit all hazards, not only the classification that happened to win precedence.
fn field_blockers(d: &Differences) -> Vec<String> {
    let mut out = Vec::new();
    if !d.type_changes.is_empty() {
        out.push("Resolve field type identities and conversion/nullability semantics; no lossy/default conversions".into());
    }
    if !d.custom_only.is_empty() {
        out.push("Custom-only fields may be projections or schema drift; do not create tables automatically".into());
    }
    if !d.generated_only.is_empty() {
        out.push(
            "Additional generated fields affect struct literals and serialized API exposure".into(),
        );
    }
    if d.attributes_differ
        || d.generics_differ
        || d.visibility_differs
        || !d.field_contract_changes.is_empty()
        || d.variants_differ
    {
        out.push(
            "Attributes, generics, visibility or variants differ; field equality is insufficient"
                .into(),
        );
    }
    out
}
