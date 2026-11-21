//! Discharge wildcard uncertainty only for source-proven value-only modules.
pub(super) mod exports;
mod target;
use super::{binding::Binding, collect, source};
use std::collections::BTreeMap;
use syn::{Item, PathArguments};

/// `use self::helpers::*` cannot shadow types if helpers export only values.
pub(super) fn bindings(
    items: &[Item],
    source: Option<&source::Context>,
    scope: usize,
    plain: bool,
) -> BTreeMap<String, Option<Binding>> {
    let mut bindings = collect::bindings(items);
    if !plain {
        return bindings;
    }
    let safe = items.iter().all(|item| {
        let Item::Use(import) = item else { return true };
        let Some(paths) = collect::glob_paths(import) else {
            return false;
        };
        if paths.is_empty() {
            return true;
        }
        if !exports::plain(&import.attrs) {
            return false;
        }
        paths.iter().all(|path| {
            if path.leading_colon.is_some()
                || path
                    .segments
                    .iter()
                    .any(|part| !matches!(part.arguments, PathArguments::None))
            {
                return false;
            }
            let target = match source {
                Some(source) => source.glob_items(path, scope),
                None => target::inline(items, path),
            };
            target.is_some_and(|items| exports::values_only(&items))
        })
    });
    if safe {
        bindings.remove("*");
    }
    bindings
}
