#![allow(unused_qualifications)]

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashSet;
use syn::{__private::TokenStream2, parse_str, Ident};

const FOREX: &str = "Forex";
const CRYPTO: &str = "Crypto";
const STOCK: &str = "Stock";
const INDEX: &str = "Index";
const SEC_FUTURE: &str = "SecFuture";
const SEC_OPTION: &str = "SecOption";
const COMMODITY: &str = "Commodity";

const CONTRACTS: [&str; 7] = [
    FOREX, CRYPTO, STOCK, INDEX, SEC_FUTURE, SEC_OPTION, COMMODITY,
];

#[inline]
fn panic_msg(s_name: &str) -> ! {
    panic!("Invalid Security name {s_name}")
}

#[proc_macro_derive(Security)]
pub fn security_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_security(&ast)
}

fn impl_try_from_other_contracts(name: &Ident) -> TokenStream2 {
    let mut idents = CONTRACTS
        .into_iter()
        .map(|c| parse_str(c).unwrap())
        .collect::<HashSet<Ident>>();
    idents.remove(name);

    let mut out = Vec::new();
    for ident in idents {
        let error_message = format!("Expected {} found {}", name, format_ident!("{}", ident));
        out.push(quote! {
            impl TryFrom<#ident> for #name {
                type Error = UnexpectedSecurityType;

                fn try_from(_: #ident) -> Result<Self, Self::Error> {
                    Err(UnexpectedSecurityType(#error_message))
                }
            }
        });
    }

    quote! { #( #out )* }
}

fn impl_into_contract(name: &Ident) -> TokenStream2 {
    quote! {
        impl From<#name> for Contract {
            fn from(value: #name) -> Self {
                Self::#name(value)
            }
        }
    }
}

fn impl_security(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let s_name = name.to_string();
    let s_name = s_name.as_str();

    let contract_id = match s_name {
        crate::FOREX
        | crate::CRYPTO
        | crate::STOCK
        | crate::INDEX
        | crate::SEC_FUTURE
        | crate::COMMODITY => quote! { self.contract_id },
        crate::SEC_OPTION => quote! {
            match self {
                SecOption::Call(inner) | SecOption::Put(inner) => inner.contract_id
            }
        },
        _ => panic_msg(s_name),
    };

    let symbol = match s_name {
        crate::FOREX
        | crate::CRYPTO
        | crate::STOCK
        | crate::INDEX
        | crate::SEC_FUTURE
        | crate::COMMODITY => quote! { self.symbol.as_str() },
        crate::SEC_OPTION => quote! {
            match self {
                SecOption::Call(inner) | SecOption::Put(inner) => inner.symbol.as_str()
            }
        },
        _ => panic_msg(s_name),
    };

    let security_type = match s_name {
        FOREX => "CASH",
        CRYPTO => "CRYPTO",
        STOCK => "STK",
        INDEX => "IND",
        SEC_FUTURE => "FUT",
        SEC_OPTION => "OPT",
        COMMODITY => "CMDTY",
        _ => panic_msg(s_name),
    };

    let expiration_date = match s_name {
        crate::FOREX | crate::CRYPTO | crate::STOCK | crate::INDEX | crate::COMMODITY => {
            quote! { None }
        }
        SEC_FUTURE => quote! { Some(self.expiration_date) },
        SEC_OPTION => quote! {
            match self {
                SecOption::Call(inner) | SecOption::Put(inner) => Some(inner.expiration_date)
            }
        },
        _ => panic_msg(s_name),
    };

    let strike = match s_name {
        crate::FOREX
        | crate::CRYPTO
        | crate::STOCK
        | crate::INDEX
        | crate::SEC_FUTURE
        | crate::COMMODITY => quote! { None },
        SEC_OPTION => quote! {
            match self {
                SecOption::Call(inner) | SecOption::Put(inner) => Some(inner.strike)
            }
        },
        _ => panic_msg(s_name),
    };

    let right = match s_name {
        crate::FOREX
        | crate::CRYPTO
        | crate::STOCK
        | crate::INDEX
        | crate::SEC_FUTURE
        | crate::COMMODITY => quote! { None },
        SEC_OPTION => quote! {
            match self {
                SecOption::Call(_) => Some("C"),
                SecOption::Put(_) => Some("P"),
            }
        },
        _ => panic_msg(s_name),
    };

    let multiplier = match s_name {
        crate::FOREX | crate::CRYPTO | crate::STOCK | crate::INDEX | crate::COMMODITY => {
            quote! { None }
        }
        SEC_FUTURE => quote! { Some(self.multiplier) },
        SEC_OPTION => quote! {
            match self {
                SecOption::Call(inner) | SecOption::Put(inner) => Some(inner.multiplier)
            }
        },
        _ => panic_msg(s_name),
    };

    let exchange = match s_name {
        crate::FOREX | crate::STOCK | crate::INDEX | crate::SEC_FUTURE | crate::COMMODITY => {
            quote! { self.exchange }
        }
        crate::CRYPTO => quote! { Routing::Primary(Primary::PaxosCryptoExchange) },
        crate::SEC_OPTION => quote! {
            match self {
                SecOption::Call(inner) | SecOption::Put(inner) => inner.exchange
            }
        },
        _ => panic_msg(s_name),
    };

    let primary_exchange = match s_name {
        crate::FOREX
        | crate::CRYPTO
        | crate::INDEX
        | crate::SEC_FUTURE
        | crate::SEC_OPTION
        | crate::COMMODITY => quote! { None },
        crate::STOCK => quote! { Some(self.primary_exchange) },
        _ => panic_msg(s_name),
    };

    let currency = match s_name {
        crate::FOREX
        | crate::CRYPTO
        | crate::STOCK
        | crate::INDEX
        | crate::SEC_FUTURE
        | crate::COMMODITY => quote! { self.currency },
        SEC_OPTION => quote! {
            match self {
                SecOption::Call(inner) | SecOption::Put(inner) => inner.currency
            }
        },
        _ => panic_msg(s_name),
    };

    let local_symbol = match s_name {
        crate::FOREX
        | crate::CRYPTO
        | crate::STOCK
        | crate::INDEX
        | crate::SEC_FUTURE
        | crate::COMMODITY => quote! { self.local_symbol.as_str() },
        SEC_OPTION => quote! {
            match self {
                SecOption::Call(inner) | SecOption::Put(inner) => inner.local_symbol.as_str()
            }
        },
        _ => panic_msg(s_name),
    };

    let trading_class = match s_name {
        crate::FOREX | crate::CRYPTO | crate::STOCK | crate::SEC_FUTURE | crate::COMMODITY => {
            quote! { Some(self.trading_class.as_str()) }
        }
        crate::INDEX => quote! { None },
        SEC_OPTION => quote! {
            match self {
                SecOption::Call(inner) | SecOption::Put(inner) => Some(inner.trading_class.as_str())
            }
        },
        _ => panic_msg(s_name),
    };

    let try_from_impl = impl_try_from_other_contracts(name);

    let into_contract_impl = impl_into_contract(name);

    let gen = quote! {
        impl crate::contract::indicators::Valid for #name {}

        impl ToString for #name {
            fn to_string(&self) -> String {
                make_body!(
                    self.get_contract_id().0,
                    self.get_symbol(),
                    self.get_security_type(),
                    self.get_expiration_date().map(|d| d.format("%Y%m%d").to_string()).unwrap_or_default(),
                    self.get_strike().map(|s| s.to_string()).unwrap_or_default(),
                    self.get_right().unwrap_or_default(),
                    self.get_multiplier().map(|m| m.to_string()).unwrap_or_default(),
                    self.get_exchange(),
                    self.get_primary_exchange().map(|p| p.to_string()).unwrap_or_default(),
                    self.get_currency(),
                    self.get_local_symbol();
                    self.get_trading_class().unwrap_or("")
                )
            }
        }

        impl serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
                (
                    self.get_contract_id(),
                    self.get_symbol(),
                    self.get_security_type(),
                    self.get_expiration_date().map(|d| d.format("%Y%m%d").to_string()),
                    self.get_strike(),
                    self.get_right(),
                    self.get_multiplier(),
                    self.get_exchange(),
                    self.get_primary_exchange(),
                    self.get_currency(),
                    self.get_local_symbol(),
                    self.get_trading_class()
                ).serialize(serializer)
            }
        }

        impl Security for #name {
            #[inline]
            fn get_contract_id(&self) -> ContractId {
                #contract_id
            }

            #[inline]
            fn get_symbol(&self) -> &str {
                #symbol
            }

            #[inline]
            fn get_security_type(&self) -> &'static str {
                #security_type
            }

            #[inline]
            fn get_expiration_date(&self) -> Option<NaiveDate> {
                #expiration_date
            }

            #[inline]
            fn get_strike(&self) -> Option<f64> {
                #strike
            }

            #[inline]
            fn get_right(&self) -> Option<&'static str> {
                #right
            }

            #[inline]
            fn get_multiplier(&self) -> Option<u32> {
                #multiplier
            }

            #[inline]
            fn get_exchange(&self) -> Routing {
                #exchange
            }

            #[inline]
            fn get_primary_exchange(&self) -> Option<Primary> {
                #primary_exchange
            }

            #[inline]
            fn get_currency(&self) -> Currency {
                #currency
            }

            #[inline]
            fn get_local_symbol(&self) -> &str {
                #local_symbol
            }

            #[inline]
            fn get_trading_class(&self) -> Option<&str> {
                #trading_class
            }
        }

        #try_from_impl

        #into_contract_impl
    };
    gen.into()
}
