use std::fmt::Formatter;
use std::{num::ParseIntError, str::FromStr};

use chrono::NaiveDate;
use ibapi_macros::{make_getters, Security};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::figi::{Figi, InvalidFigi};
use crate::{
    currency::Currency,
    exchange::{Primary, Routing},
    match_poly,
};

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
        #[doc=concat!("Try to coerce a contract reference to a ", stringify!($sec_type), " reference.")]
        ///
        /// # Returns
        #[doc=concat!("A reference to the underlying ", stringify!($sec_type),  " if the underlying contract is a ", stringify!($sec_type), " and `None` otherwise.")]
        pub fn $func_name_ref(&self) -> Option<&$sec_type> {
            match self {
                $pat => $exp,
                _ => None,
            }
        }
        #[doc=concat!("Try to coerce the contract to a ", stringify!($sec_type))]
        ///
        /// # Returns
        #[doc=concat!("The underlying ", stringify!($sec_type),  " if the underlying contract is a ", stringify!($sec_type), " and `None` otherwise.")]
        pub fn $func_name(self) -> Option<$sec_type> {
            match self {
                $pat => $exp,
                _ => None,
            }
        }
    };
}

impl Contract {
    contract_impl!(Forex, Self::Forex(t) => Some(t), forex_ref, forex);
    contract_impl!(Crypto, Self::Crypto(t) => Some(t), crypto_ref, crypto);
    contract_impl!(Stock, Self::Stock(t) => Some(t), stock_ref, stock);
    contract_impl!(Index, Self::Index(t) => Some(t), index_ref, index);
    contract_impl!(SecFuture, Self::SecFuture(t) => Some(t), secfuture_ref, secfuture);
    contract_impl!(SecOption, Self::SecOption(t) => Some(t), secoption_ref, secoption);
    contract_impl!(Commodity, Self::Commodity(t) => Some(t), commodity_ref, commodity);

    #[inline]
    /// Attempt to get the inner security's exchange.
    ///
    /// # Returns
    /// The inner security's exchange, `None` if the field doesn't exist (for a [`Crypto`] contract)
    pub fn exchange(&self) -> Option<Routing> {
        match_poly!(self;
            Contract::SecOption(s) | Contract::Forex(s) | Contract::Index(s) |
            Contract::SecFuture(s) | Contract::Commodity(s) | Contract::Stock(s) => Some(s.exchange()),
            Contract::Crypto(_) => None,
        )
    }

    #[inline]
    /// Attempt to get the inner security's trading class.
    ///
    /// # Returns
    /// The inner security's exchange, `None` if the field doesn't exist (for an [`Index`] contract)
    pub fn trading_class(&self) -> Option<&str> {
        match_poly!(self;
            Contract::SecOption(s) | Contract::Forex(s) | Contract::Crypto(s) |
            Contract::SecFuture(s) | Contract::Commodity(s) | Contract::Stock(s) => Some(s.trading_class()),
            Contract::Index(_) => None,
        )
    }

    #[inline]
    /// Attempt to get the inner security's multiplier.
    ///
    /// # Returns
    /// The inner security's multiplier if the inner contract is a [`SecOption`] or [`SecFuture`], `None` otherwise
    pub fn multiplier(&self) -> Option<u32> {
        match_poly!(self;
            Contract::SecOption(s) | Contract::SecFuture(s) => Some(s.multiplier()),
            _ => None
        )
    }

    #[inline]
    /// Attempt to get the inner security's expiration date.
    ///
    /// # Returns
    /// The inner security's expiration date if the inner contract is a [`SecOption`] or [`SecFuture`], `None` otherwise
    pub fn expiration_date(&self) -> Option<NaiveDate> {
        match_poly!(self;
            Contract::SecOption(s) | Contract::SecFuture(s) => Some(s.expiration_date()),
            _ => None
        )
    }

