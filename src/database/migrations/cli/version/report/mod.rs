//! Output for the versioned migration lifecycle.

mod state;

use crate::database::versions::lifecycle::Started;

pub(super) use state::status;

/// Prints the result of starting a migration.
pub(super) fn started(started: &Started, versioned: bool) {
    println!("Started migration '{}'.", started.version);

    if versioned {
        println!(
            "\nBoth schema versions are now being served. Point clients at a version\n\
             by setting the search path:\n\n  SET search_path TO {};\n",
            started.schema
        );
    } else {
        println!("\nThis backend does not support versioned schemas, so no views were created.");
    }

    println!(
        "The migration is not yet complete. Finish it with:\n  \
         rustyroad migration complete\n\n\
         Or undo it with:\n  rustyroad migration rollback-version"
    );
}

/// Prints the result of completing a migration.
pub(super) fn completed(version: &str) {
    println!("Completed migration '{version}'.");
    println!("The previous schema version is no longer served.");
}

/// Prints the result of rolling back a started migration.
pub(super) fn rolled_back(version: &str) {
    println!("Rolled back migration '{version}'.");
    println!("The previous schema version remains in place.");
}

/// Reports a failure and exits nonzero so callers and CI can detect it.
pub(super) fn fail(message: &str) {
    eprintln!("Error: {message}");
    std::process::exit(1);
}
