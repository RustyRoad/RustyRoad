use clap::Command;

pub(super) fn build() -> Command {
    Command::new("validate")
        .about("Validate the complete migration chain in an isolated temporary database")
        .long_about(
            "Validates every up.sql migration in timestamp order against a disposable database.\n\n\
             SAFETY:\n\
              - Reads the active rustyroad.toml environment configuration\n\
              - Never connects to or modifies the configured database\n\
              - Creates a random database and scoped login on PostgreSQL/MySQL, or a temporary file for SQLite\n\
              - Removes disposable database credentials and storage after success or failure\n\n\
             EXAMPLES:\n\
              rustyroad migration validate\n\
              ENVIRONMENT=prod rustyroad migration validate\n",
        )
}
