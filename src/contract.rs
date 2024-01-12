use chrono::NaiveDate;
use std::{num::ParseIntError, str::FromStr};

use crate::{
    currency::Currency,
    exchange::{Primary, Routing},
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

#[allow(clippy::module_name_repetitions)]
#[macro_export]
/// Call a given function on a [`Contract`] by unwrapping it and applying the function to the underlying [`Security`].
/// It is useful because it allows for a degree of polymorphism across all the possible [`Contract`] structs.
///
/// For example you might have a [`Vec<Contract>`]. With this macro, you can can call a function on every
/// Contract without having to explicitly match the specific [`Security`].
///
/// Note that the return type of the function to call MUST be the same for all variants of [`Contract`].
///
/// # Matches
/// * `con` - An expression that evaluates to a [`Contract`].
/// * `func` - A token tree containing the name of the function to call. If func is async, then `async` must be passed before `func`.
/// Doing so will await the [`std::future::Future`] returned by function.
/// * `pre_args` - A (potentially empty) comma-delimited set of positional arguments that are passed to the function before `con`.
/// * `post_args` (optional) - A comma-delimited set of positional arguments that are passed to the function after `con`.
///
/// # Examples
/// ```
/// # use ibapi::{contract_dispatch, contract::{self, Contract, Stock, Forex, ContractId}, client::{Builder, Client, Mode::Paper, Host::Gateway}, market_data::live_data::{self, RefreshType}};
/// # use anyhow::Result;
/// # struct DefaultWrapper;
/// # impl ibapi::wrapper::Remote for DefaultWrapper {}
/// # #[tokio::main]
/// # async fn main() -> Result<()> {
/// // Set up a client
/// let mut client = Builder::from_config_file(Paper, Gateway, None)?.connect(31).await?.remote(DefaultWrapper);
///
/// // Create a couple of contracts
/// let apple_inc = contract::new::<Stock>(&mut client, ContractId(242506861)).await?;
/// let gbp_usd = contract::new::<Forex>(&mut client, ContractId(12087797)).await?;
/// // Clone them so we can use them later
/// let apple_inc_2 = apple_inc.clone();
/// let gbp_usd_2 = gbp_usd.clone();
/// let contracts = vec![apple_inc.into(), gbp_usd.into()];
///
/// // Let's make a market data request for each contract
/// let mut ids_macro = Vec::with_capacity(2);
/// for con in contracts.iter() {
///     ids_macro.push(
///         contract_dispatch! {
///             con =>
///                 async (Client::req_market_data)
///                 (&mut client)
///                 (vec![live_data::data_types::Empty], RefreshType::Snapshot, false)
///         }?
///     );
///  }
///
/// // Does the exact same thing as the for loop
/// let ids_explicit = vec![
///     client.req_market_data::<Stock, live_data::data_types::Empty>(&apple_inc_2, vec![live_data::data_types::Empty], RefreshType::Snapshot, false).await?,
///     client.req_market_data::<Forex, live_data::data_types::Empty>(&gbp_usd_2, vec![live_data::data_types::Empty], RefreshType::Snapshot, false).await?
/// ];
///
/// # tokio::time::sleep(std::time::Duration::from_secs(5)).await;
/// # client.disconnect().await?;
/// assert_eq!(ids_explicit, ids_macro.iter().map(|id| id + 2).collect::<Vec<i64>>());
/// # Ok(())
/// # }
/// ```
macro_rules! contract_dispatch {
    {$con: expr => async $func: tt ($($($pre_args: expr),+)?) $(($($post_args: expr),+))?} => {
        match $con {
            Contract::Forex(fx) => {
                $func($($($pre_args),+)?, fx, $($($post_args),+)?).await
            },
            Contract::Crypto(crypto) => {
                $func($($($pre_args),+)?, crypto, $($($post_args),+)?).await
            },
            Contract::Stock(stk) => {
                $func($($($pre_args),+)?, stk, $($($post_args),+)?).await
            },
            Contract::Index(ind) => {
                $func($($($pre_args),+)?, ind, $($($post_args),+)?).await
            },
            Contract::SecFuture(fut) => {
                $func($($($pre_args),+)?, fut, $($($post_args),+)?).await
            },
            Contract::SecOption(opt) => {
                $func($($($pre_args),+)?, opt, $($($post_args),+)?).await
            },
            Contract::Commodity(cmdty) => {
                $func($($($pre_args),+)?, cmdty, $($($post_args),+)?).await
            },
        }
    };
    {$con: expr => $func: tt ($($($pre_args: expr),+)?) $(($($post_args: expr),+))?} => {
        match $con {
            Contract::Forex(fx) => {
                $func($($($pre_args),+)?, fx, $($($post_args),+)?)
            },
            Contract::Crypto(crypto) => {
                $func($($($pre_args),+)?, crypto, $($($post_args),+)?)
            },
            Contract::Stock(stk) => {
                $func($($($pre_args),+)?, stk, $($($post_args),+)?)
            },
            Contract::Index(ind) => {
                $func($($($pre_args),+)?, ind, $($($post_args),+)?)
            },
            Contract::SecFuture(fut) => {
                $func($($($pre_args),+)?, fut, $($($post_args),+)?)
            },
            Contract::SecOption(opt) => {
                $func($($($pre_args),+)?, opt, $($($post_args),+)?)
            },
            Contract::Commodity(cmdty) => {
                $func($($($pre_args),+)?, cmdty, $($($post_args),+)?)
            },
        }
    };
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
    client: &mut crate::client::Client<crate::client::indicators::Active>,
    contract_id: ContractId,
) -> anyhow::Result<S>
where
    <S as TryFrom<Forex>>::Error: 'static + std::error::Error + Send + Sync,
    <S as TryFrom<Crypto>>::Error: 'static + std::error::Error + Send + Sync,
    <S as TryFrom<Stock>>::Error: 'static + std::error::Error + Send + Sync,
    <S as TryFrom<Index>>::Error: 'static + std::error::Error + Send + Sync,
    <S as TryFrom<SecFuture>>::Error: 'static + std::error::Error + Send + Sync,
    <S as TryFrom<SecOption>>::Error: 'static + std::error::Error + Send + Sync,
    <S as TryFrom<Commodity>>::Error: 'static + std::error::Error + Send + Sync,
{
    client.send_contract_query(contract_id).await?;
    Ok(match client.recv_contract_query().await? {
        Contract::Forex(fx) => fx.try_into()?,
        Contract::Crypto(crypto) => crypto.try_into()?,
        Contract::Stock(stk) => stk.try_into()?,
        Contract::Index(ind) => ind.try_into()?,
        Contract::SecFuture(fut) => fut.try_into()?,
        Contract::SecOption(opt) => opt.try_into()?,
        Contract::Commodity(cmdty) => cmdty.try_into()?,
    })
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
