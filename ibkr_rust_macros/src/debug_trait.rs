use syn::{FnArg, ItemTrait, Pat, TraitItem};
use proc_macro2::TokenStream;
use quote::quote;

#[allow(clippy::module_name_repetitions)]
pub fn impl_debug_trait(ast: &mut ItemTrait) {
    for trait_item in &mut ast.items {
        let (signature, default_func) = match trait_item {
            TraitItem::Fn(trait_func) => match trait_func.default {
                Some(ref mut default_func) => (&trait_func.sig, default_func),
                None => continue
            },
            _ => continue
        };

        let args = signature.inputs.iter().filter_map(|arg| match arg {
            FnArg::Typed(typed_arg) => Some(&*typed_arg.pat),
            FnArg::Receiver(_) => None
        }).filter_map(|arg_name| match arg_name {
            Pat::Ident(ident) => {
                let id = &ident.ident;
                Some(quote! { &#id, })
            },
            _ => None
        }).collect::<Vec<_>>();

        let prefix = format!("[Wrapper::{}]", signature.ident);
        let args = TokenStream::from_iter(args);

        default_func.stmts.push(
            syn::Stmt::Expr(
                syn::Expr::Async(
                    syn::ExprAsync {
                        attrs: vec![],
                        async_token: std::default::Default::default(),
                        capture: Some(syn::parse(quote! { move }.into()).unwrap()),
                        block: syn::Block {
                            brace_token: std::default::Default::default(),
                            stmts: vec![
                                syn::parse(quote! { eprint!("{}", #prefix); }.into()).unwrap(),
                                syn::parse(quote! { dbg!((#args)); }.into()).unwrap(),
                                // syn::parse(quote! { return ().into(); }.into()).unwrap()
                            ]
                        },
                    }
                ),
                None
            )
        );

        // for stmt in vec![
        //     syn::parse(quote! { eprint!("{}", #prefix); }.into()).unwrap(),
        //     syn::parse(quote! { dbg!((#args)); }.into()).unwrap(),
        // ] {
        //     default_func.stmts.push(stmt);
        // }
    }
}