    #[inline]
    /// Attempt to get the underlying security's expiration date.
    ///
    /// # Returns
    /// The inner security's underlying contract ID if the inner contract is a [`SecOption`] or [`SecFuture`], `None` otherwise
    pub fn underlying_contract_id(&self) -> Option<ContractId> {
        match_poly!(self;
            Contract::SecOption(s) | Contract::SecFuture(s) => Some(s.underlying_contract_id()),
            _ => None
        )
    }
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
    fn contract_id(&self) -> ContractId {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.contract_id()
        )
    }

    #[inline]
    fn min_tick(&self) -> f64 {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.min_tick()
        )
    }

    #[inline]
    fn symbol(&self) -> &str {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.symbol()
        )
    }

    #[inline]
    fn currency(&self) -> Currency {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.currency()
        )
    }

    #[inline]
    fn local_symbol(&self) -> &str {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.local_symbol()
        )
    }

    #[inline]
    fn long_name(&self) -> &str {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.long_name()
        )
    }

    #[inline]
    fn order_types(&self) -> &Vec<String> {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.order_types()
        )
    }

    #[inline]
    fn valid_exchanges(&self) -> &Vec<Routing> {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.valid_exchanges()
        )
    }

    #[inline]
    fn contract_type(&self) -> ContractType {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.contract_type()
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
    query: Query,
) -> anyhow::Result<S> {
    client.send_contract_query(query).await?;
    match client.recv_contract_query().await? {
        Contract::Forex(fx) => fx.try_into().map_err(|_| ()),
        Contract::Crypto(crypto) => crypto.try_into().map_err(|_| ()),
        Contract::Stock(stk) => stk.try_into().map_err(|_| ()),
        Contract::Index(ind) => ind.try_into().map_err(|_| ()),
        Contract::SecFuture(fut) => fut.try_into().map_err(|_| ()),
        Contract::SecOption(opt) => opt.try_into().map_err(|_| ()),
        Contract::Commodity(cmdty) => cmdty.try_into().map_err(|_| ()),
    }
    .map_err(|()| anyhow::anyhow!("Failed to create contract from {:?}: ", query))
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
/// A type used to represent a query for a new contract, which can be made by providing either an
/// IBKR contract ID, or a FIGI.
pub enum Query {
    /// An IBKR contract ID with which to make a query. When parsing from a string, the routing field
    /// defaults to [`Routing::Smart`].
    IbContractId(ContractId, Routing),
    /// A FIGI.
    Figi(Figi),
}

#[derive(Debug, Clone)]
/// An error type representing the potential ways that a [`Query`] can be invalid.
pub enum InvalidQuery {
    /// An invalid [`Query::IbContractId`]
    IbContractId(ParseIntError),
    /// AN invalid [`Query::Figi`]
    Figi(InvalidFigi),
    /// Invalid in a way such that it's impossible to tell whether it was intended to be an [`Query::IbContractId`] or a [`'Query::Figi`].
    Empty,
}

impl std::fmt::Display for InvalidQuery {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid query. {self:?}")
    }
}

impl std::error::Error for InvalidQuery {}

