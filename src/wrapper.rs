#![allow(async_fn_in_trait)]
use crate::account::{Attribute, TagValue};
use crate::payload::{Pnl, Position, PositionSummary};
use crate::{
    payload::{self, Bar, ExchangeId, HistogramEntry, Tick},
    tick::{
        self, Accessibility, AuctionData, Class, Dividends, ExtremeValue, Ipo, MarkPrice, News,
        OpenInterest, Price, PriceFactor, QuotingExchanges, Rate, RealTimeVolume,
        SecOptionCalculationSource, SecOptionVolume, Size, SummaryVolume, TimeStamp, TradeCount,
        Volatility, Volume, Yield,
    },
};
use chrono::{NaiveDateTime, NaiveTime};

/// A standalone [`Wrapper`], which defines an application in which the client and wrapper are each
/// on their own thread. This means that trading behavior is defined in the scope in which the
/// client is created, not in the wrapper methods.
///
/// Note that this approach requires inter-thread communication between the wrapper and the client.
pub trait Standalone: Wrapper + Send + Sync {}

/// An integrated [`Wrapper`], which defines an application in which the client and wrapper share
/// the same thread. This allows the client to be invoked directly by the wrapper, without the
/// need for inter-thread communication.
pub trait Integrated: Wrapper {
    /// Attach a reference to the client to the underlying wrapper struct. This allows the wrapper
    /// methods to access the client using the [`Integrated::client`] method to respond to trading
    /// information.
    fn attach_client(&mut self, client: crate::client::ActiveClient);
    /// Return a reference to the attached client.
    fn client(&mut self) -> &mut crate::client::ActiveClient;
    /// The main entry point for an [`Integrated`] application. This method is called immediately
    /// after the client is successfully connected.
    async fn main(&mut self);
}

