macro_rules! make_variants {
    ($($( #[doc = $name_doc:expr] )? $name: ident: $repr: literal),*) => {
        $(
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
            #[serde(rename(serialize = $repr))]
            $( #[doc = $name_doc] )?
            pub struct $name;
        )*

        pub(crate) mod indicators {
            use serde::Serialize;
use super::{$($name,)*};

            pub trait Valid: Serialize + Copy + Clone {}

            $(
                impl Valid for $name {}
            )*
        }

        $(
            impl ToString for $name {
                #[inline]
                fn to_string(&self) -> String {
                    $repr.to_owned()
                }
            }
        )*


        /// Implemented by all valid data types for a given security. In particular,
        /// if a type `D` implements [`DataType<S>`], then `D` is a valid data type for `S`.
        pub trait DataType<S: Security>: ToString + Send + Sync + indicators::Valid {}
    };
}

macro_rules! impl_data_type {
    (($($d_name: ident),*); $s_names: tt) => {
        $(
            impl_data_type!($d_name; $s_names);
        )*
    };
    ($d_name: ident; ($($s_name: ident),*)) => {
        $(
            impl DataType<$s_name> for $d_name {}
        )*
    };
}

/// Contains types and traits used by [`crate::client::Client::req_historical_bar`].
pub mod historical_bar {

    // === Type definitions ===

    use serde::{Serialize, Serializer};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    /// The last time for which bar data will be returned.
    pub enum EndDateTime {
        /// The present moment.
        Present,
        /// Some date and time in the past.
        Past(chrono::NaiveDateTime),
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
                Self::Past(dt) => Some(dt.format("%Y%m%d %T").to_string()),
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

    /// Contains the potential data types for a [`crate::client::Client::req_historical_bar`] request.
    pub mod data_types {
        use crate::contract::{
            Commodity, Crypto, Forex, Index, SecFuture, SecOption, Security, Stock,
        };

        make_variants!(
            /// The actual traded prices during the bar interval.
            Trades: "TRADES",
            /// The posted midpoint price during the bar interval.
            Midpoint: "MIDPOINT",
            /// The posted bid price during the bar interval.
            Bid: "BID",
            /// The posted ask price during the bar interval.
            Ask: "ASK",
            /// The time averaged bid and ask during the bar interval.
            BidAsk: "BID_ASK",
            /// The realized volatility during the bar interval.
            HistoricalVolatility: "HISTORICAL_VOLATILITY",
            /// The options market implied volatility during the bar interval.
            SecOptionImpliedVolatility: "OPTION_IMPLIED_VOLATILITY"
        );

        impl_data_type!(
            (Trades, Midpoint, Bid, Ask, BidAsk, HistoricalVolatility, SecOptionImpliedVolatility);
            (Stock)
        );

        impl_data_type!(
            (Trades, HistoricalVolatility, SecOptionImpliedVolatility);
            (Index)
        );

        impl_data_type!(
            (Trades, Midpoint, Bid, Ask, BidAsk);
            (SecOption, SecFuture, Crypto)
        );
        impl_data_type!(
            (Midpoint, Bid, Ask, BidAsk);
            (Forex, Commodity)
        );
    }
}

/// Contains types and traits used by [`crate::client::Client::req_updating_historical_bar`].
pub mod updating_historical_bar {
    use super::historical_bar;

    // === Type definitions ===

    /// Re-export of [`historical_bar::Duration`]
    pub type Duration = historical_bar::Duration;

    /// Re-export of [`historical_bar::Size`]
    pub type Size = historical_bar::Size;

    /// Re-export of [`historical_bar::SecondSize`]
    pub type SecondSize = historical_bar::SecondSize;

    /// Re-export of [`historical_bar::MinuteSize`]
    pub type MinuteSize = historical_bar::MinuteSize;

    /// Re-export of [`historical_bar::HourSize`]
    pub type HourSize = historical_bar::HourSize;

    // === Data types ===

    /// Contains the potential data types for a [`crate::client::Client::req_updating_historical_bar`] request.
    pub mod data_types {
        use ibapi_macros::typed_variants;
        use serde::{Deserialize, Serialize};
        use crate::contract::{
            Commodity, Crypto, Forex, Index, SecFuture, SecOption, Security, Stock,
        };

        #[typed_variants]
        #[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
        #[serde(rename_all="UPPERCASE")]
        /// The possible data types for a historical bar request.
        pub enum DataTypes {
            /// The actual traded prices during the bar interval.
            Trades2,
            /// The posted midpoint price during the bar interval.
            Midpoint2,
            /// The posted bid price during the bar interval.
            Bid2,
            /// The posted ask price during the bar interval.
            Ask2,
        }

        make_variants!(
            /// The actual traded prices during the bar interval.
            Trades: "TRADES",
            /// The posted midpoint price during the bar interval.
            Midpoint: "MIDPOINT",
            /// The posted bid price during the bar interval.
            Bid: "BID",
            /// The posted ask price during the bar interval.
            Ask: "ASK"
        );

        impl_data_type!(
            (Trades, Midpoint, Bid, Ask);
            (Stock)
        );

        impl_data_type!(
            (Trades);
            (Index)
        );

        impl_data_type!(
            (Trades, Midpoint, Bid, Ask);
            (SecOption, SecFuture, Crypto)
        );
        impl_data_type!(
            (Midpoint, Bid, Ask);
            (Forex, Commodity)
        );
    }
}

/// Contains types and traits used by [`crate::client::Client::req_historical_ticks`] and
/// [`crate::client::Client::req_head_timestamp`].
pub mod historical_ticks {
    use serde::{Serialize, Serializer};

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

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
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
        /// [`crate::client::Client::req_historical_ticks`] query.
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

    /// Contains the potential data types for a [`crate::client::Client::req_historical_ticks`] or
    /// [`crate::client::Client::req_head_timestamp`] request.
    pub mod data_types {
        use crate::contract::{
            Commodity, Crypto, Forex, Index, SecFuture, SecOption, Security, Stock,
        };

        make_variants!(
            /// The prices (and sizes) of actual trades for a given tick.
            Trades: "TRADES",
            /// The posted midpoint price (and aggregated size) for a given tick.
            Midpoint: "MIDPOINT",
            /// The posted bid and ask prices (and sizes) for a given tick.
            BidAsk: "BID_ASK"
        );

        impl_data_type!(
            (Trades, Midpoint, BidAsk);
            (Stock, Forex, SecOption, SecFuture, Crypto, Index, Commodity)
        );
    }
}

/// Contains types and traits used by [`crate::client::Client::req_histogram_data`].
pub mod histogram {

    // === Type definitions ===

    use serde::{Serialize, Serializer};

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

    // === Data types ===

    /// Contains the potential data types for a [`crate::client::Client::req_real_time_bars`] request.
    pub mod data_types {
        use crate::contract::{
            Commodity, Crypto, Forex, Index, SecFuture, SecOption, Security, Stock,
        };

        make_variants!(
            /// The actual trades for a given 5-second interval.
            Trades: "TRADES",
            /// The posted midpoint prices for a given 5-second interval.
            Midpoint: "MIDPOINT",
            /// The posted bid prices for a given 5-second interval.
            Bid: "BID",
            /// The posted ask prices for a given 5-second interval.
            Ask: "ASK"
        );

        impl_data_type!(
            (Trades, Midpoint, Bid, Ask);
            (Stock, Forex, SecOption, SecFuture, Crypto, Index, Commodity)
        );
    }
}

#[allow(clippy::module_name_repetitions)]
/// Contains types and traits used by [`crate::client::Client::req_market_data`] and
/// [`crate::client::Client::req_market_data_type`].
pub mod live_data {

    // === Type definitions ===

    use serde::Serialize;
    use std::fmt::Formatter;

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

    /// Contains the potential data types for a [`crate::client::Client::req_market_data`] request.
    pub mod data_types {
        use crate::contract::{
            Commodity, Crypto, Forex, Index, SecFuture, SecOption, Security, Stock,
        };

        make_variants!(
            /// The volume of options contracts exchanged.
            SecOptionVolume: "100",
            /// The open interest of options contracts.
            SecOptionOpenInterest: "101",
            /// The realized price volatility.
            HistoricalVolatility: "104",
            /// The average options contract volume.
            AverageSecOptionVolume: "105",
            /// The implied volatility by the options market.
            SecOptionImpliedVolatility: "106",
            /// The number of points that the index is over the cash index.
            IndexFuturePremium: "162",
            /// Miscellaneous statistics associated with the stock.
            MiscellaneousStats: "165",
            /// The mark-to-market price used for margin at IBKR.
            MarkPrice: "221",
            /// The volume, price, and imbalance of an auction.
            AuctionValues: "225",
            /// Last trade's price, size, and time.
            RealTimeVolume: "233",
            /// The level of difficulty associated with short-selling a security.
            Shortable: "236",
            /// Available inventory for short-selling.
            Inventory: "256",
            /// Fundamental stock ratios.
            FundamentalRatios: "258",
            /// 30-day real time historical volatility.
            RealtimeHistoricalVolatility: "411",
            /// Information about past and future dividends.
            IBDividends: "456",
            /// No additional data
            Empty: ""
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
                Empty
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
            (Forex, SecOption, SecFuture, Crypto, Index, Commodity)
        );
    }
}

/// Contains types and traits used by [`crate::client::Client::req_tick_by_tick_data`].
pub mod live_ticks {

    // === Data types ===

    /// Re-export of [`super::historical_ticks::NumberOfTicks`] for convenience.
    pub type NumberOfTicks = super::historical_ticks::NumberOfTicks;

    /// Contains the potential data types for a [`crate::client::Client::req_tick_by_tick_data`] request.
    pub mod data_types {
        use crate::contract::{Commodity, Crypto, Forex, Index, SecFuture, Security, Stock};

        make_variants!(
            /// All the last actual trades since prior tick (and size)
            AllLast: "AllLast",
            /// The last actual trade (and size).
            Last: "Last",
            /// The posted bid and ask prices (and sizes).
            BidAsk: "BidAsk",
            /// The posted midpoint (and size).
            Midpoint: "MidPoint"
        );

        impl_data_type!(
            (AllLast, Last, BidAsk, Midpoint);
            (Stock, Forex, SecFuture, Crypto, Index, Commodity)
        );
    }
}
