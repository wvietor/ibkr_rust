use std::fmt::Formatter;
use std::str::FromStr;

use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::contract::{Contract, ExchangeProxy};

#[derive(Debug, Clone, Error)]
#[error("Invalid value encountered when attempting to parse a payload value.")]
/// An error returned when parsing any value in the [`crate::payload`] module fails.
pub enum ParsePayloadError {
    /// Invalid locate
    #[error("Invalid value encountered when attempting to parse locate. Expected \"locate\", found: {0}")]
    Locate(String),
    /// Invalid order status
    #[error("Invalid value encountered when attempting to parse order status. No such order status: {0}")]
    OrderStatus(String),
    #[error("Invalid int encountered while parsing entry side")]
    Entry,
    #[error("Invalid value encountered when attempting to parse MPID.")]
    Mpid,
    #[error("Invalid int encountered while parsing operation")]
    Operation,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
/// The result of a [`crate::client::Client::req_market_data`] request, which contains an identifier that can be passed to
/// [`crate::client::Client::req_smart_components`] request to find which exchanges are included in the SMART aggregate exchange.
pub struct ExchangeId(String);

impl std::fmt::Display for ExchangeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
    use serde::{de::Error, Deserialize, Serialize};

    use crate::exchange::Primary;
    use crate::payload::ParsePayloadError;

    #[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "operation")]
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
        type Error = ParsePayloadError;

