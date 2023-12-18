#![allow(async_fn_in_trait)]
use crate::account::{Attribute, TagValue};
use crate::payload::{self, Bar, ExchangeId, HistogramEntry, Tick, Pnl, Position, PositionSummary};
use crate::tick::{self, Accessibility, AuctionData, Class, Dividends, ExtremeValue, Ipo, MarkPrice, News,
        OpenInterest, Price, PriceFactor, QuotingExchanges, Rate, RealTimeVolume,
        SecOptionCalculationSource, SecOptionVolume, Size, SummaryVolume, TimeStamp, TradeCount,
        Volatility, Volume, Yield,
};
use chrono::{NaiveDateTime, NaiveTime};
use ibapi_macros::debug_trait;


/// An integrated [`Wrapper`], which defines an application in which the client and wrapper share
/// the same thread. This allows the client to be invoked directly by the wrapper, without the
/// need for inter-thread communication.
pub trait Owned: Wrapper {
    /// Attach the client to the underlying wrapper struct. This will make the client accessible
    /// for use in any wrapper method.
    fn attach_client(&mut self, client: crate::client::ActiveClient);
    /// The main entry point for an [`Owned`] application. This method is called exactly once
    /// immediately after the client is successfully connected.
    async fn init(&mut self);
    /// The method that is executed during each iteration of the main client loop.
    async fn recurring(&mut self);
}

/// An integrated [`Wrapper`], which defines an application in which the client and wrapper share
/// the same thread. However, unlike an [`Owned`] wrapper, in this case, implementors do not take
/// ownership of the client; instead, they receive a reference to it in each pass of the main client
/// loop via the [`Borrowed::recurring`] method.
pub trait Borrowed: Wrapper {
    /// The main entry point for a [`Borrowed`] application. This method is called exactly once
    /// immediately after the client is successfully connected.
    async fn init(&mut self, client: &mut crate::client::ActiveClient);
    /// The method that is executed during each iteration of the main client loop.
    async fn recurring(&mut self, client: &mut crate::client::ActiveClient);
}

/// A standalone [`Wrapper`], which defines an application in which the client and wrapper are each
/// on their own thread. This means that trading behavior is defined in the scope in which the
/// client is created, not in the wrapper methods.
///
/// Note that this approach requires inter-thread communication between the wrapper and the client.
// pub trait Standalone: Wrapper + Send + Sync {}

