macro_rules! make_valid {
    ($($name: ident),*) => {
        pub(crate) mod indicators {
            use serde::Serialize;
use super::{$($name,)*};

            /// A simple indicator trait to ensure that no foreign types can be implemented as valid data types.
            pub trait Valid: Serialize + Copy + Clone {}

            $(
                impl Valid for $name {}
            )*
        }

        /// Implemented by all valid data types for a given security. In particular,
        /// if a type `D` implements [`DataType<S>`], then `D` is a valid data type for `S`.
        pub trait DataType<S: crate::contract::Security>: Send + Sync + indicators::Valid {}
    };
}

macro_rules! impl_data_type_docs {
    (($first: ident $(, $rest: ident)+)) => {
        concat!("[`", stringify!($first), "`], ", impl_data_type_docs!(($($rest),*)) )
    };
    (($only: ident)) => { concat!("[`", stringify!($only), "`]") }
}

macro_rules! impl_data_type {
    (($($d_name: ident),*); $s_names: tt) => {
        $(
            impl_data_type!($d_name; $s_names);
        )*
    };
    (($($d_name: ident),*); $s_names: tt; $enum_name: ident) => {
        #[doc = concat!(
            "A helper enum to hold data types valid for particular securities: ",
            impl_data_type_docs!($s_names)
        )]
        #[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
        pub enum $enum_name {
            $(
            #[doc = concat!(stringify!($d_name), " data")]
            $d_name($d_name),
            )*
        }

        impl indicators::Valid for $enum_name {}

        impl_data_type!(($enum_name, $($d_name),*); $s_names);
    };
    ($d_name: ident; ($($s_name: ident),*)) => {
        $(
            impl DataType<$s_name> for $d_name {}
        )*
    };
}

/// Contains types and traits used by [`crate::client::Client::req_historical_bar`].
pub mod historical_bar {
    use chrono_tz::Tz;
    use ibapi_macros::typed_variants;
    use serde::{Deserialize, Serialize, Serializer};

    use crate::contract::{Commodity, Crypto, Forex, Index, SecFuture, SecOption, Stock};

    // === Type definitions ===

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    /// The last time for which bar data will be returned.
    pub enum EndDateTime {
        /// The present moment.
        Present,
        /// Some date and time in the past.
        Past(chrono::DateTime<Tz>),
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    /// The span of dates and times over which bars will be returned.
    pub enum Duration {
        /// Some number of seconds.
        Second(u32),
        /// Some number of days.
        Day(u32),
        /// Some number of weeks.
        Week(u32),
        /// Some number of months.
        Month(u32),
        /// Some number of years.
        Year(u32),
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    /// The size of each bar to be returned.
    pub enum Size {
        /// Some valid number of seconds.
        Seconds(SecondSize),
        /// Some valid number of minutes.
        Minutes(MinuteSize),
        /// Some valid number of hours.
        Hours(HourSize),
        /// One day
        Day,
        /// One week
        Week,
        /// One month
        Month,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    /// The valid sizes of any bar with second granularity.
    pub enum SecondSize {
        /// One second.
        One = 1,
        /// Five seconds.
        Five = 5,
        /// Ten seconds.
        Ten = 10,
        /// Fifteen seconds.
        Fifteen = 15,
        /// Thirty seconds.
        Thirty = 30,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    /// The valid sizes of any bar with minute granularity.
    pub enum MinuteSize {
        /// One minute.
        One = 1,
        /// Two minutes.
        Two = 2,
        /// Three minutes.
        Three = 3,
        /// Five minutes.
        Five = 5,
        /// Ten minutes.
        Ten = 10,
        /// Fifteen minutes.
        Fifteen = 15,
        /// Twenty minutes.
        Twenty = 20,
        /// Thirty minutes.
        Thirty = 30,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    /// The valid sizes of any bar with hour granularity.
    pub enum HourSize {
        /// One hour.
        One = 1,
        /// Two hours.
        Two = 2,
        /// Three hours.
        Three = 3,
        /// Four hours.
        Four = 4,
        /// Eight hours.
        Eight = 8,
    }

    // === Type implementations ===

    impl Serialize for EndDateTime {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match *self {
                Self::Past(dt) => Some(dt.to_utc().format("%Y%m%d-%T").to_string()),
                Self::Present => None,
            }
            .serialize(serializer)
        }
    }

