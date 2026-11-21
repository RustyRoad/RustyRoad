//! Flatten Rust use trees while retaining their local aliases.
use super::Bindings;
use syn::UseTree;

/// Visit grouped, renamed, self and glob imports without interpreting SQL text.
/// Example: `use sqlx::{self as db, query_as as rows};` records both bindings.
pub(super) fn visit(tree: &UseTree, prefix: &[String], bindings: &mut Bindings) {
    match tree {
        UseTree::Path(path) => {
            let mut next = prefix.to_vec();
            next.push(path.ident.to_string());
            visit(&path.tree, &next, bindings);
        }
        UseTree::Group(group) => {
            for tree in &group.items {
                visit(tree, prefix, bindings);
            }
        }
        UseTree::Name(name) => register(prefix, &name.ident.to_string(), None, bindings),
        UseTree::Rename(rename) => register(
            prefix,
            &rename.ident.to_string(),
            Some(rename.rename.to_string()),
            bindings,
        ),
        UseTree::Glob(_) => {
            if prefix.len() == 1 && bindings.modules.contains(&prefix[0]) {
                bindings.glob = true;
            }
        }
    }
}

/// Register an SQLx module or function alias, ignoring imports from other crates.
/// Example: `use db::query_as as rows;` works after `db` names the SQLx module.
fn register(prefix: &[String], name: &str, alias: Option<String>, bindings: &mut Bindings) {
    let local = alias.unwrap_or_else(|| {
        if name == "self" {
            prefix.last().cloned().unwrap_or_default()
        } else {
            name.to_owned()
        }
    });
    if prefix.is_empty() && bindings.modules.contains(name)
        || prefix.len() == 1 && bindings.modules.contains(&prefix[0]) && name == "self"
    {
        bindings.modules.insert(local);
    } else if prefix.len() == 1 && bindings.modules.contains(&prefix[0]) {
        bindings.functions.insert(local, name.to_owned());
    } else if prefix.is_empty() {
        if let Some(api) = bindings.functions.get(name).cloned() {
            bindings.functions.insert(local, api);
        }
    }
}
