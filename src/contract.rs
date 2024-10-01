use std::{num::ParseIntError, str::FromStr};
use std::fmt::{Debug, Formatter};
use std::hash::Hash;

use chrono::NaiveDate;
use ibapi_macros::{make_getters, Security};
use serde::{Deserialize, Deserializer, ser::SerializeStruct, Serialize, Serializer};
use thiserror::Error;

use crate::{
    currency::Currency,
    exchange::{Primary, Routing},
    match_poly,
};
use crate::contract::proxy_indicators::{HasExchange, NoExchange};
use crate::figi::{Figi, InvalidFigi};

// =========================================================
// === Utility Types and Functions for Contract Creation ===
// =========================================================

// todo!("Ensure that includeExpired is always set to true");

#[derive(Debug, Clone, PartialEq)]
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
        #[inline]
        #[must_use]
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
        #[inline]
        #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
/// A fully-defined contract that can be used for market data, placing orders, etc.
pub async fn new<S: Security>(
    client: &mut crate::client::ActiveClient,
    query: Query,
) -> Result<S, NewSecurityError> {
    client.send_contract_query(query).await?;
    client
        .recv_contract_query()
        .await
        .ok_or(NewSecurityError::BadResponse)?
        .try_into()
        .map_err(|e: <S as TryFrom<Contract>>::Error| {
            NewSecurityError::UnexpectedSecurityType(e.into())
        })
}

