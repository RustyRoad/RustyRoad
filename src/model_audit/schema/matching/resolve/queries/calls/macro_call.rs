//! Extract typed SQLx macros with a literal SQL argument.
use super::{literal, query, row};
use crate::model_audit::{schema::rust_types::Resolver, types::Model};
use syn::{parse::Parser, Expr, Token, Type};

/// Accept `query_as!(Row, "SELECT * FROM rows", value)` through a proven alias.
pub(in crate::model_audit::schema::matching::resolve::queries) fn macro_call(
    mac: &syn::Macro,
    model: &Model,
    own: bool,
    resolver: &Resolver,
) -> Option<String> {
    if !query(&mac.path, resolver, true) {
        return None;
    }
    let parser = |input: syn::parse::ParseStream<'_>| {
        let row: Type = input.parse()?;
        input.parse::<Token![,]>()?;
        let sql: Expr = input.parse()?;
        let _: proc_macro2::TokenStream = input.parse()?;
        Ok((row, sql))
    };
    let (ty, sql) = parser.parse2(mac.tokens.clone()).ok()?;
    if !row(&ty, model, own) {
        return None;
    }
    literal::sql(&sql)
}
