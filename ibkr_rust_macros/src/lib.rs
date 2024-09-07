mod debug_trait;
mod getters;
mod security;
mod send_trait;
mod variant_value;

use proc_macro::TokenStream;
use quote::ToTokens;

#[proc_macro_derive(Security)]
pub fn security_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    security::impl_security(&ast).into()
}

#[proc_macro_attribute]
pub fn debug_trait(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = syn::parse(item).unwrap();

    debug_trait::impl_debug_trait(&mut ast);
    ast.into_token_stream().into()
}

#[proc_macro_attribute]
pub fn make_send(attr: TokenStream, item: TokenStream) -> TokenStream {
    send_trait::impl_make_send(attr, item.clone()).into()
}

#[proc_macro_attribute]
pub fn typed_variants(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = syn::parse(item).unwrap();

    variant_value::impl_typed_variants(&mut ast).into()
}

#[proc_macro_attribute]
pub fn make_getters(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = syn::parse(item).unwrap();

    getters::impl_make_getters(&mut ast).into()
}
