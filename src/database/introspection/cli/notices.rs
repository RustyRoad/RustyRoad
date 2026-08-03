//! Warnings shown after a successful pull.

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
pub(super) fn next_steps() {
    println!(
        "\nNext:\n  npm install drizzle-orm pg zod drizzle-zod fastify \\\n\
        \x20   @orpc/server @orpc/openapi @orpc/zod"
    );
}
