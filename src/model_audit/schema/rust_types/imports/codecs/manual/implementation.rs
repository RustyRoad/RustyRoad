//! Match only the SQLx PostgreSQL Type trait, not similarly named local traits.
use super::super::super::Resolver;
use super::super::paths;
use syn::{GenericArgument, PathArguments, Type};

/// Require one proven PostgreSQL database argument and a positive trait impl.
pub(super) fn matches(item: &syn::ItemImpl, resolver: &Resolver) -> bool {
    let Some((trait_path, _)) = item.trait_.as_ref() else {
        return false;
    };
    if item.modifiers.polarity.is_some() {
        return false;
    }
    let mut path = trait_path.clone();
    let Some(last) = path.segments.last_mut() else {
        return false;
    };
    let PathArguments::AngleBracketed(args) = &last.arguments else {
        return false;
    };
    if args.args.len() != 1 {
        return false;
    }
    let GenericArgument::Type(Type::Path(database)) = args.args.first().unwrap() else {
        return false;
    };
    if database.qself.is_some() || !paths::is(&database.path, "sqlx::Postgres", resolver) {
        return false;
    }
    last.arguments = PathArguments::None;
    if !paths::is(&path, "sqlx::Type", resolver) {
        return false;
    }
    true
}
