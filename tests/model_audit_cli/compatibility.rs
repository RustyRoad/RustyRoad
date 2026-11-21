//! Compatibility and output-safety checks for offline CLI auditing.
use super::fixtures::{fixture, run};

#[test]
fn legacy_syntax_is_compatible_but_semantic_options_are_rejected() {
    let dir = fixture();
    let normal = run(dir.path(), &[]);
    let legacy = run(dir.path(), &["--symbols", "syntax"]);
    assert!(normal.status.success());
    assert!(legacy.status.success());
    assert_eq!(normal.stdout, legacy.stdout);
    for args in [
        ["--symbols", "semantic"],
        ["--rust-analyzer", "/nonexistent/analyzer"],
        ["--timeout", "120"],
    ] {
        let result = run(dir.path(), &args);
        assert_eq!(result.status.code(), Some(2));
        assert!(result.stdout.is_empty());
        assert!(!result.stderr.is_empty());
    }
    assert!(!dir.path().join("Cargo.lock").exists());
}

#[test]
fn output_never_overwrites_and_interactive_requires_a_terminal() {
    let dir = fixture();
    let result = run(
        dir.path(),
        &["--symbols", "syntax", "--output", "report.json"],
    );
    assert!(result.status.success());
    let saved = std::fs::read(dir.path().join("report.json")).unwrap();
    let result = run(
        dir.path(),
        &["--symbols", "syntax", "--output", "report.json"],
    );
    assert_eq!(result.status.code(), Some(1));
    assert_eq!(
        std::fs::read(dir.path().join("report.json")).unwrap(),
        saved
    );
    let result = run(dir.path(), &["--symbols", "syntax", "--interactive"]);
    assert_eq!(result.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&result.stdout).contains("requires terminal"));
}
