use crate::exchange::Primary;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Filter {
    pub client_id: i64,
    pub account_number: String,
    pub symbol: String,
    pub contract_type: ContractType,
    pub exchange: Primary,
    pub side: OrderSide,
}

// Add dummy time field that is not used
impl Serialize for Filter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Filter", 7)?;
        s.serialize_field("client_id", &self.client_id)?;
        s.serialize_field("account_number", &self.account_number)?;
        s.serialize_field("time", &"")?;
        s.serialize_field("symbol", &self.symbol)?;
        s.serialize_field("contract_type", &self.contract_type)?;
        s.serialize_field("exchange", &self.exchange)?;
        s.serialize_field("side", &self.side)?;
        s.end()
    }
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
