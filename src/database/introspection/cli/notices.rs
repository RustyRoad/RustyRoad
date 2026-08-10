//! Warnings shown after a successful pull.

use super::Language;

/// Warns about tables the composition file does not reference.
///
/// Their procedures were generated but are unreachable, which is invisible without
/// saying so, since nothing errors and every file looks current.
pub(super) fn unwired(tables: &[String]) {
    if tables.is_empty() {
        return;
    }

    println!(
        "\nWarning: {} table(s) are generated but not served, because db/api.ts \
         does not reference them:",
        tables.len()
    );
    for table in tables {
        println!("  {table}");
    }
    println!(
        "Add them to the router in db/api.ts, for example:\n\
        \x20 {}: generated.{},",
        tables[0], tables[0]
    );
}

/// Explains that some files were left as the developer wrote them.
pub(super) fn preserved() {
    println!(
        "\n'kept' files already existed and are yours to edit; \
         pull does not overwrite them.\n\
         Use --force to replace them with the generated version."
    );
}

/// Prints the install line for the generated code's dependencies.
pub(super) fn next_steps(language: Language) {
    match language {
        Language::TypeScript => println!(
            "\nNext:\n  npm install drizzle-orm pg zod drizzle-zod fastify \\\n\
            \x20   @orpc/server @orpc/openapi @orpc/zod"
        ),
        Language::Rust => println!(
            "\nNext:\n  cargo add actix-web serde serde_json chrono uuid bigdecimal mac_address sqlx --features serde/derive,chrono/serde,uuid/serde,bigdecimal/serde,mac_address/serde,sqlx/runtime-tokio,sqlx/postgres,sqlx/chrono,sqlx/uuid,sqlx/json,sqlx/bigdecimal,sqlx/ipnetwork,sqlx/mac_address"
        ),
    }
}