#[derive(Debug, Error)]
/// An error type that is returned if creating a [`new`] [`Security`] fails
pub enum NewSecurityError {
    /// Failed to send the contract query to the IBKR API
    #[error("Failed to send contract query to IBKR API. Cause {0}")]
    Io(#[from] std::io::Error),
    /// Failed to receive valid response form the IBKR API
    #[error("No valid contract received from the IBKR API.")]
    BadResponse,
    /// Unexpected security type returned from the IBKR API
    #[error("Invalid contract received from the IBKR API. {0}")]
    UnexpectedSecurityType(#[from] UnexpectedSecurityType),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Error)]
#[error("Unexpected security type. Expected {expected:?}. Found {found:?}")]
/// An error type that's returned when a [`Security`] of type `S` is requested, but a security of
/// another type is received from the API
pub struct UnexpectedSecurityType {
    /// The expected contract type
    expected: ContractType,
    /// The contract type that was actually found
    found: ContractType,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
/// A type used to represent a query for a new contract, which can be made by providing either an
/// IBKR contract ID, or a FIGI.
pub enum Query {
    /// An IBKR contract ID with which to make a query. When parsing from a string, the routing field
    /// defaults to [`Routing::Smart`].
    IbContractId(ContractId, Routing),
    /// A FIGI.
    Figi(Figi),
}

impl From<ContractId> for Query {
    fn from(value: ContractId) -> Self {
        Self::IbContractId(value, Routing::Smart)
    }
}

impl From<Figi> for Query {
    fn from(value: Figi) -> Self {
        Self::Figi(value)
    }
}

#[derive(Debug, Clone, Error)]
/// An error type representing the potential ways that a [`Query`] can be invalid.
pub enum ParseQueryError {
    #[error("Invalid value when parsing IBKR contract ID. Cause: {0}")]
    /// An invalid [`Query::IbContractId`]
    IbContractId(ParseContractIdError),
    #[error("Invalid value when parsing FIGI. Cause: {0}")]
    /// AN invalid [`Query::Figi`]
    Figi(InvalidFigi),
    #[error("Cannot construct query from empty string.")]
    /// Invalid in a way such that it's impossible to tell whether it was intended to be an [`Query::IbContractId`] or a [`Query::Figi`].
    Empty,
}

impl FromStr for Query {
    type Err = ParseQueryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // A FIGI always begin with a letter
        if s.chars().nth(0).ok_or(ParseQueryError::Empty)?.is_numeric() {
            Ok(Self::IbContractId(
                s.parse().map_err(ParseQueryError::IbContractId)?,
                Routing::Smart,
            ))
        } else {
            Ok(Self::Figi(s.parse().map_err(ParseQueryError::Figi)?))
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
/// A unique identifier used by both IBKR's trading systems and the API to define a specific
/// contract.
pub struct ContractId(pub i64);

#[derive(Debug, Clone, Error)]
#[error("Invalid value encountered when attempting to parse contract ID. Cause: {0}")]
/// An error returned when parsing a [`ContractId`] fails.
pub struct ParseContractIdError(pub ParseIntError);

impl FromStr for ContractId {
    type Err = ParseContractIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Self).map_err(ParseContractIdError)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    use std::convert::Infallible;

    use chrono::NaiveDate;
    use serde::{Serialize, Serializer};

    use crate::currency::Currency;
    use crate::exchange::{Primary, Routing};
    use crate::match_poly;

    use super::{
        Commodity, Contract, ContractId, Crypto, Forex, Index, SecFuture, SecOption, Stock,
        UnexpectedSecurityType,
    };

    #[derive(Debug, Clone, PartialEq)]
    pub struct SecurityOutMsg<'s> {
        pub contract_id: ContractId,
        pub symbol: &'s str,
        pub security_type: &'static str,
        pub expiration_date: Option<NaiveDate>,
        pub strike: Option<f64>,
        pub right: Option<&'static str>,
        pub multiplier: Option<u32>,
        pub exchange: Routing,
        pub primary_exchange: Option<Primary>,
        pub currency: Currency,
        pub local_symbol: &'s str,
        pub trading_class: Option<&'s str>,
    }

    impl Serialize for SecurityOutMsg<'_> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            (
                self.contract_id,
                self.symbol,
                self.security_type,
                self.expiration_date.map(|d| d.format("%Y%m%d").to_string()),
                self.strike,
                self.right,
                self.multiplier,
                self.exchange,
                self.primary_exchange,
                self.currency,
                self.local_symbol,
                self.trading_class,
            )
                .serialize(serializer)
        }
    }

    pub trait Valid:
        Serialize
        + Send
        + Sync
        + TryFrom<Forex, Error: Into<UnexpectedSecurityType>>
        + TryFrom<Crypto, Error: Into<UnexpectedSecurityType>>
        + TryFrom<Stock, Error: Into<UnexpectedSecurityType>>
        + TryFrom<Index, Error: Into<UnexpectedSecurityType>>
        + TryFrom<SecFuture, Error: Into<UnexpectedSecurityType>>
        + TryFrom<SecOption, Error: Into<UnexpectedSecurityType>>
        + TryFrom<Commodity, Error: Into<UnexpectedSecurityType>>
        + TryFrom<Contract, Error: Into<UnexpectedSecurityType>>
        + Into<Contract>
    {
        fn as_out_msg(&self) -> SecurityOutMsg<'_>;
    }

    impl Valid for Contract {
        fn as_out_msg(&self) -> SecurityOutMsg<'_> {
            match_poly!(self;
                Self::Forex(t)
                | Self::Crypto(t)
                | Self::Stock(t)
                | Self::Index(t)
                | Self::SecFuture(t)
                | Self::SecOption(t)
                | Self::Commodity(t) => t.as_out_msg()
            )
        }
    }

    impl From<Infallible> for UnexpectedSecurityType {
        fn from(_: Infallible) -> Self {
            unreachable!()
        }
    }
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
        #[derive(Debug, Clone, PartialEq, $($trt)?)]
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

#[derive(Debug, Clone, PartialEq, Security)]
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
        self.as_inner_ref().underlying_contract_id
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

// #[derive(Debug, Clone, PartialEq)]
// pub enum SecFutureOption {
//     Call(SecOptionInner),
//     Put(SecOptionInner),
// }

// #[derive(Debug, Clone, PartialEq)]
// pub enum Warrant {
//     Call(SecOptionInner),
//     Put(SecOptionInner),
// }

macro_rules! proxy_impl {
    ($sec_type: ty, $pat: pat_param => $exp: expr, $func_name: ident) => {
        #[inline]
        #[must_use]
        #[doc=concat!("Coerce the `Proxy<Contract>` to a `Proxy<", stringify!($sec_type), ">`.")]
        ///
        /// # Returns
        #[doc=concat!("A new `Proxy<", stringify!($sec_type), ">`, if the underlying contract is a ", stringify!($sec_type), " otherwise, `None`.")]
        pub fn $func_name(self) -> Option<Proxy<$sec_type, E>> {
            match (self.inner, self._exch) {
                $pat => Some($exp),
                _ => None,
            }
        }
    };
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(into = "SerProxyHelp", try_from = "SerProxyHelp")]
/// Holds information about a contract but lacks the information of a full [`Contract`].
pub struct Proxy<S: Security + Clone + Debug, E: ProxyExchange> {
    #[serde(serialize_with = "_dummy_ser", deserialize_with = "_dummy_de")]
    // Temporary until https://github.com/serde-rs/serde/pull/2239 is merged
    pub(crate) inner: S,
    // Temporary until https://github.com/serde-rs/serde/pull/2239 is merged
    #[serde(serialize_with = "_dummy_ser_2", deserialize_with = "_dummy_de_2")]
    pub(crate) _exch: std::marker::PhantomData<E>,
}

impl<S: Security + Clone + Debug, E: ProxyExchange> Debug for Proxy<S, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Proxy({:?})", self.inner)
    }
}

