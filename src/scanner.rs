use serde::{Deserialize, Serialize, Serializer};
use std::default::Default;
use std::{f64, i32};

// Serialize, Deserialize
#[derive(Debug, Clone)]
pub struct ScannerSubscription {
    // The number of rows to be returned for the query.
    pub number_of_rows: i32,
    // The instrument's type for the scan. I.e. STK, FUT.HK, etc.
    pub instrument: String,
    // The request's location (STK.US, STK.US.MAJOR, etc).
    pub location_code: String,
    // Same as TWS Market Scanner's "parameters" field, for example: TOP_PERC_GAIN.
    pub scan_code: ScanCode,
    // Filters out Contracts which price is below this value.
    pub above_price: f64,
    // Filters out contracts which price is above this value.
    pub below_price: f64,
    // Filters out Contracts which volume is above this value.
    pub above_volume: i32,
    // Filters out Contracts which option volume is above this value.
    pub average_option_volume_above: i32,
    // Filters out Contracts which market cap is above this value.
    pub market_cap_above: f64,
    // Filters out Contracts which market cap is below this value.
    pub market_cap_below: f64,
    // Filters out Contracts which Moody's rating is below this value. (AA3 A1 A2 A3 BAA1 BAA2 BAA3 BA1 BA2 BA3 B1 B2 B3 CAA1 CAA2 CAA3 CA C NR)
    pub moody_rating_above: String,
    // Filters out Contracts which Moody's rating is above this value. (AA3 A1 A2 A3 BAA1 BAA2 BAA3 BA1 BA2 BA3 B1 B2 B3 CAA1 CAA2 CAA3 CA C NR)
    pub moody_rating_below: String,
    // Filters out Contracts with a S&P rating below this value. (AAA AA+ AA AA- A+ A A- BBB+ BBB BBB- BB+ BB BB- B+ B B- CCC+ CCC CCC- CC+ CC CC- C+ C C- D NR )
    pub sp_rating_above: String,
    // Filters out Contracts with a S&P rating above this value. (AAA AA+ AA AA- A+ A A- BBB+ BBB BBB- BB+ BB BB- B+ B B- CCC+ CCC CCC- CC+ CC CC- C+ C C- D NR)
    pub sp_rating_below: String,
    // Filter out Contracts with a maturity date earlier than this value. (mm/yyyy or yyyymmdd)
    pub maturity_date_above: String,
    // Filter out Contracts with a maturity date older than this value.
    pub maturity_date_below: String,
    // Filter out Contracts with a coupon rate lower than this value.
    pub coupon_rate_above: f64,
    // Filter out Contracts with a coupon rate higher than this value.
    pub coupon_rate_below: f64,
    // Filters out Convertible bonds.
    pub exclude_convertible: String,
    // For example, a pairing "Annual, true" used on the "top Option Implied Vol % Gainers" scan would return annualized volatilities.
    pub scanner_setting_pairs: String,
    // CORP = Corporation ADR = American Depositary Receipt ETF = Exchange Traded Fund REIT = Real Estate Investment Trust CEF = Closed End Fund
    pub stock_type_filter: String,
}

const NO_ROW_NUMBER_SPECIFIED: i32 = -1;

//List<TagValue> TagValues = new List<TagValue>{t1, t2, t3};

// impl Default for ScannerSubscription {
//     fn default() -> ScannerSubscription {
//         ScannerSubscription {
//             number_of_rows: NO_ROW_NUMBER_SPECIFIED,
//             instrument: "".to_string(),
//             location_code: "".to_string(),
//             scan_code: "".to_string(),
//             above_price: f64::MAX,
//             below_price: f64::MAX,
//             above_volume: i32::MAX,
//             average_option_volume_above: i32::MAX,
//             market_cap_above: f64::MAX,
//             market_cap_below: f64::MAX,
//             moody_rating_above: "".to_string(),
//             moody_rating_below: "".to_string(),
//             sp_rating_above: "".to_string(),
//             sp_rating_below: "".to_string(),
//             maturity_date_above: "".to_string(),
//             maturity_date_below: "".to_string(),
//             coupon_rate_above: f64::MAX,
//             coupon_rate_below: f64::MAX,
//             exclude_convertible: "".to_string(),
//             scanner_setting_pairs: "".to_string(),
//             stock_type_filter: "".to_string(),
//         }
//     }
// }
//