    impl Serialize for Duration {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match *self {
                Self::Second(s) => format!("{s} S"),
                Self::Day(d) => format!("{d} D"),
                Self::Week(w) => format!("{w} W"),
                Self::Month(m) => format!("{m} M"),
                Self::Year(y) => format!("{y} Y"),
            }
            .serialize(serializer)
        }
    }

    impl Serialize for Size {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match *self {
                Self::Seconds(s) => match s {
                    SecondSize::One => "1 secs",
                    SecondSize::Five => "5 secs",
                    SecondSize::Ten => "10 secs",
                    SecondSize::Fifteen => "15 secs",
                    SecondSize::Thirty => "30 secs",
                },
                Self::Minutes(m) => match m {
                    MinuteSize::One => "1 min",
                    MinuteSize::Two => "2 mins",
                    MinuteSize::Three => "3 mins",
                    MinuteSize::Five => "5 mins",
                    MinuteSize::Ten => "10 mins",
                    MinuteSize::Fifteen => "15 mins",
                    MinuteSize::Twenty => "20 mins",
                    MinuteSize::Thirty => "30 mins",
                },
                Self::Hours(h) => match h {
                    HourSize::One => "1 hour",
                    HourSize::Two => "2 hours",
                    HourSize::Three => "3 hours",
                    HourSize::Four => "4 hours",
                    HourSize::Eight => "8 hours",
                },
                Self::Day => "1 day",
                Self::Week => "1 week",
                Self::Month => "1 month",
            }
            .serialize(serializer)
        }
    }

    // === Data types ===

    #[typed_variants]
    #[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
    #[serde(rename_all = "UPPERCASE")]
    /// The data types for a [`crate::client::Client::req_historical_bar`] request.
    pub enum Data {
        #[serde(rename = "TRADES")]
        /// The actual traded prices during the bar interval.
        Trades,
        #[serde(rename = "MIDPOINT")]
        /// The posted midpoint price during the bar interval.
        Midpoint,
        #[serde(rename = "BID")]
        /// The posted bid price during the bar interval.
        Bid,
        #[serde(rename = "ASK")]
        /// The posted ask price during the bar interval.
        Ask,
        #[serde(rename = "BID_ASK")]
        /// The time averaged bid and ask during the bar interval.
        BidAsk,
        #[serde(rename = "HISTORICAL_VOLATILITY")]
        /// The realized volatility during the bar interval.
        HistoricalVolatility,
        #[serde(rename = "OPTION_IMPLIED_VOLATILITY")]
        /// The options market implied volatility during the bar interval.
        SecOptionImpliedVolatility,
    }

    make_valid!(
        Trades,
        Midpoint,
        Bid,
        Ask,
        BidAsk,
        HistoricalVolatility,
        SecOptionImpliedVolatility,
        Data
    );

    impl_data_type!(
        (Trades, Midpoint, Bid, Ask, BidAsk, HistoricalVolatility, SecOptionImpliedVolatility, Data);
        (Stock)
    );

    impl_data_type!(
        (Trades, HistoricalVolatility, SecOptionImpliedVolatility);
        (Index);
        TradesVolData
    );

    impl_data_type!(
        (Trades, Midpoint, Bid, Ask, BidAsk);
        (SecOption, SecFuture, Crypto);
        TradesMidBidAskData
    );

    impl_data_type!(
        (Midpoint, Bid, Ask, BidAsk);
        (Forex, Commodity);
        MidBidAskData
    );
}

/// Contains types and traits used by [`crate::client::Client::req_updating_historical_bar`].
pub mod updating_historical_bar {
    use ibapi_macros::typed_variants;
    use serde::{Deserialize, Serialize};

    /// Re-export of [`historical_bar::Duration`]
    pub use historical_bar::Duration as Duration;
    /// Re-export of [`historical_bar::HourSize`]
    pub use historical_bar::HourSize as HourSize;
    /// Re-export of [`historical_bar::MinuteSize`]
    pub use historical_bar::MinuteSize as MinuteSize;
    /// Re-export of [`historical_bar::SecondSize`]
    pub use historical_bar::SecondSize as SecondSize;
    /// Re-export of [`historical_bar::Size`]
    pub use historical_bar::Size as Size;

