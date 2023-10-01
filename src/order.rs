#![allow(missing_docs)]
use std::collections::HashMap;
use serde::{Serialize, Serializer};
use serde::ser::SerializeTuple;
use crate::{
    contract::{Commodity, Crypto, Forex, Index, SecFuture, SecOption, Security, Stock},
};

// ==============================================
// === Core Order Types (Market, Limit, etc.) ===
// ==============================================

// === Type definitions ===

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
/// The time periods for which an order is active and can be executed against.
pub enum TimeInForce {
    #[default]
    #[serde(rename(serialize="DAY"))]
    /// Valid for the day only.
    Day,
    #[serde(rename(serialize="GTC"))]
    /// Good until canceled. The order will continue to work within the system and in the marketplace until it executes or is canceled. GTC orders will be automatically be cancelled under the following conditions:
    /// If a corporate action on a security results in a stock split (forward or reverse), exchange for shares, or distribution of shares. If you do not log into your IB account for 90 days.
    /// At the end of the calendar quarter following the current quarter. For example, an order placed during the third quarter of 2011 will be canceled at the end of the first quarter of 2012. If the last day is a non-trading day, the cancellation will occur at the close of the final trading day of that quarter. For example, if the last day of the quarter is Sunday, the orders will be cancelled on the preceding Friday.
    /// Orders that are modified will be assigned a new “Auto Expire” date consistent with the end of the calendar quarter following the current quarter.
    /// Orders submitted to IB that remain in force for more than one day will not be reduced for dividends. To allow adjustment to your order price on ex-dividend date, consider using a Good-Til-Date/Time (GTD) or Good-after-Time/Date (GAT) order type, or a combination of the two.
    Gtc,
    #[serde(rename(serialize="IOC"))]
    /// Immediate or Cancel. Any portion that is not filled as soon as it becomes available in the market is canceled.
    Ioc,
    // #[serde(rename(serialize="GTD"))]
    // /// Good until Date. It will remain working within the system and in the marketplace until it executes or until the close of the market on the date specified
    // Gtd,
    // #[serde(rename(serialize="OPG"))]
    // /// Use OPG to send a market-on-open (MOO) or limit-on-open (LOO) order.
    // Opg,
    #[serde(rename(serialize="FOK"))]
    /// If the entire Fill-or-Kill order does not execute as soon as it becomes available, the entire order is canceled.
    Fok,
    #[serde(rename(serialize="DTC"))]
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

impl<Sec, E> Serialize for Order<'_, Sec, E> where Sec: Security, E: Executable<Sec> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut ser = serializer.serialize_tuple(1+crate::constants::ORDER_TUPLE_SIZE)?;
        let (action, exec) = match *self {
            Self::Buy { execute_method, .. } => ("BUY", execute_method),
            Self::Sell { execute_method, .. } => ("SELL", execute_method),
        };
        ser.serialize_element(action)?;
        serialize_executable(exec, &mut ser)?;
        ser.end()
    }
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

// ==================================================
// === Order Trait Definition and Implementations ===
// ==================================================

pub type BagRequestContent<'a> = (u64, &'a str, u64, &'a str, u64, HashMap<&'a str, &'a str>);
pub type DeltaNeutralOrderContent<'a> = (i64, &'a str, &'a str, &'a str, &'a str, bool, i64, &'a str);
pub type ScaleOrderContent = (f64, i64, f64, bool, i64, i64, bool);
#[allow(clippy::module_name_repetitions)]
pub type OrderConditionsContent<'a> = (usize, HashMap<&'a str, &'a str>, bool, bool);

