use super::super::{list_migrations, validate_migrations};

pub(super) async fn validate() {
    match validate_migrations().await {
        Ok(report) => println!(
            "Validated {} migration(s) against an isolated temporary {} database. Temporary resources were removed.",
            report.migrations_validated, report.database_type
        ),
        Err(error) => {
            eprintln!("Migration validation failed: {error}");
            std::process::exit(1);
        }
    }
}

pub(super) async fn list(format: &str) {
    super::print_config();
    list_migrations(format)
        .await
        .expect("Error listing migrations");
}
