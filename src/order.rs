use serde::{Serialize, Serializer};
use crate::{
    contract::{Commodity, Crypto, Forex, Index, SecFuture, SecOption, Security, Stock},
    make_body,
};

// ==============================================
// === Core Order Types (Market, Limit, etc.) ===
// ==============================================

const ORDER_NULL_STRING: &str = "\x00\x00\x000\x00\x001\x000\x000\x000\x000\x000\x000\x000\x00\x000\x00\x00\x00\x00\x00\x00\x000\x00\x00-1\x000\x00\x00\x000\x00\x00\x000\x000\x00\x000\x00\x00\x00\x00\x00\x000\x00\x00\x00\x00\x000\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x000\x00\x00\x000\x000\x00\x00\x000\x00\x000\x000\x000\x000\x00\x001.7976931348623157e+308\x001.7976931348623157e+308\x001.7976931348623157e+308\x001.7976931348623157e+308\x001.7976931348623157e+308\x000\x00\x00\x00\x001.7976931348623157e+308\x00\x00\x00\x00\x000\x000\x000\x00\x002147483647\x002147483647\x000\x00\x00\x00";

// === Type definitions ===

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// The time periods for which an order is active and can be executed against.
pub enum TimeInForce {
    #[default]
    /// Valid for the day only.
    Day,
    /// Good until canceled. The order will continue to work within the system and in the marketplace until it executes or is canceled. GTC orders will be automatically be cancelled under the following conditions:
    /// If a corporate action on a security results in a stock split (forward or reverse), exchange for shares, or distribution of shares. If you do not log into your IB account for 90 days.
    /// At the end of the calendar quarter following the current quarter. For example, an order placed during the third quarter of 2011 will be canceled at the end of the first quarter of 2012. If the last day is a non-trading day, the cancellation will occur at the close of the final trading day of that quarter. For example, if the last day of the quarter is Sunday, the orders will be cancelled on the preceding Friday.
    /// Orders that are modified will be assigned a new “Auto Expire” date consistent with the end of the calendar quarter following the current quarter.
    /// Orders submitted to IB that remain in force for more than one day will not be reduced for dividends. To allow adjustment to your order price on ex-dividend date, consider using a Good-Til-Date/Time (GTD) or Good-after-Time/Date (GAT) order type, or a combination of the two.
    Gtc,
    /// Immediate or Cancel. Any portion that is not filled as soon as it becomes available in the market is canceled.
    Ioc,
    // /// Good until Date. It will remain working within the system and in the marketplace until it executes or until the close of the market on the date specified
    // Gtd,
    // /// Use OPG to send a market-on-open (MOO) or limit-on-open (LOO) order.
    // Opg,
    /// If the entire Fill-or-Kill order does not execute as soon as it becomes available, the entire order is canceled.
    Fok,
    /// Day until canceled.
    Dtc,
}

impl ToString for TimeInForce {
    fn to_string(&self) -> String {
        match self {
            Self::Day => "DAY",
            Self::Gtc => "GTC",
            Self::Ioc => "IOC",
            // Self::Gtd => "GTD",
            // Self::Opg => "OPG",
            Self::Fok => "FOK",
            Self::Dtc => "DTC",
        }
        .to_owned()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
/// A generic order to buy or sell a security `S`: `Security` according to the parameters specified by the generic
/// parameter `E`: `Executable`.
pub enum Order<'o, S: Security, E: Executable<S>> {
    /// An order to Buy `S`: `Security`] according to the method described by `E`: `Executable`.
    Buy {
        /// The security to buy.
        security: &'o S,
        /// The execution method to use.
        execute_method: &'o E,
    },
    /// An order to Sell `S`: `Security` according to the method described by `E`: `Executable`.
    Sell {
        /// The security to sell.
        security: &'o S,
        /// The execution method to use.
        execute_method: &'o E,
    },
}

impl<S: Security, E: Executable<S>> Order<'_, S, E> {
    #[must_use]
    /// Return the order's `security`
    pub const fn get_security(&self) -> &S {
        match self {
            Self::Buy { security, .. } | Self::Sell { security, .. } => security,
        }
    }

    #[must_use]
    /// Return the order's `execute_method`
    pub const fn get_execute_method(&self) -> &E {
        match self {
            Self::Buy { execute_method, .. } | Self::Sell { execute_method, .. } => execute_method,
        }
    }
}

