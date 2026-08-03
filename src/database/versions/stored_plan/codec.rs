//! Serialization of a stored plan.

use super::super::ops::Plan;

/// Storage shape of a plan; only completion-relevant fields are kept.
#[derive(serde::Serialize, serde::Deserialize)]
pub(super) struct Stored {
    statements: Vec<String>,
    backfill_tables: Vec<String>,
    promotions: Vec<(String, String)>,
    drops: Vec<(String, String)>,
    renames: Vec<(String, String, String)>,
    deferred_not_null: Vec<(String, String)>,
}

/// Serializes a plan for storage.
pub(super) fn encode(plan: &Plan) -> String {
    serde_json::to_string(&Stored::from(plan)).unwrap_or_default()
}

/// Deserializes a stored plan.
pub(super) fn decode(encoded: &str) -> Option<Plan> {
    serde_json::from_str::<Stored>(encoded).ok().map(Into::into)
}

impl From<&Plan> for Stored {
    fn from(plan: &Plan) -> Self {
        Self {
            statements: plan.statements.clone(),
            backfill_tables: plan.backfill_tables.clone(),
            promotions: plan.promotions.clone(),
            drops: plan.drops.clone(),
            renames: plan.renames.clone(),
            deferred_not_null: plan.deferred_not_null.clone(),
        }
    }
}

impl From<Stored> for Plan {
    fn from(stored: Stored) -> Self {
        Self {
            statements: stored.statements,
            backfill_tables: stored.backfill_tables,
            promotions: stored.promotions,
            drops: stored.drops,
            renames: stored.renames,
            deferred_not_null: stored.deferred_not_null,
        }
    }
}
