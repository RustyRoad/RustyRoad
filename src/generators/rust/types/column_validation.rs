//! Validate scoped contracts against the complete selected catalog before output.
use super::{overrides, ColumnOverride, Override};
use crate::database::introspection::Schema;
use std::{collections::BTreeMap, io};

/// Resolve exact scalar targets, rejecting missing identities and storage drift.
/// Nullability assertions inspect the catalog, not later cloned result wrappers.
/// # Examples
/// ```sh
/// rustyroad pull --schema public --language rust --out crates/db/src
/// ```
pub(in crate::generators::rust) fn validate(
    schema: &Schema,
    namespace: &str,
    entries: &[ColumnOverride],
) -> io::Result<BTreeMap<(String, String), Override>> {
    let mut resolved = BTreeMap::new();
    for entry in entries {
        let invalid = |reason: &str| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "Invalid column codec {}.{}.{}: {reason}",
                    entry.schema, entry.table, entry.column
                ),
            )
        };
        if entry.schema.is_empty() || namespace != entry.schema {
            return Err(invalid(
                "namespace differs from the explicit pull namespace",
            ));
        }
        let table = schema
            .table(&entry.table)
            .ok_or_else(|| invalid("table outside pull scope"))?;
        let column = table
            .column(&entry.column)
            .ok_or_else(|| invalid("unknown column"))?;
        let expected = overrides::normalize(&entry.sql_type);
        let observed = overrides::normalize(&column.sql_type);
        if expected.is_empty() || expected != observed {
            return Err(invalid(
                "catalog storage does not match sql_type, including modifiers",
            ));
        }
        if super::array_element(&observed).is_some()
            || observed.contains('[')
            || observed.ends_with("array")
        {
            return Err(invalid("scoped array codecs are not supported"));
        }
        if entry
            .nullable
            .is_some_and(|nullable| nullable != column.nullable)
        {
            return Err(invalid(
                "catalog nullability differs from the nullable assertion",
            ));
        }
        let codec = Override {
            rust: entry.rust.clone(),
            schema: entry.openapi_type.clone(),
        };
        overrides::validate(&BTreeMap::from([(entry.sql_type.clone(), codec.clone())]))
            .map_err(|_| invalid("invalid Rust codec path or OpenAPI type"))?;
        if resolved
            .insert((entry.table.clone(), entry.column.clone()), codec)
            .is_some()
        {
            return Err(invalid("duplicate scoped target"));
        }
    }
    Ok(resolved)
}
