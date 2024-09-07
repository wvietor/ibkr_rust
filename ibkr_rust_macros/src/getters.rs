use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_quote, ItemStruct, Type};

fn impl_method(
    struct_name: &Ident,
    meth_name: &Ident,
    return_type: &Type,
    vis: &syn::Visibility,
    can_move: bool,
) -> TokenStream {
    let d = format!(
        "Get the {}'s {}.\n\n # Returns\n The {}",
        struct_name, meth_name, meth_name
    );
    let doc: syn::Attribute = parse_quote!(#[doc = #d]);

    let body = if can_move {
        quote! {
            #vis fn #meth_name(&self) -> #return_type {
                self.#meth_name
            }
        }
    } else {
        quote! {
            #vis fn #meth_name(&self) -> &#return_type {
                &self.#meth_name
            }
        }
    };

    quote! {
        #[must_use]
        #[inline]
        #doc
        #body
    }
}

pub fn impl_make_getters(ast: &mut ItemStruct) -> TokenStream {
    let mut out_stream = TokenStream::new();

    let ItemStruct {
        ident: ref s_name,
        ref vis,
        ref fields,
        ..
    } = ast;
    let meths = fields
        .iter()
        .filter_map(|f| {
            if let Some(ref meth_name) = f.ident {
                let r_type_str = f.ty.to_token_stream().to_string();
                let (r_type, can_move) = match r_type_str.as_str() {
                    "String" => (parse_quote! { str }, false),
                    s if s.starts_with("Vec < ") => (f.ty.clone(), false),
                    _ => (f.ty.clone(), true),
                };

                Some(impl_method(s_name, meth_name, &r_type, vis, can_move))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let getter_impl = quote! {
        impl #s_name {
            #( #meths )*
        }
    };
    ast.to_tokens(&mut out_stream);
    getter_impl.to_tokens(&mut out_stream);

    out_stream
}
