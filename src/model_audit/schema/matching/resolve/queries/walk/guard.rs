//! Reject lexical shadowing and conditional or opaque expansion evidence.
use super::super::{bindings, calls, namespace};
use crate::model_audit::schema::rust_types::Resolver;
use syn::visit::{self, Visit};

/// Check every lexical binding before accepting a query in this function.
pub(super) fn safe(
    block: &syn::Block,
    signature: &syn::Signature,
    model: &str,
    resolver: &Resolver,
) -> bool {
    let mut guard = Guard {
        model,
        resolver,
        safe: true,
    };
    guard.visit_signature(signature);
    guard.visit_block(block);
    guard.safe
}

/// One unsafe binding invalidates evidence from the entire function.
struct Guard<'a> {
    /// Row name whose source declaration must remain unshadowed.
    model: &'a str,
    /// Resolve constructor aliases in the declaration's module scope.
    resolver: &'a Resolver,
    /// Whether all visited bindings and attributes are understood.
    safe: bool,
}

impl<'ast> Visit<'ast> for Guard<'_> {
    /// Reject nested imports and declarations that redirect either namespace.
    fn visit_item(&mut self, _: &'ast syn::Item) {
        self.safe = false;
    }

    /// Visit destructuring recursively, not just a simple `let sqlx` pattern.
    fn visit_pat_ident(&mut self, pattern: &'ast syn::PatIdent) {
        self.safe &= !bindings::protected(&pattern.ident, self.model, self.resolver);
        visit::visit_pat_ident(self, pattern);
    }

    /// Conditional statements and expression attributes cannot prove ownership.
    fn visit_attribute(&mut self, attribute: &'ast syn::Attribute) {
        self.safe &= namespace::unconditional(std::slice::from_ref(attribute));
    }

    /// Unknown macros may introduce bindings or replace a query expression.
    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        self.safe &= calls::query(&mac.path, self.resolver, true);
    }

    /// Generic parameters can shadow SQLx or the row type in nested closures.
    fn visit_type_param(&mut self, parameter: &'ast syn::TypeParam) {
        self.safe &= parameter.ident != "sqlx" && parameter.ident != self.model;
        visit::visit_type_param(self, parameter);
    }

    /// Const parameters share the value namespace with module-looking paths.
    fn visit_const_param(&mut self, parameter: &'ast syn::ConstParam) {
        self.safe &= parameter.ident != "sqlx" && parameter.ident != self.model;
        visit::visit_const_param(self, parameter);
    }

    /// No query inside an opaque verbatim syntax node provides evidence.
    fn visit_token_stream(&mut self, _: &'ast proc_macro2::TokenStream) {
        self.safe = false;
    }
}
