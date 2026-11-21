//! Keep a declared crate's source tree alive throughout codec lookup.
use crate::model_audit::{
    parse,
    schema::rust_types::{self, Resolver},
};

pub(super) const CODEC: &str = r#"
    #[derive(sqlx::Type)] #[sqlx(type_name = "state")]
    pub enum State { Ready }
"#;

/// Resolve a field while its complete declared source tree still exists.
pub(super) fn storage(files: &[(&str, &str)], source: &str, ty: &str) -> Option<String> {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(
        directory.path().join("Cargo.toml"),
        "[package]\nname = 'fixture'\nversion = '0.1.0'\n",
    )
    .unwrap();
    for (name, content) in files {
        let path = directory.path().join(name);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, content).unwrap();
    }
    let path = directory.path().join("src/rows.rs");
    let content = format!("{source}\nstruct Row {{ state: {ty} }}");
    std::fs::write(&path, &content).unwrap();
    let model = parse::parse(&path, directory.path(), &content)
        .unwrap()
        .pop()
        .unwrap();
    rust_types::model_column(
        "state",
        &model.fields["state"],
        &Default::default(),
        &Resolver::for_model(&model),
    )
    .ok()
    .map(|column| column.sql_type)
}
