//! Identify expressions whose queries are conditional or deferred.

/// For example, `if ready { query() }` cannot establish unconditional ownership.
pub(super) fn conditional(expression: &syn::Expr) -> bool {
    matches!(
        expression,
        syn::Expr::If(_)
            | syn::Expr::Match(_)
            | syn::Expr::While(_)
            | syn::Expr::ForLoop(_)
            | syn::Expr::Loop(_)
            | syn::Expr::Closure(_)
            | syn::Expr::Async(_)
            | syn::Expr::Const(_)
    ) || matches!(expression, syn::Expr::Binary(binary)
    if matches!(binary.op, syn::BinOp::And(_) | syn::BinOp::Or(_)))
}
