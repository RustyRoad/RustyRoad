//! Reject imports that can replace SQLx or the row declaration in this scope.
use syn::UseTree;

/// Reject wildcard or conflicting imports, including `use other as sqlx`.
pub(super) fn safe(tree: &UseTree, model: &str) -> bool {
    match tree {
        UseTree::Path(path) => {
            if let UseTree::Name(name) = &*path.tree {
                if name.ident == "self" {
                    return path.ident != "sqlx" && path.ident != model;
                }
            }
            safe(&path.tree, model)
        }
        UseTree::Name(name) => name.ident != "sqlx" && name.ident != model,
        UseTree::Rename(rename) => rename.rename != "sqlx" && rename.rename != model,
        UseTree::Group(group) => group.items.iter().all(|tree| safe(tree, model)),
        UseTree::Glob(_) => false,
    }
}
