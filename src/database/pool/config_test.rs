use super::config::build;
use super::toml_source::values;

#[test]
fn reads_string_and_integer_ports_from_the_database_section() {
    let file = r#"
[database]
database_name = "shop"
database_user = "app"
database_password = "p@ss/word"
database_host = "db.internal"
database_port = 6543
"#;

    let values = values(file, "rustyroad.toml").expect("config should parse");
    assert_eq!(values, ["db.internal", "6543", "app", "p@ss/word", "shop"]);

    let options = build(&values, "rustyroad.toml").expect("options should build");
    assert_eq!(options.get_host(), "db.internal");
    assert_eq!(options.get_port(), 6543);
    assert_eq!(options.get_username(), "app");
    assert_eq!(options.get_database(), Some("shop"));
}

#[test]
fn names_the_missing_key_and_rejects_a_bad_port() {
    let missing = values("[database]\ndatabase_name = \"shop\"\n", "cfg").unwrap_err();
    assert_eq!(missing, "cfg is missing [database].database_host");

    let parts = ["h", "not-a-port", "u", "p", "n"].map(String::from);
    let error = build(&parts, "cfg").unwrap_err();
    assert!(
        error.starts_with("cfg has an invalid database port 'not-a-port'"),
        "{error}"
    );
}

#[test]
fn rejects_a_file_without_a_database_section() {
    let error = values("[server]\nport = 80\n", "cfg").unwrap_err();
    assert_eq!(error, "cfg has no [database] section");
}
