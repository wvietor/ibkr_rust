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

/// Contains the all-important [`client::Client`] struct and its methods, which facilitate
/// communication with the IBKR. Also contains a [`client::Builder`] struct to manage the
/// creation of new connections.
pub mod client;
#[macro_use]
mod comm;
/// Contains types related to account information.
pub mod account;
mod constants;
/// Contains the definitions of all [`contract::Security`] implementors, which represent tradable
/// contracts.
///
/// Each variety of financial instrument instrument is represented as its own unique struct or
/// enum. They all implement the [`contract::Security`] trait, which means they are a valid IBKR
/// contract and that they have at least one valid order type.
pub mod contract;
/// Contains the definition of a [`currency::Currency`] enum, which represents the possible trading
/// currencies available in the API.
pub mod currency;
#[allow(
    dead_code,
    unused_variables,
    clippy::print_stdout,
    clippy::use_debug,
    clippy::too_many_lines,
    clippy::unnecessary_wraps
)]
mod decode;
/// Contains types related to security exchanges and trading venues available in the API.
pub mod exchange;
mod execution;
/// Contains modules that each relate to different market data requests. In particular, each module
/// defines: 1) General types used in a given market data query and 2) Optionally, a private
/// indicator trait that defines whether a given [`contract::Security`] allows for the data request
/// and 3) Any types associated with implementors of the indicator types.
pub mod market_data;
mod message;
mod message2;
/// Contains types and traits related to orders.
pub mod order;
/// Contains the types that are parsed from API callbacks. They are used in the [`wrapper::Wrapper`]
/// callback functions.
pub mod payload;
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
/// [`wrapper::Wrapper`] method.
pub mod tick;
/// Contains the definition of the [`wrapper::Wrapper`] trait. Implementing this trait on a custom
/// type allows users to customize callback behavior.
pub mod wrapper;

#[allow(missing_docs, clippy::use_debug, clippy::print_stdout)]
pub mod default_wrapper {
    use crate::account::Attribute;
    use crate::payload::{Pnl, Position, PositionSummary};
    use crate::tick::{
        Accessibility, AuctionData, Class, Dividends, EtfNav, ExtremeValue, Ipo, MarkPrice, News,
        OpenInterest, Price, PriceFactor, QuotingExchanges, Rate, RealTimeVolume,
        SecOptionCalculationSource, SecOptionVolume, Size, SummaryVolume, TimeStamp, TradeCount,
        Volatility, Volume, Yield,
    };
    use crate::wrapper::Wrapper;
    use chrono::NaiveTime;

    #[derive(Debug)]
    pub struct DefaultWrapper;

    impl Wrapper for DefaultWrapper {
        #[inline]
        fn error(
            &mut self,
            req_id: i64,
            error_code: i64,
            error_string: String,
            advanced_order_reject_json: String,
        ) {
            println!("\x1B[31mOh no, an error occurred! Req ID: {req_id}, Error Code {error_code}: {error_string} {advanced_order_reject_json}\x1B[0m");
        }

        #[inline]
        fn current_time(&mut self, datetime: chrono::NaiveDateTime) {
            println!("What time is it? It's {datetime} according to the IB API!");
        }

        #[inline]
        fn etf_nav(&mut self, req_id: i64, nav: EtfNav) {
            println!("Ooh! An ETF associated with Req ID: {req_id} has NAV: {nav:?}");
        }

        #[inline]
        fn price_data(&mut self, req_id: i64, price: Class<Price>) {
            println!(
                "We got some interesting price data with Req ID: {req_id}, its price is {price:?}"
            );
        }

        #[inline]
        fn size_data(&mut self, req_id: i64, size: Class<Size>) {
            println!(
                "We got some interesting size data with Req ID: {req_id}, its size is {size:?}"
            );
        }

        #[inline]
        fn yield_data(&mut self, req_id: i64, yld: Yield) {
            println!(
                "We got some interesting yield data with Req ID: {req_id}, its size is {yld:?}"
            );
        }

        #[inline]
        fn extreme_data(&mut self, req_id: i64, value: ExtremeValue) {
            println!("We got some interesting extreme value data with Req ID: {req_id}, its size is {value:?}");
        }

        #[inline]
        fn sec_option_computation(&mut self, req_id: i64, calc: Class<SecOptionCalculationSource>) {
            println!("We got some interesting option computation data with Req ID: {req_id}, its size is {calc:?}");
        }

        #[inline]
        fn quoting_exchanges(&mut self, req_id: i64, quoting_exchanges: QuotingExchanges) {
            println!(
                "The cool exchanges for Req ID: {req_id} with good prices are: {:?}",
                &quoting_exchanges
            );
        }

        #[inline]
        fn open_interest(&mut self, req_id: i64, open_interest: OpenInterest) {
            println!("The open interest for Req ID: {req_id} is {open_interest:?}");
        }

        #[inline]
        fn volatility(&mut self, req_id: i64, vol: Volatility) {
            println!("Req ID: {req_id} is exactly this volatile: {vol:?}");
        }

        #[inline]
        fn timestamp(&mut self, req_id: i64, timestamp: Class<TimeStamp>) {
            println!("Req ID: {req_id} says that something big happened at {timestamp:?}");
        }

        #[inline]
        fn auction(&mut self, req_id: i64, auction: AuctionData) {
            println!("The auctioneer at Req ID: {req_id} says {auction:?}");
        }

