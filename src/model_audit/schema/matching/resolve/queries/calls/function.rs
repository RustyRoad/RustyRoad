//! Extract typed SQLx function calls with a proven PostgreSQL database.
use super::{database, literal, query, row};
use crate::model_audit::{schema::rust_types::Resolver, types::Model};
use syn::{Expr, GenericArgument, PathArguments};

/// Extract `query_as::<_, Row>("SELECT * FROM rows")`, including PostgreSQL DBs.
pub(in crate::model_audit::schema::matching::resolve::queries) fn function(
    call: &syn::ExprCall,
    model: &Model,
    own: bool,
    resolver: &Resolver,
) -> Option<String> {
    let Expr::Path(function) = &*call.func else {
        return None;
    };
    if function.qself.is_some() || !query(&function.path, resolver, false) || call.args.len() != 1 {
        return None;
    }
    let PathArguments::AngleBracketed(arguments) = &function.path.segments.last()?.arguments else {
        return None;
    };
    let args: Vec<_> = arguments.args.iter().collect();
    let [GenericArgument::Type(db), GenericArgument::Type(ty)] = args.as_slice() else {
        return None;
    };
    if !database::postgres(db, resolver) || !row(ty, model, own) {
        return None;
    }
    literal::sql(call.args.first()?)
}
