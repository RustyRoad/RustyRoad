//! The composition point where generated procedures meet hand-written ones.
//!
//! `router.ts` is regenerated on every `pull`, so adding a hand-written procedure
//! there would be lost. This emits a separate `api.ts` that `pull` writes once and
//! then never touches, following the pattern of composing routers by spreading
//! generated ones alongside your own.

use crate::database::introspection::{Schema, Table};
use crate::generators::typescript::casing::{binding, Casing};

/// Renders `api.ts`, the file the developer owns.
pub fn composition(schema: &Schema, casing: Casing) -> String {
    let tables: Vec<&Table> = schema
        .tables
        .iter()
        .filter(|table| table.has_simple_key())
        .collect();

    format!("{}{}{}", header(), members(&tables, casing), footer())
}

/// Renders the imports and the explanatory comment.
fn header() -> String {
    String::from(
        "// Written once by `rustyroad pull`, then yours.\n\
         //\n\
         // `pull` regenerates router.ts every run but never touches this file, so\n\
         // hand-written procedures added here survive regeneration. Serve this router\n\
         // rather than the generated one.\n\n\
         import { generated } from \"./router\";\n\n\
         export type { RouterContext } from \"./router\";\n\n",
    )
}

/// Renders the router, listing each generated namespace explicitly.
///
/// A plain object literal is used rather than `os.router()`, because that helper
/// widens the context type and would reject procedures built on a typed context.
/// oRPC accepts a nested record of procedures directly.
fn members(tables: &[&Table], casing: Casing) -> String {
    let entries = tables
        .iter()
        .map(|table| {
            format!(
                "\t{}: generated.{},",
                table.name,
                binding(&table.name, casing)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "/** Root router: generated procedures plus anything you add. */\n\
         export const router = {{\n\
         {entries}\n\n\
         \t// Add hand-written routers here, for example:\n\
         \t//   billing: billingRouter,\n\
         }};\n\n"
    )
}

/// Renders the exported router type.
fn footer() -> String {
    String::from(
        "/** Router type, for building a typed client. */\n\
         export type AppRouter = typeof router;\n",
    )
}
