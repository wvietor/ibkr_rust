use chrono::serde::ts_seconds;
use chrono::Utc;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

use crate::contract::{Contract, ContractType, ExchangeProxy};
use crate::exchange::Primary;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
/// A filter for requesting executions that meet only these criteria.
pub struct Filter {
    /// Filter by client id.
    pub client_id: i64,
    /// Filter by account number.
    pub account_number: String,
    /// Filter by contract symbol.
    pub symbol: String,
    /// Filter by contract type.
    pub contract_type: ContractType,
    /// Filter by exchange.
    pub exchange: Primary,
    /// Filter by order side.
    pub side: OrderSide,
}

// Add dummy time field that is not used
impl Serialize for Filter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Filter", 7)?;
        s.serialize_field("client_id", &self.client_id)?;
        s.serialize_field("account_number", &self.account_number)?;
        s.serialize_field("time", &"")?;
        s.serialize_field("symbol", &self.symbol)?;
        s.serialize_field("contract_type", &self.contract_type)?;
        s.serialize_field("exchange", &self.exchange)?;
        s.serialize_field("side", &self.side)?;
        s.end()
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// The possible sides for an order
pub enum OrderSide {
    #[serde(rename = "BUY")]
    /// A buy order
    Buy,
    #[serde(rename = "SELL")]
    /// A sell order
    Sell,
}

#[derive(Debug, Default, Clone, thiserror::Error)]
#[error("Invalid value encountered when attempting to parse an order side. No such order side: {0}. Valid order sides \"BOT\" or \"SLD\".")]
/// An error returned when parsing an [`OrderSide`] fails.
pub struct ParseOrderSideError(String);

impl std::str::FromStr for OrderSide {
    type Err = ParseOrderSideError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BOT" => Ok(Self::Buy),
            "SLD" => Ok(Self::Sell),
            other => Err(ParseOrderSideError(other.to_owned())),
        }
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
/// Contains the core fields relating to an [`Execution`]. which occurs when a trade is made.
pub struct Exec {
    /// The contract on which the trade was made.
    pub contract: ExchangeProxy<Contract>,
    /// The ID of the order that produced the execution.
    pub order_id: i64,
    /// The execution ID.
    pub execution_id: String,
    /// The date and time at which the execution occurred.
    #[serde(with = "ts_seconds")]
    pub datetime: chrono::DateTime<Utc>,
    /// The account number for which the trade was made.
    pub account_number: String,
    /// The exchange on which the trade was made.
    pub exchange: Primary,
    /// The number of contracts traded.
    pub quantity: f64,
    /// The price at which the trade was made.
    pub price: f64,
    /// The permanent ID of the order that produced the execution.
    pub perm_id: i64,
    /// The client ID that placed the order.
    pub client_id: i64,
    /// Whether the execution was caused by an IBKR-initiated liquidation.
    pub liquidation: bool,
    /// The cumulative number of contracts traded for the underlying order after this execution.
    pub cumulative_quantity: f64,
    /// The average price at which contracts for the underlying order after this execution.
    pub average_price: f64,
    /// Whether the execution is pending a price revision.
    pub pending_price_revision: bool,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
#[serde(tag = "action")]
/// A confirmed trade.
pub enum Execution {
    /// Contracts were bought.
    Bought(Exec),
    /// Contracts were sold.
    Sold(Exec),
}

impl Execution {
    #[inline]
    #[must_use]
    /// Return a reference to the inner [`Exec`]
    pub fn as_exec(&self) -> &Exec {
        match self {
            Self::Bought(e) | Self::Sold(e) => e,
        }
    }
    #[inline]
    #[must_use]
    /// Convert the [`Execution`] into an [`Exec`] and an [`OrderSide`]
    pub fn into_exec_tuple(self) -> (Exec, OrderSide) {
        match self {
            Self::Bought(e) => (e, OrderSide::Buy),
            Self::Sold(e) => (e, OrderSide::Sell),
        }
    }

    #[inline]
    #[must_use]
    /// Construct a new [`Execution`] from an [`Exec`] and an [`OrderSide`]
    pub fn from_exec_tuple(exec: Exec, side: OrderSide) -> Self {
        match side {
            OrderSide::Buy => Self::Bought(exec),
            OrderSide::Sell => Self::Sold(exec),
        }
    }

    #[inline]
    #[must_use]
    /// Return `true` if a Buy execution
    pub fn is_buy(&self) -> bool {
        matches!(self, Execution::Bought(_))
    }

    #[inline]
    #[must_use]
    /// Return `true` if a Sell execution
    pub fn is_sell(&self) -> bool {
        matches!(self, Execution::Sold(_))
    }
}

impl From<(Exec, OrderSide)> for Execution {
    #[inline]
    fn from(value: (Exec, OrderSide)) -> Self {
        Self::from_exec_tuple(value.0, value.1)
    }
}

impl From<(OrderSide, Exec)> for Execution {
    #[inline]
    fn from(value: (OrderSide, Exec)) -> Self {
        Self::from((value.1, value.0))
    }
}
