//! Read-only equality selectors with explicit optional ordering and bounds.
mod aggregate;
mod ordered;
pub(in crate::generators::rust) use aggregate::{aggregate_bindings, qualified_aggregate_bindings};
mod filters;
use super::super::{allocation::Names, naming::sql};
use super::selector;
use crate::database::introspection::{Schema, Table};
pub(in crate::generators::rust) use filters::comparison_bindings;
pub(in crate::generators::rust) use filters::fields::bindings as equality_bindings;
pub(in crate::generators::rust) use filters::related::rows::{decode, projection};

/// Enumerate one-column selectors with supported scalar equality codecs.
/// Extension types, arrays and JSON are excluded; no uniqueness is inferred.
/// Generated reads reuse the supplied pool or transaction connection and never commit it.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(in crate::generators::rust) fn keys(table: &Table) -> Vec<Vec<String>> {
    table
        .columns
        .iter()
        .filter(|column| {
            let declared = column.sql_type.trim().to_lowercase();
            let normalized = match (declared.find('('), declared.rfind(')')) {
                (Some(start), Some(end)) => {
                    format!("{}{}", &declared[..start], &declared[end + 1..])
                }
                _ => declared.clone(),
            };
            let base = normalized.trim();
            !declared.ends_with("[]")
                && matches!(
                    base,
                    "text"
                        | "varchar"
                        | "character varying"
                        | "char"
                        | "character"
                        | "bpchar"
                        | "smallint"
                        | "int2"
                        | "smallserial"
                        | "integer"
                        | "int4"
                        | "serial"
                        | "bigint"
                        | "int8"
                        | "bigserial"
                        | "boolean"
                        | "bool"
                        | "real"
                        | "float4"
                        | "double precision"
                        | "float8"
                        | "numeric"
                        | "decimal"
                        | "uuid"
                        | "bytea"
                        | "date"
                        | "inet"
                        | "cidr"
                        | "time"
                        | "time without time zone"
                        | "timestamp"
                        | "timestamp without time zone"
                        | "timestamptz"
                        | "timestamp with time zone"
                )
        })
        .map(|column| vec![column.name.clone()])
        .collect()
}

/// Emit first-match and all-match reads with typed bound equality values.
/// Nullable columns still take non-null inputs and use SQL equality semantics.
/// First-match selection and collection order are explicitly unspecified.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(
    table: &Table,
    schema: &Schema,
    names: &Names,
    relationship_reads: bool,
) -> String {
    let mut output = filters::render(table, schema, names, relationship_reads);
    let model = names.row(table);
    for columns in keys(table) {
        output.push_str(&ordered::render(table, schema, &columns, names));
        let selector::Selector {
            parameters,
            predicate,
            binds,
            arguments,
        } = selector::render(table, schema, &columns, names);
        for (verb, result, fetch, limit) in [
            (
                "first",
                format!("Option<{model}>"),
                "fetch_optional",
                " LIMIT 1",
            ),
            ("matching", format!("Vec<{model}>"), "fetch_all", ""),
        ] {
            let function = names.selector(table, verb, &columns);
            let query = format!(
                "SELECT * FROM {} WHERE {predicate}{limit}",
                sql(&table.name)
            );
            output.push_str(&format!(
                "/// Equality read with unspecified order; e.g. `{function}(pool, {arguments}).await`.\n\
                 pub async fn {function}<'e>(executor: impl sqlx::Executor<'e, Database = sqlx::Postgres>, {parameters}) -> Result<{result}, sqlx::Error> {{\n\
                     sqlx::query_as::<_, {model}>({query:?}){binds}.{fetch}(executor).await\n\
                 }}\n\n"
            ));
        }
    }
    output
}
