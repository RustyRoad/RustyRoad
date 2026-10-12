use super::super::*;

#[test]
fn reconversion_preserves_existing_cleanup() {
    let existing = "DROP TRIGGER IF EXISTS t ON s;\nDROP FUNCTION IF EXISTS keep_me();\nALTER TABLE leads DROP COLUMN IF EXISTS IF;";
    let sql = "CREATE TRIGGER t AFTER UPDATE ON s FOR EACH ROW EXECUTE FUNCTION f();";
    let generated = generate_down_sql(&parse_sql_operations(sql));
    let merged = preserve::merge_existing_down(generated, existing);
    assert!(
        merged.contains("DROP FUNCTION IF EXISTS keep_me();"),
        "{merged}"
    );
    assert_eq!(
        merged.matches("DROP TRIGGER IF EXISTS t ON s;").count(),
        1,
        "{merged}"
    );
    assert!(!merged.contains("EXISTS IF;"), "{merged}");
}
