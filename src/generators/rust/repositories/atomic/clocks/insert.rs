//! Atomic INSERT assembly with selected database clocks and ordinary bound values.
use super::policy;
use crate::database::introspection::Table;
use crate::generators::rust::repositories::{fields, inputs};
use crate::generators::rust::{allocation::Names, naming::sql};

/// Build the insertion half, omitting unselected optional fields as usual.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out ./generated-db
/// ```
pub(super) fn render(table: &Table, names: &Names) -> String {
    let columns = inputs::new_columns(table);
    let required = columns
        .iter()
        .any(|c| !inputs::optional(c) && !policy::eligible(table, c));
    let presence = columns
        .iter()
        .map(|c| format!("input.{}.is_some()", names.field(table, &c.name)))
        .collect::<Vec<_>>()
        .join(" || ");
    let present = if required {
        "true".to_string()
    } else {
        format!("!insert_clocks.is_empty() || {presence}")
    };
    let (columns, values): (Vec<_>, Vec<_>) = columns.iter().map(|column| {
        if !policy::eligible(table, column) {
            return fields::insert(table, &[*column], names);
        }
        let field = names.field(table, &column.name);
        let name = &column.name;
        let quoted = sql(name);
        (format!("if input.{field}.is_some() || insert_clocks.contains(&{name:?}) {{ columns.push({quoted:?}); }}"),
         format!("if let Some(value) = input.{field}.as_ref() {{ values.push_bind(value); }} else if insert_clocks.contains(&{name:?}) {{ values.push(\"CURRENT_TIMESTAMP\"); }}"))
    }).unzip();
    let prefix = format!("INSERT INTO {} AS {} ", sql(&table.name), sql("existing"));
    let columns = columns.join("\n");
    let values = values.join("\n");
    format!(
        "let mut query = QueryBuilder::<Postgres>::new({prefix:?});\n\
         if {present} {{\n\
             query.push(\"(\");\n\
             {{ let mut columns = query.separated(\", \");\n{columns}\n }}\n\
             query.push(\") VALUES (\");\n\
             {{ let mut values = query.separated(\", \");\n{values}\n }}\n\
             query.push(\")\");\n\
         }} else {{ query.push(\"DEFAULT VALUES\"); }}\n"
    )
}