        fn try_from(value: (i64, CompleteEntry)) -> Result<Self, Self::Error> {
            Ok(match value.0 {
                0 => Self::Insert(value.1),
                1 => Self::Update(value.1),
                2 => Self::Delete(value.1),
                _ => return Err(ParsePayloadError::Operation),
            })
        }
    }

    #[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "type")]
    /// A single entry in a limit order book
    pub enum Entry {
        /// A resting buy order
        Bid(Row),
        /// A resting sell order
        Ask(Row),
    }

    #[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Serialize, Deserialize)]
    /// A single row in a limit order book
    pub struct Row {
        /// The position of the row in the order book.
        pub position: u64,
        /// The order's price.
        pub price: f64,
        /// The order's size.
        pub size: f64,
    }

    impl TryFrom<(u32, u64, f64, f64)> for Entry {
        type Error = ParsePayloadError;

        fn try_from(value: (u32, u64, f64, f64)) -> Result<Self, Self::Error> {
            Ok(match value.0 {
                0 => Self::Ask(Row {
                    position: value.1,
                    price: value.2,
                    size: value.3,
                }),
                1 => Self::Bid(Row {
                    position: value.1,
                    price: value.2,
                    size: value.3,
                }),
                _ => Err(ParsePayloadError::Entry)?,
            })
        }
    }

    #[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "origin")]
    /// A complete entry in a limit order book that potentially containing additional information about the market-maker / exchange from where
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
            #[serde(
                serialize_with = "serialize_mpid",
                deserialize_with = "deserialize_mpid"
            )]
            market_maker: Mpid,
            /// The entry itself.
            entry: Entry,
        },
        /// An entry that contains no additional information about the participant or exchange.
        Ordinary(Entry),
    }

    /// A unique four-character ID that identifies an individual market maker
    pub type Mpid = [char; 4];

    fn serialize_mpid<S: serde::Serializer>(mpid: &Mpid, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(mpid.iter().collect::<String>().as_str())
    }

    fn deserialize_mpid<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Mpid, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.chars()
            .take(4)
            .collect::<Vec<char>>()
            .try_into()
            .map_err(|_| Error::invalid_value(serde::de::Unexpected::Str(&s), &"Valid UTF-8 Mpid"))
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
/// A single entry in a histogram.
pub struct HistogramEntry {
    /// The price (x-value).
    pub price: f64,
    /// The frequency of the price (size / y-value).
    pub size: f64,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
/// A single historical bar
pub struct BarCore {
    /// The ending datetime for the bar.
    #[serde(with = "ts_seconds")]
    pub datetime: DateTime<Utc>,
    /// The bar's open price.
    pub open: f64,
    /// The bar's high price.
    pub high: f64,
    /// The bar's low price.
    pub low: f64,
    ///The bar's close price.
    pub close: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(tag = "bar_type")]
/// A single bar.
pub enum Bar {
    /// The ordinary bar data returned from non [`crate::market_data::historical_bar::Trades`] requests.
    Ordinary(BarCore),
    /// The bar data returned from a [`crate::market_data::historical_bar::Trades`] request.
    Trades(Trade),
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
/// A trade bar with volume, WAP, and count data.
pub struct Trade {
    #[serde(flatten)]
    /// The core bar with open, high, low, close, etc.
    pub bar: BarCore,
    /// The bar's traded volume.
    pub volume: f64,
    /// The bar's Weighted Average Price.
    pub wap: f64,
    /// The number of trades during the bar's timespan.
    pub trade_count: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(tag = "tick")]
/// A historical or live tick.
pub enum TickData {
    /// A tick representing a midpoint price.
    Midpoint(Midpoint),
    /// A tick representing the current best bid / ask prices.
    BidAsk(BidAsk),
    /// A tick representing the last trade.
    Last(Last),
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
/// A tick representing the midpoint.
pub struct Midpoint {
    /// The timestamp of the tick.
    #[serde(with = "ts_seconds")]
    pub datetime: DateTime<Utc>,
    /// The midpoint price.
    pub price: f64,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
/// A tick representing a bid/ask.
pub struct BidAsk {
    /// The timestamp of the tick.
    #[serde(with = "ts_seconds")]
    pub datetime: DateTime<Utc>,
    /// The bid price.
    pub bid_price: f64,
    /// The ask price.
    pub ask_price: f64,
    /// The bid size.
    pub bid_size: f64,
    /// The ask size.
    pub ask_size: f64,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
/// A tick representing the last traded price.
pub struct Last {
    /// The timestamp of the tick.
    #[serde(with = "ts_seconds")]
    pub datetime: DateTime<Utc>,
    /// The last traded price.
    pub price: f64,
    /// The last traded size.
    pub size: f64,
    /// The last traded exchange.
    pub exchange: crate::exchange::Primary,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
/// A single position, comprising a single security and details about its current value, P&L, etc.
pub struct Position {
    /// The ID of the underlying contract.
    pub contract: ExchangeProxy<Contract>,
    /// The number of contracts owned.
    pub position: f64,
    /// The current market price of each contract.
    pub market_price: f64,
    /// The current market value of the entire position.
    pub market_value: f64,
    /// The average cost per contract for the entire position.
    pub average_cost: f64,
    /// The unrealized P&L of the position.
    pub unrealized_pnl: f64,
    /// The realized P&L of the position.
    pub realized_pnl: f64,
    /// The account number holding the position.
    pub account_number: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
/// A single position, comprising a single security and a few details about its cost, account, etc.
pub struct PositionSummary {
    /// The underlying contract
    pub contract: ExchangeProxy<Contract>,
    /// The number of contracts owned.
    pub position: f64,
    /// The average cost per contract for the entire position.
    pub average_cost: f64,
    /// The account number holding the position.
    pub account_number: String,
}

#[derive(Debug, Default, Clone, Copy, PartialOrd, PartialEq, Serialize, Deserialize)]
/// A simple struct representing a few types of P&L.
pub struct Pnl {
    /// The daily P&L for the account in real-time.
    pub daily: f64,
    /// Total unrealized P&L for the account.
    pub unrealized: f64,
    /// Total realized P&L for the account.
    pub realized: f64,
}

#[derive(Debug, Default, Clone, Copy, PartialOrd, PartialEq, Serialize, Deserialize)]
/// A simple struct representing single position P&L
pub struct PnlSingle {
    /// The daily P&L for the position in real-time.
    pub daily: f64,
    /// Unrealized P&L for the position.
    pub unrealized: f64,
    /// Realized P&L for the position.
    pub realized: f64,
    /// Current size of the position
    pub position_size: f64,
    /// The current market value of the position
    pub market_value: f64,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
#[serde(tag = "order_status")]
/// The possible statuses for a given order.
pub enum OrderStatus {
    /// Indicates order has not yet been sent to IB server, for instance if there is a delay in receiving the security definition. Uncommonly received.
    ApiPending(OrderStatusCore),
    /// Indicates that you have transmitted the order, but have not yet received confirmation that it has been accepted by the order destination. Most commonly because exchange is closed.
    PendingSubmit(OrderStatusCore),
    /// Indicates that you have sent a request to cancel the order but have not yet received cancel confirmation from the order destination. At this point, your order is not confirmed canceled. It is not guaranteed that the cancellation will be successful.
    PendingCancel(OrderStatusCore),
    /// Indicates that a simulated order type has been accepted by the IB system and that this order has yet to be elected. The order is held in the IB system until the election criteria are met. At that time the order is transmitted to the order destination as specified.
    PreSubmitted(OrderStatusCore),
    /// Indicates that your order has been accepted at the order destination and is working.
    Submitted(OrderStatusCore),
    /// After an order has been submitted and before it has been acknowledged, an API client client can request its cancellation, producing this state.
    ApiCancelled(OrderStatusCore),
    /// Indicates that the balance of your order has been confirmed canceled by the IB system. This could occur unexpectedly when IB or the destination has rejected your order.
    Cancelled(OrderStatusCore),
    /// Indicates that the order has been completely filled. Market orders executions will not always trigger a Filled status.
    Filled(OrderStatusCore),
    /// Indicates that the order was received by the system but is no longer active because it was rejected or canceled.
    Inactive(OrderStatusCore),
}

impl TryFrom<(&str, OrderStatusCore)> for OrderStatus {
    type Error = ParsePayloadError;

