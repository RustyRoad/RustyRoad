use super::super::*;

fn down(sql: &str) -> String {
    generate_down_sql(&parse_sql_operations(sql))
}

#[test]
fn schema_qualified_table_is_preserved() {
    let d = down(
        "ALTER TABLE public.leads ADD COLUMN IF NOT EXISTS a TEXT, ADD COLUMN IF NOT EXISTS b INT;",
    );
    assert!(
        d.contains("ALTER TABLE public.leads DROP COLUMN IF EXISTS a;"),
        "{d}"
    );
    assert!(
        d.contains("ALTER TABLE public.leads DROP COLUMN IF EXISTS b;"),
        "{d}"
    );
    assert!(!d.contains("ALTER TABLE public DROP"), "{d}");
}

#[test]
fn add_constraint_gets_a_drop_constraint() {
    let d = down("ALTER TABLE leads ADD CONSTRAINT leads_x_check CHECK (x > 0);");
    assert!(
        d.contains("DROP CONSTRAINT IF EXISTS leads_x_check;"),
        "{d}"
    );
    assert!(!d.contains("CONSTRAINT;"), "{d}");
}
