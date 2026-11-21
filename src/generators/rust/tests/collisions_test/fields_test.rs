//! Field bindings and lookup predicates preserve the original SQL names.

use super::{fixture, syntax};
use crate::generators::rust::{allocation::Names, models, repositories};

#[test]
fn colliding_fields_and_functions_keep_original_sql_mappings() {
    let schema = fixture::schema();
    let names = Names::new(&schema);
    let models = models::render(&schema, &names);
    let repositories = repositories::render(&schema, &names);
    let table = schema.table("service_areas").unwrap();
    assert_ne!(names.field(table, "type"), names.field(table, "type_"));
    for column in ["type", "type_"] {
        let field = names.field(table, column);
        assert!(models.contains(&format!("#[sqlx(rename = {column:?})]")));
        assert!(models.contains(&format!("#[serde(rename = {column:?})]")));
        assert!(models.contains(&format!("pub {field}: String")));
        assert!(repositories.contains(&format!("input.{field}")));
        assert!(repositories.contains(&format!("patch.{field}")));
        let latest = names.selector(table, "latest", &[column.into()]);
        let body = syntax::function_source(&repositories, latest);
        assert!(body.contains(&format!("\\\"{column}\\\" = $1")));
        assert!(body.contains("ORDER BY \\\"updated_at\\\" DESC NULLS FIRST"));
    }
    assert_ne!(
        names.selector(table, "latest", &["type".into()]),
        names.selector(table, "latest", &["type_".into()])
    );
}
