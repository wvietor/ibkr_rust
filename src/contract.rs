use chrono::NaiveDate;
use std::{num::ParseIntError, str::FromStr};

use crate::{
    currency::Currency,
    exchange::{Primary, Routing},
    match_poly,
};
use ibapi_macros::Security;
use serde::{Deserialize, Serialize, Serializer};

// =========================================================
// === Utility Types and Functions for Contract Creation ===
// =========================================================

// todo!("Ensure that includeExpired is always set to true");

#[derive(Debug, Clone, PartialEq, PartialOrd)]
/// Wrapper enum for all possible contracts available in the API
pub enum Contract {
    /// A [`Forex`] contract.
    Forex(Forex),
    /// A [`Crypto`] contract.
    Crypto(Crypto),
    /// A [`Stock`] contract.
    Stock(Stock),
    /// An [`Index`] contract.
    Index(Index),
    //Cfd(Cfd),
    /// A [`SecFuture`] contract.
    SecFuture(SecFuture),
    /// A [`SecOption`] contract.
    SecOption(SecOption),
    //FutureSecOption(SecFutureOption),
    //Bond(Bond),
    //MutualFund(MutualFund),
    /// A [`Commodity`] contract.
    Commodity(Commodity),
    //Warrant(Warrant),
    //StructuredProduct(StructuredProduct),
}

macro_rules! contract_impl {
    ($sec_type: ty, $pat: pat_param => $exp: expr, $func_name_ref: ident, $func_name: ident) => {
        #[allow(missing_docs, clippy::missing_errors_doc)]
        pub fn $func_name_ref(&self) -> anyhow::Result<&$sec_type> {
            match self {
                $pat => $exp,
                _ => Err(anyhow::anyhow!(
                    "Expected {}; found other contract type.",
                    stringify!($func_name)
                )),
            }
        }
        #[allow(missing_docs, clippy::missing_errors_doc)]
        pub fn $func_name(self) -> anyhow::Result<$sec_type> {
            match self {
                $pat => $exp,
                _ => Err(anyhow::anyhow!(
                    "Expected {}; found other contract type.",
                    stringify!($func_name)
                )),
            }
        }
    };
}

impl Contract {
    contract_impl!(Forex, Self::Forex(t) => Ok(t), forex_ref, forex);
    contract_impl!(Crypto, Self::Crypto(t) => Ok(t), crypto_ref, crypto);
    contract_impl!(Stock, Self::Stock(t) => Ok(t), stock_ref, stock);
    contract_impl!(Index, Self::Index(t) => Ok(t), index_ref, index);
    contract_impl!(SecFuture, Self::SecFuture(t) => Ok(t), secfuture_ref, secfuture);
    contract_impl!(SecOption, Self::SecOption(t) => Ok(t), secoption_ref, secoption);
    contract_impl!(Commodity, Self::Commodity(t) => Ok(t), commodity_ref, commodity);
}

impl Serialize for Contract {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.serialize(serializer)
        )
    }
}

impl Security for Contract {
    #[inline]
    fn get_contract_id(&self) -> ContractId {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.get_contract_id()
        )
    }

    #[inline]
    fn get_symbol(&self) -> &str {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.get_symbol()
        )
    }

    #[inline]
    fn get_security_type(&self) -> &'static str {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.get_security_type()
        )
    }

    #[inline]
    fn get_expiration_date(&self) -> Option<NaiveDate> {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.get_expiration_date()
        )
    }

    #[inline]
    fn get_strike(&self) -> Option<f64> {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.get_strike()
        )
    }

    #[inline]
    fn get_right(&self) -> Option<&'static str> {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.get_right()
        )
    }

    #[inline]
    fn get_multiplier(&self) -> Option<u32> {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.get_multiplier()
        )
    }

    #[inline]
    fn get_exchange(&self) -> Routing {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.get_exchange()
        )
    }

    #[inline]
    fn get_primary_exchange(&self) -> Option<Primary> {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.get_primary_exchange()
        )
    }

    #[inline]
    fn get_currency(&self) -> Currency {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.get_currency()
        )
    }

    #[inline]
    fn get_local_symbol(&self) -> &str {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.get_local_symbol()
        )
    }

    #[inline]
    fn get_trading_class(&self) -> Option<&str> {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.get_trading_class()
        )
    }
}

