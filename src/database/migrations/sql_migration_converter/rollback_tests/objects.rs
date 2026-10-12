use super::super::*;

fn down(sql: &str) -> String {
    generate_down_sql(&parse_sql_operations(sql))
}

#[test]
fn guarded_do_block_column_is_reversed() {
    let sql = "DO $g$ BEGIN IF to_regclass('video_templates') IS NOT NULL THEN ALTER TABLE video_templates ADD COLUMN IF NOT EXISTS owner_principal TEXT; CREATE INDEX IF NOT EXISTS idx_vt_owner ON video_templates(owner_principal); END IF; END $g$;";
    let d = down(sql);
    assert!(
        d.contains("ALTER TABLE IF EXISTS video_templates DROP COLUMN IF EXISTS owner_principal;"),
        "{d}"
    );
    assert!(d.contains("DROP INDEX IF EXISTS idx_vt_owner;"), "{d}");
}

#[test]
fn dollar_quoted_function_is_one_statement_with_cleanup() {
    let sql = "CREATE OR REPLACE FUNCTION f() RETURNS TRIGGER AS $$ BEGIN IF 1=1 THEN RETURN NEW; END IF; RETURN NEW; END; $$ LANGUAGE plpgsql; CREATE TRIGGER t AFTER UPDATE ON s FOR EACH ROW EXECUTE FUNCTION f(); CREATE OR REPLACE VIEW v AS SELECT 1;";
    let d = down(sql);
    assert!(d.contains("DROP FUNCTION IF EXISTS f();"), "{d}");
    assert!(d.contains("DROP TRIGGER IF EXISTS t ON s;"), "{d}");
    assert!(d.contains("DROP VIEW IF EXISTS v;"), "{d}");
    assert!(!d.contains("RETURN NEW"), "{d}");
}
