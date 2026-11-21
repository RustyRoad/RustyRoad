//! Import-tree expansion preserves absolute crate identity and ambiguous bindings.
use std::collections::BTreeMap;
use syn::{ItemUse, Path, UseTree};

/// Collect imports without resolving an external crate against local modules.
/// For example, `use ::sqlx as db;` retains `::sqlx` as the `db` binding.
pub(super) fn bindings(import: &ItemUse) -> BTreeMap<String, Option<Path>> {
    let mut imported = BTreeMap::new();
    let prefix = if import.leading_colon.is_some() {
        "::"
    } else {
        ""
    };
    flatten(&import.tree, prefix.to_owned(), &mut imported);
    imported
}

/// Expand grouped/renamed imports; wildcard exports cannot establish a codec.
/// For example, `use sqlx::{self as db, Type};` records both explicit bindings.
fn flatten(tree: &UseTree, prefix: String, out: &mut BTreeMap<String, Option<Path>>) {
    let (name, path) = match tree {
        UseTree::Path(path) => {
            flatten(&path.tree, format!("{prefix}{}::", path.ident), out);
            return;
        }
        UseTree::Group(group) => {
            for tree in &group.items {
                flatten(tree, prefix.clone(), out);
            }
            return;
        }
        UseTree::Name(name) if name.ident == "self" => {
            let path = prefix.trim_end_matches("::").to_owned();
            (path.rsplit("::").next().unwrap_or("").to_owned(), path)
        }
        UseTree::Name(name) => (name.ident.to_string(), format!("{prefix}{}", name.ident)),
        UseTree::Rename(rename) if rename.ident == "self" => (
            rename.rename.to_string(),
            prefix.trim_end_matches("::").to_owned(),
        ),
        UseTree::Rename(rename) => (
            rename.rename.to_string(),
            format!("{prefix}{}", rename.ident),
        ),
        UseTree::Glob(_) => {
            let path = syn::parse_str(prefix.trim_end_matches("::")).ok();
            out.entry("*".into())
                .and_modify(|existing| *existing = None)
                .or_insert(path);
            return;
        }
    };
    let path = syn::parse_str(&path).ok();
    out.entry(name)
        .and_modify(|existing| *existing = None)
        .or_insert(path);
}
