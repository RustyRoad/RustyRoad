//! Inert lint levels must not make source-proven value helpers ambiguous.
use super::storage;

#[test]
fn documentation_and_lint_levels_preserve_helper_codecs() {
    for attribute in [
        "doc = \"helper\"",
        "allow(dead_code)",
        "warn(unused)",
        "deny(unused_imports)",
        "forbid(unsafe_code)",
        "expect(dead_code)",
    ] {
        let root = format!("#![{attribute}] mod rows; #[{attribute}] mod helper;");
        let helper = format!("#![{attribute}] #[{attribute}] pub fn helper() {{}}");
        let source = format!("#![{attribute}] #[{attribute}] use crate::helper::*;");
        assert_eq!(
            storage(
                &[("src/lib.rs", &root), ("src/helper.rs", &helper)],
                &source,
                "Option<String>"
            )
            .as_deref(),
            Some("text"),
            "{attribute}"
        );
    }
}

#[test]
fn tool_or_conditional_lint_attributes_remain_unknown() {
    for attribute in [
        "tool::allow(dead_code)",
        "cfg_attr(feature = \"x\", allow(dead_code))",
        "expand",
        "cfg(feature = \"x\")",
    ] {
        let helper = format!("#[{attribute}] pub fn helper() {{}}");
        assert_eq!(
            storage(
                &[
                    ("src/lib.rs", "mod rows; mod helper;"),
                    ("src/helper.rs", &helper)
                ],
                "use crate::helper::*;",
                "String"
            ),
            None,
            "{attribute}"
        );
    }
}
