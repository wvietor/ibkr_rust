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
    #[serde(rename = "CASH")]
    /// A [`crate::contract::Forex`] contract.
    Forex,
    #[serde(rename = "CRYPTO")]
    /// A [`crate::contract::Crypto`] contract.
    Crypto,
    #[serde(rename = "STK")]
    /// A [`crate::contract::Stock`] contract.
    Stock,
    #[serde(rename = "IND")]
    /// An [`crate::contract::Index`] contract.
    Index,
    //Cfd,
    #[serde(rename = "FUT")]
    /// A [`crate::contract::SecFuture`] contract.
    SecFuture,
    #[serde(rename = "OPT")]
    /// A [`crate::contract::SecOption`] contract.
    SecOption,
    //FutureSecOption,
    //Bond,
    //MutualFund,
    #[serde(rename = "CMDTY")]
    /// A [`crate::contract::Commodity`] contract.
    Commodity,
    //Warrant,
    //StructuredProduct,
}

impl std::str::FromStr for ContractType {
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

#[derive(Debug, Clone, Copy, Ord, PartialOrd, PartialEq, Eq, Hash, Serialize)]
pub enum OrderSide {
    #[serde(rename = "BUY")]
    Buy,
    #[serde(rename = "SELL")]
    Sell,
}
