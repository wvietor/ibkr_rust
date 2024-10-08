use std::str::FromStr;

use crate::contract::{Contract, Query};

#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct InvalidInMsg(pub String);

impl std::fmt::Display for InvalidInMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid message received from API: {}", self.0)
    }
}

impl std::error::Error for InvalidInMsg {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum In {
    TickPrice,
    TickSize,
    OrderStatus,
    ErrMsg,
    OpenOrder,
    AcctValue,
    PortfolioValue,
    AcctUpdateTime,
    NextValidId,
    ContractData,
    ExecutionData,
    MarketDepth,
    MarketDepthL2,
    NewsBulletins,
    ManagedAccts,
    ReceiveFa,
    HistoricalData,
    BondContractData,
    ScannerParameters,
    ScannerData,
    TickOptionComputation,
    TickGeneric,
    TickString,
    TickEfp,
    CurrentTime,
    RealTimeBars,
    FundamentalData,
    ContractDataEnd,
    OpenOrderEnd,
    AcctDownloadEnd,
    ExecutionDataEnd,
    DeltaNeutralValidation,
    TickSnapshotEnd,
    MarketDataType,
    CommissionReport,
    PositionData,
    PositionEnd,
    AccountSummary,
    AccountSummaryEnd,
    VerifyMessageApi,
    VerifyCompleted,
    DisplayGroupList,
    DisplayGroupUpdated,
    VerifyAndAuthMessageApi,
    VerifyAndAuthCompleted,
    PositionMulti,
    PositionMultiEnd,
    AccountUpdateMulti,
    AccountUpdateMultiEnd,
    SecurityDefinitionOptionParameter,
    SecurityDefinitionOptionParameterEnd,
    SoftDollarTiers,
    FamilyCodes,
    SymbolSamples,
    MktDepthExchanges,
    TickReqParams,
    SmartComponents,
    NewsArticle,
    TickNews,
    NewsProviders,
    HistoricalNews,
    HistoricalNewsEnd,
    HeadTimestamp,
    HistogramData,
    HistoricalDataUpdate,
    RerouteMktDataReq,
    RerouteMktDepthReq,
    MarketRule,
    Pnl,
    PnlSingle,
    HistoricalTicks,
    HistoricalTicksBidAsk,
    HistoricalTicksLast,
    TickByTick,
    OrderBound,
    CompletedOrder,
    CompletedOrdersEnd,
    ReplaceFaEnd,
    WshMetaData,
    WshEventData,
    HistoricalSchedule,
    UserInfo,
}

impl FromStr for In {
    type Err = InvalidInMsg;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Self::TickPrice,
            "2" => Self::TickSize,
            "3" => Self::OrderStatus,
            "4" => Self::ErrMsg,
            "5" => Self::OpenOrder,
            "6" => Self::AcctValue,
            "7" => Self::PortfolioValue,
            "8" => Self::AcctUpdateTime,
            "9" => Self::NextValidId,
            "10" => Self::ContractData,
            "11" => Self::ExecutionData,
            "12" => Self::MarketDepth,
            "13" => Self::MarketDepthL2,
            "14" => Self::NewsBulletins,
            "15" => Self::ManagedAccts,
            "16" => Self::ReceiveFa,
            "17" => Self::HistoricalData,
            "18" => Self::BondContractData,
            "19" => Self::ScannerParameters,
            "20" => Self::ScannerData,
            "21" => Self::TickOptionComputation,
            "45" => Self::TickGeneric,
            "46" => Self::TickString,
            "47" => Self::TickEfp,
            "49" => Self::CurrentTime,
            "50" => Self::RealTimeBars,
            "51" => Self::FundamentalData,
            "52" => Self::ContractDataEnd,
            "53" => Self::OpenOrderEnd,
            "54" => Self::AcctDownloadEnd,
            "55" => Self::ExecutionDataEnd,
            "56" => Self::DeltaNeutralValidation,
            "57" => Self::TickSnapshotEnd,
            "58" => Self::MarketDataType,
            "59" => Self::CommissionReport,
            "61" => Self::PositionData,
            "62" => Self::PositionEnd,
            "63" => Self::AccountSummary,
            "64" => Self::AccountSummaryEnd,
            "65" => Self::VerifyMessageApi,
            "66" => Self::VerifyCompleted,
            "67" => Self::DisplayGroupList,
            "68" => Self::DisplayGroupUpdated,
            "69" => Self::VerifyAndAuthMessageApi,
            "70" => Self::VerifyAndAuthCompleted,
            "71" => Self::PositionMulti,
            "72" => Self::PositionMultiEnd,
            "73" => Self::AccountUpdateMulti,
            "74" => Self::AccountUpdateMultiEnd,
            "75" => Self::SecurityDefinitionOptionParameter,
            "76" => Self::SecurityDefinitionOptionParameterEnd,
            "77" => Self::SoftDollarTiers,
            "78" => Self::FamilyCodes,
            "79" => Self::SymbolSamples,
            "80" => Self::MktDepthExchanges,
            "81" => Self::TickReqParams,
            "82" => Self::SmartComponents,
            "83" => Self::NewsArticle,
            "84" => Self::TickNews,
            "85" => Self::NewsProviders,
            "86" => Self::HistoricalNews,
            "87" => Self::HistoricalNewsEnd,
            "88" => Self::HeadTimestamp,
            "89" => Self::HistogramData,
            "90" => Self::HistoricalDataUpdate,
            "91" => Self::RerouteMktDataReq,
            "92" => Self::RerouteMktDepthReq,
            "93" => Self::MarketRule,
            "94" => Self::Pnl,
            "95" => Self::PnlSingle,
            "96" => Self::HistoricalTicks,
            "97" => Self::HistoricalTicksBidAsk,
            "98" => Self::HistoricalTicksLast,
            "99" => Self::TickByTick,
            "100" => Self::OrderBound,
            "101" => Self::CompletedOrder,
            "102" => Self::CompletedOrdersEnd,
            "103" => Self::ReplaceFaEnd,
            "104" => Self::WshMetaData,
            "105" => Self::WshEventData,
            "106" => Self::HistoricalSchedule,
            "107" => Self::UserInfo,
            s => return Err(InvalidInMsg(s.to_owned())),
        })
    }
}

