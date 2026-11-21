//! Bounded declaration traversal; physical directories do not imply Rust ownership.
use super::super::Node;
use std::path::{Path, PathBuf};
use syn::Item;

/// Find unique external-file ownership, rejecting cycles and oversized graphs.
/// Inline scopes are retained as graph edges without duplicating file ownership.
/// # Examples
/// ```sh
/// rustyroad model audit --source src/models --offline --report-only
/// ```
pub(super) fn find(root: &Path, target: &Path) -> Option<Vec<String>> {
    let node = Node::read(root, root.parent()?.to_path_buf())?;
    let mut matches = Vec::new();
    let mut remaining = 4096;
    visit(
        node,
        target,
        &mut Vec::new(),
        &mut Vec::new(),
        &mut matches,
        &mut remaining,
        true,
    )?;
    if matches.len() != 1 {
        return None;
    }
    matches.pop()
}

/// Traverse only declared edges and keep separate ancestry for inline modules.
/// # Examples
/// ```sh
/// rustyroad model audit --source src/models --offline --report-only
/// ```
fn visit(
    node: Node,
    target: &Path,
    modules: &mut Vec<String>,
    files: &mut Vec<PathBuf>,
    matches: &mut Vec<Vec<String>>,
    remaining: &mut usize,
    external: bool,
) -> Option<()> {
    if *remaining == 0 || modules.len() > 64 {
        return None;
    }
    *remaining -= 1;
    if external {
        if files.contains(&node.file) {
            return None;
        }
        files.push(node.file.clone());
        if node.file == target {
            matches.push(modules.clone());
        }
    }
    for item in &node.items {
        let Item::Mod(module) = item else {
            continue;
        };
        let name = module.ident.to_string();
        let Some(child) = node.child(&name) else {
            continue;
        };
        modules.push(name);
        visit(
            child,
            target,
            modules,
            files,
            matches,
            remaining,
            module.content.is_none(),
        )?;
        modules.pop();
    }
    if external {
        files.pop();
    }
    Some(())
}
