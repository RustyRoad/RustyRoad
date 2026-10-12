//! Finding the RustyRoad config file the shared pool reads.
//!
//! A server binary is not always started from the project root: a Kubernetes job runs it
//! from `/`, and `cargo run` from a subfolder. So the search walks up from both the working
//! directory and the executable, and checks the secrets mount a deployment injects.

mod search;

use std::env;
use std::path::PathBuf;

/// Where an injected secrets volume places the config.
const SECRETS_MOUNT: &str = "/vault/secrets/rustyroad.toml";

/// Returns the first config file that exists, if any.
pub(super) fn config_file() -> Option<PathBuf> {
    candidates().into_iter().find(|path| path.is_file())
}

/// Every path checked, in priority order and without duplicates.
///
/// `RUSTYROAD_CONFIG_PATH` first, then the secrets mount, then each searched directory's
/// file for the active environment followed by the shared names.
pub(super) fn candidates() -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = Vec::new();
    let explicit = env::var("RUSTYROAD_CONFIG_PATH").ok().map(PathBuf::from);
    let searched = search::roots()
        .into_iter()
        .flat_map(|root| search::names().into_iter().map(move |name| root.join(name)));

    for path in explicit
        .into_iter()
        .chain([PathBuf::from(SECRETS_MOUNT)])
        .chain(searched)
    {
        if !paths.contains(&path) {
            paths.push(path);
        }
    }

    paths
}
