//! Focused local fixtures for parsing, matching, selection, and safe output behavior.

use super::{compare, discover, execute, parse, plan, types::Model};
use std::{collections::BTreeMap, path::Path};

/// Parse fixture declarations without invoking rustc, Cargo or a database.
fn models(source: &str) -> Vec<Model> {
    parse::parse(
        Path::new("/project/src/models.rs"),
        Path::new("/project"),
        source,
    )
    .unwrap()
}

#[test]
fn where_clause_contracts_are_preserved() {
    let a = models("pub struct Row<T> where T: Clone { pub value: T }");
    let b = models("pub struct Rows<T> where T: Copy { pub value: T }");
    assert!(compare::differences(&a[0], &b[0]).generics_differ);
    assert!(a[0].generics.contains("Clone"));
}

#[test]
fn ast_ignores_comments_and_preserves_multiline_types_and_attributes() {
    let a = models("#[derive(Clone)] pub struct Row { #[serde(rename=\"other\")] pub value: Option<\n i32\n>, // not a type\n }");
    let b = models("pub struct Row { pub value: Option<i32> }");
    let diff = compare::differences(&a[0], &b[0]);
    assert!(diff.type_changes.is_empty());
    assert!(diff.attributes_differ);
    assert_eq!(diff.field_contract_changes, ["value"]);
}

#[test]
fn additional_columns_do_not_mask_type_differences() {
    let a = models("pub struct Zone { pub id: Option<i32>, pub geometry: Polygon }");
    let b = models("pub struct Zones { pub id: i32, pub geometry: String, pub metadata: String }");
    let c = plan::candidate(a[0].clone(), &b, &BTreeMap::new());
    assert_eq!(c.classification, "db_superset");
    assert_eq!(c.differences.generated_only, ["metadata"]);
    assert_eq!(c.differences.type_changes.len(), 2);
    assert!(c.blockers.iter().any(|b| b.contains("lossy")));
}

#[test]
fn duplicate_names_require_qualified_mapping() {
    let a = models(
        "mod left { pub struct Row { pub id: i32 } } mod right { pub struct Row { pub id: i32 } }",
    );
    let b = models("pub struct Rows { pub id: i32 }");
    assert_ne!(a[0].id, a[1].id);
    assert!(plan::mappings(&["Row=Rows".into()], &a, &b).is_err());
    let map = plan::mappings(&[format!("{}=Rows", a[0].id)], &a, &b).unwrap();
    assert_eq!(
        plan::candidate(a[0].clone(), &b, &map).match_basis,
        "explicit"
    );
}

#[test]
fn ambiguous_targets_and_write_inputs_are_not_silently_mapped() {
    let a = models("pub struct Row { pub id: i32 } pub struct NewRow { pub id: i32 }");
    let b = models(
        "mod a { pub struct Rows { pub id: i32 } } mod b { pub struct Rows { pub id: i32 } }",
    );
    assert_eq!(
        plan::candidate(a[0].clone(), &b, &BTreeMap::new()).classification,
        "ambiguous_mapping"
    );
    assert_eq!(
        plan::candidate(a[1].clone(), &b, &BTreeMap::new()).classification,
        "unmatched"
    );
}

#[test]
fn declaration_kinds_and_unicode_positions_are_explicit() {
    let source = "const X: &str = \"😀\"; pub struct Row { pub id: i32 }\nenum E { A }\ntype Alias = Row;\nstruct Tuple(i32);";
    let found = models(source);
    assert_eq!(
        found.iter().map(|m| m.kind.as_str()).collect::<Vec<_>>(),
        ["struct", "enum", "alias", "tuple_or_unit"]
    );
    let index = source.find("Row").unwrap();
    assert_eq!(
        found[0].character,
        source[..index].encode_utf16().count() as u32
    );
}

#[test]
fn audit_filters_and_reports_partial_discovery_without_source_changes() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::create_dir_all(root.join("src/db")).unwrap();
    let custom = "pub struct Row { pub id: i32 } pub struct Other { pub id: i32 }";
    std::fs::write(root.join("src/models.rs"), custom).unwrap();
    std::fs::write(
        root.join("src/db/models.rs"),
        "pub struct Rows { pub id: i32 }",
    )
    .unwrap();
    std::fs::write(root.join("src/broken.rs"), "pub struct {").unwrap();
    let matches = super::command()
        .try_get_matches_from([
            "model",
            "audit",
            "--root",
            root.to_str().unwrap(),
            "--offline",
            "--all-sources",
            "--name",
            "Row",
        ])
        .unwrap();
    let report = execute::audit(matches.subcommand_matches("audit").unwrap()).unwrap();
    assert_eq!(report.discovered, 2);
    assert_eq!(report.selected, 1);
    assert_eq!(report.candidates[0].classification, "field_syntax_equal");
    assert_eq!(report.diagnostics.len(), 1);
    assert_eq!(
        std::fs::read_to_string(root.join("src/models.rs")).unwrap(),
        custom
    );
    assert!(super::output::emit(&report, "json", Some(&"src/models.rs".into())).is_err());
    assert_eq!(
        std::fs::read_to_string(root.join("src/models.rs")).unwrap(),
        custom
    );
}

#[test]
fn generated_detection_requires_unique_inventory() {
    let dir = tempfile::tempdir().unwrap();
    for parent in ["src/db", "crates/db/src"] {
        std::fs::create_dir_all(dir.path().join(parent)).unwrap();
        std::fs::write(
            dir.path().join(parent).join("models.rs"),
            "pub struct Row {}",
        )
        .unwrap();
    }
    assert!(discover::generated(dir.path(), None)
        .unwrap_err()
        .contains("Multiple"));
    assert!(discover::generated(dir.path(), Some(&"src/db/models.rs".into())).is_ok());
}

#[test]
fn removed_semantic_mode_and_unknown_selection_are_rejected() {
    assert!(super::command()
        .try_get_matches_from(["model", "audit", "--symbols", "semantic"])
        .is_err());
    let args = super::command()
        .try_get_matches_from(["model", "audit", "--select", "missing"])
        .unwrap();
    assert!(super::select::apply(Vec::new(), args.subcommand_matches("audit").unwrap()).is_err());
}
