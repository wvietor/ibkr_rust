use crate::payload::CalculationResult;
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
/// The types of ticks related to ETF Net Asset Value (NAV).
pub enum EtfNav {
    /// Today's closing price of ETF's Net Asset Value (NAV). Calculation is based on prices of ETF's underlying securities.
    Close(f64),
    /// Yesterday's closing price of ETF's Net Asset Value (NAV). Calculation is based on prices of ETF's underlying securities.
    PriorClose(f64),
    /// The bid price of ETF's Net Asset Value (NAV). Calculation is based on prices of ETF's underlying securities.
    Bid(f64),
    /// The ask price of ETF's Net Asset Value (NAV). Calculation is based on prices of ETF's underlying securities.
    Ask(f64),
    /// The last price of Net Asset Value (NAV). For ETFs: Calculation is based on prices of ETF's underlying securities. For NextShares: Value is provided by NASDAQ.
    Last(f64),
    /// ETF Nav Last for Frozen data.
    FrozenLast(f64),
    /// The high price of ETF's Net Asset Value (NAV)
    High(f64),
    /// The low price of ETF's Net Asset Value (NAV)
    Low(f64),
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
/// The types of ticks related to price data.
pub enum Price {
    /// Highest priced bid for the contract.
    Bid(f64),
    /// Lowest price offer on the contract.
    Ask(f64),
    /// Last price at which the contract traded (does not include some trades in RTVolume).
    Last(f64),
    /// High price for the day.
    High(f64),
    /// Low price for the day.
    Low(f64),
    /// The last available closing price for the previous day. For US Equities, we use corporate action processing to get the closing price, so the close price is adjusted to reflect forward and reverse splits and cash and stock dividends.
    Close(f64),
    /// Current session's opening price. Before open will refer to previous day. The official opening price requires a market data subscription to the native exchange of the instrument.
    Open(f64),
    /// Last Regular Trading Hours traded price.
    LastRthTrade(f64),
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
/// The types of ticks related to size data.
pub enum Size {
    /// Number of contracts or lots offered at the bid price.
    Bid(f64),
    /// Number of contracts or lots offered at the ask price.
    Ask(f64),
    /// Number of contracts or lots traded at the last price.
    Last(f64),
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
/// The types of ticks related to yield data.
pub enum Yield {
    /// Implied yield of the bond if it is purchased at the current bid.
    Bid(f64),
    /// Implied yield of the bond if it is purchased at the current ask.
    Ask(f64),
    /// Implied yield of the bond if it is purchased at the last price.
    Last(f64),
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
/// Represents the various periods of trailing extreme value.
pub enum Period {
    /// A value over a 13-week period.
    ThirteenWeek(f64),
    /// A value over a 26-week period.
    TwentySixWeek(f64),
    /// A value over a 52-week period.
    FiftyTwoWeek(f64),
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
/// Represents the two types of extreme values.
pub enum ExtremeValue {
    /// The lowest value.
    Low(Period),
    /// The highest value.
    High(Period),
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
/// The various base prices that can be used to perform option computations.
pub enum SecOptionCalculationSource {
    /// Use the bid price to perform the computations.
    Bid(SecOptionCalculations),
    /// Use the ask price to perform the computations.
    Ask(SecOptionCalculations),
    /// Use the last price to perform the computations.
    Last(SecOptionCalculations),
    /// Use the IBKR options model price to perform the computations.
    Model(SecOptionCalculations),
    /// Use a custom price to perform the computations.
    Custom(SecOptionCalculations),
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
/// A collection of option calculations.
pub enum SecOptionCalculations {
    /// Return-based computations
    ReturnBased(SecOptionCalculationResults),
    /// Price-based computations
    PriceBased(SecOptionCalculationResults),
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
/// The core results of an option calculation.
pub struct SecOptionCalculationResults {
    /// The implied volatility calculated by the TWS option modeler, using the specified tick type value.
    pub implied_volatility: CalculationResult,
    /// The option delta value.
    pub delta: CalculationResult,
    /// The option price.
    pub price: CalculationResult,
    /// The present value of dividends expected on the option's underlying.
    pub dividend_present_value: CalculationResult,
    /// The option gamma value.
    pub gamma: CalculationResult,
    /// The option vega value.
    pub vega: CalculationResult,
    /// The option theta value.
    pub theta: CalculationResult,
    /// The price of the underlying.
    pub underlying_price: CalculationResult,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
/// The exchanges posting the best bid / best offer / last traded prices.
pub enum QuotingExchanges {
    /// For stock and options, identifies the exchange(s) posting the bid price. See Component Exchanges.
    Bid(Vec<char>),
    /// For stock and options, identifies the exchange(s) posting the ask price. See Component Exchanges.
    Ask(Vec<char>),
    /// Exchange of last traded price.
    Last(Vec<char>),
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
/// Represents the possible open interest callbacks.
pub enum OpenInterest {
    /// Call option open interest.
    SecOptionCall(f64),
    /// Put option open interest.
    SecOptionPut(f64),
    /// Total number of outstanding futures contracts (TWS v965+). *HSI open interest requested with generic tick 101
    SecFuture(f64),
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
/// The types of volatility callbacks.
pub enum Volatility {
    /// The 30-day historical volatility (currently for stocks).
    SecOptionHistorical(f64),
    /// A prediction of how volatile an underlying will be in the future. The IB 30-day volatility is the at-market volatility estimated for a maturity thirty calendar days forward of the current trading day, and is based on option prices from two consecutive expiration months.
    SecOptionImplied(f64),
    /// 30-day real time historical volatility.
    RealTimeHistorical(f64),
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, PartialEq, Eq)]
/// Represents a timestamp callback.
pub enum TimeStamp {
    /// Time of the last trade (in UNIX time).
    Last(NaiveDateTime),
    /// Timestamp (in Unix ms time) of last trade returned with regulatory snapshot.
    Regulatory(NaiveDateTime),
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
/// Represents a callback that relates to auction data, auction prices, etc.
pub enum AuctionData {
    /// The number of shares that would trade if no new orders were received and the auction were held now.
    Volume(f64),
    /// The price at which the auction would occur if no new orders were received and the auction were held now- the indicative price for the auction. Typically received after Auction imbalance (tick type 36)
    Price(f64),
    /// The number of unmatched shares for the next auction; returns how many more shares are on one side of the auction than the other. Typically received after Auction Volume (tick type 34)
    Imbalance(f64),
    /// The imbalance that is used to determine which at-the-open or at-the-close orders can be entered following the publishing of the regulatory imbalance.
    Regulatory(f64),
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
/// Represents a callback containing to mark prices.
pub enum MarkPrice {
    /// The mark price is the current theoretical calculated value of an instrument. Since it is a calculated value, it will typically have many digits of precision.
    Standard(f64),
    /// Slower mark price update used in system calculations
    Slow(f64),
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
/// A callback containing real-time volume information that is updated quickly.
pub enum RealTimeVolume {
    /// Last trade details (Including both "Last" and "Unreportable Last" trades).
    All(RealTimeVolumeBase),
    /// Last trade details that excludes "Unreportable Trades".
    Trades(RealTimeVolumeBase),
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
/// A helper struct that represents all the information returned in [`RealTimeVolume`].
pub struct RealTimeVolumeBase {
    /// The last trade's price.
    pub(crate) last_price: f64,
    /// The last trade's size.
    pub(crate) last_size: f64,
    /// The last trade's time.
    pub(crate) last_time: NaiveDateTime,
    /// The current day's total traded volume.
    pub(crate) day_volume: f64,
    /// The current day's Volume Weighted Average Price (VWAP).
    pub(crate) vwap: f64,
    /// When true, the trade was filled by a single market maker.
    pub(crate) single_mm: bool,
}

/// A callback containing volume information that is not updated as quickly as [`RealTimeVolume`]
pub type Volume = Class<f64>;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
/// A callback containing information about trades and volume on a per-minute basis.
pub enum Rate {
    /// Trade count per minute.
    Trade(f64),
    /// Volume per minute.
    Volume(f64),
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
/// A callback containing information about option volume.
pub enum SecOptionVolume {
    /// Call option volume for the trading day.
    Call(f64),
    /// Put option volume for the trading day.
    Put(f64),
    /// Average volume of the corresponding option contracts.
    Average(f64),
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
/// A callback containing information about short-term volume
pub enum SummaryVolume {
    /// The past three minutes volume. Interpolation may be applied. For stocks only.
    ThreeMinutes(f64),
    /// The past five minutes volume. Interpolation may be applied. For stocks only.
    FiveMinutes(f64),
    /// The past ten minutes volume. Interpolation may be applied. For stocks only.
    TenMinutes(f64),
    /// The average daily trading volume over 90 days. Multiplier of 100. For stocks only.
    NinetyDayAverage(f64),
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
/// A callback containing information that relates the price of an instrument to some reference value.
pub enum PriceFactor {
    /// The bond factor is a number that indicates the ratio of the current bond principal to the original principal.
    BondFactorMultiplier(f64),
    /// The number of points that the index is over the cash index.
    IndexFuturePremium(f64),
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
/// A callback containing information about a security's accessibility for shorting and trading.
pub enum Accessibility {
    /// Number of shares available to short (TWS Build 974+ is required)
    ShortableShares(f64),
    /// Describes the level of difficulty with which the contract can be sold short.
    Shortable(f64),
    /// Indicates if a contract is halted.
    Halted(f64),
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
/// A callback related to IPO information.
pub enum Ipo {
    /// Midpoint is calculated based on IPO price range.
    Estimated(f64),
    /// Final price for IPO.
    Final(f64),
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
/// Information about dividends
pub struct Dividends {
    /// The sum of dividends for the past 12 months.
    pub trailing_year: f64,
    /// The sum of dividends for the next 12 months.
    pub forward_year: f64,
    /// The next single dividend date and amount.
    pub next_dividend: (NaiveDate, f64),
}

/// A contract's news feed
pub type News = String;

/// Trade count for the day.
pub type TradeCount = f64;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
/// The two classes of data that can be returned for various market data requests.
pub enum Class<P: indicators::Valid> {
    /// Live data that requires a live data subscription.
    Live(P),
    /// Data that is delayed by at least 15-20 minutes.
    Delayed(P),
}

pub(crate) mod indicators {
    pub trait Valid {}

    impl Valid for super::Price {}
    impl Valid for super::Size {}
    impl Valid for super::SecOptionCalculationSource {}
    impl Valid for super::TimeStamp {}

    impl Valid for f64 {}
}
