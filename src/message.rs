use std::{collections::HashSet, str::FromStr};

use crate::contract::{Contract, ContractId};

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
pub enum InMsg {
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

impl FromStr for InMsg {
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

#[allow(dead_code)] // Temporary to avoid annoying lint
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OutMsg {
    ReqMktData,
    CancelMktData,
    PlaceOrder,
    CancelOrder,
    ReqOpenOrders,
    ReqAcctData,
    ReqExecutions,
    ReqIds,
    ReqContractData,
    ReqMktDepth,
    CancelMktDepth,
    ReqNewsBulletins,
    CancelNewsBulletins,
    SetServerLoglevel,
    ReqAutoOpenOrders,
    ReqAllOpenOrders,
    ReqManagedAccts,
    ReqFa,
    ReplaceFa,
    ReqHistoricalData,
    ExerciseOptions,
    ReqScannerSubscription,
    CancelScannerSubscription,
    ReqScannerParameters,
    CancelHistoricalData,
    ReqCurrentTime,
    ReqRealTimeBars,
    CancelRealTimeBars,
    ReqFundamentalData,
    CancelFundamentalData,
    ReqCalcImpliedVolat,
    ReqCalcOptionPrice,
    CancelCalcImpliedVolat,
    CancelCalcOptionPrice,
    ReqGlobalCancel,
    ReqMarketDataType,
    ReqPositions,
    ReqAccountSummary,
    CancelAccountSummary,
    CancelPositions,
    VerifyRequest,
    VerifyMessage,
    QueryDisplayGroups,
    SubscribeToGroupEvents,
    UpdateDisplayGroup,
    UnsubscribeFromGroupEvents,
    StartApi,
    VerifyAndAuthRequest,
    VerifyAndAuthMessage,
    ReqPositionsMulti,
    CancelPositionsMulti,
    ReqAccountUpdatesMulti,
    CancelAccountUpdatesMulti,
    ReqSecDefOptParams,
    ReqSoftDollarTiers,
    ReqFamilyCodes,
    ReqMatchingSymbols,
    ReqMktDepthExchanges,
    ReqSmartComponents,
    ReqNewsArticle,
    ReqNewsProviders,
    ReqHistoricalNews,
    ReqHeadTimestamp,
    ReqHistogramData,
    CancelHistogramData,
    CancelHeadTimestamp,
    ReqMarketRule,
    ReqPnl,
    CancelPnl,
    ReqPnlSingle,
    CancelPnlSingle,
    ReqHistoricalTicks,
    ReqTickByTickData,
    CancelTickByTickData,
    ReqCompletedOrders,
    ReqWshMetaData,
    CancelWshMetaData,
    ReqWshEventData,
    CancelWshEventData,
    ReqUserInfo,
}

impl ToString for OutMsg {
    fn to_string(&self) -> String {
        match self {
            Self::ReqMktData => "1",
            Self::CancelMktData => "2",
            Self::PlaceOrder => "3",
            Self::CancelOrder => "4",
            Self::ReqOpenOrders => "5",
            Self::ReqAcctData => "6",
            Self::ReqExecutions => "7",
            Self::ReqIds => "8",
            Self::ReqContractData => "9",
            Self::ReqMktDepth => "10",
            Self::CancelMktDepth => "11",
            Self::ReqNewsBulletins => "12",
            Self::CancelNewsBulletins => "13",
            Self::SetServerLoglevel => "14",
            Self::ReqAutoOpenOrders => "15",
            Self::ReqAllOpenOrders => "16",
            Self::ReqManagedAccts => "17",
            Self::ReqFa => "18",
            Self::ReplaceFa => "19",
            Self::ReqHistoricalData => "20",
            Self::ExerciseOptions => "21",
            Self::ReqScannerSubscription => "22",
            Self::CancelScannerSubscription => "23",
            Self::ReqScannerParameters => "24",
            Self::CancelHistoricalData => "25",
            Self::ReqCurrentTime => "49",
            Self::ReqRealTimeBars => "50",
            Self::CancelRealTimeBars => "51",
            Self::ReqFundamentalData => "52",
            Self::CancelFundamentalData => "53",
            Self::ReqCalcImpliedVolat => "54",
            Self::ReqCalcOptionPrice => "55",
            Self::CancelCalcImpliedVolat => "56",
            Self::CancelCalcOptionPrice => "57",
            Self::ReqGlobalCancel => "58",
            Self::ReqMarketDataType => "59",
            Self::ReqPositions => "61",
            Self::ReqAccountSummary => "62",
            Self::CancelAccountSummary => "63",
            Self::CancelPositions => "64",
            Self::VerifyRequest => "65",
            Self::VerifyMessage => "66",
            Self::QueryDisplayGroups => "67",
            Self::SubscribeToGroupEvents => "68",
            Self::UpdateDisplayGroup => "69",
            Self::UnsubscribeFromGroupEvents => "70",
            Self::StartApi => "71",
            Self::VerifyAndAuthRequest => "72",
            Self::VerifyAndAuthMessage => "73",
            Self::ReqPositionsMulti => "74",
            Self::CancelPositionsMulti => "75",
            Self::ReqAccountUpdatesMulti => "76",
            Self::CancelAccountUpdatesMulti => "77",
            Self::ReqSecDefOptParams => "78",
            Self::ReqSoftDollarTiers => "79",
            Self::ReqFamilyCodes => "80",
            Self::ReqMatchingSymbols => "81",
            Self::ReqMktDepthExchanges => "82",
            Self::ReqSmartComponents => "83",
            Self::ReqNewsArticle => "84",
            Self::ReqNewsProviders => "85",
            Self::ReqHistoricalNews => "86",
            Self::ReqHeadTimestamp => "87",
            Self::ReqHistogramData => "88",
            Self::CancelHistogramData => "89",
            Self::CancelHeadTimestamp => "90",
            Self::ReqMarketRule => "91",
            Self::ReqPnl => "92",
            Self::CancelPnl => "93",
            Self::ReqPnlSingle => "94",
            Self::CancelPnlSingle => "95",
            Self::ReqHistoricalTicks => "96",
            Self::ReqTickByTickData => "97",
            Self::CancelTickByTickData => "98",
            Self::ReqCompletedOrders => "99",
            Self::ReqWshMetaData => "100",
            Self::CancelWshMetaData => "101",
            Self::ReqWshEventData => "102",
            Self::CancelWshEventData => "103",
            Self::ReqUserInfo => "104",
        }
        .to_owned()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ToWrapper {
    StartApiManagedAccts,
    StartApiNextValidId,
    ContractQuery((ContractId, i64)),
}

#[allow(clippy::redundant_pub_crate)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ToClient {
    StartApiManagedAccts(HashSet<String>),
    StartApiNextValidId(i64),
    NewContract(Contract),
}