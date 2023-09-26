use crate::exchange::Primary;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Filter {
    pub client_id: i64,
    pub account_number: String,
    pub start_time: NaiveDateTime,
    pub symbol: String,
    pub contract_type: ContractType,
    pub exchange: Primary,
    pub side: OrderSide,
}

impl ToString for Filter {
    fn to_string(&self) -> String {
        make_body!(
            self.client_id,
            self.account_number,
            self.start_time.format("%Y%m%d %T").to_string(),
            self.symbol,
            self.contract_type,
            self.exchange;
            self.side
        )
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ContractType {
    /// A [`crate::contract::Forex`] contract.
    Forex,
    /// A [`crate::contract::Crypto`] contract.
    Crypto,
    /// A [`crate::contract::Stock`] contract.
    Stock,
    /// An [`crate::contract::Index`] contract.
    Index,
    //Cfd,
    /// A [`crate::contract::SecFuture`] contract.
    SecFuture,
    /// A [`crate::contract::SecOption`] contract.
    SecOption,
    //FutureSecOption,
    //Bond,
    //MutualFund,
    /// A [`crate::contract::Commodity`] contract.
    Commodity,
    //Warrant,
    //StructuredProduct,
}

impl ToString for ContractType {
    fn to_string(&self) -> String {
        match self {
            Self::Forex => "CASH",
            Self::Crypto => "CRYPTO",
            Self::Stock => "STK",
            Self::Index => "IND",
            Self::SecFuture => "FUT",
            Self::SecOption => "OPT",
            Self::Commodity => "CMDTY",
        }
        .to_owned()
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub enum OrderSide {
    Buy,
    Sell,
}

impl ToString for OrderSide {
    fn to_string(&self) -> String {
        match self {
            Self::Buy => "BUY",
            Self::Sell => "SELL",
        }
        .to_owned()
    }
}