/// Contains the "callback functions" that correspond to the requests made by a [`crate::client::Client`].
pub trait Wrapper {
    /// The callback that corresponds to any error that encounters after an API request.
    ///
    /// Errors sent by the TWS are received here.
    async fn error(
        &mut self,
        req_id: i64,
        error_code: i64,
        error_string: String,
        advanced_order_reject_json: String,
    );
    /// The callback message that corresponds to [`crate::client::Client::req_current_time`].
    ///
    /// This is TWS's current time. TWS is synchronized with the server (not local computer) using NTP and this function will receive the current time in TWS.
    async fn current_time(&mut self, datetime: NaiveDateTime);
    /// The callback message that corresponds to ETF Net Asset Value (NAV) data.
    async fn etf_nav(&mut self, req_id: i64, nav: tick::EtfNav);
    /// The callback message that corresponds to price data from [`crate::client::Client::req_market_data`].
    async fn price_data(&mut self, req_id: i64, price: Class<Price>);
    /// The callback message that corresponds to size data from [`crate::client::Client::req_market_data`].
    async fn size_data(&mut self, req_id: i64, size: Class<Size>);
    /// The callback message that corresponds to the price (in yield terms) data from [`crate::client::Client::req_market_data`].
    async fn yield_data(&mut self, req_id: i64, yld: Yield);
    /// The callback message that corresponds to the high/low prices over a period from [`crate::client::Client::req_market_data`]..
    async fn extreme_data(&mut self, req_id: i64, value: ExtremeValue);
    /// The callback message that corresponds to the results of options computations (implied volatility, greeks, etc.) from [`crate::client::Client::req_market_data`]..
    async fn sec_option_computation(
        &mut self,
        req_id: i64,
        calc: Class<SecOptionCalculationSource>,
    );
    /// The callback message that corresponds to the list of exchanges actively quoting the best bid / best offer / last traded prices from [`crate::client::Client::req_market_data`].
    async fn quoting_exchanges(&mut self, req_id: i64, quoting_exchanges: QuotingExchanges);
    /// The callback message that corresponds to the open interest of various derivatives contracts from [`crate::client::Client::req_market_data`].
    async fn open_interest(&mut self, req_id: i64, open_interest: OpenInterest);
    /// The callback message that corresponds to volatility data from [`crate::client::Client::req_market_data`].
    async fn volatility(&mut self, req_id: i64, vol: Volatility);
    /// The callback message that corresponds to timestamp data from [`crate::client::Client::req_market_data`].
    async fn timestamp(&mut self, req_id: i64, timestamp: Class<TimeStamp>);
    /// The callback message that corresponds to auction data from [`crate::client::Client::req_market_data`].
    async fn auction(&mut self, req_id: i64, auction: AuctionData);
    /// The callback message associated with mark price data from [`crate::client::Client::req_market_data`].
    async fn mark_price(&mut self, req_id: i64, mark: MarkPrice);
    /// The callback message associated with factors / multipliers related to prices from [`crate::client::Client::req_market_data`].
    async fn price_factor(&mut self, req_id: i64, factor: PriceFactor);
    /// The callback message associated with the ability to short or trade a security from [`crate::client::Client::req_market_data`].
    async fn accessibility(&mut self, req_id: i64, access: Accessibility);
    /// The callback message containing information about dividends from [`crate::client::Client::req_market_data`].
    async fn dividends(&mut self, req_id: i64, dividends: Dividends);
    /// The callback message containing news information from [`crate::client::Client::req_market_data`].
    async fn news(&mut self, req_id: i64, news: News);
    /// The callback message containing information about IPOs from [`crate::client::Client::req_market_data`].
    async fn ipo(&mut self, req_id: i64, ipo: Ipo);
    /// The callback message containing summary information about trading volume throughout a day or 90-day rolling period from [`crate::client::Client::req_market_data`].
    async fn summary_volume(&mut self, req_id: i64, volume: SummaryVolume);
    /// The callback message containing information about daily option volume (and average option volume) from [`crate::client::Client::req_market_data`].
    async fn sec_option_volume(&mut self, req_id: i64, volume: SecOptionVolume);
    /// The callback message containing information about the number of trades performed in a day from [`crate::client::Client::req_market_data`].
    async fn trade_count(&mut self, req_id: i64, trade_count: TradeCount);
    /// The callback message containing information about the rate of trades or volume throughout a day from [`crate::client::Client::req_market_data`].
    async fn rate(&mut self, req_id: i64, rate: Rate);
    /// The callback message containing information about trading volume for the day (live/delayed) from [`crate::client::Client::req_market_data`].
    async fn volume(&mut self, req_id: i64, volume: Volume);
    /// The callback message containing information about real-time volume from [`crate::client::Client::req_market_data`].
    async fn real_time_volume(&mut self, req_id: i64, volume: RealTimeVolume);
    /// The callback message containing information about the parameters of a market data request from [`crate::client::Client::req_market_data`].
    async fn tick_params(
        &mut self,
        req_id: i64,
        min_tick: f64,
        exchange_id: ExchangeId,
        snapshot_permissions: u32,
    );
    /// The callback message containing information about the class of data that will be returned from [`crate::client::Client::req_market_data`].
    async fn market_data_class(&mut self, req_id: i64, class: payload::MarketDataClass);
    /// The callback message containing information about updating an existing order book from [`crate::client::Client::req_market_depth`].
    async fn update_market_depth(
        &mut self,
        req_id: i64,
        operation: payload::market_depth::Operation,
    );
    /// The callback message containing a complete histogram from [`crate::client::Client::req_histogram_data`].
    async fn histogram(
        &mut self,
        req_id: i64,
        histogram: std::collections::HashMap<usize, HistogramEntry>,
    );
    /// The callback message containing historical bar data from [`crate::client::Client::req_historical_bar`].
    async fn historical_bars(&mut self, req_id: i64, bars: Vec<Bar>);
    /// The callback message containing an updated historical bar from [`crate::client::Client::req_updating_historical_bar`].
    async fn updating_historical_bar(&mut self, req_id: i64, bar: Bar);
    /// The callback message containing a timestamp for the beginning of data for a contract and specified data type from [`crate::client::Client::req_head_timestamp`].
    async fn head_timestamp(&mut self, req_id: i64, timestamp: NaiveDateTime);
    /// The callback message containing a vector of historical ticks from [`crate::client::Client::req_historical_ticks`] for [`crate::client::Client::req_tick_by_tick_data`].
    async fn historical_ticks(&mut self, req_id: i64, ticks: Vec<Tick>);
    /// The callback message containing a single tick from [`crate::client::Client::req_tick_by_tick_data`].
    async fn live_tick(&mut self, req_id: i64, tick: Tick);
    /// The callback message containing account attributes from [`crate::client::Client::req_account_updates`].
    async fn account_attribute(&mut self, attribute: Attribute, account_number: String);
    /// The callback message containing information about a single [`Position`] from [`crate::client::Client::req_positions`].
    async fn position(&mut self, position: Position);
    /// The callback message containing information about the time at which [`Wrapper::account_attribute`] data is valid.
    async fn account_attribute_time(&mut self, time: NaiveTime);
    /// The callback message containing summary information about positions from [`crate::client::Client::req_positions`]
    async fn position_summary(&mut self, summary: PositionSummary);
    /// The callback message containing aggregate P&L information from [`crate::client::Client::req_pnl`].
    async fn pnl(&mut self, req_id: i64, pnl: Pnl);
    /// The callback message containing P&L information for a single position from [`crate::client::Client::req_single_position_pnl`].
    async fn single_position_pnl(
        &mut self,
        req_id: i64,
        pnl: Pnl,
        position: f64,
        market_value: f64,
    );
    /// The callback message indicating that all the information for a given account has been received.
    async fn account_download_end(&mut self, account_number: String);
    /// The callback message associated with account summary information from [`crate::client::Client::req_account_summary`].
    async fn account_summary(&mut self, req_id: i64, account_number: String, summary: TagValue);
    /// The callback message indicating that all the position information has been received.
    async fn position_end(&mut self);
    /// The callback message indicating that all the account summary information has been received.
    async fn account_summary_end(&mut self, req_id: i64);
    /// The callback message indicating that all the contract information has been received.
    async fn contract_data_end(&mut self, req_id: i64);
    /// The callback message indicating that all order information has been received.
    async fn open_order_end(&mut self);
    /// The callback message that contains live bar data from [`crate::client::Client::req_real_time_bars`].
    async fn real_time_bar(&mut self, req_id: i64, bar: Bar);
}