// static final CodeMsgPair FAIL_SEND_REQSCANNER = new CodeMsgPair(524, "Request Scanner Subscription Sending Error - ");
// static final CodeMsgPair FAIL_SEND_CANSCANNER = new CodeMsgPair(525, "Cancel Scanner Subscription Sending Error - ");
// static final CodeMsgPair FAIL_SEND_REQSCANNERPARAMETERS = new CodeMsgPair(526, "Request Scanner Parameter Sending Error - ");

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TagValue {
    tag: String,
    value: String,
}

pub struct ScannerSubscriptionOptions {
    tags_values: Vec<TagValue>,
    index: usize,
}

impl ScannerSubscriptionOptions {
    fn new() -> Self {
        Self {
            tags_values: Vec::new(),
            index: 0,
        }
    }
    fn new_with_capacity(capacity: usize) -> Self {
        Self {
            tags_values: Vec::with_capacity(capacity),
            index: 0,
        }
    }
    fn add_option(&mut self, tag: String, value: String) {
        self.tags_values.push(TagValue { tag, value })
    }
    fn iter(&self) {
        self.tags_values.iter();
    }
}

pub struct ScannerSubscriptionFilterOptions;

enum Instrument {
    Stk,
    Bond,
    Efp,
    FutEu,
    FutHk,
    FutNa,
    FutUs,
    IndEu,
    IndHk,
    IndUs,
    Pmonitor,
    Pmonitorm,
    SlbUs,
    StockEu,
    StockHk,
    StockNa,
    WarEu,
}

impl Serialize for Instrument {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Self::Stk => "STK",
            Self::Bond => "BOND",
            Self::Efp => "EFP",
            Self::FutEu => "FUT. EU",
            Self::FutHk => "FUT.HK",
            Self::FutNa => "FUT.NA",
            Self::FutUs => "FUT.US",
            Self::IndEu => "IND. EU",
            Self::IndHk => "IND.HK",
            Self::IndUs => "IND.US",
            Self::Pmonitor => "PMONITOR",
            Self::Pmonitorm => "PMONITORM",
            Self::SlbUs => "SLB. US",
            Self::StockEu => "STOCK. EU",
            Self::StockHk => "STOCK.HK",
            Self::StockNa => "STOCK.NA",
            Self::WarEu => "WAR. EU",
        }
        .serialize(serializer)
    }
}

pub enum LocationCode {
    BondUs,
    Efp,
    FutEuBelfox,
    FutEuFta,
    FutEuIdem,
    FutEuLiffe,
    FutEuMeffrv,
    FutEuMonep,
    FutEu,
    FutHkHkfe,
    FutHkJapan,
    FutHkKse,
    FutHkNse,
    FutHkOseJpn,
    FutHkSgx,
    FutHkSnfe,
    FutHkTseJpn,
    FutHk,
    FutIpe,
    FutNaCde,
    FutNa,
    FutNybot,
    FutNyseliffe,
    FutUs,
    IndEuBelfox,
    IndEuFta,
    IndEuLiffe,
    IndEuMonep,
    IndEu,
    IndHkHkfe,
    IndHkJapan,
    IndHkKse,
    IndHkNse,
    IndHkOseJpn,
    IndHkSgx,
    IndHkSnfe,
    IndHkTseJpn,
    IndHk,
    IndUs,
    SlbAqs,
    StkAmex,
    StkArca,
    StkEuAeb,
    StkEuBm,
    StkEuBvme,
    StkEuEbs,
    StkEuIbis,
    StkEuIbisEtf,
    StkEuIbisEustars,
    StkEuIbisNewx,
    StkEuIbisUsstars,
    StkEuIbisXetra,
    StkEuLse,
    StkEuSbf,
    StkEuSbvm,
    StkEuSfb,
    StkEuSwiss,
    StkEuVirtx,
    StkEu,
    StkHkAsx,
    StkHkNse,
    StkHkSehk,
    StkHkSgx,
    StkHkTseJpn,
    StkHk,
    StkNaCanada,
    StkNaTse,
    StkNaVenture,
    StkNa,
    StkNasdaqNms,
    StkNasdaqScm,
    StkNasdaq,
    StkNyse,
    StkOtcbb,
    StkPink,
    StkUsMajor,
    StkUsMinor,
    StkUs,
    WarEuAll,
}

