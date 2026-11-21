//! Explicit PostgreSQL projection casts without changing stored codecs.
use super::{Names, Projection};
use crate::database::introspection::{Column, Schema, Table};
use crate::generators::rust::{naming::sql, types};
use std::collections::BTreeSet;

/// Require distinct selected columns with compatible scalar storage, without joins.
/// # Examples
/// ```toml
/// [models.rust_projections.route_paths]
/// table = "routes"
/// columns = ["id", "optimized_path"]
/// json_text = ["optimized_path"]
/// ```
pub(super) fn validate(table: &Table, projection: &Projection) -> Result<(), &'static str> {
    let count =
        projection.json_text.len() + projection.numeric_float.len() + projection.network_host.len();
    if count == 0 {
        return Ok(());
    }
    let distinct: BTreeSet<_> = projection
        .json_text
        .iter()
        .chain(&projection.numeric_float)
        .chain(&projection.network_host)
        .collect();
    if projection.join.is_some() || distinct.len() != count {
        return Err("projection casts require distinct selected columns without a join");
    }
    for name in distinct {
        let column = table.column(name).ok_or("unknown projection cast column")?;
        let ty = column.sql_type.trim().to_ascii_lowercase();
        let compatible = if projection.json_text.contains(name) {
            matches!(ty.as_str(), "json" | "jsonb")
        } else if projection.network_host.contains(name) {
            matches!(ty.as_str(), "inet" | "cidr")
        } else {
            numeric(&ty)
        };
        if !projection.columns.contains(name) || !compatible {
            return Err(
                "projection cast column must be selected and have compatible scalar storage",
            );
        }
    }
    Ok(())
}

/// Emit the catalog column or its explicit database projection with matching nullability.
/// SQL NULL stays None; casts and their errors are evaluated by PostgreSQL.
/// # Examples
/// ```toml
/// [models.rust_projections.route_paths]
/// table = "routes"
/// columns = ["id", "optimized_path"]
/// json_text = ["optimized_path"]
/// ```
pub(super) fn field(
    schema: &Schema,
    names: &Names,
    table: &Table,
    column: &Column,
    projection: &Projection,
) -> (String, String) {
    if projection.network_host.contains(&column.name) {
        let ty = if column.nullable {
            "Option<String>"
        } else {
            "String"
        };
        return (format!("HOST({})", sql(&column.name)), ty.into());
    }
    let (cast, ty) = if projection.json_text.contains(&column.name) {
        ("text", "String")
    } else if projection.numeric_float.contains(&column.name) {
        ("float8", "f64")
    } else {
        return (
            sql(&column.name),
            types::map_column(table, column, schema, names).rust,
        );
    };
    let ty = if column.nullable {
        format!("Option<{ty}>")
    } else {
        ty.into()
    };
    (format!("{}::{cast}", sql(&column.name)), ty)
}

/// Recognize numeric precision/scale without accepting domains or arrays.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn numeric(ty: &str) -> bool {
    if matches!(ty, "numeric" | "decimal") {
        return true;
    }
    let Some((base, rest)) = ty.split_once('(') else {
        return false;
    };
    let Some(modifiers) = rest.strip_suffix(')') else {
        return false;
    };
    let parts: Vec<_> = modifiers.split(',').collect();
    matches!(base.trim_end(), "numeric" | "decimal")
        && (1..=2).contains(&parts.len())
        && parts[0]
            .trim()
            .parse::<u16>()
            .is_ok_and(|p| (1..=1000).contains(&p))
        && (parts.len() == 1
            || parts[1]
                .trim()
                .parse::<i16>()
                .is_ok_and(|s| (-1000..=1000).contains(&s)))
}