// todo! "Add support for BAG contract type/requests, delta-neutral contracts
/// Implemented by all valid order types for a given security. In particular,
/// if a type `O` implements [`Executable<S>`], then `O` is a valid order for `S`.
pub trait Executable<S: Security>: Send + Sync {
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
    /// Return the BAG request and combo leg content, if it exists.
    fn get_bag_request_content(&self) -> MissingField<(), BagRequestContent> {
        MissingField::default()
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
    /// format "`yyyyMMdd HH:mm:ss` (optional time zone)" or UTC "yyyyMMdd-HH:mm:ss".
    fn get_good_until_date(&self) -> Option<&str> {
        None
    }

    #[inline]
    /// Return the model code associated with a given order.
    ///
    /// Is used to place an order to a model. For example, "Technology" model can be used for tech
    /// stocks first created in TWS.
    fn get_model_code(&self) -> Option<&str> {
        None
    }

    #[inline]
    /// Return the one-cancels-all group
    ///
    /// Tells how to handle remaining orders in an OCA group when one order or part of an order
    /// executes.
    fn get_one_cancels_all_type(&self) -> OneCancelsAllType {
        OneCancelsAllType::default()
    }

    #[inline]
    /// Return the Rule 80 A details for an order
    fn get_rule_80a(&self) -> Option<Rule80A> {
        None
    }

    #[inline]
    /// Returns whether or not all the order has to be filled on a single execution.
    fn get_is_all_or_none(&self) -> bool {
        false
    }

    #[inline]
    /// Returns the minimum quantity for an order (ie. a minimum quantity order type).
    fn get_minimum_quantity(&self) -> Option<u64> {
        None
    }

    #[inline]
    /// Return the percent offset amount for relative orders.
    fn get_percent_offset(&self) -> Option<f64> {
        None
    }

    #[inline]
    /// Return the auction strategy.
    ///
    /// For BOX orders only.
    fn get_box_auction_strategy(&self) -> AuctionStrategy {
        AuctionStrategy::default()
    }

    #[inline]
    /// Return the auction's starting price.
    ///
    /// For BOX orders only.

    fn get_box_starting_price(&self) -> Option<f64> {
        None
    }

    #[inline]
    /// Return the stock's reference price.
    /// The reference price is used for VOL orders to compute the limit price sent to an exchange
    /// (whether or not Continuous Update is selected), and for price range monitoring.
    fn get_box_stock_reference_price(&self) -> Option<f64> {
        None
    }

    #[inline]
    /// Return the stock's Delta. For orders on BOX only.
    fn get_box_stock_delta(&self) -> Option<f64> {
        None
    }

    #[inline]
    /// Return the lower value for the acceptable underlying stock price range.
    ///
    /// For price improvement option orders on BOX and VOL orders with dynamic management.
    fn get_box_vol_stock_range_lower(&self) -> Option<f64> {
        None
    }

    #[inline]
    /// Return the upper value for the acceptable underlying stock price range.
    ///
    /// For price improvement option orders on BOX and VOL orders with dynamic management.
    fn get_box_vol_stock_range_upper(&self) -> Option<f64> {
        None
    }

    #[inline]
    /// Return whether the order will override validation from TWS.
    ///
    ///
    /// Precautionary constraints are defined on the TWS Presets page, and help ensure that your
    /// price and size order values are reasonable. Orders sent from the API are also validated
    /// against these safety constraints, and may be rejected if any constraint is violated. To
    /// override validation, set this parameter’s value to True.
    fn get_will_override_validation(&self) -> bool {
        false
    }

    #[inline]
    /// Return the option price in volatility, as calculated by TWS' Option Analytics.
    ///
    /// This value is expressed as a percent and is used to calculate the limit
    /// price sent to the exchange.
    fn get_volatility_quote(&self) -> Option<f64> {
        None
    }

    #[inline]
    /// Return the type of volatility associated with a volatility quote.
    fn get_volatility_type(&self) -> Option<VolatilityType> {
        None
    }

    #[inline]
    /// Return the delta neutral order type.
    ///
    /// Enter an order type to instruct TWS to submit a delta neutral trade on full or partial
    /// execution of the VOL order.
    ///
    /// VOL orders only. For no hedge delta order to be sent, specify NONE.
    fn get_delta_neutral_order_type(&self) -> Option<&str> {
        None
    }

    #[inline]
    /// Return the auxiliary price associated with a delta neutral order.
    ///
    /// Use this field to enter a value if the value in the `deltaNeutralOrderType` field is an order
    /// type that requires an Aux price, such as a REL order. VOL orders only.
    fn get_delta_neutral_auxiliary_price(&self) -> Option<f64> {
        None
    }

    #[inline]
    /// Return the delta neutral order content if it exists.
    fn get_delta_neutral_order_content(&self) -> MissingField<(), DeltaNeutralOrderContent> {
        MissingField::default()
    }

    #[inline]
    /// Return whether TWS will automatically update the limit price of the order as the
    /// underlying price moves.
    ///
    /// VOL orders only.
    fn get_continuous_update(&self) -> bool {
        false
    }

    #[inline]
    /// Return how you want TWS to calculate the limit price for options, and for stock range price monitoring.
    /// VOL orders only.
    fn get_reference_price_type(&self) -> Option<ReferencePriceType> {
        None
    }

    #[inline]
    /// Return the trailing stop price for trail limit orders
    fn get_trail_stop_price(&self) -> Option<f64> {
        None
    }

    #[inline]
    /// Return the trailing amount of a trailing stop order as a percentage.
    ///
    /// Observe the following guidelines when using the trailingPercent field:
    ///
    /// This field is mutually exclusive with the existing trailing amount. That is, the API client can send one or the other but not both.
    /// This field is read AFTER the stop price (barrier price) as follows: `deltaNeutralAuxPrice` stopPrice, trailingPercent, scale order attributes
    /// The field will also be sent to the API in the openOrder message if the API client version is >= 56. It is sent after the stopPrice field as follows: stopPrice, trailingPct, basisPoint.
    fn get_trailing_percent(&self) -> Option<f64> {
        None
    }

    #[inline]
    /// Return the size of the first, or initial, order component.
    ///
    /// For Scale orders only.
    fn get_scale_initial_level_size(&self) -> Option<i64> {
        None
    }

    #[inline]
    /// Return the order size of the subsequent scale order components.
    ///
    /// For Scale orders only. Used in conjunction with `scaleInitLevelSize`.
    fn get_scale_subs_level_size(&self) -> Option<i64> {
        None
    }

    #[inline]
    /// Return the price increment between scale components.
    ///
    /// For Scale orders only. This value is compulsory.
    fn get_scale_price_increment(&self) -> Option<f64> {
        None
    }

    #[inline]
    /// Return the scale order content, if it exists.
    fn get_scale_order_content(&self) -> MissingField<(), ScaleOrderContent> {
        MissingField::default()
    }

    #[inline]
    /// Return the list of scale orders.
    ///
    /// Used for scale orders.
    fn get_scale_table(&self) -> Option<&str> {
        None
    }

    #[inline]
    /// Return the start time of a GTC order.
    fn get_active_start_time(&self) -> Option<&str> {
        None
    }

    #[inline]
    /// Return the stop time of a GTC order.
    fn get_active_stop_time(&self) -> Option<&str> {
        None
    }

    #[inline]
    /// Return the type of hedge for a hedge order.
    fn get_hedge_type(&self) -> Option<HedgeType> {
        None
    }

    #[inline]
    /// Return the hedge order content, if it exists.
    ///
    /// For hedge orders.
    /// Beta = x for Beta hedge orders, ratio = y for Pair hedge order
    fn get_hedge_parameter_content(&self) -> MissingField<(), &str> {
        MissingField::default()
    }

    #[inline]
    /// Return whether an order has opted out of `SmartRouting` for orders routed directly to ASX.
    ///
    /// This attribute defaults to false unless explicitly set to true.
    /// When set to false, orders routed directly to ASX will NOT use `SmartRouting`.
    /// When set to true, orders routed directly to ASX orders WILL use `SmartRouting`.
    fn get_opt_out_smart_routing(&self) -> bool {
        false
    }

    #[inline]
    /// Return the true beneficiary of the order.
    ///
    /// For `IBExecution` customers.
    ///
    /// This value is required for FUT/FOP orders for reporting to the exchange.
    fn get_clearing_account(&self) -> Option<&str> {
        None
    }

    #[inline]
    /// For execution-only clients to know where do they want their shares to be cleared at.
    fn get_clearing_intent(&self) -> Option<ClearingIntent> {
        None
    }

    #[inline]
    /// Orders routed to IBDARK are tagged as “post only” and are held in IB's order book, where
    /// incoming `SmartRouted` orders from other IB customers are eligible to trade against them.
    ///
    /// For IBDARK orders only.
    fn get_is_not_held(&self) -> bool {
        false
    }

    #[inline]
    /// Return the delta neutral content, if it exists
    fn get_delta_neutral_contract_content(&self) -> MissingField<bool, (bool, i64, f64, f64)> {
        MissingField::Missing(false)
    }

    #[inline]
    /// Return the algorithm strategy.
    ///
    /// For more information about IB's API algorithms, refer to IBKR's
    /// [IB algorithm description](https://interactivebrokers.github.io/tws-api/ibalgos.html)
    fn get_algo_strategy(&self) -> Option<AlgoStrategy> {
        None
    }

    #[inline]
    /// Return the algorithm strategy content (ie. The list of parameters for the IB algorithm),
    /// if it exists.
    ///
    /// For more information about IB's API algorithms, refer to IBKR's
    /// [IB algorithm description](https://interactivebrokers.github.io/tws-api/ibalgos.html)
    fn get_algo_strategy_content(&self) -> MissingField<(), (u64, HashMap<&str, &str>)> {
        MissingField::default()
    }

    #[inline]
    /// Return the ID generated by algorithmic trading.
    fn get_algo_id(&self) -> Option<&str> {
        None
    }

    #[inline]
    /// Return the what if information for an order.
    ///
    /// Allows to retrieve the commissions and margin information.
    /// When placing an order with this attribute set to true, the order will not be placed as such. Instead it will used to request the commissions and margin information that would result from this order.
    fn get_what_if(&self) -> bool {
        false
    }

    #[inline]
    /// Return whether an order was solicited.
    ///
    /// The Solicited field should be used for orders initiated or recommended by the broker or
    /// adviser that were approved by the client (by phone, email, chat, verbally, etc.) prior to
    /// entry. Please note that orders that the adviser or broker placed without specifically
    /// discussing with the client are discretionary orders, not solicited.
    fn get_solicited(&self) -> bool {
        false
    }

    #[inline]
    /// Return whether the order size will be randomized.
    ///
    /// Randomizes the order's size. Only for Volatility and Pegged to Volatility orders.
    fn get_will_randomize_size(&self) -> bool {
        false
    }

    #[inline]
    /// Return whether the order price will be randomized.
    ///
    /// Randomizes the order's price. Only for Volatility and Pegged to Volatility orders.
    fn get_will_randomize_price(&self) -> bool {
        false
    }

    #[inline]
    /// Return peg bench order content, if it exists.
    fn get_peg_bench_order_content(&self) -> MissingField<(), (i64, bool, f64, f64, &str)> {
        MissingField::default()
    }

    #[inline]
    /// Return order conditions content.
    fn get_order_conditions_content(&self) -> MissingField<usize, OrderConditionsContent> {
        MissingField::Missing(0)
    }

    #[inline]
    /// Return the adjusted order type.
    ///
    /// Adjusted Stop orders: the parent order will be adjusted to the given type when the adjusted
    /// trigger price is penetrated.
    fn get_adjusted_order_type(&self) -> Option<&str> {
        None
    }

    #[inline]
    /// Return the trigger price.
    ///
    /// Adjusted Stop orders: specifies the trigger price to execute.
    fn get_trigger_price(&self) -> f64 {
        f64::MAX
    }

    #[inline]
    /// Return limit price offset.
    ///
    /// Adjusted Stop orders: specifies the price offset for the stop to move in increments.
    fn get_limit_price_offset(&self) -> f64 {
        f64::MAX
    }

    #[inline]
    /// Return the adjusted stop price.
    ///
    /// Adjusted Stop orders: specifies the stop price of the adjusted (STP) parent.
    fn get_adjusted_stop_price(&self) -> f64 {
        f64::MAX
    }

    #[inline]
    /// Return the adjusted stop limit price.
    ///
    /// Adjusted Stop orders: specifies the stop limit price of the adjusted (STPL LMT) parent.
    fn get_adjusted_stop_limit_price(&self) -> f64 {
        f64::MAX
    }

    #[inline]
    /// Return the adjusted trailing amount.
    ///
    /// Adjusted Stop orders: specifies the trailing amount of the adjusted (TRAIL) parent.
    fn get_adjusted_trailing_amount(&self) -> f64 {
        f64::MAX
    }

    #[inline]
    /// Return the adjusted trailing unit.
    ///
    /// Adjusted Stop orders: specifies where the trailing unit is an amount
    /// (set to 0) or a percentage (set to 1)
    fn get_adjusted_trailing_unit(&self) -> AdjustedTrailingUnit {
        AdjustedTrailingUnit::default()
    }

    #[inline]
    /// Return the regulatory ext operator.
    ///
    /// This is a regulatory attribute that applies to all US Commodity (Futures) Exchanges,
    /// provided to allow client to comply with CFTC Tag 50 Rules.
    fn get_ext_operator(&self) -> Option<&str> {
        None
    }

    #[inline]
    /// Return the soft dollar tier information.
    fn get_soft_dollar_tier(&self) -> (Option<&str>, Option<&str>) {
        (None, None)
    }

    #[inline]
    /// Return the cash quantity
    fn get_cash_quantity(&self) -> f64 {
        f64::MAX
    }

    #[inline]
    /// Return the responsible party for investment decisions within the firm.
    ///
    /// Orders covered by `MiFID` 2 (Markets in Financial Instruments Directive 2) must include either
    /// `Mifid2DecisionMaker` or `Mifid2DecisionAlgo` field (but not both). Requires TWS 969+.
    fn get_decision_maker(&self) -> Option<&str> {
        None
    }

    #[inline]
    /// Return the algorithm responsible for investment decisions within the firm.
    ///
    /// Orders covered under `MiFID` 2 must include either `Mifid2DecisionMaker` or `Mifid2DecisionAlgo`,
    /// but cannot have both. Requires TWS 969+.
    fn get_decision_algorithm(&self) -> Option<&str> {
        None
    }

    #[inline]
    /// Returns the responsible party for the execution of a transaction within the firm.
    ///
    /// For `MiFID` 2 reporting; identifies a person as the responsible party for the execution of a
    /// transaction within the firm. Requires TWS 969+.
    fn get_execution_trader(&self) -> Option<&str> {
        None
    }

    #[inline]
    /// Return the algorithm responsible for the execution of a transaction within the firm.
    ///
    /// For `MiFID` 2 reporting; identifies the algorithm responsible for the execution of a
    /// transaction within the firm. Requires TWS 969+.
    fn get_execution_algorithm(&self) -> Option<&str> {
        None
    }

    #[inline]
    /// Return whether an auto price should not / should be used for hedging.
    fn get_dont_use_auto_price_for_hedge(&self) -> bool {
        false
    }

    #[inline]
    /// Return whether tickets from API orders when TWS will be used as an OMS.
    fn get_oms_container(&self) -> bool {
        false
    }

    #[inline]
    /// Return whether to convert order of type 'Primary Peg' to 'D-Peg'.
    fn get_discretionary_up_to_limit_price(&self) -> bool {
        false
    }

    #[inline]
    /// Return whether to use a price management algorithm.
    fn get_use_price_management_algorithm(&self) -> Option<bool> {
        None
    }

    #[inline]
    /// Return the duration of the order.
    fn get_duration(&self) -> i64 {
        i64::from(i32::MAX)
    }

    #[inline]
    /// Return a value must be positive, and it is number of seconds that SMART order would be
    /// parked for at IBKRATS before being routed to exchange.
    fn get_post_to_ats(&self) -> i64  {
        i64::from(i32::MAX)
    }

    #[inline]
    /// Return whether the parent order will be cancelled if child order was cancelled.
    fn get_auto_cancel_parent(&self) -> bool {
        false
    }

    #[inline]
    /// Return a list with parameters obtained from `advancedOrderRejectJson`.
    fn get_advanced_error_override(&self) -> Option<&str> {
        None
    }

    #[inline]
    /// Return the manual order time.
    ///
    /// Used by brokers and advisors when manually entering, modifying or cancelling orders at the
    /// direction of a client. Only used when allocating orders to specific groups or accounts.
    /// Excluding "All" group.
    fn get_manual_order_time(&self) -> Option<&str> {
        None
    }

    #[inline]
    /// Return the peg-to-mid order content, if it exists
    fn get_peg_to_mid_content(&self) -> MissingField<(), &str>{
        MissingField::default()
    }
}

#[inline]
#[allow(clippy::too_many_lines)]
fn serialize_executable<E, Sec, Ser>(
    exec: &E,
    ser: &mut Ser
) -> Result<(), Ser::Error>
    where E: Executable<Sec>, Sec: crate::contract::Security, Ser: SerializeTuple
{
    ser.serialize_element(&exec.get_quantity())?;
    ser.serialize_element(&exec.get_order_type())?;
    ser.serialize_element(&exec.get_limit_price())?;
    ser.serialize_element(&exec.get_auxiliary_price())?;
    ser.serialize_element(&exec.get_time_in_force())?;
    ser.serialize_element(&exec.get_one_cancels_all_group())?;
    ser.serialize_element(&exec.get_account())?;
    ser.serialize_element(&None::<()>)?;
    ser.serialize_element(&exec.get_origin())?;
    ser.serialize_element(&exec.get_order_reference())?;
    ser.serialize_element(&exec.get_will_transmit())?;
    ser.serialize_element(&exec.get_parent_id())?;
    ser.serialize_element(&exec.get_is_block_order())?;
    ser.serialize_element(&exec.get_is_sweep_to_fill())?;
    ser.serialize_element(&exec.get_iceberg_order_size())?;
    ser.serialize_element(&exec.get_trigger_method())?;
    ser.serialize_element(&exec.get_can_fill_outside_regular_trading_hours())?;
    ser.serialize_element(&exec.get_is_hidden_on_nasdaq_market_depth())?;
    ser.serialize_element(&exec.get_bag_request_content())?;
    ser.serialize_element(&None::<()>)?;
    ser.serialize_element(&exec.get_discretionary_amount())?;
    ser.serialize_element(&exec.get_good_after_time())?;
    ser.serialize_element(&exec.get_good_until_date())?;
    ser.serialize_element(&[None::<()>; 3])?;
    ser.serialize_element(&exec.get_model_code())?;
    ser.serialize_element(&0)?;
    ser.serialize_element(&None::<()>)?;
    ser.serialize_element(&-1)?;
    ser.serialize_element(&exec.get_one_cancels_all_type())?;
    ser.serialize_element(&exec.get_rule_80a())?;
    ser.serialize_element(&None::<()>)?;
    ser.serialize_element(&exec.get_is_all_or_none())?;
    ser.serialize_element(&exec.get_minimum_quantity())?;
    ser.serialize_element(&exec.get_percent_offset())?;
    ser.serialize_element(&false)?;
    ser.serialize_element(&false)?;
    ser.serialize_element(&None::<()>)?;
    ser.serialize_element(&exec.get_box_auction_strategy())?;
    ser.serialize_element(&exec.get_box_starting_price())?;
    ser.serialize_element(&exec.get_box_stock_reference_price())?;
    ser.serialize_element(&exec.get_box_stock_delta())?;
    ser.serialize_element(&exec.get_box_vol_stock_range_lower())?;
    ser.serialize_element(&exec.get_box_vol_stock_range_upper())?;
    ser.serialize_element(&exec.get_will_override_validation())?;
    ser.serialize_element(&exec.get_volatility_quote())?;
    ser.serialize_element(&exec.get_volatility_type())?;
    ser.serialize_element(&exec.get_delta_neutral_order_type())?;
    ser.serialize_element(&exec.get_delta_neutral_auxiliary_price())?;
    ser.serialize_element(&exec.get_delta_neutral_order_content())?;
    ser.serialize_element(&exec.get_continuous_update())?;
    ser.serialize_element(&exec.get_reference_price_type())?;
    ser.serialize_element(&exec.get_trail_stop_price())?;
    ser.serialize_element(&exec.get_trailing_percent())?;
    ser.serialize_element(&exec.get_scale_initial_level_size())?;
    ser.serialize_element(&exec.get_scale_subs_level_size())?;
    ser.serialize_element(&exec.get_scale_price_increment())?;
    ser.serialize_element(&exec.get_scale_order_content())?;
    ser.serialize_element(&exec.get_scale_table())?;
    ser.serialize_element(&exec.get_active_start_time())?;
    ser.serialize_element(&exec.get_active_stop_time())?;
    ser.serialize_element(&exec.get_hedge_type())?;
    ser.serialize_element(&exec.get_hedge_parameter_content())?;
    ser.serialize_element(&exec.get_opt_out_smart_routing())?;
    ser.serialize_element(&exec.get_clearing_account())?;
    ser.serialize_element(&exec.get_clearing_intent())?;
    ser.serialize_element(&exec.get_is_not_held())?;
    ser.serialize_element(&exec.get_delta_neutral_contract_content())?;
    ser.serialize_element(&exec.get_algo_strategy())?;
    ser.serialize_element(&exec.get_algo_strategy_content())?;
    ser.serialize_element(&exec.get_algo_id())?;
    ser.serialize_element(&exec.get_what_if())?;
    ser.serialize_element(&None::<()>)?;
    ser.serialize_element(&exec.get_solicited())?;
    ser.serialize_element(&exec.get_will_randomize_size())?;
    ser.serialize_element(&exec.get_will_randomize_price())?;
    ser.serialize_element(&exec.get_peg_bench_order_content())?;
    ser.serialize_element(&exec.get_order_conditions_content())?;
    ser.serialize_element(&exec.get_adjusted_order_type())?;
    ser.serialize_element(&exec.get_trigger_price())?;
    ser.serialize_element(&exec.get_limit_price_offset())?;
    ser.serialize_element(&exec.get_adjusted_stop_price())?;
    ser.serialize_element(&exec.get_adjusted_stop_limit_price())?;
    ser.serialize_element(&exec.get_adjusted_trailing_amount())?;
    ser.serialize_element(&exec.get_adjusted_trailing_unit())?;
    ser.serialize_element(&exec.get_ext_operator())?;
    ser.serialize_element(&exec.get_soft_dollar_tier())?;
    ser.serialize_element(&exec.get_cash_quantity())?;
    ser.serialize_element(&exec.get_decision_maker())?;
    ser.serialize_element(&exec.get_decision_algorithm())?;
    ser.serialize_element(&exec.get_execution_trader())?;
    ser.serialize_element(&exec.get_execution_algorithm())?;
    ser.serialize_element(&exec.get_dont_use_auto_price_for_hedge())?;
    ser.serialize_element(&exec.get_oms_container())?;
    ser.serialize_element(&exec.get_discretionary_up_to_limit_price())?;
    ser.serialize_element(&exec.get_use_price_management_algorithm())?;
    ser.serialize_element(&exec.get_duration())?;
    ser.serialize_element(&exec.get_post_to_ats())?;
    ser.serialize_element(&exec.get_auto_cancel_parent())?;
    ser.serialize_element(&exec.get_advanced_error_override())?;
    ser.serialize_element(&exec.get_manual_order_time())?;
    ser.serialize_element(&exec.get_peg_to_mid_content())
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

#[derive(Debug, Default, Clone, Copy, Ord, PartialOrd, PartialEq, Hash, Eq, Serialize)]
/// Represents the possible ways of handling one-cancels-all behavior for a group of orders.
///
/// Tells how to handle remaining orders in an OCA group when one order or part of an order
/// executes.
///
/// If you use a value "with block" it gives the order overfill protection. This means that only one
/// order in the group will be routed at a time to remove the possibility of an overfill.
pub enum OneCancelsAllType {
    #[default]
    #[serde(rename(serialize="0"))]
    /// The default one-cancels-all type, used for normal orders that do not implement
    /// One-cancels-all behavior
    Default,
    #[serde(rename(serialize="1"))]
    /// Cancel all remaining orders with block.
    CancelWithBlock,
    #[serde(rename(serialize="2"))]
    /// Remaining orders are proportionately reduced in size with block.
    ReduceWithBlock,
    #[serde(rename(serialize="3"))]
    /// Remaining orders are proportionately reduced in size with no block.
    ReduceNonBlock,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, PartialEq, Hash, Eq, Serialize)]
pub enum Rule80A {
    #[serde(rename(serialize="I"))]
    Individual,
    #[serde(rename(serialize="A"))]
    Agency,
    #[serde(rename(serialize="W"))]
    AgentOtherMember,
    #[serde(rename(serialize="J"))]
    IndividualPtia,
    #[serde(rename(serialize="U"))]
    AgencyPtia,
    #[serde(rename(serialize="M"))]
    AgentOtherMemberPtia,
    #[serde(rename(serialize="K"))]
    IndividualPt,
    #[serde(rename(serialize="Y"))]
    AgencyPt,
    #[serde(rename(serialize="N"))]
    AgentOtherMemberPt,
}

#[derive(Debug, Default, Clone, Copy, Ord, PartialOrd, PartialEq, Hash, Eq, Serialize)]
pub enum AuctionStrategy {
    #[default]
    #[serde(rename(serialize = "0"))]
    /// Used for non-box orders that define no auction strategy.
    Default,
    #[serde(rename(serialize = "1"))]
    /// Match strategy.
    Match,
    #[serde(rename(serialize = "2"))]
    /// Improvement strategy.
    Improvement,
    #[serde(rename(serialize = "3"))]
    /// transparent strategy.
    Transparent,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, PartialEq, Hash, Eq, Serialize)]