// Ok, we haven't implemented all the outgoing client messages
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
pub enum Out {
    #[serde(rename(serialize = "1"))]
    ReqMktData,
    #[serde(rename(serialize = "2"))]
    CancelMktData,
    #[serde(rename(serialize = "3"))]
    PlaceOrder,
    #[serde(rename(serialize = "4"))]
    CancelOrder,
    #[serde(rename(serialize = "5"))]
    ReqOpenOrders,
    #[serde(rename(serialize = "6"))]
    ReqAcctData,
    #[serde(rename(serialize = "7"))]
    ReqExecutions,
    #[serde(rename(serialize = "8"))]
    ReqIds,
    #[serde(rename(serialize = "9"))]
    ReqContractData,
    #[serde(rename(serialize = "10"))]
    ReqMktDepth,
    #[serde(rename(serialize = "11"))]
    CancelMktDepth,
    #[serde(rename(serialize = "12"))]
    ReqNewsBulletins,
    #[serde(rename(serialize = "13"))]
    CancelNewsBulletins,
    #[serde(rename(serialize = "14"))]
    SetServerLoglevel,
    #[serde(rename(serialize = "15"))]
    ReqAutoOpenOrders,
    #[serde(rename(serialize = "16"))]
    ReqAllOpenOrders,
    #[serde(rename(serialize = "17"))]
    ReqManagedAccts,
    #[serde(rename(serialize = "18"))]
    ReqFa,
    #[serde(rename(serialize = "19"))]
    ReplaceFa,
    #[serde(rename(serialize = "20"))]
    ReqHistoricalData,
    #[serde(rename(serialize = "21"))]
    ExerciseOptions,
    #[serde(rename(serialize = "22"))]
    ReqScannerSubscription,
    #[serde(rename(serialize = "23"))]
    CancelScannerSubscription,
    #[serde(rename(serialize = "24"))]
    ReqScannerParameters,
    #[serde(rename(serialize = "25"))]
    CancelHistoricalData,
    #[serde(rename(serialize = "49"))]
    ReqCurrentTime,
    #[serde(rename(serialize = "50"))]
    ReqRealTimeBars,
    #[serde(rename(serialize = "51"))]
    CancelRealTimeBars,
    #[serde(rename(serialize = "52"))]
    ReqFundamentalData,
    #[serde(rename(serialize = "53"))]
    CancelFundamentalData,
    #[serde(rename(serialize = "54"))]
    ReqCalcImpliedVolatility,
    #[serde(rename(serialize = "55"))]
    ReqCalcOptionPrice,
    #[serde(rename(serialize = "56"))]
    CancelCalcImpliedVolatility,
    #[serde(rename(serialize = "57"))]
    CancelCalcOptionPrice,
    #[serde(rename(serialize = "58"))]
    ReqGlobalCancel,
    #[serde(rename(serialize = "59"))]
    ReqMarketDataType,
    #[serde(rename(serialize = "61"))]
    ReqPositions,
    #[serde(rename(serialize = "62"))]
    ReqAccountSummary,
    #[serde(rename(serialize = "63"))]
    CancelAccountSummary,
    #[serde(rename(serialize = "64"))]
    CancelPositions,
    #[serde(rename(serialize = "65"))]
    VerifyRequest,
    #[serde(rename(serialize = "66"))]
    VerifyMessage,
    #[serde(rename(serialize = "67"))]
    QueryDisplayGroups,
    #[serde(rename(serialize = "68"))]
    SubscribeToGroupEvents,
    #[serde(rename(serialize = "69"))]
    UpdateDisplayGroup,
    #[serde(rename(serialize = "70"))]
    UnsubscribeFromGroupEvents,
    #[serde(rename(serialize = "71"))]
    StartApi,
    #[serde(rename(serialize = "72"))]
    VerifyAndAuthRequest,
    #[serde(rename(serialize = "73"))]
    VerifyAndAuthMessage,
    #[serde(rename(serialize = "74"))]
    ReqPositionsMulti,
    #[serde(rename(serialize = "75"))]
    CancelPositionsMulti,
    #[serde(rename(serialize = "76"))]
    ReqAccountUpdatesMulti,
    #[serde(rename(serialize = "77"))]
    CancelAccountUpdatesMulti,
    #[serde(rename(serialize = "78"))]
    ReqSecDefOptParams,
    #[serde(rename(serialize = "79"))]
    ReqSoftDollarTiers,
    #[serde(rename(serialize = "80"))]
    ReqFamilyCodes,
    #[serde(rename(serialize = "81"))]
    ReqMatchingSymbols,
    #[serde(rename(serialize = "82"))]
    ReqMktDepthExchanges,
    #[serde(rename(serialize = "83"))]
    ReqSmartComponents,
    #[serde(rename(serialize = "84"))]
    ReqNewsArticle,
    #[serde(rename(serialize = "85"))]
    ReqNewsProviders,
    #[serde(rename(serialize = "86"))]
    ReqHistoricalNews,
    #[serde(rename(serialize = "87"))]
    ReqHeadTimestamp,
    #[serde(rename(serialize = "88"))]
    ReqHistogramData,
    #[serde(rename(serialize = "89"))]
    CancelHistogramData,
    #[serde(rename(serialize = "90"))]
    CancelHeadTimestamp,
    #[serde(rename(serialize = "91"))]
    ReqMarketRule,
    #[serde(rename(serialize = "92"))]
    ReqPnl,
    #[serde(rename(serialize = "93"))]
    CancelPnl,
    #[serde(rename(serialize = "94"))]
    ReqPnlSingle,
    #[serde(rename(serialize = "95"))]
    CancelPnlSingle,
    #[serde(rename(serialize = "96"))]
    ReqHistoricalTicks,
    #[serde(rename(serialize = "97"))]
    ReqTickByTickData,
    #[serde(rename(serialize = "98"))]
    CancelTickByTickData,
    #[serde(rename(serialize = "99"))]
    ReqCompletedOrders,
    #[serde(rename(serialize = "100"))]
    ReqWshMetaData,
    #[serde(rename(serialize = "101"))]
    CancelWshMetaData,
    #[serde(rename(serialize = "102"))]
    ReqWshEventData,
    #[serde(rename(serialize = "103"))]
    CancelWshEventData,
    #[serde(rename(serialize = "104"))]
    ReqUserInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ToWrapper {
    ContractQuery((Query, i64)),
}

#[allow(clippy::redundant_pub_crate)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ToClient {
    NewContract(Contract),
}
