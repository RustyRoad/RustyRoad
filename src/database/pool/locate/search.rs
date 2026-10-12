//! The directories and file names the config search tries.

use crate::database::get_config_file_name;
use std::env;
use std::path::{Path, PathBuf};

/// Shared config names tried after the active environment's file.
const SHARED_NAMES: [&str; 3] = [
    "rustyroad.toml",
    "rustyroad.dev.toml",
    "rustyroad.prod.toml",
];

/// Config file names, the active environment's first.
pub(super) fn names() -> Vec<String> {
    let mut names = vec![get_config_file_name()];

    for shared in SHARED_NAMES {
        if !names.iter().any(|name| name == shared) {
            names.push(shared.to_string());
        }
    }

    names
}

/// Directories searched: the working directory and the executable's, plus their ancestors.
pub(super) fn roots() -> Vec<PathBuf> {
    let cwd = env::current_dir().ok();
    let exe_dir = env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(Path::to_path_buf));

    cwd.into_iter()
        .chain(exe_dir)
        .flat_map(|start| start.ancestors().map(Path::to_path_buf).collect::<Vec<_>>())
        .collect()
}