/// Create a new contract based on the unique IBKR contract ID. These contract IDs can be found
/// either in the Trader Workstation software or online at the
/// [IBKR Contract Information Center](https://contract.ibkr.info/v3.10/index.php).
///
/// # Arguments
/// * `client` - The client with which to send the validation request.
/// * `contract_id` - The IBKR contract ID corresponding to the contract that will be created.
///
/// # Errors
/// Returns any error encountered while writing the query string to the outgoing buffer, while
/// sending the creation signal to the client loop thread, or while receiving the complete contract
/// from the client loop thread. Additionally, this function will error if the contract does not
/// match the generic type specified in the function call.
///
/// # Returns
/// Returns a fully-defined contract that can be used for market data, placing orders, etc.
pub async fn new<S: Security>(
    client: &mut crate::client::ActiveClient,
    contract_id: ContractId,
) -> anyhow::Result<S> {
    client.send_contract_query(contract_id).await?;
    match client.recv_contract_query().await? {
        Contract::Forex(fx) => fx.try_into().map_err(|_| ()),
        Contract::Crypto(crypto) => crypto.try_into().map_err(|_| ()),
        Contract::Stock(stk) => stk.try_into().map_err(|_| ()),
        Contract::Index(ind) => ind.try_into().map_err(|_| ()),
        Contract::SecFuture(fut) => fut.try_into().map_err(|_| ()),
        Contract::SecOption(opt) => opt.try_into().map_err(|_| ()),
        Contract::Commodity(cmdty) => cmdty.try_into().map_err(|_| ()),
    }
    .map_err(|()| anyhow::anyhow!("Failed to create contract from {:?}: ", contract_id))
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
/// An error caused when a call to [`new`] returns a contract that differs from
/// the type defined in the initial call.
pub struct UnexpectedSecurityType(&'static str);

impl std::fmt::Display for UnexpectedSecurityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for UnexpectedSecurityType {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
/// A unique identifier used by both IBKR's trading systems and the API to define a specific
/// contract.
pub struct ContractId(pub i64);

impl FromStr for ContractId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Identifiers used by the broader industry / regulators to define a specific contract / asset.
pub enum SecurityId {
    /// For details, see:
    /// [CUSIP Description](https://www.cusip.com/identifiers.html?section=CUSIP).
    Cusip(String),
    /// For details, see:
    /// [SEDOL Description](https://www.lseg.com/en/data-indices-analytics/data/sedol).
    Sedol(String),
    /// For details, see:
    /// [ISIN Description](https://www.cusip.com/identifiers.html?section=ISIN#/ISIN).
    Isin(String),
    /// For details, see:
    /// [RIC Description](https://en.wikipedia.org/wiki/Refinitiv_Identification_Code).
    Ric(String),
}

// =================================
// === Valid Trait Definition ===
// =================================

mod indicators {
    use super::{Commodity, Contract, Crypto, Forex, Index, SecFuture, SecOption, Stock};
    use serde::Serialize;

    pub trait Valid:
        Serialize
        + Send
        + Sync
        + TryFrom<Forex>
        + TryFrom<Crypto>
        + TryFrom<Stock>
        + TryFrom<Index>
        + TryFrom<SecFuture>
        + TryFrom<SecOption>
        + TryFrom<Commodity>
        + Into<Contract>
    {
    }

    impl Valid for Contract {}
}

#[doc(alias = "Contract")]
/// Attributes shared by a tradable contract or asset. All valid contracts implement this trait.
pub trait Security: indicators::Valid {
    /// Get the security's [`ContractId`].
    ///
    /// # Returns
    /// The security's contract ID.
    fn get_contract_id(&self) -> ContractId;
    /// Get the security's symbol.
    ///
    /// # Returns
    /// The security's symbol.
    fn get_symbol(&self) -> &str;
    /// Get a text representation of the security's type.
    ///
    /// # Returns
    /// The security's type, encoded as a [`&'static str`].
    fn get_security_type(&self) -> &'static str;
    /// Get the security's expiration / last-trade date, if it has one.
    ///
    /// # Returns
    /// The security's expiration / last-trade date, provided that it exists.
    fn get_expiration_date(&self) -> Option<NaiveDate>;
    /// Get the security's strike price, if it has one.
    ///
    /// # Returns
    /// The security's strike price, provided that it exists.
    fn get_strike(&self) -> Option<f64>;
    /// Get the security's "right," which is a character/symbol representing its class, if it has
    ///  one.
    ///
    /// # Returns
    /// The security's right, provided that it exists.
    fn get_right(&self) -> Option<&'static str>;
    /// Get the security's multiplier (the ratio of the actual price paid to the quoted price), if
    ///  it has one.
    ///
    /// # Returns
    /// The security's multiplier, provided that it exists.
    fn get_multiplier(&self) -> Option<u32>;
    /// Get the security's routing exchange (ie: The exchange to which orders will be routed.
    ///
    /// # Returns
    /// The security's exchange.
    fn get_exchange(&self) -> Routing;
    /// Get the security's primary exchange, which is the physical exchange where it is listed, if
    /// it has one.
    ///
    /// # Returns
    /// The security's primary exchange, provided that it exists.
    fn get_primary_exchange(&self) -> Option<Primary>;
    /// Get the security's trading currency.
    ///
    /// # Returns
    /// The security's trading currency.
    fn get_currency(&self) -> Currency;
    /// Get the security's local symbol.
    ///
    /// # Returns
    /// The security's local symbol.
    fn get_local_symbol(&self) -> &str;
    /// Get the security's trading class (mainly a regulatory indicator), if it has one.
    ///
    /// # Returns
    /// The security's trading class.
    fn get_trading_class(&self) -> Option<&str>;
}

// =======================================
// === Definitions of Contract Structs ===
// =======================================

macro_rules! make_contract {
    ($( #[doc = $name_doc:expr] )? $name: ident $(,$trt: ident)?; $($field: ident: $f_type: ty),* $(,)?) => {
        $( #[doc = $name_doc] )?
        #[derive(Debug, Clone, PartialEq, PartialOrd, $($trt)?)]
        pub struct $name {
            pub(crate) contract_id: ContractId,
            pub(crate) min_tick: f64,
            pub(crate) symbol: String,
            $(pub(crate) $field: $f_type,)*
            pub(crate) currency: Currency,
            pub(crate) local_symbol: String,
            pub(crate) long_name: String,
            pub(crate) order_types: Vec<String>,
            pub(crate) valid_exchanges: Vec<Routing>,
        }

    }
}

make_contract!(
    /// A [forex contract](https://interactivebrokers.github.io/tws-api/basic_contracts.html#cash), like GBPUSD.
    Forex,
    Security;
    exchange: Routing,
    trading_class: String
);
make_contract!(
    /// A [crypto contract](https://interactivebrokers.github.io/tws-api/basic_contracts.html#crypto), like BTC.
    Crypto,
    Security;
    trading_class: String
);
make_contract!(
    /// An [equity contract](https://interactivebrokers.github.io/tws-api/basic_contracts.html#stk), like AAPL.
    Stock,
    Security;
    exchange: Routing,
    primary_exchange: Primary,
    stock_type: String,
    security_ids: Vec<SecurityId>,
    sector: String,
    trading_class: String
);
make_contract!(
    /// An [index](https://interactivebrokers.github.io/tws-api/basic_contracts.html#ind), like SPX.
    Index,
    Security;
    exchange: Routing
);
make_contract!(
    /// A [commodity](https://interactivebrokers.github.io/tws-api/basic_contracts.html#Commodities), like XAUUSD.
    Commodity,
    Security;
    exchange: Routing,
    trading_class: String
);
make_contract!(
    /// A [futures contract](https://interactivebrokers.github.io/tws-api/basic_contracts.html#fut), like FGBL MAR 23.
    SecFuture,
    Security;
    exchange: Routing,
    multiplier: u32,
    expiration_date: NaiveDate,
    trading_class: String,
    underlying_contract_id: ContractId
);

make_contract!(
    /// Helper struct to hold the fields of a [`SecOption`].
    SecOptionInner;
    exchange: Routing,
    strike: f64,
    multiplier: u32,
    expiration_date: NaiveDate,
    underlying_contract_id: ContractId,
    sector: String,
    trading_class: String
);

#[derive(Debug, Clone, PartialEq, PartialOrd, Security)]
/// A [vanilla option contract](https://interactivebrokers.github.io/tws-api/basic_contracts.html#opt), like P BMW  20221216 72 M.
pub enum SecOption {
    /// A vanilla call option, defined by the following payoff function: max(S<sub>T</sub> - K, 0)
    Call(SecOptionInner),
    /// A vanilla put option, defined by the following payoff function: max(K - S<sub>T</sub>, 0)
    Put(SecOptionInner),
}

// ===============================
// === Unimplemented Contracts ===
// ===============================

// make_contract!(Cfd; exchange: Routing);
// make_contract!(Bond; exchange: Routing);
// make_contract!(MutualFund; exchange: Routing);
// make_contract!(StructuredProduct; exchange: Routing, multiplier: u32, expiration_date: NaiveDate);

// #[derive(Debug, Clone, PartialEq, PartialOrd)]
// pub enum SecFutureOption {
//     Call(SecOptionInner),
//     Put(SecOptionInner),
// }

// #[derive(Debug, Clone, PartialEq, PartialOrd)]
// pub enum Warrant {
//     Call(SecOptionInner),
//     Put(SecOptionInner),
// }