    use crate::contract::{Commodity, Crypto, Forex, Index, SecFuture, SecOption, Stock};

    use super::historical_bar;

    // === Type definitions ===

    // === Data types ===

    #[typed_variants]
    #[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
    #[serde(rename_all = "UPPERCASE")]
    /// The data types for a ['crate::client::Client::req_updating_historical_bar'] request or a [`crate::client::Client::req_real_time_bars`] request.
    pub enum Data {
        #[serde(rename = "TRADES")]
        /// The actual traded prices during the bar interval.
        Trades,
        #[serde(rename = "MIDPOINT")]
        /// The posted midpoint price during the bar interval.
        Midpoint,
        #[serde(rename = "BID")]
        /// The posted bid price during the bar interval.
        Bid,
        #[serde(rename = "ASK")]
        /// The posted ask price during the bar interval.
        Ask,
    }

    make_valid!(Trades, Midpoint, Bid, Ask, Data);

    impl_data_type!(
        (Trades, Midpoint, Bid, Ask, Data);
        (Stock, SecOption, SecFuture, Crypto)
    );

    impl_data_type!(
        (Trades);
        (Index)
    );

    impl_data_type!(
        (Midpoint, Bid, Ask);
        (Forex, Commodity);
        MidBidAskData
    );
}

/// Contains types and traits used by [`crate::client::Client::req_historical_ticks`] and
/// [`crate::client::Client::req_head_timestamp`].
pub mod historical_ticks {
    use ibapi_macros::typed_variants;
    use serde::{Deserialize, Serialize, Serializer};

    use crate::contract::{Commodity, Contract, Crypto, Forex, Index, SecFuture, SecOption, Stock};

    // === Type definitions ===

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    /// The timestamp that dictates the start or end of the period for which historical ticks will
    /// be returned.
    pub enum TimeStamp {
        /// A starting date: Return some number ticks beginning at the provided date and time.
        StartDateTime(chrono::DateTime<chrono::Utc>),
        /// An ending date: Return some number ticks ending at the provided date and time.
        EndDateTime(chrono::DateTime<chrono::Utc>),
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
    /// A simple struct to ensure that the number of ticks to return never exceeds 1,000.
    pub struct NumberOfTicks(u16);

    // === Type implementations ===

    impl NumberOfTicks {
        #[must_use]
        /// Create a new [`NumberOfTicks`] struct, which will request some number of historical
        /// ticks equal to min(1,000, `number_of_ticks`).
        ///
        /// # Arguments
        /// * `number_of_ticks` - The number of ticks to return from a
        ///   [`crate::client::Client::req_historical_ticks`] query.
        ///
        /// # Returns
        /// A new, valid [`NumberOfTicks`] struct.
        pub const fn new(number_of_ticks: u16) -> Self {
            Self(if number_of_ticks > 1_000 {
                1_000
            } else {
                number_of_ticks
            })
        }
    }

    impl Serialize for TimeStamp {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match *self {
                Self::StartDateTime(dt) => {
                    (dt.format("%Y%m%d-%T").to_string(), None::<()>).serialize(serializer)
                }
                Self::EndDateTime(dt) => {
                    (None::<()>, dt.format("%Y%m%d-%T").to_string()).serialize(serializer)
                }
            }
        }
    }

    // === Data types ===

    #[typed_variants]
    #[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
    #[serde(rename_all = "UPPERCASE")]
    /// The data types for a [`crate::client::Client::req_historical_ticks`] request or a
    /// [`crate::client::Client::req_head_timestamp`] request.
    pub enum Data {
        #[serde(rename = "TRADES")]
        /// The prices (and sizes) of actual trades for a given tick.
        Trades,
        #[serde(rename = "MIDPOINT")]
        /// The posted midpoint price (and aggregated size) for a given tick.
        Midpoint,
        #[serde(rename = "BID_ASK")]
        /// The posted bid and ask prices (and sizes) for a given tick.
        BidAsk,
    }

    make_valid!(Trades, Midpoint, BidAsk, Data);

    impl_data_type!(
        (Trades, Midpoint, BidAsk, Data);
        (Contract, Stock, Forex, SecOption, SecFuture, Crypto, Index, Commodity)
    );
}

/// Contains types and traits used by [`crate::client::Client::req_histogram_data`].
pub mod histogram {
    use serde::{Serialize, Serializer};

