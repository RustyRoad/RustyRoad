//! Literal SQL is the only supported query-source ownership evidence.
use syn::Expr;

/// Dynamic SQL, concatenation, and SQL files are not literal ownership evidence.
pub(super) fn sql(expression: &Expr) -> Option<String> {
    if let Expr::Lit(literal) = expression {
        if let syn::Lit::Str(value) = &literal.lit {
            return Some(value.value());
        }
    }
    None
}
