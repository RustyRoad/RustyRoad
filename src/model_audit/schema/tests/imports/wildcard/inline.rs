//! Local value-only helper modules cannot introduce shadowing type bindings.
use super::super::audits;

#[test]
fn inline_value_helpers_allow_prelude_types_and_known_codecs() {
    let result = audits(
        r#"
        mod helper { pub fn value() {} }
        mod other { pub const VALUE: i32 = 1; }
        use self::{helper::*, other::*};
        struct Row { value: Option<String>, count: i32, id: uuid::Uuid }
        "#,
    );
    let row = result.last().unwrap();
    assert!(row.blockers.is_empty(), "{:?}", row.blockers);
    let column = |name| {
        row.desired_columns
            .iter()
            .find(|column| column.name == name)
            .unwrap()
    };
    assert_eq!(column("value").sql_type, "text");
    assert!(column("value").nullable);
    assert_eq!(column("count").sql_type, "integer");
    assert_eq!(column("id").sql_type, "uuid");
}

#[test]
fn inline_ambiguous_or_conditional_helpers_remain_unknown() {
    for helper in [
        "pub type String = external::String;",
        "pub use external::*;",
        "#[expand] pub fn value() {}",
        "mod nested {}",
    ] {
        let result = audits(&format!(
            "mod helper {{ {helper} }} use self::helper::*; struct Row {{ value: String }}"
        ));
        assert!(result.last().unwrap().proposal.is_none(), "{helper}");
    }
}

#[test]
fn importing_scope_attributes_cannot_discharge_inline_globs() {
    for attribute in ["#[expand]", "#[cfg(feature = \"x\")]"] {
        let result = audits(&format!(
            "{attribute} mod inner {{
                mod helper {{ pub fn value() {{}} }}
                use self::helper::*; struct Row {{ value: String }}
            }}"
        ));
        assert!(result.last().unwrap().proposal.is_none(), "{attribute}");
    }
}
