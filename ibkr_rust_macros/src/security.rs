use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::quote;
#[allow(clippy::enum_glob_use)]
use SecType::*;
use syn::{Ident, parse_str};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum SecType {
    Forex,
    Crypto,
    Stock,
    Index,
    SecFuture,
    SecOption,
    Commodity,
}

impl SecType {
    #[inline]
    const fn as_str(self) -> &'static str {
        match self {
            Forex => "Forex",
            Crypto => "Crypto",
            Stock => "Stock",
            Index => "Index",
            SecFuture => "SecFuture",
            SecOption => "SecOption",
            Commodity => "Commodity",
        }
    }
}

impl From<&str> for SecType {
    fn from(s: &str) -> Self {
        match s {
            "Forex" => Forex,
            "Crypto" => Crypto,
            "Stock" => Stock,
            "Index" => Index,
            "SecFuture" => SecFuture,
            "SecOption" => SecOption,
            "Commodity" => Commodity,
            _ => panic!("Invalid Security name {s}."),
        }
    }
}

impl From<&SecType> for &'static str {
    fn from(value: &SecType) -> Self {
        value.as_str()
    }
}

impl From<&Ident> for SecType {
    fn from(value: &Ident) -> Self {
        let s = value.to_string();
        s.as_str().into()
    }
}

const CONTRACTS: [SecType; 7] = [Forex, Crypto, Stock, Index, SecFuture, SecOption, Commodity];

fn impl_try_from_other_contracts(name: &Ident) -> TokenStream {
    let idents = CONTRACTS
        .into_iter()
        .filter_map(|c| {
            if c == name.into() {
                None
            } else {
                parse_str(c.as_str()).unwrap()
            }
        })
        .collect::<HashSet<Ident>>()
        .into_iter();

    quote! {
        #(impl TryFrom<#idents> for #name {
            type Error = UnexpectedSecurityType;

            fn try_from(_: #idents) -> Result<Self, Self::Error> {
                Err(UnexpectedSecurityType {
                    expected: ContractType::#name,
                    found: ContractType::#idents,
                })
            }
        })*
    }
}

fn impl_into_contract(name: &Ident) -> TokenStream {
    let idents = CONTRACTS
        .into_iter()
        .filter_map(|c| {
            if c == name.into() {
                None
            } else {
                parse_str(c.as_str()).unwrap()
            }
        })
        .collect::<HashSet<Ident>>()
        .into_iter();

    quote! {
        impl From<#name> for Contract {
            fn from(value: #name) -> Self {
                Self::#name(value)
            }
        }

        impl TryFrom<Contract> for #name {
            type Error = UnexpectedSecurityType;

            fn try_from(value: Contract) -> Result<Self, Self::Error> {
                match value {
                    Contract::#name(t) => Ok(t),
                    #(Contract::#idents(_) => Err(
                        UnexpectedSecurityType {
                            expected: ContractType::#name,
                            found: ContractType::#idents
                        }
                    )),*
                }
            }
        }
    }
}

