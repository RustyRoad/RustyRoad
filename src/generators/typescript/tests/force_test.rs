//! Overriding preservation with `--force`.

use super::support::schema;
use super::writer_support::{outcome_for, pull, scratch};

#[test]
fn force_overwrites_files_you_own() {
    let out = scratch("force");
    pull(&out, &schema(), false);

    let api = out.join("api.ts");
    std::fs::write(&api, "// mine\n").unwrap();

    let report = pull(&out, &schema(), true);

    assert!(!outcome_for(&report.outcomes, "api.ts").is_preserved());
    let restored = std::fs::read_to_string(&api).unwrap();
    assert!(restored.contains("export const router = {"));
    assert!(restored.contains("generated.users"));

    let _ = std::fs::remove_dir_all(&out);
}

#[test]
fn the_hey_api_config_is_also_preserved() {
    let out = scratch("config");
    pull(&out, &schema(), false);

    // A project will edit `output`; losing that every run would be worse than
    // leaving the config slightly stale.
    let config = out.join("openapi/openapi-ts.config.ts");
    std::fs::write(&config, "// my output path\n").unwrap();

    let report = pull(&out, &schema(), false);

    assert!(outcome_for(&report.outcomes, "openapi-ts.config.ts").is_preserved());
    assert_eq!(
        std::fs::read_to_string(&config).unwrap(),
        "// my output path\n"
    );
    // The document itself is derived and must still refresh.
    assert!(!outcome_for(&report.outcomes, "openapi.json").is_preserved());

    let _ = std::fs::remove_dir_all(&out);
}
