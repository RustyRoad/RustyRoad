use super::super::{allocation::Names, models, procedures, repositories};
use crate::database::introspection::Schema;
use crate::generators::{
    ownership::{write_owned, Ownership},
    report::Outcome,
};
use std::{fs, io, path::Path};

/// Writes schema-derived artifacts while preserving scaffold ownership rules.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn write(
    out: &Path,
    schema: &Schema,
    names: &Names,
    force: bool,
    aggregates: &str,
    relationship_reads: bool,
) -> io::Result<Vec<Outcome>> {
    fs::create_dir_all(out)?;

    let files = [
        (
            "models.rs",
            models::render(schema, names),
            Ownership::Generated,
        ),
        (
            "repositories.rs",
            format!(
                "{}{aggregates}",
                repositories::render_with_relationship_reads(schema, names, relationship_reads)
            ),
            Ownership::Generated,
        ),
        (
            "procedures.rs",
            procedures::render(schema, names),
            Ownership::Generated,
        ),
        ("api.rs", procedures::composition(), Ownership::Scaffold),
        ("mod.rs", procedures::module(), Ownership::Scaffold),
    ];

    files
        .into_iter()
        .map(|(name, contents, ownership)| write_owned(out, name, &contents, ownership, force))
        .collect()
}
