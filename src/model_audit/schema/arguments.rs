//! Catalog evidence and reviewed storage-contract CLI options.
mod generation;
use clap::Arg;

/// Defines database evidence and reviewed storage-contract command arguments.
/// # Examples
/// ```sh
/// rustyroad model audit --help
/// ```
pub(super) fn args() -> Vec<Arg> {
    let mut args = generation::args();
    args.extend([
        Arg::new("database")
            .long("database")
            .action(clap::ArgAction::SetTrue)
            .conflicts_with("schema-snapshot")
            .help("Read PostgreSQL catalog in a read-only transaction"),
        Arg::new("schema-snapshot")
            .long("schema-snapshot")
            .num_args(1)
            .help("Catalog JSON snapshot; a complete snapshot must include every schema relation"),
        Arg::new("schema")
            .long("schema")
            .default_value("public")
            .help("PostgreSQL schema to inspect"),
        Arg::new("table")
            .long("table")
            .action(clap::ArgAction::Append)
            .num_args(1)
            .help("Review persistence mapping MODEL_ID_OR_UNIQUE_NAME=TABLE"),
        Arg::new("primary-key")
            .long("primary-key")
            .action(clap::ArgAction::Append)
            .num_args(1)
            .help("Reviewed primary key TABLE=COLUMN[,COLUMN]; never inferred"),
        Arg::new("storage-projection")
            .long("storage-projection")
            .action(clap::ArgAction::Append)
            .num_args(1)
            .help("Reviewed SOURCE=GENERATED projection; keeps source differences and forbids projection DDL"),
        Arg::new("type-map")
            .long("type-map")
            .action(clap::ArgAction::Append)
            .num_args(1)
            .help("Reviewed storage mapping RUST_TYPE=POSTGRES_TYPE; repeatable"),
    ]);
    args
}
