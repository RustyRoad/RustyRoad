use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

pub(super) fn render(model: &ItemStruct) -> syn::Result<TokenStream> {
    let ident = &model.ident;
    let (impl_generics, type_generics, where_clause) = model.generics.split_for_impl();
    let guards = super::guards::attributes(&model.attrs)?;
    let mut parameters = Vec::new();
    let mut initializers = Vec::new();
    for field in &model.fields {
        let name = &field.ident;
        let ty = &field.ty;
        let guards = super::guards::attributes(&field.attrs)?;
        parameters.push(quote!(#(#guards)* #name: #ty));
        initializers.push(quote!(#(#guards)* #name));
    }
    Ok(quote! {
        #(#guards)*
        impl #impl_generics #ident #type_generics #where_clause {
            pub fn new(#(#parameters),*) -> Self {
                Self { #(#initializers),* }
            }
        }
    })
}