    // === Type definitions ===

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    /// The span of dates and times over which bars will be returned.
    pub enum Duration {
        /// Some number of seconds.
        Second(u32),
        /// Some number of days.
        Day(u32),
        /// Some number of weeks.
        Week(u32),
        /// Some number of months.
        Month(u32),
        /// Some number of years.
        Year(u32),
    }

    // === Type implementations ===

    impl Serialize for Duration {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match *self {
                Self::Second(s) => format!("{s} seconds"),
                Self::Day(d) => format!("{d} days"),
                Self::Week(w) => format!("{w} weeks"),
                Self::Month(m) => format!("{m} months"),
                Self::Year(y) => format!("{y} years"),
            }
            .serialize(serializer)
        }
    }
}

/// Contains the types and traits used by [`crate::client::Client::req_real_time_bars`].
pub mod live_bar {
    /// Re-export of [`updating_historical_bar::Ask`]
    pub use updating_historical_bar::Ask as Ask;
    /// Re-export of [`updating_historical_bar::Bid`]
    pub use updating_historical_bar::Bid as Bid;
    /// Re-export of [`updating_historical_bar::Data`]
    pub use updating_historical_bar::Data as Data;
    /// Re-export of [`updating_historical_bar::Midpoint`]
    pub use updating_historical_bar::Midpoint as Midpoint;
    /// Re-export of [`updating_historical_bar::Trades`]
    pub use updating_historical_bar::Trades as Trades;

    use crate::contract::{Commodity, Contract, Crypto, Forex, Index, SecFuture, SecOption, Stock};

    use super::updating_historical_bar;

    // === Data types ===

    make_valid!(Trades, Midpoint, Bid, Ask, Data);

    impl_data_type!(
        (Trades, Midpoint, Bid, Ask, Data);
        (Stock, Forex, SecOption, SecFuture, Crypto, Index, Commodity, Contract)
    );
}

#[allow(clippy::module_name_repetitions)]
/// Contains types and traits used by [`crate::client::Client::req_market_data`] and
/// [`crate::client::Client::req_market_data_type`].
pub mod live_data {
    use std::fmt::Formatter;

    use ibapi_macros::typed_variants;
    use serde::{Deserialize, Serialize};

    use crate::contract::{Commodity, Crypto, Forex, Index, SecFuture, SecOption, Stock};

    // === Type definitions ===

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
    /// The frequency at which data will be updated.
    pub enum RefreshType {
        #[serde(rename(serialize = "1"))]
        /// Return a snapshot of the market at a specific point in time.
        Snapshot,
        #[serde(rename(serialize = "0"))]
        /// Begin a streaming subscription.
        Streaming,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
    /// The type of market data to return.
    pub enum Class {
        #[serde(rename(serialize = "1"))]
        /// Real-time streaming data, which requires a subscription.
        Live,
        #[serde(rename(serialize = "2"))]
        /// The last data recorded at market close, which requires a subscription.
        Frozen,
        #[serde(rename(serialize = "3"))]
        /// Delayed data by 15-20 minutes, which does not require any subscription.
        Delayed,
        #[serde(rename(serialize = "4"))]
        /// Same as frozen, but does not require any subscription.
        DelayedFrozen,
    }

    #[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    /// An error type that represents an invalid [`Class`] has been received.
    pub struct ParseClassError(String);

    // === Type implementations ===

    impl std::str::FromStr for Class {
        type Err = ParseClassError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match s {
                "1" => Self::Live,
                "2" => Self::Frozen,
                "3" => Self::Delayed,
                "4" => Self::DelayedFrozen,
                _ => return Err(ParseClassError(s.to_owned())),
            })
        }
    }

