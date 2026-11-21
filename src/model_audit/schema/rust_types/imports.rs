//! Source-backed import resolution; unknown codecs remain unknown, not guessed.
mod binding;
mod build;
mod codecs;
mod collect;
mod glob;
mod namespace;
mod resolve;
mod source;
use binding::Binding;
use std::collections::BTreeMap;
use syn::Type;

/// Each scope owns its explicit bindings; `super` resolution uses parent scopes.
#[derive(Default)]
pub(in crate::model_audit::schema) struct Resolver {
    scopes: Vec<BTreeMap<String, Option<Binding>>>,
    codecs: Vec<BTreeMap<String, Option<String>>>,
    source: Option<source::Context>,
}

impl Resolver {
    /// Expand aliases recursively, including arguments in `Vec<Uuid>`.
    pub(in crate::model_audit::schema) fn resolve(&self, ty: &Type) -> Type {
        self.resolve_at(ty, self.scopes.len().saturating_sub(1), 0)
    }

    /// Read an explicit local SQLx contract, never a similarly named external type.
    pub(in crate::model_audit::schema) fn storage(&self, ty: &Type) -> Option<String> {
        codecs::lookup(self, ty, self.scopes.len().checked_sub(1)?, 0)
    }
}
