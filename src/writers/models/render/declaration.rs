use quote::quote;
use std::io::{self, ErrorKind};
use syn::{parse_quote, Fields, Ident, ItemStruct};

/// Owns a validated declaration; callers cannot bypass named-field validation.
pub(super) struct Declaration {
    model: ItemStruct,
}

impl Declaration {
    pub(super) fn parse(model_name: &str, source: &str) -> io::Result<Self> {
        let ident = syn::parse_str::<Ident>(model_name)
            .map_err(|error| io::Error::new(ErrorKind::InvalidInput, error))?;
        let mut model = syn::parse_str::<ItemStruct>(source)
            .map_err(|error| io::Error::new(ErrorKind::InvalidData, error))?;
        let Fields::Named(fields) = &mut model.fields else {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                "SQL-derived models must have named fields",
            ));
        };
        for field in &mut fields.named {
            field.vis = parse_quote!(pub);
        }
        model.ident = ident;
        model.vis = parse_quote!(pub);
        super::derives::add_required_derives(&mut model)?;
        Ok(Self { model })
    }

    pub(super) fn render(&self) -> io::Result<String> {
        let model = &self.model;
        let constructor = super::constructor::render(model)
            .map_err(|error| io::Error::new(ErrorKind::InvalidData, error))?;
        let output = quote!(#model #constructor).to_string();
        syn::parse_file(&output).map_err(|error| io::Error::new(ErrorKind::InvalidData, error))?;
        Ok(output)
    }
}
