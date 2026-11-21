//! Snapshot-backed CLI checks: mocked local evidence, never live DB validation.
#[path = "model_audit_schema/additions.rs"]
mod additions;
#[path = "model_audit_schema/fixture.rs"]
mod fixture;
#[path = "model_audit_schema/key_fixture.rs"]
mod key_fixture;
#[path = "model_audit_schema/key_redesign.rs"]
mod key_redesign;
#[path = "model_audit_schema/key_safety.rs"]
mod key_safety;
#[path = "model_audit_schema/preview.rs"]
mod preview;
#[path = "model_audit_schema/retry.rs"]
mod retry;
#[path = "model_audit_schema/safety.rs"]
mod safety;
#[path = "model_audit_schema/selection.rs"]
mod selection;
