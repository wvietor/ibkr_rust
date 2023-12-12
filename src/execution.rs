use crate::comm::serialize_naive_datetime_yyyymmdd_hhcolon_mm_colon_ss;
use crate::exchange::Primary;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize)]
pub struct Filter {
    pub client_id: i64,
    pub account_number: String,
    #[serde(serialize_with = "serialize_naive_datetime_yyyymmdd_hhcolon_mm_colon_ss")]
    pub start_time: NaiveDateTime,
    pub symbol: String,
    pub contract_type: ContractType,
    pub exchange: Primary,
    pub side: OrderSide,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize)]
pub enum ContractType {
    #[serde(rename(serialize = "CASH"))]
    /// A [`crate::contract::Forex`] contract.
    Forex,
    #[serde(rename(serialize = "CRYPTO"))]
    /// A [`crate::contract::Crypto`] contract.
    Crypto,
    #[serde(rename(serialize = "STK"))]
    /// A [`crate::contract::Stock`] contract.
    Stock,
    #[serde(rename(serialize = "IND"))]
    /// An [`crate::contract::Index`] contract.
    Index,
    //Cfd,
    #[serde(rename(serialize = "FUT"))]
    /// A [`crate::contract::SecFuture`] contract.
    SecFuture,
    #[serde(rename(serialize = "OPT"))]
    /// A [`crate::contract::SecOption`] contract.
    SecOption,
    //FutureSecOption,
    //Bond,
    //MutualFund,
    #[serde(rename(serialize = "CMDTY"))]
    /// A [`crate::contract::Commodity`] contract.
    Commodity,
    //Warrant,
    //StructuredProduct,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, PartialEq, Eq, Hash, Serialize)]
pub enum OrderSide {
    #[serde(rename(serialize = "BUY"))]
    Buy,
    #[serde(rename(serialize = "SELL"))]
    Sell,
}
