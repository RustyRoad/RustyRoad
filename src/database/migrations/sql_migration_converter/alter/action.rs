use regex::Regex;

pub(in crate::database::migrations::sql_migration_converter) const IDENT: &str =
    r#"(?:"[^"]+"|`[^`]+`|\[[^\]]+\]|\w+)"#;

pub(in crate::database::migrations::sql_migration_converter) fn qualified() -> String {
    format!(r"{IDENT}(?:\s*\.\s*{IDENT})*")
}

pub(in crate::database::migrations::sql_migration_converter) fn clean(name: &str) -> String {
    name.split('.').map(str::trim).collect::<Vec<_>>().join(".")
}

pub(super) enum Action {
    Add(String),
    Drop(String),
    Constraint(String),
    Raw(String),
}

pub(super) fn classify(action: &str) -> Action {
    let id = format!("({IDENT})");
    let capture = |pattern: String| Regex::new(&pattern).unwrap();
    if let Some(c) = capture(format!(r"(?i)^ADD\s+CONSTRAINT\s+{id}")).captures(action) {
        return Action::Constraint(c[1].to_string());
    }
    if Regex::new(r"(?i)^ADD\s+(PRIMARY\s+KEY|FOREIGN\s+KEY|UNIQUE|CHECK|EXCLUDE)\b")
        .unwrap()
        .is_match(action)
        || Regex::new(r"(?i)^DROP\s+(CONSTRAINT|DEFAULT|NOT)\b")
            .unwrap()
            .is_match(action)
    {
        return Action::Raw(action.to_string());
    }
    let add = capture(format!(
        r"(?i)^ADD\s+(?:COLUMN\s+)?(?:IF\s+NOT\s+EXISTS\s+)?{id}"
    ));
    if let Some(c) = add.captures(action) {
        return Action::Add(clean(&c[1]));
    }
    let drop = capture(format!(
        r"(?i)^DROP\s+(?:COLUMN\s+)?(?:IF\s+EXISTS\s+)?{id}"
    ));
    drop.captures(action).map_or_else(
        || Action::Raw(action.to_string()),
        |c| Action::Drop(clean(&c[1])),
    )
}