pub enum VolatilityType {
    #[serde(rename(serialize="1"))]
    Daily,
    #[serde(rename(serialize="2"))]
    Annual,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, PartialEq, Hash, Eq, Serialize)]
pub enum ReferencePriceType {
    #[serde(rename(serialize="1"))]
    /// Average of NBBO.
    Average,
    #[serde(rename(serialize="2"))]
    /// NBB or the NBO depending on the action and right.
    BidOrAsk,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, PartialEq, Hash, Eq, Serialize)]
pub enum HedgeType {
    #[serde(rename(serialize="D"))]
    Delta,
    #[serde(rename(serialize="B"))]
    Beta,
    #[serde(rename(serialize="F"))]
    Forex,
    #[serde(rename(serialize="P"))]
    Pair,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, PartialEq, Hash, Eq, Serialize)]
pub enum ClearingIntent {
    #[serde(rename(serialize="IB"))]
    /// Interactive Brokers clearing
    Ib,
    #[serde(rename(serialize="Away"))]
    /// Away
    Away,
    #[serde(rename(serialize="PTA"))]
    /// Post-trade allocation
    PostTradeAllocation,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, PartialEq, Hash, Eq, Serialize)]
pub enum AlgoStrategy {
    #[serde(rename(serialize="ArrivalPx"))]
    /// Arrival price algorithm.
    ArrivalPrice,
    /// Dark ice algorithm.
    DarkIce,
    #[serde(rename(serialize="PctVol"))]
    /// Percentage of volume algorithm.
    PercentVolume,
    /// TWAP (Time Weighted Average Price) algorithm.
    Twap,
    /// VWAP (Volume Weighted Average Price) algorithm.
    Vwap,
}

