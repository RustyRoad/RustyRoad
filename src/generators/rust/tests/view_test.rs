//! A view generates reads and nothing else.
//!
//! Postgres rejects writes to a plain view, so emitting `create`/`update`/`delete` would
//! advertise methods that can only fail at runtime. The spotlessbinco database has 48 views;
//! before this, `pull` skipped them entirely and every cross-table read stayed hand-written.

use super::support::{column, required, resolve};
use crate::database::introspection::{Schema, Table};
use crate::generators::rust::render;

/// A `purchase_option_catalog` view, as a migration would create it.
pub(super) fn view() -> Table {
    Table {
        name: "purchase_option_catalog".to_string(),
        columns: vec![
            column("component_id", "integer"),
            column("component_version_id", "integer"),
            column("option_id", "text"),
            required("label", "text"),
        ],
        view: true,
        ..Table::default()
    }
}

/// The schema holding just the view.
pub(super) fn schema() -> Schema {
    Schema {
        tables: vec![view()],
        enums: Vec::new(),
    }
}

/// Renders the view's module root.
fn root() -> String {
    let schema = schema();
    let model = resolve(&view(), &schema);

    render::files(&model, &schema)
        .into_iter()
        .find(|file| file.name == "mod.rs")
        .expect("mod.rs should be emitted")
        .contents
}

/// Renders the view's read queries.
fn reads(table: &Table) -> String {
    let schema = Schema {
        tables: vec![table.clone()],
        enums: Vec::new(),
    };
    let model = resolve(table, &schema);

    render::files(&model, &schema)
        .into_iter()
        .find(|file| file.name == "read.rs")
        .expect("read.rs should be emitted")
        .contents
}

#[test]
fn a_view_emits_only_the_root_and_the_reads() {
    let schema = schema();
    let model = resolve(&view(), &schema);
    let names: Vec<&str> = render::files(&model, &schema)
        .iter()
        .map(|file| file.name)
        .collect();

    assert_eq!(names, vec!["mod.rs", "read.rs"], "a view is read-only");
}

#[test]
fn a_views_root_declares_no_write_submodules() {
    let root = root();

    // The declarations must match the files that exist, or the module does not compile.
    assert!(root.contains("mod read;"), "{root}");
    for absent in ["mod create;", "mod update;", "mod delete;"] {
        assert!(!root.contains(absent), "{absent} declared:\n{root}");
    }
}

#[test]
fn a_view_has_no_constructors() {
    let root = root();

    // A view's rows exist only in the database; nothing client-side ever builds one.
    assert!(!root.contains("impl Default"), "{root}");
    assert!(!root.contains("pub fn new("), "{root}");
}

#[test]
fn a_view_with_an_id_gets_all_and_find_queries() {
    let mut table = view();
    table.columns.insert(0, column("id", "integer"));
    let reads = reads(&table);

    assert!(reads.contains("pub async fn all()"), "{reads}");
    assert!(reads.contains("pub async fn find(id: i32)"), "{reads}");
    assert!(reads.contains("WHERE id = $1"), "{reads}");
}

#[test]
fn a_view_without_an_id_gets_only_the_all_query() {
    let reads = reads(&view());

    assert!(reads.contains("pub async fn all()"), "{reads}");
    assert!(!reads.contains("pub async fn find("), "{reads}");
}

#[test]
fn a_view_gets_optional_filters_for_every_projected_column() {
    let root = root();
    let reads = reads(&view());

    assert!(
        root.contains("pub struct PurchaseOptionCatalogAllOptions"),
        "{root}"
    );
    assert!(root.contains("pub component_id: Option<i32>"), "{root}");
    assert!(
        root.contains("pub component_version_id: Option<i32>"),
        "{root}"
    );
    assert!(
        reads.contains("pub async fn all_options(options: PurchaseOptionCatalogAllOptions)"),
        "{reads}"
    );
    assert!(
        reads.contains("filters.push(\"component_id = \")"),
        "{reads}"
    );
    assert!(
        reads.contains("filters.push(\"component_version_id = \")"),
        "{reads}"
    );
    assert!(reads.contains("push_bind_unseparated(value)"), "{reads}");
    assert!(reads.contains("query.separated(\" AND \")"), "{reads}");
    assert!(reads.contains("query.build_query_as::<Self>()"), "{reads}");
}