#[allow(clippy::module_name_repetitions, clippy::too_many_lines)]
pub fn impl_security(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let s_name: SecType = name.into();

    let contract_id = match s_name {
        Forex | Crypto | Stock | Index | SecFuture | Commodity => quote! { self.contract_id },
        SecOption => quote! {
            match self {
                SecOption::Call(inner) | SecOption::Put(inner) => inner.contract_id
            }
        },
    };
    let symbol = match s_name {
        Forex | Crypto | Stock | Index | SecFuture | Commodity => quote! { self.symbol.as_str() },
        SecOption => quote! {
            match self {
                SecOption::Call(inner) | SecOption::Put(inner) => inner.symbol.as_str()
            }
        },
    };
    let security_type = match s_name {
        Forex => "CASH",
        Crypto => "Crypto",
        Stock => "STK",
        Index => "IND",
        SecFuture => "FUT",
        SecOption => "OPT",
        Commodity => "CMDTY",
    };
    let expiration_date = match s_name {
        Forex | Crypto | Stock | Index | Commodity => {
            quote! { None::<NaiveDate> }
        }
        SecFuture => quote! { Some(self.expiration_date) },
        SecOption => quote! {
            match self {
                SecOption::Call(inner) | SecOption::Put(inner) => Some(inner.expiration_date)
            }
        },
    };
    let strike = match s_name {
        Forex | Crypto | Stock | Index | SecFuture | Commodity => quote! { None::<f64> },
        SecOption => quote! {
            match self {
                SecOption::Call(inner) | SecOption::Put(inner) => Some(inner.strike)
            }
        },
    };
    let right = match s_name {
        Forex | Crypto | Stock | Index | SecFuture | Commodity => quote! { None::<&str> },
        SecOption => quote! {
            match self {
                SecOption::Call(_) => Some("C"),
                SecOption::Put(_) => Some("P"),
            }
        },
    };
    let multiplier = match s_name {
        Forex | Crypto | Stock | Index | Commodity => {
            quote! { None::<u32> }
        }
        SecFuture => quote! { Some(self.multiplier) },
        SecOption => quote! {
            match self {
                SecOption::Call(inner) | SecOption::Put(inner) => Some(inner.multiplier)
            }
        },
    };
    let exchange = match s_name {
        Forex | Stock | Index | SecFuture | Commodity => {
            quote! { self.exchange }
        }
        Crypto => quote! { Routing::Primary(Primary::PaxosCryptoExchange) },
        SecOption => quote! {
            match self {
                SecOption::Call(inner) | SecOption::Put(inner) => inner.exchange
            }
        },
    };
    let primary_exchange = match s_name {
        Forex | Crypto | Index | SecFuture | SecOption | Commodity => quote! { None::<Primary> },
        Stock => quote! { Some(self.primary_exchange) },
    };
    let currency = match s_name {
        Forex | Crypto | Stock | Index | SecFuture | Commodity => quote! { self.currency },
        SecOption => quote! {
            match self {
                SecOption::Call(inner) | SecOption::Put(inner) => inner.currency
            }
        },
    };
    let local_symbol = match s_name {
        Forex | Crypto | Stock | Index | SecFuture | Commodity => {
            quote! { self.local_symbol.as_str() }
        }
        SecOption => quote! {
            match self {
                SecOption::Call(inner) | SecOption::Put(inner) => inner.local_symbol.as_str()
            }
        },
    };
    let min_tick = match s_name {
        Forex | Crypto | Stock | Index | SecFuture | Commodity => quote! { self.min_tick },
        SecOption => quote! {
            match self {
                SecOption::Call(inner) | SecOption::Put(inner) => inner.min_tick
            }
        },
    };
    let trading_class = match s_name {
        Forex | Crypto | Stock | SecFuture | Commodity => {
            quote! { Some(self.trading_class.as_str()) }
        }
        Index => quote! { None::<&str> },
        SecOption => quote! {
            match self {
                SecOption::Call(inner) | SecOption::Put(inner) => Some(inner.trading_class.as_str())
            }
        },
    };
    let long_name = match s_name {
        Forex | Crypto | Stock | Index | SecFuture | Commodity => {
            quote! { self.long_name.as_str() }
        }
        SecOption => quote! {
            match self {
                SecOption::Call(inner) | SecOption::Put(inner) => inner.long_name.as_str()
            }
        },
    };
    let order_types = match s_name {
        Forex | Crypto | Stock | Index | SecFuture | Commodity => quote! { &self.order_types },
        SecOption => quote! {
            match self {
                SecOption::Call(inner) | SecOption::Put(inner) => &inner.order_types
            }
        },
    };
    let valid_exchanges = match s_name {
        Forex | Crypto | Stock | Index | SecFuture | Commodity => quote! { &self.valid_exchanges },
        SecOption => quote! {
            match self {
                SecOption::Call(inner) | SecOption::Put(inner) => &inner.valid_exchanges
            }
        },
    };

    let try_from_impl = impl_try_from_other_contracts(name);
    let into_contract_impl = impl_into_contract(name);

    let gen_tokens = quote! {
        impl crate::contract::indicators::Valid for #name {
            fn as_out_msg(&self) -> crate::contract::indicators::SecurityOutMsg<'_> {
                crate::contract::indicators::SecurityOutMsg {
                    contract_id: #contract_id,
                    symbol: #symbol,
                    security_type: #security_type,
                    expiration_date: #expiration_date,
                    strike: #strike,
                    right: #right,
                    multiplier: #multiplier,
                    exchange: #exchange,
                    primary_exchange: #primary_exchange,
                    currency: #currency,
                    local_symbol: #local_symbol,
                    trading_class: #trading_class,
                }
            }
        }

        impl serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
                let mut state = serializer.serialize_struct("Contract", 14)?;
                state.serialize_field("contract_id", &#contract_id)?;
                state.serialize_field("security_type", &#security_type)?;
                state.serialize_field("symbol", &#symbol)?;
                state.serialize_field("long_name", &#long_name)?;
                state.serialize_field("min_tick", &#min_tick)?;
                state.serialize_field("exchange", &#exchange)?;
                state.serialize_field("primary_exchange", &#primary_exchange)?;
                state.serialize_field("currency", &#currency)?;
                state.serialize_field("local_symbol", &#local_symbol)?;
                state.serialize_field("trading_class", &#trading_class)?;
                state.serialize_field("expiration_date", &#expiration_date.map(|d| d.format("%Y%m%d").to_string()))?;
                state.serialize_field("strike", &#strike)?;
                state.serialize_field("option_class", &#right)?;
                state.serialize_field("multiplier", &#multiplier)?;
                state.end()
            }
        }

        impl Security for #name {
            #[inline]
            fn contract_id(&self) -> ContractId {
                #contract_id
            }
            #[inline]
            fn min_tick(&self) -> f64 {
                #min_tick
            }
            #[inline]
            fn symbol(&self) -> &str {
                #symbol
            }
            #[inline]
            fn currency(&self) -> Currency {
                #currency
            }
            #[inline]
            fn local_symbol(&self) -> &str {
                #local_symbol
            }
            #[inline]
            fn long_name(&self) -> &str {
                #long_name
            }
            #[inline]
            fn order_types(&self) -> &Vec<String> {
                #order_types
            }
            #[inline]
            fn valid_exchanges(&self) -> &Vec<Routing> {
                #valid_exchanges
            }
            #[inline]
            fn contract_type(&self) -> ContractType {
                ContractType::#name
            }
        }

        #try_from_impl

        #into_contract_impl
    };
    gen_tokens
}