#[debug_trait]
/// Contains the "callback functions" that correspond to the requests made by a [`crate::client::Client`].
pub trait Wrapper {
    /// The callback that corresponds to any error that encounters after an API request.
    ///
    /// Errors sent by the TWS are received here.
    fn error(
        &mut self,
        req_id: i64,
        error_code: i64,
        error_string: String,
        advanced_order_reject_json: String,
    ) -> impl std::future::Future {
    }
    /// The callback message that corresponds to [`crate::client::Client::req_current_time`].
    ///
    /// This is TWS's current time. TWS is synchronized with the server (not local computer) using NTP and this function will receive the current time in TWS.
    fn current_time(&mut self, datetime: NaiveDateTime) -> impl std::future::Future {}
    /// The callback message that corresponds to ETF Net Asset Value (NAV) data.
    fn etf_nav(&mut self, req_id: i64, nav: tick::EtfNav) -> impl std::future::Future {}
    /// The callback message that corresponds to price data from [`crate::client::Client::req_market_data`].
    fn price_data(&mut self, req_id: i64, price: Class<Price>) -> impl std::future::Future {}
    /// The callback message that corresponds to size data from [`crate::client::Client::req_market_data`].
    fn size_data(&mut self, req_id: i64, size: Class<Size>) -> impl std::future::Future {}
    /// The callback message that corresponds to the price (in yield terms) data from [`crate::client::Client::req_market_data`].
    fn yield_data(&mut self, req_id: i64, yld: Yield) -> impl std::future::Future {}
    /// The callback message that corresponds to the high/low prices over a period from [`crate::client::Client::req_market_data`]..
    fn extreme_data(&mut self, req_id: i64, value: ExtremeValue) -> impl std::future::Future {}
    /// The callback message that corresponds to the results of options computations (implied volatility, greeks, etc.) from [`crate::client::Client::req_market_data`]..
    fn sec_option_computation(&mut self, req_id: i64, calc: Class<SecOptionCalculationSource>) -> impl std::future::Future {}
    /// The callback message that corresponds to the list of exchanges actively quoting the best bid / best offer / last traded prices from [`crate::client::Client::req_market_data`].
    fn quoting_exchanges(&mut self, req_id: i64, quoting_exchanges: QuotingExchanges) -> impl std::future::Future {}
    /// The callback message that corresponds to the open interest of various derivatives contracts from [`crate::client::Client::req_market_data`].
    fn open_interest(&mut self, req_id: i64, open_interest: OpenInterest) -> impl std::future::Future {}
    /// The callback message that corresponds to volatility data from [`crate::client::Client::req_market_data`].
    fn volatility(&mut self, req_id: i64, vol: Volatility) -> impl std::future::Future {}
    /// The callback message that corresponds to timestamp data from [`crate::client::Client::req_market_data`].
    fn timestamp(&mut self, req_id: i64, timestamp: Class<TimeStamp>) -> impl std::future::Future {}
    /// The callback message that corresponds to auction data from [`crate::client::Client::req_market_data`].
    fn auction(&mut self, req_id: i64, auction: AuctionData) -> impl std::future::Future {}
    /// The callback message associated with mark price data from [`crate::client::Client::req_market_data`].
    fn mark_price(&mut self, req_id: i64, mark: MarkPrice) -> impl std::future::Future {}
    /// The callback message associated with factors / multipliers related to prices from [`crate::client::Client::req_market_data`].
    fn price_factor(&mut self, req_id: i64, factor: PriceFactor) -> impl std::future::Future {}
    /// The callback message associated with the ability to short or trade a security from [`crate::client::Client::req_market_data`].
    fn accessibility(&mut self, req_id: i64, access: Accessibility) -> impl std::future::Future {}
    /// The callback message containing information about dividends from [`crate::client::Client::req_market_data`].
    fn dividends(&mut self, req_id: i64, dividends: Dividends) -> impl std::future::Future {}
    /// The callback message containing news information from [`crate::client::Client::req_market_data`].
    fn news(&mut self, req_id: i64, news: News) -> impl std::future::Future {}
    /// The callback message containing information about IPOs from [`crate::client::Client::req_market_data`].
    fn ipo(&mut self, req_id: i64, ipo: Ipo) -> impl std::future::Future {}
    /// The callback message containing summary information about trading volume throughout a day or 90-day rolling period from [`crate::client::Client::req_market_data`].
    fn summary_volume(&mut self, req_id: i64, volume: SummaryVolume) -> impl std::future::Future {}
    /// The callback message containing information about daily option volume (and average option volume) from [`crate::client::Client::req_market_data`].
    fn sec_option_volume(&mut self, req_id: i64, volume: SecOptionVolume) -> impl std::future::Future {}
    /// The callback message containing information about the number of trades performed in a day from [`crate::client::Client::req_market_data`].
    fn trade_count(&mut self, req_id: i64, trade_count: TradeCount) -> impl std::future::Future {}
    /// The callback message containing information about the rate of trades or volume throughout a day from [`crate::client::Client::req_market_data`].
    fn rate(&mut self, req_id: i64, rate: Rate) -> impl std::future::Future {}
    /// The callback message containing information about trading volume for the day (live/delayed) from [`crate::client::Client::req_market_data`].
    fn volume(&mut self, req_id: i64, volume: Volume) -> impl std::future::Future {}
    /// The callback message containing information about real-time volume from [`crate::client::Client::req_market_data`].
    fn real_time_volume(&mut self, req_id: i64, volume: RealTimeVolume) -> impl std::future::Future {}
    /// The callback message containing information about the parameters of a market data request from [`crate::client::Client::req_market_data`].
    fn tick_params(
        &mut self,
        req_id: i64,
        min_tick: f64,
        exchange_id: ExchangeId,
        snapshot_permissions: u32,
    ) -> impl std::future::Future {
    }
    /// The callback message containing information about the class of data that will be returned from [`crate::client::Client::req_market_data`].
    fn market_data_class(&mut self, req_id: i64, class: payload::MarketDataClass) -> impl std::future::Future {}
    /// The callback message containing information about updating an existing order book from [`crate::client::Client::req_market_depth`].
    fn update_market_depth(&mut self, req_id: i64, operation: payload::market_depth::Operation) -> impl std::future::Future {}
    /// The callback message containing a complete histogram from [`crate::client::Client::req_histogram_data`].
    fn histogram(
        &mut self,
        req_id: i64,
        histogram: std::collections::HashMap<usize, HistogramEntry>,
    ) -> impl std::future::Future {
    }
    /// The callback message containing historical bar data from [`crate::client::Client::req_historical_bar`].
    fn historical_bars(&mut self, req_id: i64, bars: Vec<Bar>) -> impl std::future::Future {}
    /// The callback message containing an updated historical bar from [`crate::client::Client::req_updating_historical_bar`].
    fn updating_historical_bar(&mut self, req_id: i64, bar: Bar) -> impl std::future::Future {}
    /// The callback message containing a timestamp for the beginning of data for a contract and specified data type from [`crate::client::Client::req_head_timestamp`].
    fn head_timestamp(&mut self, req_id: i64, timestamp: NaiveDateTime) -> impl std::future::Future {}
    /// The callback message containing a vector of historical ticks from [`crate::client::Client::req_historical_ticks`] for [`crate::client::Client::req_tick_by_tick_data`].
    fn historical_ticks(&mut self, req_id: i64, ticks: Vec<Tick>) -> impl std::future::Future {}
    /// The callback message containing a single tick from [`crate::client::Client::req_tick_by_tick_data`].
    fn live_tick(&mut self, req_id: i64, tick: Tick) -> impl std::future::Future {}
    /// The callback message containing account attributes from [`crate::client::Client::req_account_updates`].
    fn account_attribute(&mut self, attribute: Attribute, account_number: String) -> impl std::future::Future {}
    /// The callback message containing information about a single [`Position`] from [`crate::client::Client::req_positions`].
    fn position(&mut self, position: Position) -> impl std::future::Future {}
    /// The callback message containing information about the time at which [`Wrapper::account_attribute`] data is valid.
    fn account_attribute_time(&mut self, time: NaiveTime) -> impl std::future::Future {}
    /// The callback message containing summary information about positions from [`crate::client::Client::req_positions`]
    fn position_summary(&mut self, summary: PositionSummary) -> impl std::future::Future {}
    /// The callback message containing aggregate P&L information from [`crate::client::Client::req_pnl`].
    fn pnl(&mut self, req_id: i64, pnl: Pnl) -> impl std::future::Future {}
    /// The callback message containing P&L information for a single position from [`crate::client::Client::req_single_position_pnl`].
    fn single_position_pnl(&mut self, req_id: i64, pnl: Pnl, position: f64, market_value: f64) -> impl std::future::Future {}
    /// The callback message indicating that all the information for a given account has been received.
    fn account_download_end(&mut self, account_number: String) -> impl std::future::Future {}
    /// The callback message associated with account summary information from [`crate::client::Client::req_account_summary`].
    fn account_summary(&mut self, req_id: i64, account_number: String, summary: TagValue) -> impl std::future::Future {}
    /// The callback message indicating that all the position information has been received.
    fn position_end(&mut self) -> impl std::future::Future {}
    /// The callback message indicating that all the account summary information has been received.
    fn account_summary_end(&mut self, req_id: i64) -> impl std::future::Future {}
    /// The callback message indicating that all the contract information has been received.
    fn contract_data_end(&mut self, req_id: i64) -> impl std::future::Future {}
    /// The callback message indicating that all order information has been received.
    fn open_order_end(&mut self) -> impl std::future::Future {}
    /// The callback message that contains live bar data from [`crate::client::Client::req_real_time_bars`].
    fn real_time_bar(&mut self, req_id: i64, bar: Bar) -> impl std::future::Future {}
}