impl Serialize for LocationCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Self::BondUs => "BOND.US",
            Self::Efp => "EFP",
            Self::FutEuBelfox => "FUT.EU.BELFOX",
            Self::FutEuFta => "FUT.EU.FTA",
            Self::FutEuIdem => "FUT.EU.IDEM",
            Self::FutEuLiffe => "FUT.EU.LIFFE",
            Self::FutEuMeffrv => "FUT.EU.MEFFRV",
            Self::FutEuMonep => "FUT.EU.MONEP",
            Self::FutEu => "FUT.EU",
            Self::FutHkHkfe => "FUT.HK.HKFE",
            Self::FutHkJapan => "FUT.HK.JAPAN",
            Self::FutHkKse => "FUT.HK.KSE",
            Self::FutHkNse => "FUT.HK.NSE",
            Self::FutHkOseJpn => "FUT.HK.OSE.JPN",
            Self::FutHkSgx => "FUT.HK.SGX",
            Self::FutHkSnfe => "FUT.HK.SNFE",
            Self::FutHkTseJpn => "FUT.HK.TSE.JPN",
            Self::FutHk => "FUT.HK",
            Self::FutIpe => "FUT.IPE",
            Self::FutNaCde => "FUT.NA.CDE",
            Self::FutNa => "FUT.NA",
            Self::FutNybot => "FUT.NYBOT",
            Self::FutNyseliffe => "FUT.NYSELIFFE",
            Self::FutUs => "FUT.US",
            Self::IndEuBelfox => "IND.EU.BELFOX",
            Self::IndEuFta => "IND.EU.FTA",
            Self::IndEuLiffe => "IND.EU.LIFFE",
            Self::IndEuMonep => "IND.EU.MONEP",
            Self::IndEu => "IND.EU",
            Self::IndHkHkfe => "IND.HK.HKFE",
            Self::IndHkJapan => "IND.HK.JAPAN",
            Self::IndHkKse => "IND.HK.KSE",
            Self::IndHkNse => "IND.HK.NSE",
            Self::IndHkOseJpn => "IND.HK.OSE.JPN",
            Self::IndHkSgx => "IND.HK.SGX",
            Self::IndHkSnfe => "IND.HK.SNFE",
            Self::IndHkTseJpn => "IND.HK.TSE.JPN",
            Self::IndHk => "IND.HK",
            Self::IndUs => "IND.US",
            Self::SlbAqs => "SLB.AQS",
            Self::StkAmex => "STK.AMEX",
            Self::StkArca => "STK.ARCA",
            Self::StkEuAeb => "STK.EU.AEB",
            Self::StkEuBm => "STK.EU.BM",
            Self::StkEuBvme => "STK.EU.BVME",
            Self::StkEuEbs => "STK.EU.EBS",
            Self::StkEuIbis => "STK.EU.IBIS",
            Self::StkEuIbisEtf => "STK.EU.IBIS-ETF",
            Self::StkEuIbisEustars => "STK.EU.IBIS-EUSTARS",
            Self::StkEuIbisNewx => "STK.EU.IBIS-NEWX",
            Self::StkEuIbisUsstars => "STK.EU.IBIS-USSTARS",
            Self::StkEuIbisXetra => "STK.EU.IBIS-XETRA",
            Self::StkEuLse => "STK.EU.LSE",
            Self::StkEuSbf => "STK.EU.SBF",
            Self::StkEuSbvm => "STK.EU.SBVM",
            Self::StkEuSfb => "STK.EU.SFB",
            Self::StkEuSwiss => "STK.EU.SWISS",
            Self::StkEuVirtx => "STK.EU.VIRTX",
            Self::StkEu => "STK.EU",
            Self::StkHkAsx => "STK.HK.ASX",
            Self::StkHkNse => "STK.HK.NSE",
            Self::StkHkSehk => "STK.HK.SEHK",
            Self::StkHkSgx => "STK.HK.SGX",
            Self::StkHkTseJpn => "STK.HK.TSE.JPN",
            Self::StkHk => "STK.HK",
            Self::StkNaCanada => "STK.NA.CANADA",
            Self::StkNaTse => "STK.NA.TSE",
            Self::StkNaVenture => "STK.NA.VENTURE",
            Self::StkNa => "STK.NA",
            Self::StkNasdaqNms => "STK.NASDAQ.NMS",
            Self::StkNasdaqScm => "STK.NASDAQ.SCM",
            Self::StkNasdaq => "STK.NASDAQ",
            Self::StkNyse => "STK.NYSE",
            Self::StkOtcbb => "STK.OTCBB",
            Self::StkPink => "STK.PINK",
            Self::StkUsMajor => "STK.US.MAJOR",
            Self::StkUsMinor => "STK.US.MINOR",
            Self::StkUs => "STK.US",
            Self::WarEuAll => "WAR.EU.ALL",
        }
        .serialize(serializer)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ScanCode {
    TopPercGain,
    TopPercLose,
    MostActive,
    AllSymbolsAsc,
    AllSymbolsDesc,
    BondCusipAz,
    BondCusipZa,
    FarMaturityDate,
    Halted,
    HighBondAskCurrentYieldAll,
    HighBondAskYieldAll,
    HighBondDebt2BookRatio,
    HighBondDebt2EquityRatio,
    HighBondDebt2TanBookRatio,
    HighBondEquity2BookRatio,
    HighBondEquity2TanBookRatio,
    HighBondNetAskCurrentYieldAll,
    HighBondNetAskYieldAll,
    HighBondNetSpreadAll,
    HighBondSpreadAll,
    HighCouponRate,
    HighDividendYield,
    HighDividendYieldIb,
    HighestSlbBid,
    HighGrowthRate,
    HighMoodyRatingAll,
    HighOpenGap,
    HighOptImpVolat,
    HighOptImpVolatOverHist,
    HighOptOpenInterestPutCallRatio,
    HighOptVolumePutCallRatio,
    HighPeRatio,
    HighPrice2BookRatio,
    HighPrice2TanBookRatio,
    HighQuickRatio,
    HighReturnOnEquity,
    HighSynthBidRevNatYield,
    HighVs13wHl,
    HighVs26wHl,
    HighVs52wHl,
    HotByOptVolume,
    HotByPrice,
    HotByPriceRange,
    HotByVolume,
    LimitUpDown,
    LowBondBidCurrentYieldAll,
    LowBondBidYieldAll,
    LowBondDebt2BookRatio,
    LowBondDebt2EquityRatio,
    LowBondDebt2TanBookRatio,
    LowBondEquity2BookRatio,
    LowBondEquity2TanBookRatio,
    LowBondNetBidCurrentYieldAll,
    LowBondNetBidYieldAll,
    LowBondNetSpreadAll,
    LowBondSpreadAll,
    LowCouponRate,
    LowestSlbAsk,
    LowGrowthRate,
    LowMoodyRatingAll,
    LowOpenGap,
    LowOptImpVolat,
    LowOptImpVolatOverHist,
    LowOptOpenInterestPutCallRatio,
    LowOptVolumePutCallRatio,
    LowPeRatio,
    LowPrice2BookRatio,
    LowPrice2TanBookRatio,
    LowQuickRatio,
    LowReturnOnEquity,
    LowSynthAskRevNatYield,
    LowVs13wHl,
    LowVs26wHl,
    LowVs52wHl,
    LowWarRelImpVolat,
    MarketCapUsdAsc,
    MarketCapUsdDesc,
    MostActiveAvgUsd,
    MostActiveUsd,
    NearMaturityDate,
    NotOpen,
    OptOpenInterestMostActive,
    OptVolumeMostActive,
    PmonitorAvailContracts,
    PmonitorCtt,
    PmonitorIbond,
    PmonitorRfq,
    TopOpenPercGain,
    TopOpenPercLose,
    TopOptImpVolatGain,
    TopOptImpVolatLose,
    TopPriceRange,
    TopStockBuyImbalanceAdvRatio,
    TopStockSellImbalanceAdvRatio,
    TopTradeCount,
    TopTradeRate,
    TopVolumeRate,
    WshNextAnalystMeeting,
    WshNextEarnings,
    WshNextEvent,
    WshNextMajorEvent,
    WshPrevAnalystMeeting,
    WshPrevEarnings,
    WshPrevEvent,
    // ComboLatestTrade,
    // ComboQuotes,
    // ComboMostActive,
    // ComboAllTradeTimeAsc,
    // ComboAllTradeTimeDesc,
    // ComboAllQuoteTimeAsc,
    // ComboAllQuoteTimeDesc,
    // ComboAllTradeQuoteTimeAsc,
    // ComboAllTradeQuoteTimeDesc,
    // ComboAllVolumeAsc,
    // ComboAllVolumeDesc,
}

