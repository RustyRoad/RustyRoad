//! Two-edge foreign-key reads share one snapshot and preserve all generated rows.
mod reads;
use super::{fields, keys, rows, sql, Names, Schema, Table};
use crate::database::introspection::ForeignKey;

/// Emits ordered paths only for two complete, introspected foreign-key edges.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn render(
    table: &Table,
    related: &Table,
    first: &ForeignKey,
    schema: &Schema,
    names: &Names,
) -> String {
    let mut output = String::new();
    for second in &related.foreign_keys {
        let Some(parent) = schema
            .tables
            .iter()
            .find(|t| t.name == second.foreign_table)
        else {
            continue;
        };
        if second.columns.is_empty()
            || second.columns.len() != second.foreign_columns.len()
            || keys(parent).is_empty()
            || second
                .columns
                .iter()
                .any(|column| related.column(column).is_none())
            || second
                .foreign_columns
                .iter()
                .any(|column| parent.column(column).is_none())
        {
            continue;
        }
        let predicate = |foreign: &ForeignKey, source: &str, target: &str| {
            foreign
                .columns
                .iter()
                .zip(&foreign.foreign_columns)
                .map(|(local, remote)| {
                    format!(
                        "{}.{} = {}.{}",
                        sql(source),
                        sql(local),
                        sql(target),
                        sql(remote)
                    )
                })
                .collect::<Vec<_>>()
                .join(" AND ")
        };
        let from = format!(" FROM {} AS \"source_row\" INNER JOIN {} AS \"related_row\" ON {} INNER JOIN {} AS \"parent_row\" ON {} WHERE TRUE",
            sql(&table.name), sql(&related.name), predicate(first, "source_row", "related_row"),
            sql(&parent.name), predicate(second, "related_row", "parent_row"));
        let tables = [table, related, parent];
        let aliases = ["source_row", "related_row", "parent_row"];
        let inputs = ["row_filter", "related_filter", "parent_filter"];
        let bindings = tables
            .iter()
            .zip(aliases)
            .zip(inputs)
            .map(|((table, alias), input)| {
                let columns: Vec<_> = keys(table)
                    .iter()
                    .map(|key| table.column(&key[0]).unwrap())
                    .collect();
                fields::bindings_for(table, &columns, names, input, Some(alias))
            })
            .collect::<Vec<_>>()
            .join("\n");
        output.push_str(&reads::render(
            tables,
            [first, second],
            names,
            &from,
            &bindings,
        ));
    }
    output
}
