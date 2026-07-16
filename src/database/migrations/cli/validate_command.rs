use clap::Command;

pub(super) fn build() -> Command {
    Command::new("validate")
        .about("Validate the complete migration chain in an isolated temporary database")
        .long_about(
            "Validates every up.sql migration in timestamp order against a disposable database.\n\n\
             SAFETY:\n\
              - Requires ENVIRONMENT=test (or ENV=test)\n\
              - Reads server credentials only from ./rustyroad.test.toml\n\
              - Never connects to or modifies the configured shared test database\n\
              - Creates a random database and scoped login on PostgreSQL/MySQL, or a temporary file for SQLite\n\
              - Removes disposable database credentials and storage after success or failure\n\n\
             EXAMPLE:\n\
              ENVIRONMENT=test rustyroad migration validate\n",
        )
}
