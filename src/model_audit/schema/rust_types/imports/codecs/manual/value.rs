//! Only an unconditional, single-expression SQLx `with_name` body supplies evidence.
use super::super::super::Resolver;
use super::super::{paths, sql_name, unconditional};
use syn::{Expr, ImplItemFn, Stmt};

/// Recognize `{ PgTypeInfo::with_name("geometry") }`; computed branches stay unknown.
pub(super) fn name(method: &ImplItemFn, resolver: &Resolver) -> Option<String> {
    if !unconditional(&method.attrs)
        || method.block.stmts.len() != 1
        || !method.sig.inputs.is_empty()
        || !method.sig.generics.params.is_empty()
    {
        return None;
    }
    let Stmt::Expr(expression, terminator) = &method.block.stmts[0] else {
        return None;
    };
    let expression = match expression {
        Expr::Return(value) => value.expr.as_deref()?,
        value if terminator.is_none() => value,
        _ => return None,
    };
    let Expr::Call(call) = expression else {
        return None;
    };
    let Expr::Path(function) = call.func.as_ref() else {
        return None;
    };
    if function.qself.is_some() || call.args.len() != 1 {
        return None;
    }
    let mut owner = function.path.clone();
    let segment = owner.segments.pop()?;
    if segment.ident != "with_name"
        || !matches!(segment.arguments, syn::PathArguments::None)
        || !paths::is(&owner, "sqlx::postgres::PgTypeInfo", resolver)
    {
        return None;
    }
    let Expr::Lit(value) = call.args.first()? else {
        return None;
    };
    let syn::Lit::Str(name) = &value.lit else {
        return None;
    };
    sql_name(&name.value())
}
