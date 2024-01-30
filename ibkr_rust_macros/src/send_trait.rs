use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Generics, ItemTrait, parenthesized, parse_macro_input, Token, TypeParamBound, TraitItem, TraitItemFn, ReturnType, TypeImplTrait, Type, parse_quote, Signature};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

#[allow(dead_code)]
#[derive(Clone)]
struct Attr {
    name: Ident,
    generics: Generics,
    paren: syn::token::Paren,
    make_traits: Punctuated<TypeParamBound, syn::token::Plus>,
    colon: Option<Token![:]>,
    super_traits: Punctuated<TypeParamBound, syn::token::Plus>,
}

impl Parse for Attr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let make_trait;
        Ok(Attr {
            name: input.parse()?,
            generics: input.parse()?,
            paren: parenthesized!(make_trait in input),
            make_traits: make_trait.parse_terminated(syn::TypeParamBound::parse, syn::token::Plus)?,
            colon: input.parse()?,
            super_traits: input.parse_terminated(syn::TypeParamBound::parse, syn::token::Plus)?,
        })
    }
}

#[allow(clippy::module_name_repetitions)]
pub fn impl_make_send(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream  {
    let attrs = parse_macro_input!(attr as Attr);
    let item = parse_macro_input!(item as ItemTrait);

    let variant = impl_make_variant(&attrs, &item);
    let new_item = impl_remove_async(item);

    quote! {
        #new_item

        #variant
    }.into()
}

fn impl_make_variant(attrs: &Attr, item: &ItemTrait) -> TokenStream {
    let make_traits= attrs.make_traits.clone().into_iter().collect::<Vec<TypeParamBound>>();
    let var = ItemTrait {
        ident: attrs.name.clone(),
        generics: attrs.generics.clone(),
        colon_token: attrs.colon,
        supertraits: attrs.super_traits.clone(),
        items: item.items.iter().map(|t| insert_trait_bounds(t, &make_traits)).collect(),
        ..item.clone()
    };

    quote! { #var }
}

fn insert_trait_bounds(item: &TraitItem, make_traits: &Vec<TypeParamBound>) -> TraitItem {
    let TraitItem::Fn(func @ TraitItemFn { sig, .. }) = item else {
        return item.clone();
    };

    let output_type = if sig.asyncness.is_some() {
        let fut_output = match sig.output {
            ReturnType::Default => quote! { () },
            ReturnType::Type(_, ref t) => quote! { #t },
        };

        Type::ImplTrait(TypeImplTrait {
            impl_token: parse_quote!{ impl },
            bounds: std::iter::once(TypeParamBound::Trait(parse_quote! { core::future::Future<Output=#fut_output> })).chain(make_traits.iter().cloned()).collect(),
        })
    } else {
        match sig.output {
            ReturnType::Type(_, ref t) => {
                match *t.to_owned() {
                    Type::ImplTrait(TypeImplTrait { bounds, ..}) => {
                        Type::ImplTrait(TypeImplTrait {
                            impl_token: parse_quote! { impl },
                            bounds: bounds.into_iter().chain(make_traits.iter().cloned()).collect(),
                        })
                    }
                    _ => { return item.clone(); }
                }
            },
            ReturnType::Default => { return item.clone(); },
        }
    };

    TraitItem::Fn(TraitItemFn {
        sig: Signature {
            asyncness: None,
            output: ReturnType::Type(Default::default(), Box::new(output_type)),
            ..sig.clone()
        },
        ..func.clone()
    })
}

fn remove_async_add_impl(item: &TraitItem) -> TraitItem {
    let TraitItem::Fn(func @ TraitItemFn { sig, .. }) = item else {
        return item.clone();
    };

    let output_type = if sig.asyncness.is_some() {
        let fut_output = match sig.output {
            ReturnType::Default => quote! { () },
            ReturnType::Type(_, ref t) => quote! { #t },
        };

        Type::ImplTrait(TypeImplTrait {
            impl_token: parse_quote!{ impl },
            bounds: std::iter::once(TypeParamBound::Trait(parse_quote! { core::future::Future<Output=#fut_output> })).collect(),
        })
    } else {
        return item.clone();
    };

    TraitItem::Fn(TraitItemFn {
        sig: Signature {
            asyncness: None,
            output: ReturnType::Type(Default::default(), Box::new(output_type)),
            ..sig.clone()
        },
        ..func.clone()
    })
}

fn impl_remove_async(item: ItemTrait) -> TokenStream {
    let new = ItemTrait {
        items: item.items.iter().map(|t| remove_async_add_impl(t)).collect(),
        ..item.clone()
    };

    quote! { #new }
}