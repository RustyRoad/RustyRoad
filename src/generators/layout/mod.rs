//! Laying out a folder of generated model modules.
//!
//! Both the Rust and TetherScript generators write the same structure: one folder per
//! table, plus a parent module that declares them. The parent is a scaffold rather than
//! derived output — a project keeps hand-written models beside the generated ones, and
//! rewriting the file would delete their declarations — so both also need the same
//! "written but not declared" reporting. That logic lives here once.

mod parts;
mod scan;

use super::ownership::{write_owned, Outcome, Ownership};
use std::path::Path;
use std::{fs, io};

pub use parts::{File, Layout, Module};

/// Writes `modules` into `out`, plus a parent module declaring them.
///
/// `parent` renders the parent's contents, and `declares` decides whether an existing
/// parent already covers a module — both differ by language, and nothing else does.
pub fn write<P, D>(
    out: &Path,
    modules: &[Module],
    root: &'static str,
    parent: P,
    declares: D,
    force: bool,
) -> io::Result<Layout>
where
    P: Fn(&[Module]) -> String,
    D: Fn(&str, &str) -> bool,
{
    fs::create_dir_all(out)?;

    let mut outcomes = Vec::new();
    for module in modules {
        outcomes.extend(write_module(out, module, force)?);
    }

    let written = write_owned(out, root, &parent(modules), Ownership::Scaffold, force)?;
    let undeclared = if written.is_preserved() {
        scan::missing(&out.join(root), modules, declares)
    } else {
        Vec::new()
    };
    outcomes.push(written);

    Ok(Layout {
        outcomes,
        undeclared,
    })
}

/// Writes one model's folder.
fn write_module(out: &Path, module: &Module, force: bool) -> io::Result<Vec<Outcome>> {
    let directory = out.join(&module.name);
    fs::create_dir_all(&directory)?;

    module
        .files
        .iter()
        .map(|file| write_owned(&directory, file.name, &file.contents, file.ownership, force))
        .collect()
}
