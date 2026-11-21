//! Schema-derived narrow reads and atomic inserts with PostgreSQL projections.
mod casts;
mod insert;
mod ordering;
mod query;
mod selection;
use super::{allocation::Names, repositories::equality};
use crate::database::introspection::Schema;
use std::{
    collections::{BTreeMap, BTreeSet},
    io,
};

mod configuration;
mod joined;
pub use configuration::{InsertInput, JoinProjection, Projection, ProjectionOrder};

/// Validates selected columns and renders typed tuple operations before output is written.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(
    schema: &Schema,
    names: &Names,
    projections: &BTreeMap<String, Projection>,
) -> io::Result<String> {
    let mut output = String::new();
    let mut input_names = BTreeSet::new();
    for (name, projection) in projections {
        let invalid = |reason: &str| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid projection {name:?}: {reason}"),
            )
        };
        if name.is_empty()
            || !name.starts_with(|c: char| c.is_ascii_lowercase())
            || !name
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
            || syn::parse_str::<syn::Ident>(name).is_err()
            || names.function_reserved(name)
        {
            return Err(invalid("invalid or reserved function name"));
        }
        let table = schema
            .table(&projection.table)
            .ok_or_else(|| invalid("table outside pull scope"))?;
        casts::validate(table, projection).map_err(|reason| invalid(reason))?;
        ordering::validate(table, projection).map_err(|reason| invalid(reason))?;
        insert::policy::validate(table, projection).map_err(|reason| invalid(reason))?;
        if projection.insert_input.is_some() {
            let input = insert::input::name(name);
            if names.type_reserved(&input) || !input_names.insert(input) {
                return Err(invalid(
                    "projection input type name collision; rename the operation",
                ));
            }
        }
        if projection.join.is_some() {
            if projection.comparisons {
                return Err(invalid(
                    "typed projection comparisons require an unjoined source",
                ));
            }
            output.push_str(&joined::render(schema, names, name, table, projection)?);
            continue;
        }
        let distinct: BTreeSet<_> = projection.columns.iter().collect();
        if projection.columns.is_empty()
            || projection.columns.len() > 16
            || distinct.len() != projection.columns.len()
            || projection
                .columns
                .iter()
                .any(|column| table.column(column).is_none())
            || (!projection.insert && equality::keys(table).is_empty())
        {
            return Err(invalid(
                "requires 1-16 distinct known columns; reads need a supported filter",
            ));
        }
        if projection.insert {
            output.push_str(&insert::render(schema, names, name, table, projection));
        } else {
            output.push_str(&query::render(schema, names, name, table, projection));
        }
    }
    Ok(output)
}
