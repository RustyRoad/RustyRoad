//! Visit safe expressions while retaining every distinct physical table target.
use super::{super::calls, super::sql, conditional};
use crate::model_audit::{schema::rust_types::Resolver, types::Model};
use std::collections::BTreeSet;
use syn::visit::{self, Visit};

/// Store source-resolved targets, e.g. `sessions` from a typed whole-row query.
pub(super) struct Rows<'a> {
    /// Exact model declaration, e.g. `Row`.
    pub(super) model: &'a Model,
    /// Whether `Self` denotes this model in an inherent implementation.
    pub(super) own: bool,
    /// Scope-local constructor aliases must resolve to SQLx.
    pub(super) resolver: &'a Resolver,
    /// Distinct targets retain conflicts rather than selecting the first call.
    pub(super) targets: &'a mut BTreeSet<String>,
}

/// Traverse expressions but never nested item scopes with different bindings.
impl<'ast> Visit<'ast> for Rows<'_> {
    fn visit_item(&mut self, _: &'ast syn::Item) {}

    /// Do not infer ownership from a query in a conditional or deferred scope.
    fn visit_expr(&mut self, expression: &'ast syn::Expr) {
        if conditional::conditional(expression) {
            return;
        }
        visit::visit_expr(self, expression);
    }

    fn visit_expr_call(&mut self, call: &'ast syn::ExprCall) {
        if let Some(target) = calls::function(call, self.model, self.own, self.resolver)
            .and_then(|source| sql::target(&source))
        {
            self.targets.insert(target);
        }
        visit::visit_expr_call(self, call);
    }

    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        if let Some(target) = calls::macro_call(mac, self.model, self.own, self.resolver)
            .and_then(|source| sql::target(&source))
        {
            self.targets.insert(target);
        }
    }
}