        #[inline]
        fn mark_price(&mut self, req_id: i64, mark: MarkPrice) {
            println!("The Req ID: {req_id} says the current mark price is {mark:?}");
        }

        #[inline]
        fn price_factor(&mut self, req_id: i64, factor: PriceFactor) {
            println!("A cool number came in from Req ID: {req_id}, which is: {factor:?}");
        }

        #[inline]
        fn accessibility(&mut self, req_id: i64, access: Accessibility) {
            println!("I wonder if Req ID: {req_id} can be shorted or if it's halted... {access:?}");
        }

        #[inline]
        fn dividends(&mut self, req_id: i64, dividends: Dividends) {
            println!("Some juicy dividends from Req ID: {req_id} are {dividends:?}");
        }

        #[inline]
        fn news(&mut self, req_id: i64, news: News) {
            println!("You've got news from Req ID: {req_id}! {news}");
        }

        #[inline]
        fn ipo(&mut self, req_id: i64, ipo: Ipo) {
            println!("Someone's going public from Req ID: {req_id}, it's {ipo:?}");
        }

        #[inline]
        fn summary_volume(&mut self, req_id: i64, volume: SummaryVolume) {
            println!("Some cool volume information Req ID: {req_id}, {volume:?}");
        }

        #[inline]
        fn sec_option_volume(&mut self, req_id: i64, volume: SecOptionVolume) {
            println!("Some cool option volume information Req ID: {req_id}, {volume:?}");
        }

        #[inline]
        fn trade_count(&mut self, req_id: i64, trade_count: TradeCount) {
            println!("One, two, three, ... Req ID: {req_id} says {trade_count:?} trades today");
        }

        #[inline]
        fn rate(&mut self, req_id: i64, rate: Rate) {
            println!("How fast is the market going? Req ID: {req_id} says {rate:?}");
        }

        #[inline]
        fn volume(&mut self, req_id: i64, volume: Volume) {
            println!("No summaries here; interesting volume information from Req ID: {req_id}, which is {volume:?}");
        }

        #[inline]
        fn real_time_volume(&mut self, req_id: i64, volume: RealTimeVolume) {
            println!(
                "Look how fast the real time volume from Req ID: {req_id} is coming! {volume:?}"
            );
        }

        #[inline]
        fn tick_params(
            &mut self,
            req_id: i64,
            min_tick: f64,
            exchange_id: crate::payload::ExchangeId,
            snapshot_permissions: u32,
        ) {
            println!("Look at some interesting parameters for Req ID: {req_id}. It has min_tick {min_tick}, SMART components {exchange_id:?}. We have permissions {snapshot_permissions}");
        }

        #[inline]
        fn market_data_class(&mut self, req_id: i64, class: crate::payload::MarketDataClass) {
            println!("Req ID: {req_id} says the market data class is {class:?}");
        }

        #[inline]
        fn update_market_depth(
            &mut self,
            req_id: i64,
            operation: crate::payload::market_depth::Operation,
        ) {
            println!(
                "New market depth info from Req ID: {req_id}. We're supposed to do {operation:?}"
            );
        }

        #[inline]
        fn histogram(
            &mut self,
            req_id: i64,
            histogram: std::collections::HashMap<usize, crate::payload::HistogramEntry>,
        ) {
            println!("New histogram from Req ID: {req_id}. It is as follows: {histogram:?}");
        }

        #[inline]
        fn historical_bars(&mut self, req_id: i64, bars: Vec<crate::payload::HistoricalBar>) {
            println!("Some cool historical data from Req ID: {req_id}. The bars are {bars:?}");
        }

        #[inline]
        fn updating_historical_bar(&mut self, req_id: i64, bar: crate::payload::HistoricalBar) {
            println!(
                "We're updating our historical data from Req ID: {req_id}. The bar is {bar:?}"
            );
        }

        #[inline]
        fn head_timestamp(&mut self, req_id: i64, timestamp: chrono::NaiveDateTime) {
            println!("The first timestamp for Req ID: {req_id} is {timestamp}");
        }

        #[inline]
        fn historical_ticks(&mut self, req_id: i64, ticks: Vec<crate::payload::Tick>) {
            println!("The historical ticks for Req ID: {req_id} are {ticks:?}");
        }

        #[inline]
        fn live_tick(&mut self, req_id: i64, tick: crate::payload::Tick) {
            println!("New live tick for Req ID: {req_id}! {tick:?}");
        }

        #[inline]
        fn account_attribute(&mut self, attribute: Attribute, account_number: String) {
            println!("Updating account attribute for account {account_number}: {attribute:?}");
        }

        #[inline]
        fn position(&mut self, position: Position) {
            println!("New information about our position: {position:?}");
        }

        #[inline]
        fn account_attribute_time(&mut self, time: NaiveTime) {
            println!("The account attributes were refreshed at {time:?}");
        }

        #[inline]
        fn position_summary(&mut self, summary: PositionSummary) {
            println!("Summary information about position: {summary:?}");
        }

        #[inline]
        fn pnl(&mut self, req_id: i64, pnl: Pnl) {
            println!("According to Req ID: {req_id}, our current P&L is {pnl:?}");
        }

        #[inline]
        fn single_position_pnl(&mut self, req_id: i64, pnl: Pnl, position: f64, market_value: f64) {
            println!("According to Req ID: {req_id}, we have {position} of something with market_value {market_value}. Out P&L is {pnl:?}");
        }
    }
}