#[derive(Debug, Default, Clone, Copy, Ord, PartialOrd, PartialEq, Hash, Eq, Serialize)]
pub enum AdjustedTrailingUnit {
    #[default]
    #[serde(rename(serialize="0"))]
    Amount,
    #[serde(rename(serialize="1"))]
    Percentage,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, PartialEq, Hash, Eq, Serialize)]
pub enum MissingField<T, U> {
    /// A missing field
    Missing(T),
    /// A present field
    Present(U),
}

impl<T: Default, U> Default for MissingField<T, U> {
    fn default() -> Self {
        MissingField::Missing(T::default())
    }
}

macro_rules! impl_executable {
    ($o_name: ident; $($s_name: ident),*; $executable_impl: tt) => {
        $(
            impl Executable<$s_name> for $o_name
                $executable_impl
        )*
    };
}

impl_executable!(Market; Forex, Crypto, Stock, Index, SecFuture, SecOption, Commodity; {
    fn get_quantity(&self) -> f64 {
        self.quantity
    }

    fn get_order_type(&self) -> &'static str {
        "MKT"
    }

    fn get_time_in_force(&self) -> TimeInForce {
        self.time_in_force
    }
});
impl_executable!(Limit; Forex, Crypto, Stock, Index, SecFuture, SecOption, Commodity; {
    fn get_quantity(&self) -> f64 {
        self.quantity
    }

    fn get_order_type(&self) -> &'static str {
        "LMT"
    }

    fn get_time_in_force(&self) -> TimeInForce {
        self.time_in_force
    }

    fn get_limit_price(&self) -> Option<f64> {
        Some(self.price)
    }
});
