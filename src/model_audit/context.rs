//! Resolve project defaults once, without mutating process environment or exposing credentials.

use super::cli::values;
use clap::{parser::ValueSource, ArgMatches};
use serde::Serialize;
use std::{
    env,
    path::{Path, PathBuf},
};

/// Non-secret resolution evidence included in every report.
#[derive(Debug, Serialize)]
pub(super) struct Context {
    pub environment: String,
    pub config_file: Option<PathBuf>,
    pub connection_source: Option<String>,
    pub catalog_mode: String,
    pub sources: Vec<PathBuf>,
    pub generated: Option<PathBuf>,
}

/// Resolve `--env > ENVIRONMENT > ENV > dev`; do not silently switch environments.
fn environment(args: &ArgMatches) -> Result<String, String> {
    let name = args
        .get_one::<String>("env")
        .cloned()
        .unwrap_or_else(crate::database::get_environment);
    if name.is_empty()
        || name.len() > 64
        || !name
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || c == b'_' || c == b'-')
    {
        return Err("Invalid environment name; use letters, digits, underscores or hyphens".into());
    }
    Ok(name)
}

/// Infer the nearest project, stopping at a Git boundary; explicit --root is never changed.
pub(super) fn root(args: &ArgMatches) -> Result<PathBuf, String> {
    let start = Path::new(args.get_one::<String>("root").unwrap())
        .canonicalize()
        .map_err(|e| e.to_string())?;
    if !start.is_dir() {
        return Err("--root must be a directory".into());
    }
    if args.value_source("root") == Some(ValueSource::CommandLine) {
        return Ok(start);
    }
    let env_file = filename(&environment(args)?);
    for path in start.ancestors() {
        if path.join("rustyroad.toml").is_file() || path.join(&env_file).is_file() {
            return Ok(path.into());
        }
        if path.join(".git").exists() {
            break;
        }
    }
    Ok(start)
}

/// Config filename follows RustyRoad's existing environment convention.
fn filename(environment: &str) -> String {
    if environment == "dev" {
        "rustyroad.toml".into()
    } else {
        format!("rustyroad.{environment}.toml")
    }
}

/// Default to the configured catalog and the generator's source/output paths.
pub(super) fn resolve(root: &Path, args: &ArgMatches) -> Result<Context, String> {
    let environment = environment(args)?;
    let explicit_config = args
        .get_one::<String>("config")
        .cloned()
        .or_else(|| env::var("RUSTYROAD_CONFIG_PATH").ok());
    let config = root.join(
        explicit_config
            .as_deref()
            .map(Path::new)
            .unwrap_or(Path::new(&filename(&environment))),
    );
    let mut connection_source = if env::var_os("DATABASE_URL").is_some() {
        Some("DATABASE_URL".to_string())
    } else if ["HOST", "PORT", "USER", "PASSWORD", "NAME"]
        .iter()
        .any(|k| env::var_os(format!("DATABASE_{k}")).is_some())
    {
        Some("DATABASE_*".to_string())
    } else {
        None
    };
    let forced_off =
        args.get_flag("offline") || args.get_one::<String>("schema-snapshot").is_some();
    if connection_source.is_none() && !forced_off {
        if config.is_file() {
            let content = std::fs::read_to_string(&config)
                .map_err(|_| "Cannot read selected RustyRoad config")?;
            let parsed: toml::Value = toml::from_str(&content)
                .map_err(|_| "Invalid selected RustyRoad config TOML (contents redacted)")?;
            if parsed.get("database").is_some() {
                connection_source = Some("rustyroad_config".into());
            } else if args.get_flag("database") || explicit_config.is_some() || environment != "dev"
            {
                return Err("Selected config has no [database] section; use --offline for a code-only audit".into());
            }
        } else if args.get_flag("database")
            || explicit_config.is_some()
            || environment != "dev"
            || root.join("rustyroad.toml").is_file()
        {
            return Err(format!("Selected config {} is missing; no fallback to another environment. Use --offline for a code-only audit", config.display()));
        }
    }
    let catalog_mode = if args.get_one::<String>("schema-snapshot").is_some() {
        "snapshot"
    } else if args.get_flag("offline") {
        "offline"
    } else if connection_source.is_some() {
        "database"
    } else {
        "code_only"
    }
    .to_string();
    let layout = crate::generators::model_paths::resolve_with(
        root,
        args.get_one::<String>("generated").map(Path::new),
    )?;
    let generated = match layout.generated {
        Some(path) if path.is_file() => Some(path.canonicalize().map_err(|e| e.to_string())?),
        Some(path) if layout.generated_explicit => {
            return Err(format!(
                "Configured generated models {} not found",
                path.display()
            ))
        }
        _ => None,
    };
    let sources = if !values(args, "source").is_empty() {
        values(args, "source")
            .iter()
            .map(|p| root.join(p))
            .collect()
    } else if args.get_flag("all-sources") {
        ["src", "crates"]
            .iter()
            .map(|p| root.join(p))
            .filter(|p| p.is_dir())
            .collect()
    } else {
        // Prefer generator conventions. Never silently broaden to all controllers/jobs.
        if layout.source_explicit || layout.source.is_dir() {
            vec![layout.source]
        } else if root.join("src/models.rs").is_file() {
            vec![root.join("src/models.rs")]
        } else {
            Vec::new()
        }
    };
    if sources.is_empty() {
        return Err("No model source location found; expected src/models/ or crates/models/src/. Use --source or --all-sources, or set [models].source".into());
    }
    if !["database", "snapshot"].contains(&catalog_mode.as_str()) && generated.is_none() {
        return Err("No generated models found; generate them with rustyroad pull --language rust, select --generated, or configure database catalog auditing".into());
    }
    Ok(Context {
        environment,
        config_file: (config.is_file() || explicit_config.is_some()).then_some(config),
        connection_source,
        catalog_mode,
        sources,
        generated,
    })
}

/// Selectors for conventional discovery, environment and explicit code-only operation.
pub(super) fn arguments(command: clap::Command) -> clap::Command {
    command
        .arg(clap::Arg::new("env").long("env").alias("environment").help("Environment config: overrides ENVIRONMENT, then ENV, then dev"))
        .arg(clap::Arg::new("config").long("config").help("Explicit database config; overrides RUSTYROAD_CONFIG_PATH; relative to project root"))
        .arg(clap::Arg::new("offline").long("offline").action(clap::ArgAction::SetTrue).conflicts_with("database").help("Never connect to PostgreSQL; still uses model layout and any explicit schema snapshot"))
        .arg(clap::Arg::new("all-sources").long("all-sources").action(clap::ArgAction::SetTrue).conflicts_with("source").help("Opt into scanning all src/ and crates/ rather than model locations"))
}
