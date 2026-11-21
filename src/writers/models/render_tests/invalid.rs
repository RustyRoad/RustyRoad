use super::render_base_model;
use std::io::ErrorKind;

#[test]
fn rejects_malformed_or_unsupported_declarations() {
    for source in [
        "",
        "struct broken {",
        "struct outer { struct inner {} }",
        "struct first {} struct second {}",
        "struct tuple(i64);",
        "struct unit;",
        "enum Choice { One }",
        "#[derive(Clone = true)] struct broken {}",
    ] {
        assert_eq!(
            render_base_model("User", source).unwrap_err().kind(),
            ErrorKind::InvalidData,
            "source: {source}"
        );
    }
}

#[test]
fn rejects_invalid_model_names() {
    for name in [
        "",
        "../../other",
        "Two Words",
        "struct",
        "User<T>",
        "User; struct Other",
    ] {
        assert_eq!(
            render_base_model(name, "struct users {}")
                .unwrap_err()
                .kind(),
            ErrorKind::InvalidInput,
            "name: {name}"
        );
    }
}
