use std::future::Future;

use chrono::{DateTime, NaiveTime, Utc};
use ibapi_macros::debug_trait;

use crate::account::{Attribute, TagValue};
use crate::client::ActiveClient;
use crate::contract::{Contract, Proxy};
use crate::execution::Execution;
use crate::payload::{
    self, Bar, ExchangeId, HistogramEntry, OrderStatus, Pnl, PnlSingle, Position, PositionSummary,
    TickData,
};
use crate::tick::{
    self, Accessibility, AuctionData, Class, Dividends, ExtremeValue, Ipo, MarkPrice, News,
    OpenInterest, Price, PriceFactor, QuotingExchanges, Rate, RealTimeVolume,
    SecOptionCalculationSource, SecOptionVolume, Size, SummaryVolume, TimeStamp, TradeCount,
    Volatility, Volume, Yield,
};

// todo! Updated Bar payload api to make it more clear that BidAsk callback isn't just a "normal Bar"

/// Re-export of [`tokio_util::sync::CancellationToken`]
pub type CancelToken = tokio_util::sync::CancellationToken;

#[allow(clippy::module_name_repetitions)]
#[trait_variant::make(Wrapper: Send)]
#[debug_trait]
/// Contains the "callback functions" that correspond to the requests made by a [`crate::client::Client`].
pub trait LocalWrapper {
    /// The callback that corresponds to any error that encounters after an API request.
    ///
    /// Errors sent by the TWS are received here.
    fn error(
        &mut self,
        req_id: i64,
        error_code: i64,
        error_string: String,
        advanced_order_reject_json: String,
    ) -> impl Future {
    }
    /// The callback message that corresponds to [`crate::client::Client::req_current_time`].
    ///
    /// This is TWS's current time. TWS is synchronized with the server (not local computer) using NTP and this function will receive the current time in TWS.
    fn current_time(&mut self, req_id: i64, datetime: DateTime<Utc>) -> impl Future {}
    /// The callback message that corresponds to ETF Net Asset Value (NAV) data.
    fn etf_nav(&mut self, req_id: i64, nav: tick::EtfNav) -> impl Future {}
    /// The callback message that corresponds to price data from [`crate::client::Client::req_market_data`].
    fn price_data(&mut self, req_id: i64, price: Class<Price>) -> impl Future {}
    /// The callback message that corresponds to size data from [`crate::client::Client::req_market_data`].
    fn size_data(&mut self, req_id: i64, size: Class<Size>) -> impl Future {}
    /// The callback message that corresponds to the price (in yield terms) data from [`crate::client::Client::req_market_data`].
    fn yield_data(&mut self, req_id: i64, yld: Yield) -> impl Future {}
    /// The callback message that corresponds to the high/low prices over a period from [`crate::client::Client::req_market_data`].
    fn extreme_data(&mut self, req_id: i64, value: ExtremeValue) -> impl Future {}
    /// The callback message that corresponds to the results of options computations (implied volatility, greeks, etc.) from [`crate::client::Client::req_market_data`].
    fn sec_option_computation(
        &mut self,
        req_id: i64,
        calc: Class<SecOptionCalculationSource>,
    ) -> impl Future {
    }
    /// The callback message that corresponds to the list of exchanges actively quoting the best bid / best offer / last traded prices from [`crate::client::Client::req_market_data`].
    fn quoting_exchanges(
        &mut self,
        req_id: i64,
        quoting_exchanges: QuotingExchanges,
    ) -> impl Future {
    }
    /// The callback message that corresponds to the open interest of various derivatives contracts from [`crate::client::Client::req_market_data`].
    fn open_interest(&mut self, req_id: i64, open_interest: OpenInterest) -> impl Future {}
    /// The callback message that corresponds to volatility data from [`crate::client::Client::req_market_data`].
    fn volatility(&mut self, req_id: i64, vol: Volatility) -> impl Future {}
    /// The callback message that corresponds to timestamp data from [`crate::client::Client::req_market_data`].
    fn timestamp(&mut self, req_id: i64, timestamp: Class<TimeStamp>) -> impl Future {}
    /// The callback message that corresponds to auction data from [`crate::client::Client::req_market_data`].
    fn auction(&mut self, req_id: i64, auction: AuctionData) -> impl Future {}
    /// The callback message associated with mark price data from [`crate::client::Client::req_market_data`].
    fn mark_price(&mut self, req_id: i64, mark: MarkPrice) -> impl Future {}
    /// The callback message associated with factors / multipliers related to prices from [`crate::client::Client::req_market_data`].
    fn price_factor(&mut self, req_id: i64, factor: PriceFactor) -> impl Future {}
    /// The callback message associated with the ability to short or trade a security from [`crate::client::Client::req_market_data`].
    fn accessibility(&mut self, req_id: i64, access: Accessibility) -> impl Future {}
    /// The callback message containing information about dividends from [`crate::client::Client::req_market_data`].
    fn dividends(&mut self, req_id: i64, dividends: Dividends) -> impl Future {}
    /// The callback message containing news information from [`crate::client::Client::req_market_data`].
    fn news(&mut self, req_id: i64, news: News) -> impl Future {}
    /// The callback message containing information about IPOs from [`crate::client::Client::req_market_data`].
    fn ipo(&mut self, req_id: i64, ipo: Ipo) -> impl Future {}
    /// The callback message containing summary information about trading volume throughout a day or 90-day rolling period from [`crate::client::Client::req_market_data`].
    fn summary_volume(&mut self, req_id: i64, volume: SummaryVolume) -> impl Future {}
    /// The callback message containing information about daily option volume (and average option volume) from [`crate::client::Client::req_market_data`].
    fn sec_option_volume(&mut self, req_id: i64, volume: SecOptionVolume) -> impl Future {}
    /// The callback message containing information about the number of trades performed in a day from [`crate::client::Client::req_market_data`].
    fn trade_count(&mut self, req_id: i64, trade_count: TradeCount) -> impl Future {}
    /// The callback message containing information about the rate of trades or volume throughout a day from [`crate::client::Client::req_market_data`].
    fn rate(&mut self, req_id: i64, rate: Rate) -> impl Future {}
    /// The callback message containing information about trading volume for the day (live/delayed) from [`crate::client::Client::req_market_data`].
    fn volume(&mut self, req_id: i64, volume: Volume) -> impl Future {}
    /// The callback message containing information about real-time volume from [`crate::client::Client::req_market_data`].
    fn real_time_volume(&mut self, req_id: i64, volume: RealTimeVolume) -> impl Future {}
    /// The callback message containing information about the parameters of a market data request from [`crate::client::Client::req_market_data`].
    fn tick_params(
        &mut self,
        req_id: i64,
        min_tick: f64,
        exchange_id: ExchangeId,
        snapshot_permissions: u32,
    ) -> impl Future {
    }
    /// The callback message containing information about the class of data that will be returned from [`crate::client::Client::req_market_data`].
    fn market_data_class(&mut self, req_id: i64, class: payload::MarketDataClass) -> impl Future {}
    /// The callback message containing information about updating an existing order book from [`crate::client::Client::req_market_depth`].
    fn update_market_depth(
        &mut self,
        req_id: i64,
        operation: payload::market_depth::Operation,
    ) -> impl Future {
    }
    /// The callback message containing a complete histogram from [`crate::client::Client::req_histogram_data`].
    fn histogram(
        &mut self,
        req_id: i64,
        histogram: std::collections::HashMap<usize, HistogramEntry>,
    ) -> impl Future {
    }
    /// The callback message containing historical bar data from [`crate::client::Client::req_historical_bar`].
    fn historical_bars(&mut self, req_id: i64, bars: Vec<Bar>) -> impl Future {}
    /// The callback message containing an updated historical bar from [`crate::client::Client::req_updating_historical_bar`].
    fn updating_historical_bar(&mut self, req_id: i64, bar: Bar) -> impl Future {}
    /// The callback message containing a timestamp for the beginning of data for a contract and specified data type from [`crate::client::Client::req_head_timestamp`].
    fn head_timestamp(&mut self, req_id: i64, timestamp: DateTime<Utc>) -> impl Future {}
    /// The callback message containing a vector of historical ticks from [`crate::client::Client::req_historical_ticks`] for [`crate::client::Client::req_tick_by_tick_data`].
    fn historical_ticks(&mut self, req_id: i64, ticks: Vec<TickData>) -> impl Future {}
    /// The callback message containing a single tick from [`crate::client::Client::req_tick_by_tick_data`].
    fn live_tick(&mut self, req_id: i64, tick: TickData) -> impl Future {}
    /// The callback message containing account attributes from [`crate::client::Client::req_account_updates`].
    fn account_attribute(&mut self, attribute: Attribute, account_number: String) -> impl Future {}
    /// The callback message containing information about a single [`Position`] from [`crate::client::Client::req_account_updates`].
    fn portfolio_value(&mut self, position: Position) -> impl Future {}
    /// The callback message containing information about the time at which account attribute data is valid.
    fn account_attribute_time(&mut self, time: NaiveTime) -> impl Future {}
    /// The callback message containing summary information about positions from [`crate::client::Client::req_positions`].
    fn position_summary(&mut self, summary: PositionSummary) -> impl Future {}
    /// The callback message containing aggregate P&L information from [`crate::client::Client::req_pnl`].
    fn pnl(&mut self, req_id: i64, pnl: Pnl) -> impl Future {}
    /// The callback message that contains information about PNL of a single position from ['crate::client::Client::req_pnl_single'].
    fn single_position_pnl(&mut self, req_id: i64, pnl: PnlSingle) -> impl Future {}
    /// The callback message indicating that all the information for a given account has been received.
    fn account_download_end(&mut self, account_number: String) -> impl Future {}
    /// The callback message associated with account summary information from [`crate::client::Client::req_account_summary`].
    fn account_summary(
        &mut self,
        req_id: i64,
        account_number: String,
        summary: TagValue,
    ) -> impl Future {
    }
    /// The callback message indicating that all the position information has been received.
    fn position_end(&mut self) -> impl Future {}
    /// The callback message indicating that all the account summary information has been received.
    fn account_summary_end(&mut self, req_id: i64) -> impl Future {}
    /// The callback message indicating that all the contract information has been received.
    fn contract_data_end(&mut self, req_id: i64) -> impl Future {}
    /// The callback message indicating that all order information has been received.
    fn open_order_end(&mut self) -> impl Future {}
    /// The callback message that contains live bar data from [`crate::client::Client::req_real_time_bars`].
    fn real_time_bar(&mut self, req_id: i64, bar: Bar) -> impl Future {}
    /// The callback message that contains order status data from [`crate::client::Client::req_place_order`].
    fn order_status(&mut self, status: OrderStatus) -> impl Future {}
    /// The callback message that contains information about currently open orders from [`crate::client::Client::req_place_order`].
    fn open_order<E>(
        &mut self,
        order_id: i64,
        proxy: Proxy<Contract, E>,
        client_id: i64,
        parent_id: Option<i64>,
        permanent_id: i64,
    ) -> impl Future {
    }
    /// The callback message that contains information about an execution.
    fn execution(&mut self, req_id: i64, execution: Execution) -> impl Future {}
}