impl FromStr for Query {
    type Err = InvalidQuery;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // A FIGI always begin with a letter
        if s.chars().nth(0).ok_or(InvalidQuery::Empty)?.is_numeric() {
            Ok(Self::IbContractId(
                s.parse().map_err(InvalidQuery::IbContractId)?,
                Routing::Smart,
            ))
        } else {
            Ok(Self::Figi(s.parse().map_err(InvalidQuery::Figi)?))
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
/// An error caused when a call to [`new`] returns a contract that differs from
/// the type defined in the initial call.
pub struct UnexpectedSecurityType(&'static str);

impl std::fmt::Display for UnexpectedSecurityType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
    use serde::Serialize;

    use super::{Commodity, Contract, Crypto, Forex, Index, SecFuture, SecOption, Stock};

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
    /// Get the security's contract ID
    ///
    /// # Returns
    /// The security's unique contract ID
    fn contract_id(&self) -> ContractId;
    /// Get the security's minimum tick size.
    ///
    /// # Returns
    /// The security's minimum tick size
    fn min_tick(&self) -> f64;
    /// Get the security's symbol.
    ///
    /// # Returns
    /// The security's symbol.
    fn symbol(&self) -> &str;
    /// Get the security's currency.
    ///
    /// # Returns
    /// The security's currency.
    fn currency(&self) -> Currency;
    /// Get the security's local symbol.
    ///
    /// # Returns
    /// The security's local symbol.
    fn local_symbol(&self) -> &str;
    /// Get the security's long name.
    ///
    /// # Returns
    /// The security's long name.
    fn long_name(&self) -> &str;
    /// Get the security's order types.
    ///
    /// # Returns
    /// The security's order types.
    fn order_types(&self) -> &Vec<String>;
    /// Get the security's valid exchanges.
    ///
    /// # Returns
    /// The security's valid exchanges.
    fn valid_exchanges(&self) -> &Vec<Routing>;
    /// Get the security's contract type.
    ///
    /// # Returns
    /// The security's contract type.
    fn contract_type(&self) -> ContractType;
}

// =======================================
// === Definitions of Contract Structs ===
// =======================================

macro_rules! make_contract {
    ($( #[doc = $name_doc:expr] )? $name: ident $(,$trt: ident)?; $($field: ident: $f_type: ty),* $(,)?) => {
        $( #[doc = $name_doc] )?
        #[make_getters]
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

impl SecOption {
    #[must_use]
    #[inline]
    /// Construct a new option from its class and inner contract
    pub fn from_components(class: SecOptionClass, inner: SecOptionInner) -> Self {
        match class {
            SecOptionClass::Call => SecOption::Call(inner),
            SecOptionClass::Put => SecOption::Put(inner),
        }
    }

    #[must_use]
    #[inline]
    /// Return `true` if the option is a call option.
    pub fn is_call(&self) -> bool {
        matches!(self, SecOption::Call(_))
    }

    #[must_use]
    #[inline]
    /// Return `true` if the option is a put option.
    pub fn is_put(&self) -> bool {
        !self.is_call()
    }

    #[must_use]
    #[inline]
    /// Get the option's class
    pub fn class(&self) -> SecOptionClass {
        match self {
            SecOption::Call(_) => SecOptionClass::Call,
            SecOption::Put(_) => SecOptionClass::Put,
        }
    }

    #[must_use]
    #[inline]
    /// Get a reference to the inner contract's specifications.
    pub fn as_inner_ref(&self) -> &SecOptionInner {
        let (SecOption::Call(inner) | SecOption::Put(inner)) = self;
        inner
    }

    #[must_use]
    #[inline]
    /// Transform the option into the inner contract
    pub fn into_inner(self) -> SecOptionInner {
        let (SecOption::Call(inner) | SecOption::Put(inner)) = self;
        inner
    }

    #[must_use]
    #[inline]
    /// Unfold the option into its class and inner contract
    pub fn unfold(self) -> (SecOptionClass, SecOptionInner) {
        (self.class(), self.into_inner())
    }

    #[must_use]
    #[inline]
    /// Get the inner contract's exchange
    pub fn exchange(&self) -> Routing {
        self.as_inner_ref().exchange
    }

    #[must_use]
    #[inline]
    /// Get the inner contract's strike price
    pub fn strike(&self) -> f64 {
        self.as_inner_ref().strike
    }

    #[must_use]
    #[inline]
    /// Get the inner contract's multiplier
    pub fn multiplier(&self) -> u32 {
        self.as_inner_ref().multiplier
    }

    #[must_use]
    #[inline]
    /// Get the inner contract's expiration date
    pub fn expiration_date(&self) -> NaiveDate {
        self.as_inner_ref().expiration_date
    }

    #[must_use]
    #[inline]
    /// Get the underlying security's contract ID for the inner contract
    pub fn underlying_contract_id(&self) -> ContractId {
        self.as_inner_ref().contract_id
    }

    #[must_use]
    #[inline]
    /// Get a reference to the inner contract's sector
    pub fn sector(&self) -> &str {
        &self.as_inner_ref().sector
    }

    #[must_use]
    #[inline]
    /// Get a reference to the inner contract's trading class
    pub fn trading_class(&self) -> &str {
        &self.as_inner_ref().trading_class
    }
}

impl From<(SecOptionClass, SecOptionInner)> for SecOption {
    #[inline]
    fn from(value: (SecOptionClass, SecOptionInner)) -> Self {
        Self::from_components(value.0, value.1)
    }
}

impl From<(SecOptionInner, SecOptionClass)> for SecOption {
    #[inline]
    fn from(value: (SecOptionInner, SecOptionClass)) -> Self {
        Self::from_components(value.1, value.0)
    }
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

macro_rules! proxy_impl {
    ($sec_type: ty, $pat: pat_param => $exp: expr, $func_name: ident) => {
        #[doc=concat!("Coerce the `Proxy<Contract>` to a `Proxy<", stringify!($sec_type), ">`.")]
        ///
        /// # Returns
        #[doc=concat!("A new `Proxy<", stringify!($sec_type), ">`, if the underlying contract is a ", stringify!($sec_type), " otherwise, `None`.")]
        pub fn $func_name(self) -> Option<Proxy<$sec_type>> {
            match self.inner {
                $pat => Some($exp),
                _ => None,
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(into = "SerProxyHelp", try_from = "SerProxyHelp")]
/// Holds information about a contract but lacks the information of a full [`Contract`].
pub struct Proxy<S: Security + Clone> {
    #[serde(serialize_with = "_dummy_ser", deserialize_with = "_dummy_de")]
    // Temporary until https://github.com/serde-rs/serde/pull/2239 is merged
    pub(crate) inner: S,
}

#[allow(clippy::needless_pass_by_value)]
fn _dummy_ser<Sec: Security + Clone, Ser: Serializer>(
    _t: &Proxy<Sec>,
    _ser: Ser,
) -> Result<Ser::Ok, Ser::Error> {
    unreachable!()
}

#[allow(clippy::needless_pass_by_value)]
fn _dummy_de<'de, Sec: Security + Clone, De: Deserializer<'de>>(
    _de: De,
) -> Result<Proxy<Sec>, De::Error> {
    unreachable!()
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
struct SerProxyHelp {
    contract_type: ContractType,
    contract_id: ContractId,
    symbol: String,
    currency: Currency,
    local_symbol: String,
    trading_class: Option<String>,
    primary_exchange: Option<Primary>,
    expiration_date: Option<NaiveDate>,
    multiplier: Option<u32>,
    option_type: Option<SecOptionClass>,
    strike: Option<f64>,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
/// The possible option classes
pub enum SecOptionClass {
    /// A call option
    Call,
    /// A put option
    Put,
}

impl<S: Security + Clone> From<Proxy<S>> for SerProxyHelp {
    #[allow(clippy::too_many_lines)]
    fn from(value: Proxy<S>) -> Self {
        let contract_type = value.contract_type();
        let contract_id = value.contract_id();
        let currency = value.currency();

        match value.inner.into() {
            Contract::Stock(stk) => Self {
                contract_type,
                contract_id,
                symbol: stk.symbol,
                currency,
                local_symbol: stk.local_symbol,
                trading_class: Some(stk.trading_class),
                primary_exchange: Some(stk.primary_exchange),
                expiration_date: None,
                multiplier: None,
                strike: None,
                option_type: None,
            },
            Contract::SecOption(opt) => {
                let option_type = Some(if opt.is_call() {
                    SecOptionClass::Call
                } else {
                    SecOptionClass::Put
                });
                let opt = opt.into_inner();
                Self {
                    contract_type,
                    contract_id,
                    symbol: opt.symbol,
                    currency,
                    local_symbol: opt.local_symbol,
                    trading_class: Some(opt.trading_class),
                    primary_exchange: None,
                    expiration_date: Some(opt.expiration_date),
                    multiplier: Some(opt.multiplier),
                    strike: Some(opt.strike),
                    option_type,
                }
            }
            Contract::SecFuture(fut) => Self {
                contract_type,
                contract_id,
                symbol: fut.symbol,
                currency,
                local_symbol: fut.local_symbol,
                trading_class: Some(fut.trading_class),
                primary_exchange: None,
                expiration_date: Some(fut.expiration_date),
                multiplier: Some(fut.multiplier),
                strike: None,
                option_type: None,
            },
            Contract::Commodity(cmdty) => Self {
                contract_type,
                contract_id,
                symbol: cmdty.symbol,
                currency,
                local_symbol: cmdty.local_symbol,
                trading_class: Some(cmdty.trading_class),
                primary_exchange: None,
                expiration_date: None,
                multiplier: None,
                strike: None,
                option_type: None,
            },
            Contract::Crypto(crypto) => Self {
                contract_type,
                contract_id,
                symbol: crypto.symbol,
                currency,
                local_symbol: crypto.local_symbol,
                trading_class: Some(crypto.trading_class),
                primary_exchange: None,
                expiration_date: None,
                multiplier: None,
                strike: None,
                option_type: None,
            },
            Contract::Index(ind) => Self {
                contract_type,
                contract_id,
                symbol: ind.symbol,
                currency,
                local_symbol: ind.local_symbol,
                trading_class: None,
                primary_exchange: None,
                expiration_date: None,
                multiplier: None,
                strike: None,
                option_type: None,
            },
            Contract::Forex(fx) => Self {
                contract_type,
                contract_id,
                symbol: fx.symbol,
                currency,
                local_symbol: fx.local_symbol,
                trading_class: Some(fx.trading_class),
                primary_exchange: None,
                expiration_date: None,
                multiplier: None,
                strike: None,
                option_type: None,
            },
        }
    }
}

impl<S: Security + Clone> TryFrom<SerProxyHelp> for Proxy<S> {
    type Error = anyhow::Error;

    #[allow(clippy::too_many_lines)]
    fn try_from(value: SerProxyHelp) -> Result<Self, Self::Error> {
        let SerProxyHelp {
            contract_type,
            trading_class,
            contract_id,
            symbol,
            currency,
            local_symbol,
            primary_exchange,
            multiplier,
            strike,
            option_type,
            expiration_date,
        } = value;

        let inner = match contract_type {
            ContractType::Stock => Stock {
                contract_id,
                min_tick: f64::default(),
                symbol,
                currency,
                local_symbol,
                long_name: String::default(),
                order_types: Vec::default(),
                trading_class: trading_class.ok_or(anyhow::anyhow!("Missing data"))?,
                primary_exchange: primary_exchange.ok_or(anyhow::anyhow!("Missing data"))?,
                stock_type: String::default(),
                security_ids: Vec::default(),
                exchange: Routing::Smart,
                sector: String::default(),
                valid_exchanges: Vec::default(),
            }
            .try_into()
            .map_err(|_| ()),
            ContractType::Index => Index {
                contract_id,
                min_tick: f64::default(),
                symbol,
                exchange: Routing::Smart,
                currency,
                local_symbol,
                long_name: String::default(),
                order_types: Vec::default(),
                valid_exchanges: Vec::default(),
            }
            .try_into()
            .map_err(|_| ()),
            ContractType::Commodity => Commodity {
                contract_id,
                min_tick: f64::default(),
                symbol,
                exchange: Routing::Smart,
                trading_class: trading_class.ok_or(anyhow::anyhow!("Missing data"))?,
                currency,
                local_symbol,
                long_name: String::default(),
                order_types: Vec::default(),
                valid_exchanges: Vec::default(),
            }
            .try_into()
            .map_err(|_| ()),
            ContractType::Crypto => Crypto {
                contract_id,
                min_tick: f64::default(),
                symbol,
                trading_class: trading_class.ok_or(anyhow::anyhow!("Missing data"))?,
                currency,
                local_symbol,
                long_name: String::default(),
                order_types: Vec::default(),
                valid_exchanges: Vec::default(),
            }
            .try_into()
            .map_err(|_| ()),
            ContractType::Forex => Forex {
                contract_id,
                min_tick: f64::default(),
                symbol,
                exchange: Routing::Smart,
                trading_class: trading_class.ok_or(anyhow::anyhow!("Missing data"))?,
                currency,
                local_symbol,
                long_name: String::default(),
                order_types: Vec::default(),
                valid_exchanges: Vec::default(),
            }
            .try_into()
            .map_err(|_| ()),
            ContractType::SecFuture => SecFuture {
                contract_id,
                min_tick: f64::default(),
                symbol,
                exchange: Routing::Smart,
                multiplier: multiplier.ok_or(anyhow::anyhow!("Missing data"))?,
                expiration_date: expiration_date.ok_or(anyhow::anyhow!("Missing data"))?,
                trading_class: trading_class.ok_or(anyhow::anyhow!("Missing data"))?,
                underlying_contract_id: contract_id,
                currency,
                local_symbol,
                long_name: String::default(),
                order_types: Vec::default(),
                valid_exchanges: Vec::default(),
            }
            .try_into()
            .map_err(|_| ()),
            ContractType::SecOption => {
                let inner = SecOptionInner {
                    contract_id,
                    min_tick: f64::default(),
                    symbol,
                    exchange: Routing::Smart,
                    strike: strike.ok_or(anyhow::anyhow!("Missing data"))?,
                    multiplier: multiplier.ok_or(anyhow::anyhow!("Missing data"))?,
                    expiration_date: expiration_date.ok_or(anyhow::anyhow!("Missing data"))?,
                    underlying_contract_id: contract_id,
                    sector: String::default(),
                    trading_class: trading_class.ok_or(anyhow::anyhow!("Missing data"))?,
                    currency,
                    local_symbol,
                    long_name: String::default(),
                    order_types: Vec::default(),
                    valid_exchanges: Vec::default(),
                };
                match option_type.ok_or(anyhow::anyhow!("Missing data"))? {
                    SecOptionClass::Call => SecOption::Call(inner),
                    SecOptionClass::Put => SecOption::Put(inner),
                }
                .try_into()
                .map_err(|_| ())
            }
        }
        .map_err(|()| anyhow::anyhow!("Failed to coerce contract into desired security type."))?;

        Ok(Self { inner })
    }
}

impl<S: Security + Clone> From<Proxy<S>> for ContractId {
    fn from(value: Proxy<S>) -> Self {
        value.inner.contract_id()
    }
}

impl<S: Security + Clone> Proxy<S> {
    #[inline]
    /// Get the type of contract.
    pub fn contract_type(&self) -> ContractType {
        self.inner.contract_type()
    }
    #[inline]
    /// Get the underlying Security's contract ID.
    pub fn contract_id(&self) -> ContractId {
        self.inner.contract_id()
    }

    #[inline]
    /// Get the underlying Security's symbol.
    pub fn symbol(&self) -> &str {
        self.inner.symbol()
    }

    #[inline]
    /// Get the underlying Security's currency.
    pub fn currency(&self) -> Currency {
        self.inner.currency()
    }

    #[inline]
    /// Get the underlying Security's symbol.
    pub fn local_symbol(&self) -> &str {
        self.inner.symbol()
    }
}

impl Proxy<Contract> {
    proxy_impl!(Forex, Contract::Forex(t) => Proxy::<Forex> { inner: t }, forex);
    proxy_impl!(Crypto, Contract::Crypto(t) => Proxy::<Crypto> { inner: t }, crypto);
    proxy_impl!(Stock, Contract::Stock(t) => Proxy::<Stock> { inner: t }, stock);
    proxy_impl!(Index, Contract::Index(t) => Proxy::<Index> { inner: t }, index);
    proxy_impl!(Commodity, Contract::Commodity(t) => Proxy::<Commodity> { inner: t }, commodity);
    proxy_impl!(SecFuture, Contract::SecFuture(t) => Proxy::<SecFuture> { inner: t }, sec_future);
    proxy_impl!(SecOption, Contract::SecOption(t) => Proxy::<SecOption> { inner: t }, sec_option);
}

impl Proxy<Forex> {
    #[inline]
    #[must_use]
    /// Get the [`Forex`] trading class.
    pub fn trading_class(&self) -> &str {
        self.inner.trading_class()
    }
}

impl Proxy<Crypto> {
    #[inline]
    #[must_use]
    /// Get the [`Crypto`] trading class.
    pub fn trading_class(&self) -> &str {
        self.inner.trading_class()
    }
}

impl Proxy<Stock> {
    #[inline]
    #[must_use]
    /// Get the [`Stock`] trading class.
    pub fn trading_class(&self) -> &str {
        self.inner.trading_class()
    }

    #[inline]
    #[must_use]
    /// Get the [`Stock`] primary exchange.
    pub fn primary_exchange(&self) -> Primary {
        self.inner.primary_exchange
    }
}

impl Proxy<Commodity> {
    #[inline]
    #[must_use]
    /// Get the [`Commodity`] trading class.
    pub fn trading_class(&self) -> &str {
        self.inner.trading_class()
    }
}

impl Proxy<SecFuture> {
    #[inline]
    #[must_use]
    /// Get the [`SecFuture`] trading class.
    pub fn trading_class(&self) -> &str {
        self.inner.trading_class()
    }

    #[inline]
    #[must_use]
    /// Get the [`SecFuture`] `expiration_date`.
    pub fn expiration_date(&self) -> NaiveDate {
        self.inner.expiration_date
    }

    #[inline]
    #[must_use]
    /// Get the [`SecFuture`] `multiplier`.
    pub fn multiplier(&self) -> u32 {
        self.inner.multiplier
    }
}

impl Proxy<SecOption> {
    #[inline]
    #[must_use]
    /// Get the [`SecOption`] trading class.
    pub fn trading_class(&self) -> &str {
        self.inner.as_inner_ref().trading_class.as_str()
    }

    #[inline]
    #[must_use]
    /// Get the [`SecOption`] `expiration_date`.
    pub fn expiration_date(&self) -> NaiveDate {
        self.inner.as_inner_ref().expiration_date
    }

    #[inline]
    #[must_use]
    /// Get the [`SecOption`] `strike` price.
    pub fn strike(&self) -> f64 {
        self.inner.as_inner_ref().strike
    }

    #[inline]
    #[must_use]
    /// Return true if the [`SecOption`] is a call.
    pub fn is_call(&self) -> bool {
        self.inner.is_call()
    }

    #[inline]
    #[must_use]
    /// Return true if the [`SecOption`] is a put.
    pub fn is_put(&self) -> bool {
        self.inner.is_put()
    }

    #[inline]
    #[must_use]
    /// Get the [`SecOption`] `multiplier`.
    pub fn multiplier(&self) -> u32 {
        self.inner.as_inner_ref().multiplier
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
/// The possible contract types
pub enum ContractType {
    #[serde(rename = "CASH")]
    /// A [`Forex`] contract.
    Forex,
    #[serde(rename = "CRYPTO")]
    /// A [`Crypto`] contract.
    Crypto,
    #[serde(rename = "STK")]
    /// A [`Stock`] contract.
    Stock,
    #[serde(rename = "IND")]
    /// An [`Index`] contract.
    Index,
    //Cfd,
    #[serde(rename = "FUT")]
    /// A [`SecFuture`] contract.
    SecFuture,
    #[serde(rename = "OPT")]
    /// A [`SecOption`] contract.
    SecOption,
    //FutureSecOption,
    //Bond,
    //MutualFund,
    #[serde(rename = "CMDTY")]
    /// A [`Commodity`] contract.
    Commodity,
    //Warrant,
    //StructuredProduct,
}

impl FromStr for ContractType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "CASH" => Self::Forex,
            "CRYPTO" => Self::Crypto,
            "STK" => Self::Stock,
            "IND" => Self::Index,
            "FUT" => Self::SecFuture,
            "OPT" => Self::SecOption,
            "CMDTY" => Self::Commodity,
            v => return Err(anyhow::anyhow!("Invalid contract type {}", v)),
        })
    }
}