    impl std::fmt::Display for ParseClassError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Invalid market data class encountered: {}", self.0)
        }
    }

    impl std::error::Error for ParseClassError {}

    // === Data types ===

    #[typed_variants]
    #[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Serialize, Deserialize)]
    /// Contains the data types for a [`crate::client::Client::req_market_data`] request.
    pub enum Data {
        #[serde(rename = "100")]
        /// The volume of options contracts exchanged.
        SecOptionVolume,
        #[serde(rename = "101")]
        /// The open interest of options contracts.
        SecOptionOpenInterest,
        #[serde(rename = "104")]
        /// The realized price volatility.
        HistoricalVolatility,
        #[serde(rename = "105")]
        /// The average options contract volume.
        AverageSecOptionVolume,
        #[serde(rename = "106")]
        /// The implied volatility by the options market.
        SecOptionImpliedVolatility,
        #[serde(rename = "162")]
        /// The number of points that the index is over the cash index.
        IndexFuturePremium,
        #[serde(rename = "165")]
        /// Miscellaneous statistics associated with the stock.
        MiscellaneousStats,
        #[serde(rename = "221")]
        /// The mark-to-market price used for margin at IBKR.
        MarkPrice,
        #[serde(rename = "225")]
        /// The volume, price, and imbalance of an auction.
        AuctionValues,
        #[serde(rename = "233")]
        /// Last trade's price, size, and time.
        RealTimeVolume,
        #[serde(rename = "236")]
        /// The level of difficulty associated with short-selling a security.
        Shortable,
        #[serde(rename = "256")]
        /// Available inventory for short-selling.
        Inventory,
        #[serde(rename = "258")]
        /// Fundamental stock ratios.
        FundamentalRatios,
        #[serde(rename = "411")]
        /// 30-day real time historical volatility.
        RealtimeHistoricalVolatility,
        #[serde(rename = "456")]
        /// Information about past and future dividends.
        IBDividends,
        #[serde(rename = "")]
        /// No additional data
        Empty,
    }

    make_valid!(
        SecOptionVolume,
        SecOptionOpenInterest,
        HistoricalVolatility,
        AverageSecOptionVolume,
        SecOptionImpliedVolatility,
        IndexFuturePremium,
        MiscellaneousStats,
        MarkPrice,
        AuctionValues,
        RealTimeVolume,
        Shortable,
        Inventory,
        FundamentalRatios,
        RealtimeHistoricalVolatility,
        IBDividends,
        Empty,
        Data
    );

    impl_data_type!(
        (
            SecOptionVolume,
            SecOptionOpenInterest,
            HistoricalVolatility,
            AverageSecOptionVolume,
            SecOptionImpliedVolatility,
            IndexFuturePremium,
            MiscellaneousStats,
            MarkPrice,
            AuctionValues,
            RealTimeVolume,
            Shortable,
            Inventory,
            FundamentalRatios,
            RealtimeHistoricalVolatility,
            IBDividends,
            Empty,
            Data
        );
        (Stock)
    );

    impl_data_type!(
        (
            IndexFuturePremium,
            MiscellaneousStats,
            MarkPrice,
            AuctionValues,
            RealTimeVolume,
            Shortable,
            Inventory,
            FundamentalRatios,
            RealtimeHistoricalVolatility,
            IBDividends,
            Empty
        );
        (Forex, SecOption, SecFuture, Crypto, Index, Commodity);
        NonStockData
    );
}

/// Contains types and traits used by [`crate::client::Client::req_tick_by_tick_data`].
pub mod live_ticks {
    use ibapi_macros::typed_variants;
    use serde::{Deserialize, Serialize};

    use crate::contract::{Commodity, Crypto, Forex, Index, SecFuture, Stock};

    /// Re-export of [`super::historical_ticks::NumberOfTicks`] for convenience.
    pub use super::historical_ticks::NumberOfTicks as NumberOfTicks;

    // === Data types ===

    #[typed_variants]
    #[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Serialize, Deserialize)]
    /// The data types for a [`crate::client::Client::req_tick_by_tick_data`] request.
    pub enum Data {
        #[serde(rename = "AllLast")]
        /// All the last actual trades since prior tick (and size)
        AllLast,
        #[serde(rename = "Last")]
        /// The last actual trade (and size).
        Last,
        #[serde(rename = "BidAsk")]
        /// The posted bid and ask prices (and sizes).
        BidAsk,
        #[serde(rename = "MidPoint")]
        /// The posted midpoint (and size).
        Midpoint,
    }

    make_valid!(Data, AllLast, Last, BidAsk, Midpoint);

    impl_data_type!(
        (Data, AllLast, Last, BidAsk, Midpoint);
        (Stock, Forex, SecFuture, Crypto, Index, Commodity)
    );
}