    fn try_from(value: (&str, OrderStatusCore)) -> Result<Self, Self::Error> {
        Ok(match value.0 {
            "ApiPending" => OrderStatus::ApiPending(value.1),
            "PendingSubmit" => OrderStatus::PendingSubmit(value.1),
            "PendingCancel" => OrderStatus::PendingCancel(value.1),
            "PreSubmitted" => OrderStatus::PreSubmitted(value.1),
            "Submitted" => OrderStatus::Submitted(value.1),
            "ApiCancelled" => OrderStatus::ApiCancelled(value.1),
            "Cancelled" => OrderStatus::Cancelled(value.1),
            "Filled" => OrderStatus::Filled(value.1),
            "Inactive" => OrderStatus::Inactive(value.1),
            s => return Err(ParsePayloadError::OrderStatus(s.to_owned())),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Serialize, Deserialize)]
/// The core fields of an Order's Status
pub struct OrderStatusCore {
    /// The order's ID.
    pub order_id: i64,
    /// The details of how many contracts have been filled.
    pub fill: Option<Fill>,
    /// The remnant positions.
    pub remaining: f64,
    /// The order’s permId used by the TWS to identify orders.
    pub permanent_id: i64,
    /// Parent’s id. Used for bracket and auto trailing stop orders.
    pub parent_id: Option<i64>,
    /// API client which submitted the order.
    pub client_id: i64,
    /// This field is used to identify an order held when TWS is trying to locate shares for a short sell.
    pub why_held: Option<Locate>,
    /// If an order has been capped, this indicates the current capped price.
    pub market_cap_price: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Serialize, Deserialize)]
/// Contains the details of an order's filled positions.
pub struct Fill {
    /// Number of filled positions.
    pub filled: f64,
    /// Average filling price.
    pub average_price: f64,
    /// Price at which the last positions were filled.
    pub last_price: f64,
}

#[derive(Debug, Default, Clone, Copy, PartialOrd, Eq, Ord, PartialEq, Serialize, Deserialize)]
/// Indicates whether an order is being held because IBKR is trying to locate shares for a short sale.
pub struct Locate;

impl FromStr for Locate {
    type Err = ParsePayloadError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "locate" => Ok(Locate),
            s => Err(ParsePayloadError::Locate(s.to_owned())),
        }
    }
}

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
/// An error that represents an invalid order status.
pub struct ParseOrderStatusError(pub String);

impl std::fmt::Display for ParseOrderStatusError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid order status message: {}", self.0)
    }
}

impl std::error::Error for ParseOrderStatusError {}

#[allow(non_snake_case, missing_docs)]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct OrderDetails {
    pub OcaGroup: Option<String>,
    pub Account: Option<String>,
    pub OpenClose: Option<String>,
    pub Origin: Option<String>,
    pub OrderRef: Option<String>,
    pub ClientId: Option<String>,
    pub PermId: Option<String>,
    pub OutsideRth: Option<String>,
    pub Hidden: Option<String>,
    pub DiscretionaryAmt: Option<String>,
    pub GoodAfterTime: Option<String>,
    pub SkipSharesAllocation: Option<String>,
    pub FAParams: Option<String>,
    pub ModelCode: Option<String>,
    pub GoodTillDate: Option<String>,
    pub Rule80A: Option<String>,
    pub PercentOffset: Option<String>,
    pub SettlingFirm: Option<String>,
    pub ShortSaleParams: Option<String>,
    pub AuctionStrategy: Option<String>,
    pub BoxOrderParams: Option<String>,
    pub PegToStkOrVolOrderParams: Option<String>,
    pub DisplaySize: Option<String>,
    pub BlockOrder: Option<String>,
    pub SweepToFill: Option<String>,
    pub AllOrNone: Option<String>,
    pub MinQty: Option<String>,
    pub OcaType: Option<String>,
    pub skipETradeOnly: Option<String>,
    pub skipFirmQuoteOnly: Option<String>,
    pub skipNbboPriceCap: Option<String>,
    pub ParentId: Option<String>,
    pub TriggerMethod: Option<String>,
    pub VolOrderParams: Option<String>,
    pub TrailParams: Option<String>,
    pub BasisPoints: Option<String>,
    pub ComboLegs: Option<String>,
    pub SmartComboRoutingParams: Option<String>,
    pub ScaleOrderParams: Option<String>,
    pub HedgeParams: Option<String>,
    pub OptOutSmartRouting: Option<String>,
    pub ClearingParams: Option<String>,
    pub NotHeld: Option<String>,
    pub DeltaNeutral: Option<String>,
    pub AlgoParams: Option<String>,
    pub Solicited: Option<String>,
    pub WhatIfInfoAndCommission: Option<String>,
    pub VolRandomizeFlags: Option<String>,
    pub PegToBenchParams: Option<String>,
    pub Conditions: Option<String>,
    pub AdjustedOrderParams: Option<String>,
    pub SoftDollarTier: Option<String>,
    pub CashQty: Option<String>,
    pub DontUseAutoPriceForHedge: Option<String>,
    pub IsOmsContainers: Option<String>,
    pub DiscretionaryUpToLimitPrice: Option<String>,
    pub UsePriceMgmtAlgo: Option<String>,
    pub Duration: Option<String>,
    pub PostToAts: Option<String>,
    pub AutoCancelParent: Option<String>,
    pub PegBestPegMidOrderAttributes: Option<String>,
}
