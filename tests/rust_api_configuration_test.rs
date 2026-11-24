//! Mocked-local writer contracts: API compatibility and validation before file writes.
use rust::{
    aggregates::GroupStats,
    group_metrics::{ConditionalMetric, GroupMetrics},
};
use rustyroad::database::introspection::{Column, Schema, Table};
use rustyroad::generators::{report::Outcome, rust};
use std::{collections::BTreeMap, fs, io, path::Path};

/// A small catalog supporting both grouped statistics and global metrics.
fn schema() -> Schema {
    Schema {
        tables: vec![Table {
            name: "events".into(),
            columns: [
                ("id", "integer"),
                ("campaign", "text"),
                ("created_at", "timestamp"),
            ]
            .into_iter()
            .map(|(name, sql_type)| Column {
                name: name.into(),
                sql_type: sql_type.into(),
                nullable: false,
                default: None,
                auto_increment: false,
                json_schema: None,
            })
            .collect(),
            primary_key: vec!["id".into()],
            ..Table::default()
        }],
        ..Schema::default()
    }
}

/// Exercise the published positional API with two independently configured families.
fn generate(
    out: &Path,
    reports: &BTreeMap<String, GroupStats>,
    metrics: &BTreeMap<String, GroupMetrics>,
) -> io::Result<Vec<Outcome>> {
    rust::write_api_with_scoped_generation(
        out,
        &schema(),
        false,
        &Default::default(),
        reports,
        &Default::default(),
        metrics,
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &[],
        "public",
        &[],
        true,
    )
}

/// A schema-backed count and time-range report used by the configured writer.
fn reports() -> BTreeMap<String, GroupStats> {
    BTreeMap::from([(
        "event_summary".into(),
        GroupStats {
            table: "events".into(),
            group_by: "campaign".into(),
            timestamp: "created_at".into(),
        },
    )])
}

/// A global count uses the separate conditional-metrics family.
fn metric() -> GroupMetrics {
    GroupMetrics {
        table: "events".into(),
        group_by: String::new(),
        filter_by: None,
        metrics: vec![ConditionalMetric::CountAll],
    }
}

#[test]
fn published_writer_entry_points_preserve_equivalent_output() {
    let root = tempfile::tempdir().unwrap();
    let basic = root.path().join("basic");
    let legacy = root.path().join("legacy");
    let scoped = root.path().join("scoped");
    let generation = root.path().join("generation");
    rust::write_api(&basic, &schema(), false).unwrap();
    rust::write_api_with_reports(
        &legacy,
        &schema(),
        false,
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &[],
    )
    .unwrap();
    rust::write_api_with_scoped_reports(
        &scoped,
        &schema(),
        false,
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &Default::default(),
        &[],
        "public",
        &[],
    )
    .unwrap();
    generate(&generation, &Default::default(), &Default::default()).unwrap();
    for file in [
        "models.rs",
        "repositories.rs",
        "procedures.rs",
        "api.rs",
        "mod.rs",
    ] {
        let expected = fs::read(basic.join(file)).unwrap();
        for directory in [&legacy, &scoped, &generation] {
            assert_eq!(fs::read(directory.join(file)).unwrap(), expected, "{file}");
        }
    }
}

#[test]
fn configured_families_render_together_without_overwriting_scaffolds() {
    let root = tempfile::tempdir().unwrap();
    let out = root.path().join("generated");
    let metrics = BTreeMap::from([("event_count".into(), metric())]);
    generate(&out, &reports(), &metrics).unwrap();
    let source = fs::read_to_string(out.join("repositories.rs")).unwrap();
    let functions: Vec<_> = syn::parse_file(&source)
        .unwrap()
        .items
        .into_iter()
        .filter_map(|item| match item {
            syn::Item::Fn(function) => Some(function.sig.ident.to_string()),
            _ => None,
        })
        .collect();
    assert!(functions.iter().any(|name| name == "event_summary"));
    assert!(functions.iter().any(|name| name == "event_count"));
    fs::write(out.join("api.rs"), "// user-owned composition\n").unwrap();
    generate(&out, &reports(), &metrics).unwrap();
    assert_eq!(
        fs::read_to_string(out.join("api.rs")).unwrap(),
        "// user-owned composition\n"
    );
    assert_eq!(
        fs::read_to_string(out.join("repositories.rs")).unwrap(),
        source
    );
}

#[test]
fn invalid_configuration_does_not_create_or_overwrite_artifacts() {
    let root = tempfile::tempdir().unwrap();
    let out = root.path().join("generated");
    let duplicate = BTreeMap::from([("event_summary".into(), metric())]);
    let error = generate(&out, &reports(), &duplicate).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
    assert!(error
        .to_string()
        .contains("Duplicate configured report name"));
    assert!(!out.exists());
    fs::create_dir(&out).unwrap();
    fs::write(out.join("repositories.rs"), "retained bytes").unwrap();
    let mut invalid = reports();
    invalid.get_mut("event_summary").unwrap().table = "absent".into();
    let error = generate(&out, &invalid, &Default::default()).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
    assert!(error.to_string().contains("table absent from pull scope"));
    assert_eq!(
        fs::read_to_string(out.join("repositories.rs")).unwrap(),
        "retained bytes"
    );
    assert_eq!(fs::read_dir(out).unwrap().count(), 1);
}