impl Serialize for ScanCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Self::TopPercGain => "TOP_PERC_GAIN",
            Self::TopPercLose => "TOP_PERC_LOSE",
            Self::MostActive => "MOST_ACTIVE",
            Self::AllSymbolsAsc => "ALL_SYMBOLS_ASC",
            Self::AllSymbolsDesc => "ALL_SYMBOLS_DESC",
            Self::BondCusipAz => "BOND_CUSIP_AZ",
            Self::BondCusipZa => "BOND_CUSIP_ZA",
            Self::FarMaturityDate => "FAR_MATURITY_DATE",
            Self::Halted => "HALTED",
            Self::HighBondAskCurrentYieldAll => "HIGH_BOND_ASK_CURRENT_YIELD_ALL",
            Self::HighBondAskYieldAll => "HIGH_BOND_ASK_YIELD_ALL",
            Self::HighBondDebt2BookRatio => "HIGH_BOND_DEBT_2_BOOK_RATIO",
            Self::HighBondDebt2EquityRatio => "HIGH_BOND_DEBT_2_EQUITY_RATIO",
            Self::HighBondDebt2TanBookRatio => "HIGH_BOND_DEBT_2_TAN_BOOK_RATIO",
            Self::HighBondEquity2BookRatio => "HIGH_BOND_EQUITY_2_BOOK_RATIO",
            Self::HighBondEquity2TanBookRatio => "HIGH_BOND_EQUITY_2_TAN_BOOK_RATIO",
            Self::HighBondNetAskCurrentYieldAll => "HIGH_BOND_NET_ASK_CURRENT_YIELD_ALL",
            Self::HighBondNetAskYieldAll => "HIGH_BOND_NET_ASK_YIELD_ALL",
            Self::HighBondNetSpreadAll => "HIGH_BOND_NET_SPREAD_ALL",
            Self::HighBondSpreadAll => "HIGH_BOND_SPREAD_ALL",
            Self::HighCouponRate => "HIGH_COUPON_RATE",
            Self::HighDividendYield => "HIGH_DIVIDEND_YIELD",
            Self::HighDividendYieldIb => "HIGH_DIVIDEND_YIELD_IB",
            Self::HighestSlbBid => "HIGHEST_SLB_BID",
            Self::HighGrowthRate => "HIGH_GROWTH_RATE",
            Self::HighMoodyRatingAll => "HIGH_MOODY_RATING_ALL",
            Self::HighOpenGap => "HIGH_OPEN_GAP",
            Self::HighOptImpVolat => "HIGH_OPT_IMP_VOLAT",
            Self::HighOptImpVolatOverHist => "HIGH_OPT_IMP_VOLAT_OVER_HIST",
            Self::HighOptOpenInterestPutCallRatio => "HIGH_OPT_OPEN_INTEREST_PUT_CALL_RATIO",
            Self::HighOptVolumePutCallRatio => "HIGH_OPT_VOLUME_PUT_CALL_RATIO",
            Self::HighPeRatio => "HIGH_PE_RATIO",
            Self::HighPrice2BookRatio => "HIGH_PRICE_2_BOOK_RATIO",
            Self::HighPrice2TanBookRatio => "HIGH_PRICE_2_TAN_BOOK_RATIO",
            Self::HighQuickRatio => "HIGH_QUICK_RATIO",
            Self::HighReturnOnEquity => "HIGH_RETURN_ON_EQUITY",
            Self::HighSynthBidRevNatYield => "HIGH_SYNTH_BID_REV_NAT_YIELD",
            Self::HighVs13wHl => "HIGH_VS_13W_HL",
            Self::HighVs26wHl => "HIGH_VS_26W_HL",
            Self::HighVs52wHl => "HIGH_VS_52W_HL",
            Self::HotByOptVolume => "HOT_BY_OPT_VOLUME",
            Self::HotByPrice => "HOT_BY_PRICE",
            Self::HotByPriceRange => "HOT_BY_PRICE_RANGE",
            Self::HotByVolume => "HOT_BY_VOLUME",
            Self::LimitUpDown => "LIMIT_UP_DOWN",
            Self::LowBondBidCurrentYieldAll => "LOW_BOND_BID_CURRENT_YIELD_ALL",
            Self::LowBondBidYieldAll => "LOW_BOND_BID_YIELD_ALL",
            Self::LowBondDebt2BookRatio => "LOW_BOND_DEBT_2_BOOK_RATIO",
            Self::LowBondDebt2EquityRatio => "LOW_BOND_DEBT_2_EQUITY_RATIO",
            Self::LowBondDebt2TanBookRatio => "LOW_BOND_DEBT_2_TAN_BOOK_RATIO",
            Self::LowBondEquity2BookRatio => "LOW_BOND_EQUITY_2_BOOK_RATIO",
            Self::LowBondEquity2TanBookRatio => "LOW_BOND_EQUITY_2_TAN_BOOK_RATIO",
            Self::LowBondNetBidCurrentYieldAll => "LOW_BOND_NET_BID_CURRENT_YIELD_ALL",
            Self::LowBondNetBidYieldAll => "LOW_BOND_NET_BID_YIELD_ALL",
            Self::LowBondNetSpreadAll => "LOW_BOND_NET_SPREAD_ALL",
            Self::LowBondSpreadAll => "LOW_BOND_SPREAD_ALL",
            Self::LowCouponRate => "LOW_COUPON_RATE",
            Self::LowestSlbAsk => "LOWEST_SLB_ASK",
            Self::LowGrowthRate => "LOW_GROWTH_RATE",
            Self::LowMoodyRatingAll => "LOW_MOODY_RATING_ALL",
            Self::LowOpenGap => "LOW_OPEN_GAP",
            Self::LowOptImpVolat => "LOW_OPT_IMP_VOLAT",
            Self::LowOptImpVolatOverHist => "LOW_OPT_IMP_VOLAT_OVER_HIST",
            Self::LowOptOpenInterestPutCallRatio => "LOW_OPT_OPEN_INTEREST_PUT_CALL_RATIO",
            Self::LowOptVolumePutCallRatio => "LOW_OPT_VOLUME_PUT_CALL_RATIO",
            Self::LowPeRatio => "LOW_PE_RATIO",
            Self::LowPrice2BookRatio => "LOW_PRICE_2_BOOK_RATIO",
            Self::LowPrice2TanBookRatio => "LOW_PRICE_2_TAN_BOOK_RATIO",
            Self::LowQuickRatio => "LOW_QUICK_RATIO",
            Self::LowReturnOnEquity => "LOW_RETURN_ON_EQUITY",
            Self::LowSynthAskRevNatYield => "LOW_SYNTH_ASK_REV_NAT_YIELD",
            Self::LowVs13wHl => "LOW_VS_13W_HL",
            Self::LowVs26wHl => "LOW_VS_26W_HL",
            Self::LowVs52wHl => "LOW_VS_52W_HL",
            Self::LowWarRelImpVolat => "LOW_WAR_REL_IMP_VOLAT",
            Self::MarketCapUsdAsc => "MARKET_CAP_USD_ASC",
            Self::MarketCapUsdDesc => "MARKET_CAP_USD_DESC",
            Self::MostActiveAvgUsd => "MOST_ACTIVE_AVG_USD",
            Self::MostActiveUsd => "MOST_ACTIVE_USD",
            Self::NearMaturityDate => "NEAR_MATURITY_DATE",
            Self::NotOpen => "NOT_OPEN",
            Self::OptOpenInterestMostActive => "OPT_OPEN_INTEREST_MOST_ACTIVE",
            Self::OptVolumeMostActive => "OPT_VOLUME_MOST_ACTIVE",
            Self::PmonitorAvailContracts => "PMONITOR_AVAIL_CONTRACTS",
            Self::PmonitorCtt => "PMONITOR_CTT",
            Self::PmonitorIbond => "PMONITOR_IBOND",
            Self::PmonitorRfq => "PMONITOR_RFQ",
            Self::TopOpenPercGain => "TOP_OPEN_PERC_GAIN",
            Self::TopOpenPercLose => "TOP_OPEN_PERC_LOSE",
            Self::TopOptImpVolatGain => "TOP_OPT_IMP_VOLAT_GAIN",
            Self::TopOptImpVolatLose => "TOP_OPT_IMP_VOLAT_LOSE",
            Self::TopPriceRange => "TOP_PRICE_RANGE",
            Self::TopStockBuyImbalanceAdvRatio => "TOP_STOCK_BUY_IMBALANCE_ADV_RATIO",
            Self::TopStockSellImbalanceAdvRatio => "TOP_STOCK_SELL_IMBALANCE_ADV_RATIO",
            Self::TopTradeCount => "TOP_TRADE_COUNT",
            Self::TopTradeRate => "TOP_TRADE_RATE",
            Self::TopVolumeRate => "TOP_VOLUME_RATE",
            Self::WshNextAnalystMeeting => "WSH_NEXT_ANALYST_MEETING",
            Self::WshNextEarnings => "WSH_NEXT_EARNINGS",
            Self::WshNextEvent => "WSH_NEXT_EVENT",
            Self::WshNextMajorEvent => "WSH_NEXT_MAJOR_EVENT",
            Self::WshPrevAnalystMeeting => "WSH_PREV_ANALYST_MEETING",
            Self::WshPrevEarnings => "WSH_PREV_EARNINGS",
            Self::WshPrevEvent => "WSH_PREV_EVENT",
        }
        .serialize(serializer)
    }
}

// if (m_serverVersion < 24) {
//           error(EClientErrors.NO_VALID_ID, EClientErrors.UPDATE_TWS,
//                 "  It does not support API scanner subscription.");

//           return;
//         }

// MIN_SERVER_VER_SCANNER_GENERIC_OPTS = 143
// if (m_serverVersion < MIN_SERVER_VER_SCANNER_GENERIC_OPTS && scannerSubscriptionFilterOptions != null) {
//     error(EClientErrors.NO_VALID_ID, EClientErrors.UPDATE_TWS,
//             " It does not support API scanner subscription generic filter options");

//     return;
// }
//
//   final int VERSION = 4;
