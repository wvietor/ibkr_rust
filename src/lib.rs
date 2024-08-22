//! A Rust port of the Interactive Brokers (IBKR) TWS API. Its goal is to be as expressive,
//! performant, and as safe as possible.

#![warn(missing_docs)]
#![allow(
    clippy::implicit_return,
    clippy::missing_docs_in_private_items,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::question_mark_used,
    clippy::separated_literal_suffix,
    clippy::single_char_lifetime_names
)]

pub(crate) use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};
pub(crate) use tracing::{
    debug, debug_span, error, error_span, event, info, info_span, instrument, span, trace,
    trace_span, warn, warn_span, Level,
};
pub(crate) use tracing_test::traced_test;
pub(crate) use {tracing, tracing_test};

/// Contains types related to account information.
pub mod account;
/// Contains the all-important [`client::Client`] struct and its methods, which facilitate
/// communication with the IBKR. Also contains a [`client::Builder`] struct to manage the
/// creation of new connections.
pub mod client;
mod comm;
mod constants;
/// Contains the definitions of all [`contract::Security`] implementors, which represent tradable
/// contracts.
///
/// Each variety of financial instrument is represented as its own unique struct or
/// enum. They all implement the [`contract::Security`] trait, which means they are a valid IBKR
/// contract and that they have at least one valid order type.
pub mod contract;
/// Contains the definition of a [`currency::Currency`] enum, which represents the possible trading
/// currencies available in the API.
pub mod currency;
#[allow(
    unused_variables,
    clippy::print_stdout,
    clippy::use_debug,
    clippy::too_many_lines,
    clippy::unnecessary_wraps,
    clippy::unused_async
)]
mod decode;
/// Contains types related to security exchanges and trading venues available in the API.
pub mod exchange;
/// Contains types related to executions, which are produced after a trade is made.
pub mod execution;
/// Contains an implementation of the [FIGI  alphanumeric identifier](https://www.openfigi.com/about/figi#!) for use in contract specification.
pub mod figi;
/// Contains modules that each relate to different market data requests. In particular, each module
/// defines: 1) General types used in a given market data query and 2) Optionally, a private
/// indicator trait that defines whether a given [`contract::Security`] allows for the data request
/// and 3) Any types associated with implementors of the indicator types.
pub mod market_data;
mod message;
/// Contains types and traits related to orders.
pub mod order;
/// Contains the types that are parsed from API callbacks. They are used in the [`wrapper::LocalWrapper`] and
/// [`wrapper::Wrapper`] callback functions.
pub mod payload;
/// Convenience module containing commonly-used types, functions, and modules.
pub mod prelude;
mod reader;
/// Contains modules, types, and functions related to live data subscriptions, namely those
/// that are created in [`client::Client::req_market_data`].
///
/// IBKR confusingly calls these callbacks "ticks" even though they are entirely separate from
/// tick-by-tick data. Many of these "ticks" (read data types) are returned by default with any
/// [`client::Client::req_market_data`] request; others are received only if they are
/// specified in the `additional_data` field.
///
/// IBKR groups these ticks into several distinct types. Some of these groups are sensible; others
/// are far too broad. Therefore, our version of the API groups these "ticks" differently. Inside
///this module, each of our groups gets its own submodule and corresponds one-to-one with a
/// [`wrapper::LocalWrapper`] or [`wrapper::Wrapper`] method.
pub mod tick;
/// Contains a [`chrono::TimeZone`] enum to represent all the possible timezones available in the API.
pub mod timezone;

pub mod limits;
pub mod scanner_parameters;
pub mod scanner_subscription;

/// Contains the definition of the [`wrapper::LocalWrapper`] and [`wrapper::Wrapper`] traits. Implementing these traits for a
/// type allows users to customize callback behavior.
pub mod wrapper;

#[macro_export]
/// Match across typed variant values
macro_rules! match_poly {
    ($self: expr; $($($pat: pat_param)|* => $meth_call: expr),* $(,)?) => {
        match $self {
            $($($pat => $meth_call),*),*
        }
    };
    ($self: expr; $($($pat: pat_param)|* => $meth_call: block),* $(,)?) => {
        match $self {
            $($($pat => $meth_call),*),*
        }
    };
}
