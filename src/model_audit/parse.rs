//! Rust AST extraction. Comments and multiline field types are not regex input.

use super::types::{Field, Model};
use quote::ToTokens;
use std::path::Path;
use syn::{Attribute, Fields, Item};

/// Parse declarations, including inline modules; `parse(path, root, source)` is read-only.
pub(super) fn parse(path: &Path, root: &Path, source: &str) -> Result<Vec<Model>, String> {
    let file = syn::parse_file(source).map_err(|error| format!("{}: {error}", path.display()))?;
    let mut output = Vec::new();
    items(&file.items, path, root, source, &[], &mut output);
    Ok(output)
}

/// Compare token spelling without comments; e.g. spaced generic types normalize equally.
fn tokens(value: &impl ToTokens) -> String {
    value.to_token_stream().to_string()
}

/// Documentation is not a wire contract; serde, sqlx, cfg and derives are retained.
fn attributes(attrs: &[Attribute]) -> Vec<String> {
    attrs
        .iter()
        .filter(|a| !a.path().is_ident("doc"))
        .map(tokens)
        .collect()
}

/// Walk real AST modules; tuple structs and enums remain explicit review cases.
fn items(
    children: &[Item],
    path: &Path,
    root: &Path,
    source: &str,
    modules: &[String],
    out: &mut Vec<Model>,
) {
    for item in children {
        if let Item::Mod(module) = item {
            if let Some((_, children)) = &module.content {
                let mut nested = modules.to_vec();
                nested.push(module.ident.to_string());
                items(children, path, root, source, &nested, out);
            }
        }
        let (ident, attrs, generics, visibility, fields, kind, variants) = match item {
            Item::Struct(s) => (
                &s.ident,
                &s.attrs,
                &s.generics,
                &s.vis,
                Some(&s.fields),
                if matches!(s.fields, Fields::Named(_)) {
                    "struct"
                } else {
                    "tuple_or_unit"
                },
                Vec::new(),
            ),
            Item::Enum(e) => (
                &e.ident,
                &e.attrs,
                &e.generics,
                &e.vis,
                None,
                "enum",
                e.variants.iter().map(tokens).collect(),
            ),
            Item::Type(t) => (
                &t.ident,
                &t.attrs,
                &t.generics,
                &t.vis,
                None,
                "alias",
                vec![tokens(&t.ty)],
            ),
            _ => continue,
        };
        let pos = ident.span().start();
        let line = source.lines().nth(pos.line.saturating_sub(1)).unwrap_or("");
        let character = line
            .chars()
            .take(pos.column)
            .map(char::len_utf16)
            .sum::<usize>() as u32;
        let mut names = modules.to_vec();
        names.push(ident.to_string());
        out.push(Model {
            id: format!(
                "{}::{}",
                path.strip_prefix(root).unwrap_or(path).display(),
                names.join("::")
            ),
            name: ident.to_string(),
            path: path.to_path_buf(),
            line: pos.line.saturating_sub(1) as u32,
            character,
            kind: kind.into(),
            attributes: attributes(attrs),
            generics: format!("{} {}", tokens(generics), tokens(&generics.where_clause))
                .trim()
                .to_owned(),
            visibility: tokens(visibility),
            variants,
            fields: fields
                .into_iter()
                .flat_map(|f| f.iter())
                .enumerate()
                .map(|(index, f)| {
                    (
                        f.ident
                            .as_ref()
                            .map(ToString::to_string)
                            .unwrap_or_else(|| index.to_string()),
                        Field {
                            ty: tokens(&f.ty),
                            attributes: attributes(&f.attrs),
                            visibility: tokens(&f.vis),
                        },
                    )
                })
                .collect(),
        });
    }
}
