use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, Attribute, Generics, ItemEnum, Token};

fn make_struct(
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    vis: syn::Visibility,
    fields: syn::Fields,
) -> syn::ItemStruct {
    let span = ident.span();
    let semi_token = match fields {
        syn::Fields::Named(_) => None,
        _ => Some(Token![;](span)),
    };
    syn::ItemStruct {
        attrs,
        vis,
        struct_token: Token![struct](span),
        ident,
        generics: Generics::default(),
        fields,
        semi_token,
    }
}

pub fn impl_typed_variants(ast: &mut ItemEnum) -> proc_macro2::TokenStream {
    let mut out_stream = TokenStream::new();

    for new_struct in ast.variants.iter().map(|var| {
        let mut attrs = ast
            .attrs
            .clone()
            .into_iter()
            .filter(|a| match a.meta {
                syn::Meta::NameValue(syn::MetaNameValue { ref path, .. }) => !path.is_ident("doc"),
                _ => true,
            })
            .collect::<Vec<Attribute>>();
        attrs.extend(var.attrs.clone());
        make_struct(
            var.ident.clone(),
            attrs,
            ast.vis.clone(),
            var.fields.clone(),
        )
    }) {
        new_struct.to_tokens(&mut out_stream);
    }

    for var in &mut ast.variants {
        let name = &var.ident;
        let mut new: syn::Variant = parse_quote! { #name(#name) };
        new.attrs.clone_from(&var.attrs);
        *var = new;
    }
    ast.to_tokens(&mut out_stream);

    out_stream
}