/// A proxy for an underlying contract where the contract has an exchange field
pub type ExchangeProxy<S> = Proxy<S, HasExchange>;
/// A proxy for an underlying contract where the contract does not have an exchange field
pub type NoExchangeProxy<S> = Proxy<S, NoExchange>;

/// An indicator trait used to identify the two types of valid [`Proxy`] contracts
pub trait ProxyExchange: proxy_indicators::Valid {}

pub(crate) mod proxy_indicators {
    use crate::decode::DecodeError;
    use crate::exchange::{Primary, Routing};

    use super::ProxyExchange;

    pub trait Valid: std::fmt::Debug + Send + Sync + Clone {
        // False positive
        #[allow(private_interfaces)]
        fn decode(exch_or_primary: String) -> Result<(Routing, Primary), DecodeError>;
        fn deserialize(
            exch: Option<Routing>,
            primary: Option<Primary>,
        ) -> (Option<Routing>, Option<Primary>);
        fn get_primary(primary: Primary) -> Option<Primary>;
        fn get_exchange(routing: Routing) -> Option<Routing>;
    }

    #[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct HasExchange;

    #[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct NoExchange;

    impl Valid for HasExchange {
        // False positive
        #[allow(private_interfaces)]
        #[inline]
        fn decode(exch_or_primary: String) -> Result<(Routing, Primary), DecodeError> {
            Ok((
                exch_or_primary.parse().map_err(|e| ("exchange", e))?,
                Primary::InteractiveBrokersDealingSystem,
            ))
        }

        #[inline]
        fn deserialize(
            exch: Option<Routing>,
            _primary: Option<Primary>,
        ) -> (Option<Routing>, Option<Primary>) {
            (exch, Some(Primary::InteractiveBrokersDealingSystem))
        }
        #[inline]
        fn get_primary(_primary: Primary) -> Option<Primary> {
            None
        }
        #[inline]
        fn get_exchange(routing: Routing) -> Option<Routing> {
            Some(routing)
        }
    }

    impl Valid for NoExchange {
        // False positive
        #[allow(private_interfaces)]
        #[inline]
        fn decode(exch_or_primary: String) -> Result<(Routing, Primary), DecodeError> {
            Ok((
                Routing::Smart,
                exch_or_primary
                    .parse()
                    .map_err(|e| ("primary_exchange", e))?,
            ))
        }
        #[inline]
        fn deserialize(
            _exch: Option<Routing>,
            primary: Option<Primary>,
        ) -> (Option<Routing>, Option<Primary>) {
            (Some(Routing::Smart), primary)
        }
        #[inline]
        fn get_primary(primary: Primary) -> Option<Primary> {
            Some(primary)
        }
        #[inline]
        fn get_exchange(_routing: Routing) -> Option<Routing> {
            None
        }
    }

    impl ProxyExchange for HasExchange {}

    impl ProxyExchange for NoExchange {}
}

#[allow(clippy::needless_pass_by_value)]
fn _dummy_ser<E: ProxyExchange, Sec: Security + Clone + Debug, Ser: Serializer>(
    _t: &Proxy<Sec, E>,
    _ser: Ser,
) -> Result<Ser::Ok, Ser::Error> {
    unreachable!()
}

#[allow(clippy::needless_pass_by_value)]
fn _dummy_de<'de, E: ProxyExchange, Sec: Security + Clone + Debug, De: Deserializer<'de>>(
    _de: De,
) -> Result<Proxy<Sec, E>, De::Error> {
    unreachable!()
}

