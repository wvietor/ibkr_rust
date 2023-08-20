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

// ==============================
// === Valid Trait Definition ===
// ==============================

pub(crate) mod indicators {
    use super::{Limit, Market};

    pub trait Valid {}

    macro_rules! impl_valid {
        ($t_name: ident) => {
            impl Valid for $t_name {}
        };
    }

    impl_valid!(Market);
    impl_valid!(Limit);
}

// ==================================================
// === Order Trait Definition and Implementations ===
// ==================================================

/// Implemented by all valid order types for a given security. In particular,
/// if a type `O` implements [`Executable<S>`], then `O` is a valid order for `S`.
pub trait Executable<S: Security>: ToString + Send + Sync + indicators::Valid {}

macro_rules! impl_executable {
    ($o_name: ident; $($s_name: ident),*) => {
        $(
            impl Executable<$s_name> for $o_name {}
        )*
    };
}

impl_executable!(Market; Forex, Crypto, Stock, Index, SecFuture, SecOption, Commodity);
impl_executable!(Limit; Forex, Crypto, Stock, Index, SecFuture, SecOption, Commodity);
