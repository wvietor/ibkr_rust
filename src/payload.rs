use chrono::NaiveDateTime;

use crate::payload::CalculationResult::{NotComputed, NotYetComputed};
use std::num::ParseFloatError;
use std::str::FromStr;

// macro_rules! make_error {
//     ($( #[doc = $name_doc:expr] )? $name: ident: $msg: literal) => {
//         $( #[doc = $name_doc] )?
//         #[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
//         pub struct $name(pub String);
//
//         impl std::error::Error for $name {}
//
//         impl std::fmt::Display for $name {
//             fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
//                 write!(f, "{}: {}", $msg, self.0)
//             }
//         }
//     };
// }

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
/// The result of an option calculation.
pub enum CalculationResult {
    /// The computed value.
    Computed(f64),
    /// Indicates that the computation has not been computed yet but will be at some point.
    NotYetComputed,
    /// Indicates that the computation will not be computed.
    NotComputed,
}

impl FromStr for CalculationResult {
    type Err = ParseFloatError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "-1" => NotComputed,
            "-2" => NotYetComputed,
            s => Self::Computed(s.parse()?),
        })
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// The result of a [`crate::client::Client::req_market_data`] request, which contains an identifier that can be passed to
/// [`crate::client::Client::req_smart_components`] request to find which exchanges are included in the SMART aggregate exchange.
pub struct ExchangeId(String);

impl std::fmt::Display for ExchangeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
/// An error type returned when a given exchange ID cannot be parsed (likely due to invalid UTF-8)
pub struct ParseExchangeIdError;

impl std::error::Error for ParseExchangeIdError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl std::fmt::Display for ParseExchangeIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid exchange ID, likely due to a bad UTF-8 code")
    }
}

impl FromStr for ExchangeId {
    type Err = ParseExchangeIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}

/// Re-export of [`crate::market_data::live_data::Class`].
pub type MarketDataClass = crate::market_data::live_data::Class;

/// Contains types related to market depth updates from [`crate::client::Client::req_market_depth`]
pub mod market_depth {
    use crate::exchange::Primary;

    #[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
    /// Represents a single change to an existing order book
    pub enum Operation {
        /// Insert a given row
        Insert(CompleteEntry),
        /// Update a given row
        Update(CompleteEntry),
        /// Delete a given row
        Delete(CompleteEntry),
    }

    impl TryFrom<(i64, CompleteEntry)> for Operation {
        type Error = anyhow::Error;

        fn try_from(value: (i64, CompleteEntry)) -> Result<Self, Self::Error> {
            Ok(match value.0 {
                0 => Self::Insert(value.1),
                1 => Self::Update(value.1),
                2 => Self::Delete(value.1),
                _ => Err(anyhow::Error::msg(
                    "Invalid int encountered while parsing operation",
                ))?,
            })
        }
    }

    #[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
    /// A single entry in a limit order book
    pub enum Entry {
        /// A resting buy order
        Bid {
            /// The order book's row being updated
            position: u64,
            /// The order's price
            price: f64,
            /// The order's size
            size: f64,
        },
        /// A resting sell order
        Ask {
            /// The order book's row being updated
            position: u64,
            /// The order's price
            price: f64,
            /// The order's size
            size: f64,
        },
    }

    impl TryFrom<(u32, u64, f64, f64)> for Entry {
        type Error = anyhow::Error;

        fn try_from(value: (u32, u64, f64, f64)) -> Result<Self, Self::Error> {
            Ok(match value.0 {
                0 => Self::Ask {
                    position: value.1,
                    price: value.2,
                    size: value.3,
                },
                1 => Self::Bid {
                    position: value.1,
                    price: value.2,
                    size: value.3,
                },
                _ => Err(anyhow::Error::msg(
                    "Invalid int encountered while parsing side",
                ))?,
            })
        }
    }

    #[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
    /// A complete entyr in a limit order book that potentially containing additional information about the market-maker / exchange from where
    /// the quote was sourced.
    pub enum CompleteEntry {
        /// An entry that indicates additional information about the exchange from which the information has been aggregated
        SmartDepth {
            /// The exchange from which the entry is sourced.
            exchange: Primary,
            /// The entry itself.
            entry: Entry,
        },
        /// An entry that indicates additional information about the market maker that has posted a given entry.
        MarketMaker {
            /// A unique identifier which conveys information about the market maker posting the entry.
            market_maker: Mpid,
            /// The entry itself.
            entry: Entry,
        },
        /// An entry that contains no additional information about the participant or exchange.
        Ordinary(Entry),
    }

    /// A unique four-character ID that identifies an individual market maker
    pub type Mpid = [char; 4];
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
/// A single entry in a histogram.
pub struct HistogramEntry {
    /// The price (x-value).
    pub price: f64,
    /// The frequency of the price (size / y-value).
    pub size: f64,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
/// A single historical bar
pub struct HistoricalBarCore {
    /// The ending datetime for the bar.
    pub datetime: NaiveDateTime,
    /// The bar's open price.
    pub open: f64,
    /// The bar's high price.
    pub high: f64,
    /// The bar's low price.
    pub low: f64,
    ///The bar's close price.
    pub close: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
/// A single historical bar.
pub enum HistoricalBar {
    /// The ordinary bar data returned from non [`crate::market_data::historical_bar::data_types::Trades`] requests.
    Ordinary(HistoricalBarCore),
    /// The bar data returned from a [`crate::market_data::historical_bar::data_types::Trades`] request.
    Trades {
        /// The core bar with open, high, low, close, etc.
        bar: HistoricalBarCore,
        /// The bar's traded volume.
        volume: f64,
        /// The bar's Weighted Average Price.
        wap: f64,
        /// The number of trades during the bar's timespan.
        trade_count: u64,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
/// A historical or live tick.
pub enum Tick {
    /// A tick representing a midpoint price.
    Midpoint {
        /// The timestamp of the tick.
        datetime: NaiveDateTime,
        /// The midpoint price.
        price: f64,
    },
    /// A tick representing the current best bid / ask prices.
    BidAsk {
        /// The timestamp of the tick.
        datetime: NaiveDateTime,
        /// The bid price.
        bid_price: f64,
        /// The ask price.
        ask_price: f64,
        /// The bid size.
        bid_size: f64,
        /// The ask size.
        ask_size: f64,
    },
    /// A tick representing the last trade.
    Last {
        /// The timestamp of the tick.
        datetime: NaiveDateTime,
        /// The last traded price.
        price: f64,
        /// The last traded size.
        size: f64,
        /// The last traded exchange.
        exchange: crate::exchange::Primary,
    },
}