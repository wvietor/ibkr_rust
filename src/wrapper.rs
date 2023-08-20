use crate::tick::{
    self, Accessibility, AuctionData, Class, Dividends, ExtremeValue, Ipo, MarkPrice, News,
    OpenInterest, Price, PriceFactor, QuotingExchanges, Rate, RealTimeVolume,
    SecOptionCalculationSource, SecOptionVolume, Size, SummaryVolume, TimeStamp, TradeCount,
    Volatility, Volume, Yield,
};
use chrono::NaiveDateTime;

/// Contains the "callback functions" that correspond to the requests made by a [`crate::client::Client`].
pub trait Wrapper: Send + Sync {
    /// The callback that corresponds to any error that encounters after an API request.
    ///
    /// Errors sent by the TWS are received here.
    fn error(
        &mut self,
        req_id: i64,
        error_code: i64,
        error_string: String,
        advanced_order_reject_json: String,
    );
    /// The callback message that corresponds to [`crate::client::Client::req_current_time`].
    ///
    /// This is TWS's current time. TWS is synchronized with the server (not local computer) using NTP and this function will receive the current time in TWS.
    fn current_time(&mut self, datetime: NaiveDateTime);
    /// The callback message that corresponds to ETF Net Asset Value (NAV) data.
    fn etf_nav(&mut self, req_id: i64, nav: tick::EtfNav);
    /// The callback message that corresponds to price data from [`crate::client::Client::req_market_data`].
    fn price_data(&mut self, req_id: i64, price: Class<Price>);
    /// The callback message that corresponds to size data from [`crate::client::Client::req_market_data`].
    fn size_data(&mut self, req_id: i64, size: Class<Size>);
    /// The callback message that corresponds to the price (in yield terms) data from [`crate::client::Client::req_market_data`].
    fn yield_data(&mut self, req_id: i64, yld: Yield);
    /// The callback message that corresponds to the high/low prices over a period from [`crate::client::Client::req_market_data`]..
    fn extreme_data(&mut self, req_id: i64, value: ExtremeValue);
    /// The callback message that corresponds to the results of options computations (implied volatility, greeks, etc.) from [`crate::client::Client::req_market_data`]..
    fn sec_option_computation(&mut self, req_id: i64, calc: Class<SecOptionCalculationSource>);
    /// The callback message that corresponds to the list of exchanges actively quoting the best bid / best offer / last traded prices from [`crate::client::Client::req_market_data`].
    fn quoting_exchanges(&mut self, req_id: i64, quoting_exchanges: QuotingExchanges);
    /// The callback message that corresponds to the open interest of various derivatives contracts from [`crate::client::Client::req_market_data`].
    fn open_interest(&mut self, req_id: i64, open_interest: OpenInterest);
    /// The callback message that corresponds to volatility data from [`crate::client::Client::req_market_data`].
    fn volatility(&mut self, req_id: i64, vol: Volatility);
    /// The callback message that corresponds to timestamp data from [`crate::client::Client::req_market_data`].
    fn timestamp(&mut self, req_id: i64, timestamp: Class<TimeStamp>);
    /// The callback message that corresponds to auction data from [`crate::client::Client::req_market_data`].
    fn auction(&mut self, req_id: i64, auction: AuctionData);
    /// The callback message associated with mark price data from [`crate::client::Client::req_market_data`].
    fn mark_price(&mut self, req_id: i64, mark: MarkPrice);
    /// The callback message associated with factors / multipliers related to prices from [`crate::client::Client::req_market_data`].
    fn price_factor(&mut self, req_id: i64, factor: PriceFactor);
    /// The callback message associated with the ability to short or trade a security from [`crate::client::Client::req_market_data`].
    fn accessibility(&mut self, req_id: i64, access: Accessibility);
    /// The callback message containing information about dividends from [`crate::client::Client::req_market_data`].
    fn dividends(&mut self, req_id: i64, dividends: Dividends);
    /// The callback message containing news information from [`crate::client::Client::req_market_data`].
    fn news(&mut self, req_id: i64, news: News);
    /// The callback message containing information about IPOs from [`crate::client::Client::req_market_data`].
    fn ipo(&mut self, req_id: i64, ipo: Ipo);
    /// The callback message containing summary information about trading volume throughout a day or 90-day rolling period from [`crate::client::Client::req_market_data`].
    fn summary_volume(&mut self, req_id: i64, volume: SummaryVolume);
    /// The callback message containing information about daily option volume (and average option volume) from [`crate::client::Client::req_market_data`].
    fn sec_option_volume(&mut self, req_id: i64, volume: SecOptionVolume);
    /// The callback message containing information about the number of trades performed in a day from [`crate::client::Client::req_market_data`].
    fn trade_count(&mut self, req_id: i64, trade_count: TradeCount);
    /// The callback message containing information about the rate of trades or volume throughout a day from [`crate::client::Client::req_market_data`].
    fn rate(&mut self, req_id: i64, rate: Rate);
    /// The callback message containing information about trading volume for the day (live/delayed) from [`crate::client::Client::req_market_data`].
    fn volume(&mut self, req_id: i64, volume: Volume);
    /// The callback message containing information about real-time volume from [`crate::client::Client::req_market_data`].
    fn real_time_volume(&mut self, req_id: i64, volume: RealTimeVolume);
}
