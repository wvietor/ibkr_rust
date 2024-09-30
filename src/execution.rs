use chrono::serde::ts_seconds;
use chrono::Utc;
use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;

use crate::contract::{Contract, ContractType, ExchangeProxy};
use crate::currency::Currency;
use crate::exchange::Primary;

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
/// A filter for requesting executions that meet only these criteria.
pub struct Filter {
    /// Filter by API client id that placed the order.
    pub client_id: i64,
    /// Filter by account number to which the order was allocated
    pub account_number: String,
    /// Filter by orders placed after this date and time
    pub datetime: Option<chrono::DateTime<chrono_tz::Tz>>,
    /// Filter by contract symbol.
    pub symbol: String,
    /// Filter by contract type.
    pub contract_type: Option<ContractType>,
    /// Filter by the exchange at which the execution was produced.
    pub exchange: Option<Primary>,
    /// Filter by order side.
    pub side: Option<OrderSide>,
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

#[derive(Debug, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
/// Details the commissions paid regarding a given [`Execution`]
pub struct CommissionReport {
    /// The ID of the [`Execution`] with which the report corresponds
    pub exec_id: String,
    /// The commission cost
    pub commission: f64,
    /// The reporting currency
    pub currency: Currency,
    /// The realized profit and loss
    pub realized_pnl: f64,
    /// The income return
    pub yld: f64,
    /// The redemption date for the yield
    pub yld_redemption_date: chrono::NaiveDate,
}
