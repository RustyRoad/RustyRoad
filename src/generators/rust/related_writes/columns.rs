use super::{Names, RelatedOperation, RelatedWrite};
use crate::database::introspection::{Schema, Table};
use crate::generators::rust::{naming::sql, types};

/// Derives argument names, codecs, bindings and predicates from selected columns.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn arguments(
    table: &Table,
    related: &Table,
    schema: &Schema,
    names: &Names,
    write: &RelatedWrite,
) -> (String, String, String) {
    let mut parameters = Vec::new();
    let mut binds = String::new();
    let mut predicates = Vec::new();
    for (index, (owner, column, predicate)) in write
        .columns
        .iter()
        .map(|c| (table, c, false))
        .chain(std::iter::once((related, &write.lookup, true)))
        .chain(write.predicates.iter().map(|c| (table, c, true)))
        .enumerate()
    {
        let mut field = owner.column(column).unwrap().clone();
        if predicate {
            field.nullable = false;
        }
        let arg = format!("value_{index}");
        parameters.push(format!(
            "{arg}: {}",
            types::map_column(owner, &field, schema, names).rust
        ));
        binds.push_str(&format!(".bind({arg})"));
        if predicate {
            let alias = if index == write.columns.len() {
                "related_row"
            } else {
                "source_row"
            };
            predicates.push(format!("{alias}.{} = ${}", sql(column), index + 1));
        }
    }
    (parameters.join(", "), binds, predicates.join(" AND "))
}

/// Generates assignments or insertion values while keeping catalog defaults omitted.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn mutation(
    table: &Table,
    related: &Table,
    write: &RelatedWrite,
    predicates: &str,
) -> String {
    let foreign = table
        .foreign_keys
        .iter()
        .find(|f| f.name == write.foreign_key)
        .unwrap();
    if write.operation == RelatedOperation::Create {
        let mut columns: Vec<_> = write.columns.iter().map(|c| sql(c)).collect();
        columns.push(sql(&foreign.columns[0]));
        let mut values: Vec<_> = (1..=write.columns.len()).map(|i| format!("${i}")).collect();
        values.push(format!("related_row.{}", sql(&foreign.foreign_columns[0])));
        return format!(
            "INSERT INTO {} ({}) SELECT {} FROM {} AS related_row WHERE {predicates} RETURNING *",
            sql(&table.name),
            columns.join(", "),
            values.join(", "),
            sql(&related.name)
        );
    }
    let mut assignments: Vec<_> = write
        .columns
        .iter()
        .enumerate()
        .map(|(i, c)| format!("{} = ${}", sql(c), i + 1))
        .collect();
    if let Some(clock) = &write.clock {
        assignments.push(format!("{} = CURRENT_TIMESTAMP", sql(clock)));
    }
    format!("UPDATE {} AS source_row SET {} FROM {} AS related_row WHERE source_row.{} = related_row.{} AND {predicates} RETURNING source_row.*",
        sql(&table.name), assignments.join(", "), sql(&related.name),
        sql(&foreign.columns[0]), sql(&foreign.foreign_columns[0]))
}
