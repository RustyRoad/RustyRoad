use super::{scalar, Mapping};
use crate::database::introspection::{Column, Schema};
use crate::generators::rust::allocation::Names;

/// Resolve a non-null codec, checking explicit declarations before standard mappings.
/// # Examples
/// ```sh
/// rustyroad pull --language rust --out crates/db/src
/// ```
pub(super) fn map(column: &Column, schema: &Schema, names: &Names) -> Mapping {
    let declared = column.sql_type.trim();
    if let Some(codec) = names.codec(declared) {
        return Mapping::plain(&codec.rust);
    }

    // A user-defined enum arrives as its bare type name, so it is matched before the
    // scalar table, where it would fall through to text and lose its variants.
    if schema
        .enums
        .iter()
        .any(|item| item.name == declared.trim_matches('"'))
    {
        return Mapping::plain(names.enum_type(declared.trim_matches('"')));
    }

    let lowered = declared.to_lowercase();

    if let Some(element) = scalar::array_element(declared) {
        let mapped = names
            .codec(&element)
            .map(|codec| Mapping::plain(&codec.rust))
            .unwrap_or_else(|| scalar::map(&scalar::strip_modifier(&element.to_lowercase())));
        return mapped.collected();
    }

    scalar::map(&scalar::strip_modifier(&lowered))
}
