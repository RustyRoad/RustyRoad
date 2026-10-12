//! Reading connection settings from a RustyRoad config file's `[database]` section.

use super::config::KEYS;
use toml::Value;

/// Returns the settings ordered as `KEYS`, naming the first one missing.
///
/// `database_port` may be a string or an integer: the generated `rustyroad.toml` writes a
/// string, but a hand-edited file commonly uses a number.
pub(super) fn values(contents: &str, source: &str) -> Result<Vec<String>, String> {
    let document: Value =
        toml::from_str(contents).map_err(|error| format!("failed to parse {source}: {error}"))?;
    let database = document
        .get("database")
        .ok_or_else(|| format!("{source} has no [database] section"))?;

    KEYS.iter()
        .map(|key| match database.get(format!("database_{key}")) {
            Some(Value::String(value)) => Ok(value.clone()),
            Some(Value::Integer(value)) => Ok(value.to_string()),
            _ => Err(format!("{source} is missing [database].database_{key}")),
        })
        .collect()
}