#[allow(clippy::needless_pass_by_value)]
fn _dummy_ser_2<E: ProxyExchange, Sec: Security + Clone + Debug, Ser: Serializer>(
    _t: &Proxy<Sec, E>,
    _ser: Ser,
) -> Result<Ser::Ok, Ser::Error> {
    unreachable!()
}

#[allow(clippy::needless_pass_by_value)]
fn _dummy_de_2<'de, E: ProxyExchange, Sec: Security + Clone + Debug, De: Deserializer<'de>>(
    _de: De,
) -> Result<Proxy<Sec, E>, De::Error> {
    unreachable!()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct SerProxyHelp {
    contract_type: ContractType,
    contract_id: ContractId,
    symbol: String,
    currency: Currency,
    local_symbol: String,
    exchange: Option<Routing>,
    trading_class: Option<String>,
    primary_exchange: Option<Primary>,
    expiration_date: Option<NaiveDate>,
    multiplier: Option<u32>,
    option_type: Option<SecOptionClass>,
    strike: Option<f64>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
/// The possible option classes
pub enum SecOptionClass {
    /// A call option
    Call,
    /// A put option
    Put,
}

impl From<SecOptionClass> for char {
    fn from(value: SecOptionClass) -> Self {
        match value {
            SecOptionClass::Call => 'C',
            SecOptionClass::Put => 'P',
        }
    }
}

impl<S: Security + Clone + Debug, E: ProxyExchange> From<Proxy<S, E>> for SerProxyHelp {
    #[allow(clippy::too_many_lines)]
    fn from(value: Proxy<S, E>) -> Self {
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
                exchange: E::get_exchange(stk.exchange),
                trading_class: Some(stk.trading_class),
                primary_exchange: E::get_primary(stk.primary_exchange),
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
                    exchange: E::get_exchange(opt.exchange),
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
                exchange: E::get_exchange(fut.exchange),
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
                exchange: E::get_exchange(cmdty.exchange),
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
                exchange: Some(Routing::Primary(Primary::PaxosCryptoExchange)),
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
                exchange: E::get_exchange(ind.exchange),
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
                exchange: E::get_exchange(fx.exchange),
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

impl<S: Security + Clone + Debug, E: ProxyExchange> TryFrom<SerProxyHelp> for Proxy<S, E> {
    type Error = SerializeProxyError;

    #[allow(clippy::too_many_lines)]
    fn try_from(value: SerProxyHelp) -> Result<Self, Self::Error> {
        let SerProxyHelp {
            contract_type,
            trading_class,
            contract_id,
            symbol,
            currency,
            local_symbol,
            exchange,
            primary_exchange,
            multiplier,
            strike,
            option_type,
            expiration_date,
        } = value;

        let (exchange, primary_exchange) = E::deserialize(exchange, primary_exchange);

        let inner: Result<S, UnexpectedSecurityType> = match contract_type {
            ContractType::Stock => Stock {
                contract_id,
                min_tick: f64::default(),
                symbol,
                currency,
                local_symbol,
                long_name: String::default(),
                order_types: Vec::default(),
                trading_class: trading_class
                    .ok_or(SerializeProxyError::MissingData("trading_class"))?,
                primary_exchange: primary_exchange
                    .ok_or(SerializeProxyError::MissingData("primary_exchange"))?,
                stock_type: String::default(),
                security_ids: Vec::default(),
                exchange: exchange.ok_or(SerializeProxyError::MissingData("exchange"))?,
                sector: String::default(),
                valid_exchanges: Vec::default(),
            }
            .try_into()
            .map_err(|e: <S as TryFrom<Stock>>::Error| e.into()),
            ContractType::Index => Index {
                contract_id,
                min_tick: f64::default(),
                symbol,
                exchange: exchange.ok_or(SerializeProxyError::MissingData("exchange"))?,
                currency,
                local_symbol,
                long_name: String::default(),
                order_types: Vec::default(),
                valid_exchanges: Vec::default(),
            }
            .try_into()
            .map_err(|e: <S as TryFrom<Index>>::Error| e.into()),
            ContractType::Commodity => Commodity {
                contract_id,
                min_tick: f64::default(),
                symbol,
                exchange: exchange.ok_or(SerializeProxyError::MissingData("exchange"))?,
                trading_class: trading_class
                    .ok_or(SerializeProxyError::MissingData("trading_class"))?,
                currency,
                local_symbol,
                long_name: String::default(),
                order_types: Vec::default(),
                valid_exchanges: Vec::default(),
            }
            .try_into()
            .map_err(|e: <S as TryFrom<Commodity>>::Error| e.into()),
            ContractType::Crypto => Crypto {
                contract_id,
                min_tick: f64::default(),
                symbol,
                trading_class: trading_class
                    .ok_or(SerializeProxyError::MissingData("trading_class"))?,
                currency,
                local_symbol,
                long_name: String::default(),
                order_types: Vec::default(),
                valid_exchanges: Vec::default(),
            }
            .try_into()
            .map_err(|e: <S as TryFrom<Crypto>>::Error| e.into()),
            ContractType::Forex => Forex {
                contract_id,
                min_tick: f64::default(),
                symbol,
                exchange: exchange.ok_or(SerializeProxyError::MissingData("exchange"))?,
                trading_class: trading_class
                    .ok_or(SerializeProxyError::MissingData("trading_class"))?,
                currency,
                local_symbol,
                long_name: String::default(),
                order_types: Vec::default(),
                valid_exchanges: Vec::default(),
            }
            .try_into()
            .map_err(|e: <S as TryFrom<Forex>>::Error| e.into()),
            ContractType::SecFuture => SecFuture {
                contract_id,
                min_tick: f64::default(),
                symbol,
                exchange: exchange.ok_or(SerializeProxyError::MissingData("exchange"))?,
                multiplier: multiplier.ok_or(SerializeProxyError::MissingData("multiplier"))?,
                expiration_date: expiration_date
                    .ok_or(SerializeProxyError::MissingData("expiration_date"))?,
                trading_class: trading_class
                    .ok_or(SerializeProxyError::MissingData("trading_class"))?,
                underlying_contract_id: contract_id,
                currency,
                local_symbol,
                long_name: String::default(),
                order_types: Vec::default(),
                valid_exchanges: Vec::default(),
            }
            .try_into()
            .map_err(|e: <S as TryFrom<SecFuture>>::Error| e.into()),
            ContractType::SecOption => {
                let inner = SecOptionInner {
                    contract_id,
                    min_tick: f64::default(),
                    symbol,
                    exchange: exchange.ok_or(SerializeProxyError::MissingData("exchange"))?,
                    strike: strike.ok_or(SerializeProxyError::MissingData("strike"))?,
                    multiplier: multiplier.ok_or(SerializeProxyError::MissingData("multiplier"))?,
                    expiration_date: expiration_date
                        .ok_or(SerializeProxyError::MissingData("expiration_date"))?,
                    underlying_contract_id: contract_id,
                    sector: String::default(),
                    trading_class: trading_class
                        .ok_or(SerializeProxyError::MissingData("trading_class"))?,
                    currency,
                    local_symbol,
                    long_name: String::default(),
                    order_types: Vec::default(),
                    valid_exchanges: Vec::default(),
                };
                match option_type.ok_or(SerializeProxyError::MissingData("option_type"))? {
                    SecOptionClass::Call => SecOption::Call(inner),
                    SecOptionClass::Put => SecOption::Put(inner),
                }
            }
            .try_into()
            .map_err(|e: <S as TryFrom<SecOption>>::Error| e.into()),
        };

        Ok(Self {
            inner: inner?,
            _exch: std::marker::PhantomData,
        })
    }
}

impl<S: Security + Clone + Debug, E: ProxyExchange> From<Proxy<S, E>> for ContractId {
    fn from(value: Proxy<S, E>) -> Self {
        value.inner.contract_id()
    }
}

impl<S: Security + Clone + Debug, E: ProxyExchange> Proxy<S, E> {
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

impl<E: ProxyExchange> Proxy<Contract, E> {
    proxy_impl!(Forex, (Contract::Forex(t), e) => Proxy::<Forex, E> { inner: t, _exch: e }, forex);
    proxy_impl!(Crypto, (Contract::Crypto(t), e) => Proxy::<Crypto, E> { inner: t, _exch: e }, crypto);
    proxy_impl!(Stock, (Contract::Stock(t), e) => Proxy::<Stock, E> { inner: t, _exch: e }, stock);
    proxy_impl!(Index, (Contract::Index(t), e) => Proxy::<Index, E> { inner: t, _exch: e }, index);
    proxy_impl!(Commodity, (Contract::Commodity(t), e) => Proxy::<Commodity, E> { inner: t, _exch: e }, commodity);
    proxy_impl!(SecFuture, (Contract::SecFuture(t), e) => Proxy::<SecFuture, E> { inner: t, _exch: e }, sec_future);
    proxy_impl!(SecOption, (Contract::SecOption(t), e) => Proxy::<SecOption, E> { inner: t, _exch: e }, sec_option);
}

impl<E: ProxyExchange> Proxy<Forex, E> {
    #[inline]
    #[must_use]
    /// Get the [`Forex`] trading class.
    pub fn trading_class(&self) -> &str {
        self.inner.trading_class()
    }
}

impl<E: ProxyExchange> Proxy<Crypto, E> {
    #[inline]
    #[must_use]
    /// Get the [`Crypto`] trading class.
    pub fn trading_class(&self) -> &str {
        self.inner.trading_class()
    }
}

impl<E: ProxyExchange> Proxy<Stock, E> {
    #[inline]
    #[must_use]
    /// Get the [`Stock`] trading class.
    pub fn trading_class(&self) -> &str {
        self.inner.trading_class()
    }
}

impl Proxy<Stock, NoExchange> {
    #[inline]
    #[must_use]
    /// Get the [`Stock`] primary exchange.
    pub fn primary_exchange(&self) -> Primary {
        self.inner.primary_exchange
    }
}

impl<E: ProxyExchange> Proxy<Commodity, E> {
    #[inline]
    #[must_use]
    /// Get the [`Commodity`] trading class.
    pub fn trading_class(&self) -> &str {
        self.inner.trading_class()
    }
}

impl<E: ProxyExchange> Proxy<SecFuture, E> {
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

impl<E: ProxyExchange> Proxy<SecOption, E> {
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

impl Proxy<Forex, HasExchange> {
    #[must_use]
    /// Get the [`Forex`] `exchange`
    pub fn exchange(&self) -> Routing {
        self.inner.exchange()
    }
}

impl Proxy<Stock, HasExchange> {
    #[must_use]
    /// Get the [`Stock`] `exchange`
    pub fn exchange(&self) -> Routing {
        self.inner.exchange()
    }
}

impl Proxy<Commodity, HasExchange> {
    #[must_use]
    /// Get the [`Commodity`] `exchange`
    pub fn exchange(&self) -> Routing {
        self.inner.exchange()
    }
}

impl Proxy<SecFuture, HasExchange> {
    #[must_use]
    /// Get the [`SecFuture`] `exchange`
    pub fn exchange(&self) -> Routing {
        self.inner.exchange()
    }
}

impl Proxy<SecOption, HasExchange> {
    #[must_use]
    /// Get the [`SecOption`] `exchange`
    pub fn exchange(&self) -> Routing {
        self.inner.exchange()
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Error)]
#[error(
    "Invalid value encountered when attempting to parse contract type. No such contract type: {0}"
)]
/// An error returned when parsing a [`ContractType`] fails.
pub struct ParseContractTypeError(pub String);

impl FromStr for ContractType {
    type Err = ParseContractTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "CASH" => Self::Forex,
            "CRYPTO" => Self::Crypto,
            "STK" => Self::Stock,
            "IND" => Self::Index,
            "FUT" => Self::SecFuture,
            "OPT" => Self::SecOption,
            "CMDTY" => Self::Commodity,
            v => return Err(ParseContractTypeError(v.to_owned())),
        })
    }
}

impl std::fmt::Display for ContractType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Forex => "CASH",
            Self::Crypto => "CRYPTO",
            Self::Stock => "STK",
            Self::Index => "IND",
            Self::SecFuture => "FUT",
            Self::SecOption => "OPT",
            Self::Commodity => "CMDTY",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
/// An error type returned upon failure to serialize a [`Proxy`].
pub enum SerializeProxyError {
    #[error("Missing data for field {0}")]
    /// Missing data
    MissingData(&'static str),
    #[error("Unexpected security type {0}")]
    /// Unexpected security type
    UnexpectedContractType(#[from] UnexpectedSecurityType),
}
