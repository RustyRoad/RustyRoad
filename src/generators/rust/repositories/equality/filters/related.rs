//! Inner and left joins follow introspected foreign keys and retain generated rows.
mod left;
mod path;
mod reads;
pub(in crate::generators::rust::repositories::equality) mod rows;
use super::{fields, keys, sql, Names, Schema, Table};

/// Emits relationship reads only when both tables and their key columns exist.
/// Relationships are never guessed; left joins retain unmatched source rows.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(table: &Table, schema: &Schema, names: &Names) -> String {
    let mut output = String::new();
    for foreign in &table.foreign_keys {
        let Some(target) = schema
            .tables
            .iter()
            .find(|item| item.name == foreign.foreign_table)
        else {
            continue;
        };
        if foreign.columns.is_empty()
            || foreign.columns.len() != foreign.foreign_columns.len()
            || keys(target).is_empty()
            || foreign
                .columns
                .iter()
                .any(|name| table.column(name).is_none())
            || foreign
                .foreign_columns
                .iter()
                .any(|name| target.column(name).is_none())
        {
            continue;
        }
        let joined = names.joined(table, foreign);
        let source_row = names.row(table);
        let target_row = names.row(target);
        output.push_str(&format!(
            "/// Both generated rows of the {:?} inner-join relationship.\n\
             /// Example: `let source: &{source_row} = &joined.row;`.\n\
             #[derive(Debug, Clone)]\n\
             pub struct {joined} {{ pub row: {source_row}, pub related: {target_row} }}\n\n",
            foreign.name,
        ));
        let source_columns: Vec<_> = keys(table)
            .iter()
            .map(|key| table.column(&key[0]).unwrap())
            .collect();
        let target_columns: Vec<_> = keys(target)
            .iter()
            .map(|key| target.column(&key[0]).unwrap())
            .collect();
        let source_bindings = fields::bindings_for(
            table,
            &source_columns,
            names,
            "row_filter",
            Some("source_row"),
        );
        let target_bindings = fields::bindings_for(
            target,
            &target_columns,
            names,
            "related_filter",
            Some("related_row"),
        );
        let predicate = foreign
            .columns
            .iter()
            .zip(&foreign.foreign_columns)
            .map(|(local, remote)| {
                format!(
                    "\"source_row\".{} = \"related_row\".{}",
                    sql(local),
                    sql(remote)
                )
            })
            .collect::<Vec<_>>()
            .join(" AND ");
        let from = format!(
            " FROM {} AS \"source_row\" INNER JOIN {} AS \"related_row\" ON {predicate} WHERE TRUE",
            sql(&table.name),
            sql(&target.name)
        );
        let bindings = format!("{source_bindings}\n{target_bindings}");
        output.push_str(&reads::render(
            table, target, foreign, names, &from, &bindings,
        ));
        let left_from = format!(
            " FROM {} AS \"source_row\" LEFT JOIN {} AS \"related_row\" ON {predicate} WHERE TRUE",
            sql(&table.name),
            sql(&target.name)
        );
        output.push_str(&left::render(
            table, target, foreign, names, &left_from, &bindings,
        ));
        output.push_str(&path::render(table, target, foreign, schema, names));
    }
    output
}
