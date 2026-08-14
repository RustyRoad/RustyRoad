//! Assigning each table a module and type name that no other table claims.
//!
//! Singularizing each table independently is unsafe: a schema holding both `user` and
//! `users` maps both to `user`, and the second model silently overwrites the first. One
//! table loses its model entirely while the survivor carries the other's SQL — and nothing
//! errors, because writing a file twice is legal. The spotlessbinco database has eight such
//! pairs, so this is the common case rather than a curiosity.
//!
//! Names are therefore resolved for the whole schema at once: singularize where it is
//! unambiguous, and keep the table's own name where it is not.

use crate::database::introspection::Schema;
use crate::generators::rust::casing;

/// The names one table is generated under.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Names {
    /// The module folder, snake_case.
    pub module: String,
    /// The struct name, PascalCase.
    pub type_name: String,
}

/// Resolves the module and type name for every table in `schema`.
///
/// Returned in table order, so the caller can zip it against `schema.tables`.
pub fn resolve(schema: &Schema) -> Vec<Names> {
    let singular: Vec<String> = schema
        .tables
        .iter()
        .map(|table| casing::singularize(&table.name))
        .collect();

    schema
        .tables
        .iter()
        .zip(&singular)
        .map(|(table, candidate)| {
            // Only claim the singular form when exactly one table wants it. Two tables
            // asking for the same name means neither may have it, so both keep their own.
            let contested = singular.iter().filter(|other| *other == candidate).count() > 1;
            let stem = if contested { &table.name } else { candidate };

            casing::to_snake(stem)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .scan(reserved(), |taken: &mut Vec<String>, module| {
            // Casing can still fold two distinct table names together — `a-b` and `a_b`
            // both become `a_b`. Table names are unique, so a numeric suffix is enough to
            // separate them and is preferable to overwriting a model.
            let module = unique(module, taken);
            taken.push(module.clone());

            Some(Names {
                type_name: casing::to_pascal(&module),
                module,
            })
        })
        .collect()
}

/// Names no table may claim, because the layout itself uses them.
///
/// The shared `enums` module holds one declaration per Postgres enum type, and `api` holds the
/// generated HTTP surface; a table that happened to produce either name would overwrite it.
fn reserved() -> Vec<String> {
    vec!["enums".to_string(), "api".to_string()]
}

/// Returns `candidate`, suffixed if it is already taken.
fn unique(candidate: String, taken: &[String]) -> String {
    if !taken.contains(&candidate) {
        return candidate;
    }

    // Starts at 2 because the unsuffixed name is conceptually the first.
    (2..)
        .map(|index| format!("{candidate}_{index}"))
        .find(|suffixed| !taken.contains(suffixed))
        .expect("an unbounded range always yields an unused name")
}