#[trait_variant::make(Recurring: Send)]
/// A trait with a single method that will be called in the main message loop.
pub trait LocalRecurring {
    /// A method that is called in the body of the main message loop. The method is called in
    /// a [`tokio::select!`] block.  
    ///
    /// This method needs to have a .await point, or the entire program will block.
    /// See [`tokio::task::yield_now`].
    fn cycle(&mut self) -> impl Future<Output = ()>;
}

impl Recurring for () {
    async fn cycle(&mut self) {
        tokio::task::yield_now().await;
    }
}

/// An initializer for a new [`LocalWrapper`].
pub trait LocalInitializer {
    /// The wrapper
    type Wrap<'c>: LocalWrapper;
    /// The recurring struct, which mediates repeated calls in the main client loop.
    type Recur<'c>: LocalRecurring;
    /// The method to build the wrapper
    fn build(
        self,
        client: &mut ActiveClient,
        cancel_loop: CancelToken,
    ) -> impl Future<Output = (Self::Wrap<'_>, Self::Recur<'_>)>;
}

/// An initializer for a new [`Wrapper`].
pub trait Initializer: Send {
    /// The wrapper
    type Wrap<'c>: Wrapper;
    /// The recurring struct, which mediates repeated calls in the main client loop.
    type Recur<'c>: Recurring;
    /// The method to build the wrapper
    fn build(
        self,
        client: &mut ActiveClient,
        cancel_loop: CancelToken,
    ) -> impl Future<Output = (Self::Wrap<'_>, Self::Recur<'_>)> + Send;
}

impl<I: Initializer> LocalInitializer for I {
    type Wrap<'c> = <I as Initializer>::Wrap<'c>;
    type Recur<'c> = <I as Initializer>::Recur<'c>;

    fn build(
        self,
        client: &mut ActiveClient,
        cancel_loop: CancelToken,
    ) -> impl Future<Output = (Self::Wrap<'_>, Self::Recur<'_>)> {
        <I as Initializer>::build(self, client, cancel_loop)
    }
}
