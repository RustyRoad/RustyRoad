//! Declared siblings, reexports, and nested modules retain explicit codecs.
use super::{storage, CODEC};

#[test]
fn declared_sibling_reexports_and_optional_aliases_supply_storage() {
    let files = [
        ("src/lib.rs", "mod rows; mod codec; pub use codec::State;"),
        ("src/codec.rs", CODEC),
    ];
    for (source, ty) in [
        ("use crate::codec::State;", "State"),
        ("use super::codec::State as Status;", "Option<Status>"),
        ("", "crate::State"),
        ("", "super::codec::State"),
    ] {
        assert_eq!(
            storage(&files, source, ty).as_deref(),
            Some("\"state\""),
            "{ty}"
        );
    }
}

#[test]
fn nested_file_and_inline_modules_supply_manual_contracts() {
    let files = [
        ("src/lib.rs", "mod rows; mod codec;"),
        (
            "src/codec/mod.rs",
            r#"
        pub mod inner {
            use sqlx::{Type as Codec, Postgres as Pg, postgres::PgTypeInfo as Info};
            pub struct Point;
            impl Codec<Pg> for Point {
                fn type_info() -> Info { Info::with_name("geometry") }
            }
        }
    "#,
        ),
    ];
    assert_eq!(
        storage(&files, "", "crate::codec::inner::Point").as_deref(),
        Some("\"geometry\"")
    );
}
