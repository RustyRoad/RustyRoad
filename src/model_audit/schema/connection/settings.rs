//! Resolve environment/configuration fields without logging credential values.
use std::{env, path::Path};

/// Complete DATABASE_* settings override the environment-specific TOML file.
pub(super) fn values(root: &Path, config: Option<&Path>) -> Result<Vec<String>, String> {
    let keys = ["HOST", "PORT", "USER", "PASSWORD", "NAME"];
    let environment: Vec<_> = keys
        .iter()
        .map(|key| env::var(format!("DATABASE_{key}")).ok())
        .collect();
    if environment.iter().any(Option::is_some) {
        return environment
            .into_iter()
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| {
                "Partial DATABASE_* configuration; all five connection fields are required".into()
            });
    }
    let path = config
        .map(Path::to_path_buf)
        .unwrap_or_else(|| root.join(crate::database::get_config_file_name()));
    let source =
        std::fs::read_to_string(&path).map_err(|_| format!("Cannot read {}", path.display()))?;
    let document: toml::Value = toml::from_str(&source)
        .map_err(|_| "Invalid RustyRoad configuration TOML (contents redacted)")?;
    let db = document
        .get("database")
        .ok_or("Missing [database] section")?;
    if db.get("database_type").and_then(toml::Value::as_str) != Some("postgres") {
        return Err("Table auditing currently requires database_type = postgres".into());
    }
    keys.iter()
        .map(|key| {
            let name = format!("database_{}", key.to_lowercase());
            match db.get(&name) {
                Some(toml::Value::String(v)) => Ok(v.clone()),
                Some(toml::Value::Integer(v)) if *key == "PORT" => Ok(v.to_string()),
                _ => Err(format!("Missing or invalid [database].{name}")),
            }
        })
        .collect()
}
