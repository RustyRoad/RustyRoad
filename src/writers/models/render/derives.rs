use std::io::{self, ErrorKind};
use syn::punctuated::Punctuated;
use syn::{parse_quote, ItemStruct, Path, Token};

pub(super) fn add_required_derives(model: &mut ItemStruct) -> io::Result<()> {
    let mut derives = Vec::<Path>::new();
    for attribute in &model.attrs {
        if attribute.path().is_ident("derive") {
            derives.extend(
                attribute
                    .parse_args_with(Punctuated::<Path, Token![,]>::parse_terminated)
                    .map_err(|error| io::Error::new(ErrorKind::InvalidData, error))?,
            );
        }
    }
    let required: [Path; 3] = [
        parse_quote!(serde::Serialize),
        parse_quote!(serde::Deserialize),
        parse_quote!(Clone),
    ];
    for path in required {
        let name = &path.segments.last().unwrap().ident;
        if !derives
            .iter()
            .any(|existing| existing.segments.last().map(|segment| &segment.ident) == Some(name))
        {
            model.attrs.push(parse_quote!(#[derive(#path)]));
        }
    }
    Ok(())
}
