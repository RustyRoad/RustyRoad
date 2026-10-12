//! Writes a crate around generated models plus the `db_pool` scaffold, for a `cargo check`.
//!
//! Run `RUSTYROAD_E2E_OUT=<dir> cargo test --lib pool_crate -- --ignored`, then `cargo
//! check` in `<dir>`. The crate depends on this checkout by path, so it proves generated
//! models compile against `rustyroad::database::pool`.

use super::support::{enum_type, required, serial_key, with_enums};
use crate::database::introspection::Table;
use crate::generators::rust::{write, write_pool_module};
use std::{env, fs, path::PathBuf};

/// The crate root: declares both modules and calls into a generated model.
const MAIN: &str = "mod db_pool;\nmod models;\n\n#[tokio::main]\nasync fn main() {\n    \
                    db_pool::init_shared_pool().await.ok();\n    \
                    let _ = models::Product::all().await;\n}\n";

/// Dependencies the generated models name directly; `{rustyroad}` is this checkout.
const MANIFEST: &str = "[package]\nname = \"pool-e2e\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\
                        publish = false\n\n[workspace]\n\n[dependencies]\n\
                        rustyroad = { path = \"{rustyroad}\" }\n\
                        sqlx = { version = \"0.9.0\", features = [\"postgres\", \"runtime-tokio\", \"macros\"] }\n\
                        serde = { version = \"1\", features = [\"derive\"] }\n\
                        tokio = { version = \"1\", features = [\"full\"] }\n\
                        utoipa = \"6\"\nts-rs = \"12\"\n";

#[test]
#[ignore = "writes a crate for an end-to-end cargo check"]
fn write_pool_crate() {
    let Ok(root) = env::var("RUSTYROAD_E2E_OUT").map(PathBuf::from) else {
        return;
    };
    let src = root.join("src");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&src).unwrap();

    let table = Table {
        name: "products".to_string(),
        columns: vec![
            serial_key("id"),
            required("name", "text"),
            required("status", "product_status"),
        ],
        primary_key: vec!["id".to_string()],
        foreign_keys: Vec::new(),
        uniques: Vec::new(),
        indexes: Vec::new(),
        view: false,
    };
    let statuses = enum_type("product_status", &["active", "back_ordered"]);
    let schema = with_enums(vec![table], vec![statuses]);

    fs::write(src.join("main.rs"), MAIN).unwrap();
    write(&src.join("models"), &schema, false).expect("models should write");
    write_pool_module(&src.join("models"), false).expect("pool module should write");
    let manifest = MANIFEST.replace("{rustyroad}", env!("CARGO_MANIFEST_DIR"));
    fs::write(root.join("Cargo.toml"), manifest).unwrap();
}
