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
        "
Next:
  npm install drizzle-orm pg zod drizzle-zod fastify \
         @orpc/server @orpc/openapi @orpc/zod"
    );
}

/// Warns about models the parent module does not declare.
///
/// Their files were written but are not compiled, which is invisible without saying
/// so: a module Rust never sees produces no error of its own.
pub(super) fn undeclared(models: &[String]) {
    if models.is_empty() {
        return;
    }

    println!(
        "\nWarning: {} model(s) were written but are not declared, because the \
         parent mod.rs is yours and does not list them:",
        models.len()
    );
    for model in models {
        println!("  {model}");
    }
    println!(
        "Declare them in the parent mod.rs, for example:\n\
        \x20 pub mod {};",
        models[0]
    );
}
