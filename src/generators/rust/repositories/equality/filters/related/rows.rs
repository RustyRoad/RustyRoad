//! Select lists and positional decoding use the same introspected column order.
use super::{sql, Names, Table};

/// Emits qualified select columns while leaving their storage codecs unchanged.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(in crate::generators::rust) fn projection(table: &Table, alias: &str) -> String {
    table
        .columns
        .iter()
        .map(|column| format!("{}.{}", sql(alias), sql(&column.name)))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Constructs a generated row using positional SQLx decoding, avoiding alias collisions.
/// Decode errors propagate normally, including NULL in a required column.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(in crate::generators::rust) fn decode(table: &Table, names: &Names, offset: usize) -> String {
    let fields = table
        .columns
        .iter()
        .enumerate()
        .map(|(index, column)| {
            format!(
                "{}: sqlx::Row::try_get(&record, {}usize)?,",
                names.field(table, &column.name),
                offset + index
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!("{} {{ {fields} }}", names.row(table))
}
