use syn::{parse_quote, punctuated::Punctuated, Attribute, Meta, Token};

/// Copy only attributes that determine whether the declaration exists.
pub(super) fn attributes(source: &[Attribute]) -> syn::Result<Vec<Attribute>> {
    let mut guards = Vec::new();
    for attribute in source {
        if let Some(meta) = existence_guard(&attribute.meta)? {
            guards.push(parse_quote!(#[#meta]));
        }
    }
    Ok(guards)
}

fn existence_guard(meta: &Meta) -> syn::Result<Option<Meta>> {
    if meta.path().is_ident("cfg") {
        return Ok(Some(meta.clone()));
    }
    if !meta.path().is_ident("cfg_attr") {
        return Ok(None);
    }
    let Meta::List(list) = meta else {
        return Err(syn::Error::new_spanned(meta, "expected cfg_attr arguments"));
    };
    let arguments = list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
    if arguments.len() < 2 {
        return Err(syn::Error::new_spanned(
            meta,
            "expected predicate and attribute",
        ));
    }
    let mut arguments = arguments.iter();
    let predicate = arguments.next().expect("checked argument count");
    let mut guards = Vec::new();
    for argument in arguments {
        if let Some(guard) = existence_guard(argument)? {
            guards.push(guard);
        }
    }
    if guards.is_empty() {
        Ok(None)
    } else {
        Ok(Some(parse_quote!(cfg_attr(#predicate, #(#guards),*))))
    }
}
