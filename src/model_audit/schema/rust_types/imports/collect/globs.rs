//! Preserve every grouped wildcard target without merging its namespace binding.
use syn::{ItemUse, Path, UseTree};

/// Collect `use self::{one::*, two::*}` as two independently checked targets.
pub(in crate::model_audit::schema::rust_types::imports) fn paths(
    import: &ItemUse,
) -> Option<Vec<Path>> {
    let mut paths = Vec::new();
    let prefix = if import.leading_colon.is_some() {
        "::"
    } else {
        ""
    };
    collect(&import.tree, prefix.to_owned(), &mut paths)?;
    Some(paths)
}

/// A malformed wildcard target cannot discharge the enclosing import's blocker.
fn collect(tree: &UseTree, prefix: String, paths: &mut Vec<Path>) -> Option<()> {
    match tree {
        UseTree::Path(path) => {
            collect(&path.tree, format!("{prefix}{}::", path.ident), paths)?;
        }
        UseTree::Group(group) => {
            for tree in &group.items {
                collect(tree, prefix.clone(), paths)?;
            }
        }
        UseTree::Glob(_) => paths.push(syn::parse_str(prefix.trim_end_matches("::")).ok()?),
        UseTree::Name(_) | UseTree::Rename(_) => {}
    }
    Some(())
}
