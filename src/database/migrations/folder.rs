//! Migration folder naming.

use chrono::Local;
use std::path::Path;

/// Timestamp prefix format shared by every migration folder.
const TIMESTAMP: &str = "%Y%m%d%H%M%S";

/// Returns a migration folder path that does not already exist.
///
/// The prefix has second resolution, so generating two migrations with the same
/// name inside one second would otherwise collide. That happens in practice when a
/// tool retries, so a numeric suffix is appended until the path is free rather than
/// failing the generate.
pub fn folder_for(migrations_dir: &str, name: &str) -> String {
    let timestamp = Local::now().format(TIMESTAMP).to_string();
    let base = format!("{migrations_dir}/{timestamp}-{name}");

    if !Path::new(&base).exists() {
        return base;
    }

    // Keep the `<timestamp>-<name>` shape so directory parsing is unaffected.
    for attempt in 2..1000 {
        let candidate = format!("{migrations_dir}/{timestamp}-{name}_{attempt}");
        if !Path::new(&candidate).exists() {
            return candidate;
        }
    }

    base
}