impl<S: Security, E: Executable<S>> ToString for Order<'_, S, E> {
    fn to_string(&self) -> String {
        let (action, execute_method) = match self {
            Self::Buy { execute_method, .. } => ("BUY", execute_method),
            Self::Sell { execute_method, .. } => ("SELL", execute_method),
        };
        make_body!(action, execute_method.to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
/// A market order: Buy or sell at the best available price for a given quantity. Sensitive to price fluctuations.
pub struct Market {
    /// The number of shares/units to execute.
    pub quantity: f64,
    /// The time for which the order will remain valid
    pub time_in_force: TimeInForce,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
/// A market order: Buy or sell at a price as good or better than the limit price. May not be filled.
pub struct Limit {
    /// The number of shares/units to buy.
    pub quantity: f64,
    /// The limit price, which sets the upper / lower bound on the price per unit.
    pub price: f64,
    /// The time for which the order will remain valid
    pub time_in_force: TimeInForce,
}

// === Type implementations ===

impl ToString for Market {
    fn to_string(&self) -> String {
        make_body!(
            self.quantity,
            "MKT",
            "",
            "",
            self.time_in_force;
            ORDER_NULL_STRING
        )
    }
}

impl ToString for Limit {
    fn to_string(&self) -> String {
        make_body!(
            self.quantity,
            "LMT",
            self.price,
            "",
            self.time_in_force;
            ORDER_NULL_STRING
        )
    }
}

// ==================================================
// === Order Trait Definition and Implementations ===
// ==================================================

/// Implemented by all valid order types for a given security. In particular,
/// if a type `O` implements [`Executable<S>`], then `O` is a valid order for `S`.
pub trait Executable<S: Security>: ToString + Send + Sync {
    /// Return the total number of contracts being bought/sold.
    fn get_quantity(&self) -> f64;

    /// Return the order's type
    fn get_order_type(&self) -> &'static str;

    #[inline]
    /// Return the order's limit price
    fn get_limit_price(&self) -> Option<f64> {
        None
    }

    #[inline]
    /// Return the order's auxiliary price, which is a generic price used for specifying parameters
    /// like the stop price in a stop-limit order.
    fn get_auxiliary_price(&self) -> Option<f64> {
        None
    }

    /// Return the order's time in force, which specifies how long the order will be active.
    fn get_time_in_force(&self) -> TimeInForce;

    #[inline]
    /// Return the One-Cancels-All group identifier.
    fn get_one_cancels_all_group(&self) -> Option<&str> {
        None
    }
    #[inline]
    /// Return the account to which the trade will be allocated.
    fn get_account(&self) -> Option<&str> {
        None
    }
    #[inline]
    /// Return the order's origin.
    fn get_origin(&self) -> Origin {
        Origin::default()
    }

    #[inline]
    /// Return the order reference
    ///
    /// Intended for institutional customers only, although all customers may use it to identify the
    /// API client that sent the order when multiple API clients are running.
    fn get_order_reference(&self) -> Option<&str> {
        None
    }

    #[inline]
    /// Return whether the order will be transmitted by TWS.
    ///
    /// If set to false, the order will be created at TWS but will not be sent.
    fn get_will_transmit(&self) -> bool {
        true
    }

    #[inline]
    /// Return the order ID of the parent order, used for bracket and auto trailing stop orders.
    fn get_parent_id(&self)  -> i64 {
        0
    }

    #[inline]
    /// Return whether the order is an ISE block order.
    ///
    /// If set to true, specifies that the order is an ISE Block order.
    fn get_is_block_order(&self) -> bool {
        false
    }

    #[inline]
    /// Return whether the order is a sweep-to-fill order.
    ///
    /// If set to true, specifies that the order is a Sweep-to-Fill order.
    fn get_is_sweep_to_fill(&self) -> bool {
        false
    }

    #[inline]
    /// Return the publicly disclosed order size, used when placing Iceberg orders.
    fn get_iceberg_order_size(&self) -> u64 {
        0
    }

    #[inline]
    /// Return the trigger method, which specifies how Simulated Stop, Stop-Limit and Trailing
    /// Stop orders are triggered.
    fn get_trigger_method(&self) -> TriggerMethod {
        TriggerMethod::default()
    }

    #[inline]
    /// Return whether the order can fill outside of regular trading hours.
    ///
    /// If set to true, allows orders to also trigger or fill outside of regular trading hours.
    fn get_can_fill_outside_regular_trading_hours(&self) -> bool {
        false
    }

    #[inline]
    /// Return whether the order will be visible when viewing the market depth.
    ///
    /// If set to true, the order will not be visible when viewing the market depth. This option
    /// only applies to orders routed to the NASDAQ exchange.
    fn get_is_hidden_on_nasdaq_market_depth(&self) -> bool {
        false
    }

    #[inline]
    /// Return the amount off the limit price allowed for discretionary orders.
    fn get_discretionary_amount(&self) -> f64 {
        0.0
    }

    #[inline]
    /// Return the date and time after which the order will be active.
    ///
    /// Format: yyyymmdd hh:mm:ss {optional Timezone}.
    fn get_good_after_time(&self) -> Option<&str> {
        None
    }

    #[inline]
    /// Return the date and time until the order will be active.
    ///
    /// You must enter GTD as the time in force to use this string. The trade's "Good Till Date,"
    /// format "yyyyMMdd HH:mm:ss (optional time zone)" or UTC "yyyyMMdd-HH:mm:ss".
    fn get_good_until_date(&self) -> Option<&str> {
        None
    }

    #[inline]
    /// Get the model code associated with a given order.
    ///
    /// Is used to place an order to a model. For example, "Technology" model can be used for tech
    /// stocks first created in TWS.
    fn get_model_code(&self) -> Option<&str> {
        None
    }

    // Next is oca_type
}


#[inline]
fn serialize_executable<E, Sec, Ser>(exec: &E, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
    where E: Executable<Sec>, Ser: Serializer {
        (
            exec.get_quantity(),
            exec.get_order_type(),
            exec.get_limit_price(),
            exec.get_auxiliary_price(),
            exec.get_time_in_force(),
            exec.get_one_cancels_all_group(),
            exec.get_account(),
            None::<()>,
            exec.get_origin(),
            exec.get_order_reference(),
            exec.get_will_transmit(),
            exec.get_parent_id(),
            exec.get_is_block_order(),
            exec.get_is_sweep_to_fill(),
            exec.get_iceberg_order_size(),
            exec.get_trigger_method(),
            exec.get_can_fill_outside_regular_trading_hours(),
            exec.get_is_hidden_on_nasdaq_market_depth(),
            None::<()>,
            exec.get_discretionary_amount(),
            exec.get_good_after_time(),
            exec.get_good_until_date(),
            [None::<()>; 3],
            exec.get_model_code(),
            0,
            None::<()>,
            -1,


        ).serialize(serializer)
}

#[derive(Debug, Default, Clone, Copy, Ord, PartialOrd, PartialEq, Hash, Eq, Serialize)]
pub enum TriggerMethod {
    #[default]
    #[serde(rename(serialize = "0"))]
    /// The default value. The "double bid/ask" function will be used for orders for OTC stocks and US options. All other orders will used the "last" function.
    Default,
    #[serde(rename(serialize = "1"))]
    /// Use "double bid/ask" function, where stop orders are triggered based on two consecutive bid or ask prices.
    DoubleBidAsk,
    #[serde(rename(serialize = "2"))]
    /// Stop orders are triggered based on the last price.
    Last,
    #[serde(rename(serialize = "3"))]
    /// Double last function.
    DoubleLast,
    #[serde(rename(serialize = "4"))]
    /// Bid/ask function
    BidAsk,
    #[serde(rename(serialize = "7"))]
    /// Last or bid/ask function
    LastOrBidAsk,
    #[serde(rename(serialize = "8"))]
    /// Mid-point function.
    MidPoint,
}

#[derive(Debug, Default, Clone, Copy, Ord, PartialOrd, PartialEq, Hash, Eq, Serialize)]
pub enum Origin {
    #[default]
    #[serde(rename(serialize="0"))]
    Customer,
    #[serde(rename(serialize="1"))]
    Firm,
}

macro_rules! impl_executable {
    ($o_name: ident; $($s_name: ident),*) => {
        $(
            impl Executable<$s_name> for $o_name {}
        )*
    };
}

impl_executable!(Market; Forex, Crypto, Stock, Index, SecFuture, SecOption, Commodity);
impl_executable!(Limit; Forex, Crypto, Stock, Index, SecFuture, SecOption, Commodity);
