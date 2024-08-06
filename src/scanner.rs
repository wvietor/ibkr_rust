use ibapi_macros::typed_variants;

use super::*;
use std::{f64, i32};

//== SCANNER PARAMETERS ==
// #[derive(Debug)]
// pub struct Xml {
//     data: String,
// }
// impl Xml {
//     pub fn len(&self) -> usize {
//         self.data.len()
//     }
// }
// impl FromStr for Xml {
//     type Err = std::error::Error;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {}
// }

// + -> Instrument -> subset of ScannerFiler(optional) and ScanCode
//
const NO_ROW_NUMBER_SPECIFIED: i32 = -1;

#[derive(Debug, Clone)]
/// Defines a market scanner request, including its filters.
///
/// # Summary
///
/// The `ScannerSubscription` struct represents a market scanner subscription,
/// containing filters that refine the scanning results. It allows specifying the
/// instrument type, location, market scan parameters, price, volume, ratings, and more.
pub struct ScannerSubscription {
    /// The number of rows to be returned for the query. Default: -1.
    number_of_rows: i32,
    /// The instrument's type for the scan. I.e. STK, FUT.HK, etc.
    instrument: Instrument,
    /// The request's location (STK.US, STK.US.MAJOR, etc).
    location_code: LocationCode,
    /// Same as TWS Market Scanner's "parameters" field, for example: TOP_PERC_GAIN.
    scan_code: ScanCode,
    /// Filters out Contracts which price is below this value.
    above_price: f64,
    /// Filters out contracts which price is above this value.
    below_price: f64,
    /// Filters out Contracts which volume is above this value.
    above_volume: i32,
    /// Filters out Contracts which option volume is above this value.
    average_option_volume_above: i32,
    /// Filters out Contracts which market cap is above this value.
    market_cap_above: f64,
    /// Filters out Contracts which market cap is below this value.
    market_cap_below: f64,
    /// Filters out Contracts which Moody's rating is below this value. (AA3 A1 A2 A3 BAA1 BAA2 BAA3 BA1 BA2 BA3 B1 B2 B3 CAA1 CAA2 CAA3 CA C NR)
    moody_rating_above: MoodyRatingFilters,
    /// Filters out Contracts which Moody's rating is above this value. (AA3 A1 A2 A3 BAA1 BAA2 BAA3 BA1 BA2 BA3 B1 B2 B3 CAA1 CAA2 CAA3 CA C NR)
    moody_rating_below: MoodyRatingFilters,
    /// Filters out Contracts with a S&P rating below this value. (AAA AA+ AA AA- A+ A A- BBB+ BBB BBB- BB+ BB BB- B+ B B- CCC+ CCC CCC- CC+ CC CC- C+ C C- D NR )
    sp_rating_above: SnPRatingFilters,
    /// Filters out Contracts with a S&P rating above this value. (AAA AA+ AA AA- A+ A A- BBB+ BBB BBB- BB+ BB BB- B+ B B- CCC+ CCC CCC- CC+ CC CC- C+ C C- D NR)
    sp_rating_below: SnPRatingFilters,
    /// Filter out Contracts with a maturity date earlier than this value. (mm/yyyy or yyyymmdd)
    maturity_date_above: String,
    /// Filter out Contracts with a maturity date older than this value.
    maturity_date_below: String,
    /// Filter out Contracts with a coupon rate lower than this value.
    coupon_rate_above: f64,
    /// Filter out Contracts with a coupon rate higher than this value.
    coupon_rate_below: f64,
    /// Filters out Convertible bonds.
    exclude_convertible: String,
    /// For example, a pairing "Annual, true" used on the "top Option Implied Vol % Gainers" scan would return annualized volatilities.
    scanner_setting_pairs: String,
    /// CORP = Corporation ADR = American Depositary Receipt ETF = Exchange Traded Fund REIT = Real Estate Investment Trust CEF = Closed End Fund
    stock_type_filter: StockTypesFilters,
}

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
#[allow(missing_docs)]
pub struct TagValue {
    tag: String,
    value: String,
}

pub struct ScannerOptions {
    tags_values: Vec<TagValue>,
    index: usize,
}

impl ScannerOptions {
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

pub struct ScannerFilterOptions;
//Instrument < - > SecType ?

#[typed_variants]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum Instrument {
    #[serde(rename = "STK")]
    UsStocks,

    #[serde(rename = "IND.US")]
    UsIndexes,
    #[serde(rename = "SLB.US")]
    UsSbls,
    #[serde(rename = "BOND")]
    CorporateBonds,
    #[serde(rename = "BOND.CD")]
    UsCds,
    #[serde(rename = "BOND.AGNCY")]
    UsAgencyBonds,
    #[serde(rename = "BOND.GOVT")]
    UsTreasuries,
    #[serde(rename = "BOND.MUNI")]
    UsMunicipalBonds,
    #[serde(rename = "BOND.GOVT.NON-US")]
    NonUsSovereignBonds,
    #[serde(rename = "FUND.ALL")]
    MutualFunds,
    #[serde(rename = "STOCK.NA")]
    AmericaNonUsStocks,
    #[serde(rename = "FUT.NA")]
    AmericaNonUsFutures,
    #[serde(rename = "SSF.NA")]
    AmericaNonUsSsfs,
    #[serde(rename = "STOCK.EU")]
    EuropeStocks,
    #[serde(rename = "FUT.EU")]
    EuropeFutures,
    #[serde(rename = "IND.EU")]
    EuropeIndexes,
    #[serde(rename = "SSF.EU")]
    EuropeSsfs,
    #[serde(rename = "STOCK.ME")]
    MidEastStocks,
    #[serde(rename = "STOCK.HK")]
    AsiaStocks,
    #[serde(rename = "FUT.HK")]
    AsiaFutures,
    #[serde(rename = "IND.HK")]
    AsiaIndexes,
    #[serde(rename = "SSF.HK")]
    AsiaSsfs,
    #[serde(rename = "NATCOMB")]
    NativeCombos,
    // SSF.US ?
    // FUND.ALL
    // <name>Global Stocks</name>
    // <type>Global</type>
    // <name>Global Futures</name>
    // <type>Global</type>
    // <name>Global Indexes</name>
    // <type>Global</type>
    // <name>Global SSFs</name>
    // <type>Global</type>
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum LocationCode {
    //ETF.EQ.US.MAJOR
    //ETF.EQ.ARCA
    //ETF.EQ.BATS
    // ETF.FI.US.MAJOR
    // ETF.FI.ARCA
    // ETF.FI.NASDAQ.NMS
    // ETF.FI.BATS
    // FUT.CME
    // FUT.CBOT
    // FUT.NYMEX
    // FUT.COMEX
    // FUT.IPE
    // FUT.ENDEX
    // FUT.NYBOT
    // FUT.NYSELIFFE
    // FUT.CFE
    // FUT.ICECRYPTO
    // sub Location ???
    #[serde(rename = "BOND.US")]
    BondUs,
    #[serde(rename = "EFP")]
    Efp,
    #[serde(rename = "FUT.EU.BELFOX")]
    FutEuBelfox,
    #[serde(rename = "FUT.EU.FTA")]
    FutEuFta,
    #[serde(rename = "FUT.EU.IDEM")]
    FutEuIdem,
    #[serde(rename = "FUT.EU.LIFFE")]
    FutEuLiffe,
    #[serde(rename = "FUT.EU.MEFFRV")]
    FutEuMeffrv,
    #[serde(rename = "FUT.EU.MONEP")]
    FutEuMonep,
    #[serde(rename = "FUT.EU")]
    FutEu,
    #[serde(rename = "FUT.HK.HKFE")]
    FutHkHkfe,
    #[serde(rename = "FUT.HK.JAPAN")]
    FutHkJapan,
    #[serde(rename = "FUT.HK.KSE")]
    FutHkKse,
    #[serde(rename = "FUT.HK.NSE")]
    FutHkNse,
    #[serde(rename = "FUT.HK.OSE.JPN")]
    FutHkOseJpn,
    #[serde(rename = "FUT.HK.SGX")]
    FutHkSgx,
    #[serde(rename = "FUT.HK.SNFE")]
    FutHkSnfe,
    #[serde(rename = "FUT.HK.TSE.JPN")]
    FutHkTseJpn,
    #[serde(rename = "FUT.HK")]
    FutHk,
    #[serde(rename = "FUT.IPE")]
    FutIpe,
    #[serde(rename = "FUT.NA.CDE")]
    FutNaCde,
    #[serde(rename = "FUT.NA")]
    FutNa,
    #[serde(rename = "FUT.NYBOT")]
    FutNybot,
    #[serde(rename = "FUT.NYSELIFFE")]
    FutNyseliffe,
    #[serde(rename = "FUT.US")]
    FutUs,
    #[serde(rename = "IND.EU.BELFOX")]
    IndEuBelfox,
    #[serde(rename = "IND.EU.FTA")]
    IndEuFta,
    #[serde(rename = "IND.EU.LIFFE")]
    IndEuLiffe,
    #[serde(rename = "IND.EU.MONEP")]
    IndEuMonep,
    #[serde(rename = "IND.EU")]
    IndEu,
    #[serde(rename = "IND.HK.HKFE")]
    IndHkHkfe,
    #[serde(rename = "IND.HK.JAPAN")]
    IndHkJapan,
    #[serde(rename = "IND.HK.KSE")]
    IndHkKse,
    #[serde(rename = "IND.HK.NSE")]
    IndHkNse,
    #[serde(rename = "IND.HK.OSE.JPN")]
    IndHkOseJpn,
    #[serde(rename = "IND.HK.SGX")]
    IndHkSgx,
    #[serde(rename = "IND.HK.SNFE")]
    IndHkSnfe,
    #[serde(rename = "IND.HK.TSE.JPN")]
    IndHkTseJpn,
    #[serde(rename = "IND.HK")]
    IndHk,
    #[serde(rename = "IND.US")]
    IndUs,
    #[serde(rename = "SLB.AQS")]
    SlbAqs,
    #[serde(rename = "STK.AMEX")]
    StkAmex,
    #[serde(rename = "STK.ARCA")]
    StkArca,
    #[serde(rename = "STK.EU.AEB")]
    StkEuAeb,
    #[serde(rename = "STK.EU.BM")]
    StkEuBm,
    #[serde(rename = "STK.EU.BVME")]
    StkEuBvme,
    #[serde(rename = "STK.EU.EBS")]
    StkEuEbs,
    #[serde(rename = "STK.EU.IBIS")]
    StkEuIbis,
    #[serde(rename = "STK.EU.IBIS-ETF")]
    StkEuIbisEtf,
    #[serde(rename = "STK.EU.IBIS-EUSTARS")]
    StkEuIbisEustars,
    #[serde(rename = "STK.EU.IBIS-NEWX")]
    StkEuIbisNewx,
    #[serde(rename = "STK.EU.IBIS-USSTARS")]
    StkEuIbisUsstars,
    #[serde(rename = "STK.EU.IBIS-XETRA")]
    StkEuIbisXetra,
    #[serde(rename = "STK.EU.LSE")]
    StkEuLse,
    #[serde(rename = "STK.EU.SBF")]
    StkEuSbf,
    #[serde(rename = "STK.EU.SBVM")]
    StkEuSbvm,
    #[serde(rename = "STK.EU.SFB")]
    StkEuSfb,
    #[serde(rename = "STK.EU.SWISS")]
    StkEuSwiss,
    #[serde(rename = "STK.EU.VIRTX")]
    StkEuVirtx,
    #[serde(rename = "STK.EU")]
    StkEu,
    #[serde(rename = "STK.HK.ASX")]
    StkHkAsx,
    #[serde(rename = "STK.HK.NSE")]
    StkHkNse,
    #[serde(rename = "STK.HK.SEHK")]
    StkHkSehk,
    #[serde(rename = "STK.HK.SGX")]
    StkHkSgx,
    #[serde(rename = "STK.HK.TSE.JPN")]
    StkHkTseJpn,
    #[serde(rename = "STK.HK")]
    StkHk,
    #[serde(rename = "STK.NA.CANADA")]
    StkNaCanada,
    #[serde(rename = "STK.NA.TSE")]
    StkNaTse,
    #[serde(rename = "STK.NA.VENTURE")]
    StkNaVenture,
    #[serde(rename = "STK.NA")]
    StkNa,
    #[serde(rename = "STK.NASDAQ.NMS")]
    StkNasdaqNms,
    #[serde(rename = "STK.NASDAQ.SCM")]
    StkNasdaqScm,
    #[serde(rename = "STK.NASDAQ")]
    StkNasdaq,
    #[serde(rename = "STK.NYSE")]
    StkNyse,
    #[serde(rename = "STK.OTCBB")]
    StkOtcbb,
    #[serde(rename = "STK.PINK")]
    StkPink,
    #[serde(rename = "STK.US.MAJOR")]
    StkUsMajor,
    #[serde(rename = "STK.US.MINOR")]
    StkUsMinor,
    #[serde(rename = "STK.US")]
    StkUs,
    #[serde(rename = "WAR.EU.ALL")]
    WarEuAll,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum ScanCode {
    #[serde(rename = "TOP_PERC_GAIN")]
    TopPercGain,
    #[serde(rename = "TOP_PERC_LOSE")]
    TopPercLose,
    #[serde(rename = "MOST_ACTIVE")]
    MostActive,
    #[serde(rename = "ALL_SYMBOLS_ASC")]
    AllSymbolsAsc,
    #[serde(rename = "ALL_SYMBOLS_DESC")]
    AllSymbolsDesc,
    #[serde(rename = "BOND_CUSIP_AZ")]
    BondCusipAz,
    #[serde(rename = "BOND_CUSIP_ZA")]
    BondCusipZa,
    #[serde(rename = "FAR_MATURITY_DATE")]
    FarMaturityDate,
    #[serde(rename = "HALTED")]
    Halted,
    #[serde(rename = "HIGH_BOND_ASK_CURRENT_YIELD_ALL")]
    HighBondAskCurrentYieldAll,
    #[serde(rename = "HIGH_BOND_ASK_YIELD_ALL")]
    HighBondAskYieldAll,
    #[serde(rename = "HIGH_BOND_DEBT_2_BOOK_RATIO")]
    HighBondDebt2BookRatio,
    #[serde(rename = "HIGH_BOND_DEBT_2_EQUITY_RATIO")]
    HighBondDebt2EquityRatio,
    #[serde(rename = "HIGH_BOND_DEBT_2_TAN_BOOK_RATIO")]
    HighBondDebt2TanBookRatio,
    #[serde(rename = "HIGH_BOND_EQUITY_2_BOOK_RATIO")]
    HighBondEquity2BookRatio,
    #[serde(rename = "HIGH_BOND_EQUITY_2_TAN_BOOK_RATIO")]
    HighBondEquity2TanBookRatio,
    #[serde(rename = "HIGH_BOND_NET_ASK_CURRENT_YIELD_ALL")]
    HighBondNetAskCurrentYieldAll,
    #[serde(rename = "HIGH_BOND_NET_ASK_YIELD_ALL")]
    HighBondNetAskYieldAll,
    #[serde(rename = "HIGH_BOND_NET_SPREAD_ALL")]
    HighBondNetSpreadAll,
    #[serde(rename = "HIGH_BOND_SPREAD_ALL")]
    HighBondSpreadAll,
    #[serde(rename = "HIGH_COUPON_RATE")]
    HighCouponRate,
    #[serde(rename = "HIGH_DIVIDEND_YIELD")]
    HighDividendYield,
    #[serde(rename = "HIGH_DIVIDEND_YIELD_IB")]
    HighDividendYieldIb,
    #[serde(rename = "HIGHEST_SLB_BID")]
    HighestSlbBid,
    #[serde(rename = "HIGH_GROWTH_RATE")]
    HighGrowthRate,
    #[serde(rename = "HIGH_MOODY_RATING_ALL")]
    HighMoodyRatingAll,
    #[serde(rename = "HIGH_OPEN_GAP")]
    HighOpenGap,
    #[serde(rename = "HIGH_OPT_IMP_VOLAT")]
    HighOptImpVolat,
    #[serde(rename = "HIGH_OPT_IMP_VOLAT_OVER_HIST")]
    HighOptImpVolatOverHist,
    #[serde(rename = "HIGH_OPT_OPEN_INTEREST_PUT_CALL_RATIO")]
    HighOptOpenInterestPutCallRatio,
    #[serde(rename = "HIGH_OPT_VOLUME_PUT_CALL_RATIO")]
    HighOptVolumePutCallRatio,
    #[serde(rename = "HIGH_PE_RATIO")]
    HighPeRatio,
    #[serde(rename = "HIGH_PRICE_2_BOOK_RATIO")]
    HighPrice2BookRatio,
    #[serde(rename = "HIGH_PRICE_2_TAN_BOOK_RATIO")]
    HighPrice2TanBookRatio,
    #[serde(rename = "HIGH_QUICK_RATIO")]
    HighQuickRatio,
    #[serde(rename = "HIGH_RETURN_ON_EQUITY")]
    HighReturnOnEquity,
    #[serde(rename = "HIGH_SYNTH_BID_REV_NAT_YIELD")]
    HighSynthBidRevNatYield,
    #[serde(rename = "HIGH_VS_13W_HL")]
    HighVs13wHl,
    #[serde(rename = "HIGH_VS_26W_HL")]
    HighVs26wHl,
    #[serde(rename = "HIGH_VS_52W_HL")]
    HighVs52wHl,
    #[serde(rename = "HOT_BY_OPT_VOLUME")]
    HotByOptVolume,
    #[serde(rename = "HOT_BY_PRICE")]
    HotByPrice,
    #[serde(rename = "HOT_BY_PRICE_RANGE")]
    HotByPriceRange,
    #[serde(rename = "HOT_BY_VOLUME")]
    HotByVolume,
    #[serde(rename = "LIMIT_UP_DOWN")]
    LimitUpDown,
    #[serde(rename = "LOW_BOND_BID_CURRENT_YIELD_ALL")]
    LowBondBidCurrentYieldAll,
    #[serde(rename = "LOW_BOND_BID_YIELD_ALL")]
    LowBondBidYieldAll,
    #[serde(rename = "LOW_BOND_DEBT_2_BOOK_RATIO")]
    LowBondDebt2BookRatio,
    #[serde(rename = "LOW_BOND_DEBT_2_EQUITY_RATIO")]
    LowBondDebt2EquityRatio,
    #[serde(rename = "LOW_BOND_DEBT_2_TAN_BOOK_RATIO")]
    LowBondDebt2TanBookRatio,
    #[serde(rename = "LOW_BOND_EQUITY_2_BOOK_RATIO")]
    LowBondEquity2BookRatio,
    #[serde(rename = "LOW_BOND_EQUITY_2_TAN_BOOK_RATIO")]
    LowBondEquity2TanBookRatio,
    #[serde(rename = "LOW_BOND_NET_BID_CURRENT_YIELD_ALL")]
    LowBondNetBidCurrentYieldAll,
    #[serde(rename = "LOW_BOND_NET_BID_YIELD_ALL")]
    LowBondNetBidYieldAll,
    #[serde(rename = "LOW_BOND_NET_SPREAD_ALL")]
    LowBondNetSpreadAll,
    #[serde(rename = "LOW_BOND_SPREAD_ALL")]
    LowBondSpreadAll,
    #[serde(rename = "LOW_COUPON_RATE")]
    LowCouponRate,
    #[serde(rename = "LOWEST_SLB_ASK")]
    LowestSlbAsk,
    #[serde(rename = "LOW_GROWTH_RATE")]
    LowGrowthRate,
    #[serde(rename = "LOW_MOODY_RATING_ALL")]
    LowMoodyRatingAll,
    #[serde(rename = "LOW_OPEN_GAP")]
    LowOpenGap,
    #[serde(rename = "LOW_OPT_IMP_VOLAT")]
    LowOptImpVolat,
    #[serde(rename = "LOW_OPT_IMP_VOLAT_OVER_HIST")]
    LowOptImpVolatOverHist,
    #[serde(rename = "LOW_OPT_OPEN_INTEREST_PUT_CALL_RATIO")]
    LowOptOpenInterestPutCallRatio,
    #[serde(rename = "LOW_OPT_VOLUME_PUT_CALL_RATIO")]
    LowOptVolumePutCallRatio,
    #[serde(rename = "LOW_PE_RATIO")]
    LowPeRatio,
    #[serde(rename = "LOW_PRICE_2_BOOK_RATIO")]
    LowPrice2BookRatio,
    #[serde(rename = "LOW_PRICE_2_TAN_BOOK_RATIO")]
    LowPrice2TanBookRatio,
    #[serde(rename = "LOW_QUICK_RATIO")]
    LowQuickRatio,
    #[serde(rename = "LOW_RETURN_ON_EQUITY")]
    LowReturnOnEquity,
    #[serde(rename = "LOW_SYNTH_ASK_REV_NAT_YIELD")]
    LowSynthAskRevNatYield,
    #[serde(rename = "LOW_VS_13W_HL")]
    LowVs13wHl,
    #[serde(rename = "LOW_VS_26W_HL")]
    LowVs26wHl,
    #[serde(rename = "LOW_VS_52W_HL")]
    LowVs52wHl,
    #[serde(rename = "LOW_WAR_REL_IMP_VOLAT")]
    LowWarRelImpVolat,
    #[serde(rename = "MARKET_CAP_USD_ASC")]
    MarketCapUsdAsc,
    #[serde(rename = "MARKET_CAP_USD_DESC")]
    MarketCapUsdDesc,
    #[serde(rename = "MOST_ACTIVE_AVG_USD")]
    MostActiveAvgUsd,
    #[serde(rename = "MOST_ACTIVE_USD")]
    MostActiveUsd,
    #[serde(rename = "NEAR_MATURITY_DATE")]
    NearMaturityDate,
    #[serde(rename = "NOT_OPEN")]
    NotOpen,
    #[serde(rename = "OPT_OPEN_INTEREST_MOST_ACTIVE")]
    OptOpenInterestMostActive,
    #[serde(rename = "OPT_VOLUME_MOST_ACTIVE")]
    OptVolumeMostActive,
    #[serde(rename = "PMONITOR_AVAIL_CONTRACTS")]
    PmonitorAvailContracts,
    #[serde(rename = "PMONITOR_CTT")]
    PmonitorCtt,
    #[serde(rename = "PMONITOR_IBOND")]
    PmonitorIbond,
    #[serde(rename = "PMONITOR_RFQ")]
    PmonitorRfq,
    #[serde(rename = "TOP_OPEN_PERC_GAIN")]
    TopOpenPercGain,
    #[serde(rename = "TOP_OPEN_PERC_LOSE")]
    TopOpenPercLose,
    #[serde(rename = "TOP_OPT_IMP_VOLAT_GAIN")]
    TopOptImpVolatGain,
    #[serde(rename = "TOP_OPT_IMP_VOLAT_LOSE")]
    TopOptImpVolatLose,
    #[serde(rename = "TOP_PRICE_RANGE")]
    TopPriceRange,
    #[serde(rename = "TOP_STOCK_BUY_IMBALANCE_ADV_RATIO")]
    TopStockBuyImbalanceAdvRatio,
    #[serde(rename = "TOP_STOCK_SELL_IMBALANCE_ADV_RATIO")]
    TopStockSellImbalanceAdvRatio,
    #[serde(rename = "TOP_TRADE_COUNT")]
    TopTradeCount,
    #[serde(rename = "TOP_TRADE_RATE")]
    TopTradeRate,
    #[serde(rename = "TOP_VOLUME_RATE")]
    TopVolumeRate,
    #[serde(rename = "WSH_NEXT_ANALYST_MEETING")]
    WshNextAnalystMeeting,
    #[serde(rename = "WSH_NEXT_EARNINGS")]
    WshNextEarnings,
    #[serde(rename = "WSH_NEXT_EVENT")]
    WshNextEvent,
    #[serde(rename = "WSH_NEXT_MAJOR_EVENT")]
    WshNextMajorEvent,
    #[serde(rename = "WSH_PREV_ANALYST_MEETING")]
    WshPrevAnalystMeeting,
    #[serde(rename = "WSH_PREV_EARNINGS")]
    WshPrevEarnings,
    #[serde(rename = "WSH_PREV_EVENT")]
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

#[typed_variants]
#[derive(Debug, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum ScannerFilters {
    #[serde(rename = "PRICE")] //DoubleField: priceAbove, priceBelow
    Price,
    #[serde(rename = "PRICE_USD")]
    PriceUsd,
    #[serde(rename = "VOLUME")]
    Volume,
    #[serde(rename = "VOLUME_USD")]
    VolumeUsd,
    #[serde(rename = "AVGVOLUME")]
    AvgVolume,
    #[serde(rename = "AVGVOLUME_USD")]
    AvgVolumeUsd,
    #[serde(rename = "NUMSHARESINSIDER")]
    NumSharesInsider,
    #[serde(rename = "INSIDEROFFLOATPERC")]
    InsiderOfFloatPerc,
    #[serde(rename = "NUMSHARESINSTITUTIONAL")]
    NumSharesInstitutional,
    #[serde(rename = "INSTITUTIONALOFFLOATPERC")]
    InstitutionalOfFloatPerc,
    #[serde(rename = "MKTCAP")]
    MktCap,
    #[serde(rename = "MOODY")] //ComboField: moodyRatingAbove, moodyRatingBelow
    Moody,
    #[serde(rename = "SP")]
    Sp,
    #[serde(rename = "BOND_RATINGS_RELATION")]
    BondRatingsRelation,
    #[serde(rename = "BONDCREDITRATING")]
    BondCreditRating,
    #[serde(rename = "MATDATE")]
    MatDate,
    #[serde(rename = "CURRENCY")]
    Currency,
    #[serde(rename = "CONVOPT")]
    ConvOpt,
    #[serde(rename = "CPNRATE")]
    CpnRate,
    #[serde(rename = "OPTVOLUME")]
    OptVolume,
    #[serde(rename = "AVGOPTVOLUME")]
    AvgOptVolume,
    #[serde(rename = "OPTVOLUMEPCRATIO")]
    OptVolumePcRatio,
    #[serde(rename = "IMPVOLAT")]
    ImpVolat,
    #[serde(rename = "IMPVOLATOVERHIST")]
    ImpVolatOverHist,
    #[serde(rename = "IMBALANCE")]
    Imbalance,
    #[serde(rename = "IMBALANCEADVRATIOPERC")]
    ImbalanceAdvRatioPerc,
    #[serde(rename = "REGIMBALANCE")]
    RegImbalance,
    #[serde(rename = "REGIMBALANCEADVRATIOPERC")]
    RegImbalanceAdvRatioPerc,
    #[serde(rename = "AVGRATING")]
    AvgRating,
    #[serde(rename = "NUMRATINGS")]
    NumRatings,
    #[serde(rename = "AVGPRICETARGET")]
    AvgPriceTarget,
    #[serde(rename = "NUMPRICETARGETS")]
    NumPriceTargets,
    #[serde(rename = "AVGTARGET2PRICERATIO")]
    AvgTarget2PriceRatio,
    #[serde(rename = "STKTYPE")]
    StkType,
    #[serde(rename = "HASOPTIONS")] //bool
    HasOptions,
    #[serde(rename = "PEAELIGIBLE")]
    PeaEligible,
    #[serde(rename = "LEADFUT")]
    LeadFut,
    #[serde(rename = "DIVIB")]
    DivIb,
    #[serde(rename = "DIVYIELDIB")]
    DivYieldIb,
    #[serde(rename = "NEXTDIVDATE")]
    NextDivDate,
    #[serde(rename = "NEXTDIVAMOUNT")]
    NextDivAmount,
    #[serde(rename = "HISTDIVIB")]
    HistDivIb,
    #[serde(rename = "HISTDIVYIELDIB")]
    HistDivYieldIb,
    #[serde(rename = "GROWTHRATE")]
    GrowthRate,
    #[serde(rename = "PERATIO")]
    PeRatio,
    #[serde(rename = "QUICKRATIO")]
    QuickRatio,
    #[serde(rename = "RETEQUITY")]
    RetEquity,
    #[serde(rename = "PRICE2BK")]
    Price2Bk,
    #[serde(rename = "PRICE2TANBK")]
    Price2TanBk,
    #[serde(rename = "FIRSTTRADEDATE")]
    FirstTradeDate,
    #[serde(rename = "CHANGEPERC")]
    ChangePerc,
    #[serde(rename = "AFTERHRSCHANGE")]
    AfterHrsChange,
    #[serde(rename = "AFTERHRSCHANGEPERC")]
    AfterHrsChangePerc,
    #[serde(rename = "CHANGEOPENPERC")]
    ChangeOpenPerc,
    #[serde(rename = "OPENGAPPERC")]
    OpenGapPerc,
    #[serde(rename = "PRICERANGE")]
    PriceRange,
    #[serde(rename = "TRADECOUNT")]
    TradeCount,
    #[serde(rename = "TRADERATE")]
    TradeRate,
    #[serde(rename = "VOLUMERATE")]
    VolumeRate,
    #[serde(rename = "STVOLUME_3MIN")]
    StVolume3Min,
    #[serde(rename = "STVOLUME_5MIN")]
    StVolume5Min,
    #[serde(rename = "STVOLUME_10MIN")]
    StVolume10Min,
    #[serde(rename = "PRODCAT")]
    ProdCat,
    #[serde(rename = "SIC")]
    Sic,
    #[serde(rename = "BOND_US_STATE")]
    BondUsState,
    #[serde(rename = "ISSUER_COUNTRY_CODE")]
    IssuerCountryCode,
    #[serde(rename = "BOND_STK_SYMBOL")]
    BondStkSymbol,
    #[serde(rename = "BOND_ISSUER")]
    BondIssuer,
    #[serde(rename = "CALLOPT")]
    CallOpt,
    #[serde(rename = "UNSHORTABLE")]
    Unshortable,
    #[serde(rename = "HALTED")] //bool, false: halted)
    Halted,
    #[serde(rename = "SHORTSALERESTRICTED")]
    ShortSaleRestricted,
    #[serde(rename = "SHORTABLESHARES")]
    ShortableShares,
    #[serde(rename = "FEERATE")]
    FeeRate,
    #[serde(rename = "UTILIZATION")]
    Utilization,
    #[serde(rename = "DEFAULTED")]
    Defaulted,
    #[serde(rename = "TRADING_FLAT")]
    TradingFlat,
    #[serde(rename = "EXCHLISTED")]
    ExchListed,
    #[serde(rename = "FDICINS")]
    FdicIns,
    #[serde(rename = "VARCPNRATE")]
    VarCpnRate,
    #[serde(rename = "INSURED")]
    Insured,
    #[serde(rename = "GENERAL_OBLIGATION")]
    GeneralObligation,
    #[serde(rename = "REVENUE")]
    Revenue,
    #[serde(rename = "SUBJECT_TO_AMT")]
    SubjectToAmt,
    #[serde(rename = "REFUNDED")]
    Refunded,
    #[serde(rename = "NO_FEDERAL_TAX")]
    NoFederalTax,
    #[serde(rename = "BANK_QUALIFIED")]
    BankQualified,
    #[serde(rename = "BUILD_AMERICA")]
    BuildAmerica,
    #[serde(rename = "BOND_BID_OR_ASK_VALID")]
    BondBidOrAskValid,
    #[serde(rename = "BOND_BID")]
    BondBid,
    #[serde(rename = "BOND_ASK")]
    BondAsk,
    #[serde(rename = "BOND_BID_OR_ASK")]
    BondBidOrAsk,
    #[serde(rename = "BOND_BID_SZ_VALUE")]
    BondBidSzValue,
    #[serde(rename = "BOND_ASK_SZ_VALUE")]
    BondAskSzValue,
    #[serde(rename = "BOND_BID_OR_ASK_SZ_VALUE")]
    BondBidOrAskSzValue,
    #[serde(rename = "BOND_BID_YIELD")]
    BondBidYield,
    #[serde(rename = "BOND_ASK_YIELD")]
    BondAskYield,
    #[serde(rename = "BOND_BID_OR_ASK_YIELD")]
    BondBidOrAskYield,
    #[serde(rename = "BOND_SPREAD")]
    BondSpread,
    #[serde(rename = "BOND_PAYMENT_FREQ")]
    BondPaymentFreq,
    #[serde(rename = "BOND_AMT_OUTSTANDING")]
    BondAmtOutstanding,
    #[serde(rename = "BOND_CALL_PROT")]
    BondCallProt,
    #[serde(rename = "BOND_DURATION")]
    BondDuration,
    #[serde(rename = "BOND_CONVEXITY")]
    BondConvexity,
    #[serde(rename = "BOND_STK_MKTCAP")]
    BondStkMktCap,
    #[serde(rename = "BOND_DEBT_OUTSTANDING")]
    BondDebtOutstanding,
    #[serde(rename = "BOND_DEBT_OUTSTANDING_MUNI")]
    BondDebtOutstandingMuni,
    #[serde(rename = "BOND_DEBT_2_EQUITY_RATIO")]
    BondDebt2EquityRatio,
    #[serde(rename = "BOND_DEBT_2_BOOK_RATIO")]
    BondDebt2BookRatio,
    #[serde(rename = "BOND_DEBT_2_TAN_BOOK_RATIO")]
    BondDebt2TanBookRatio,
    #[serde(rename = "BOND_EQUITY_2_BOOK_RATIO")]
    BondEquity2BookRatio,
    #[serde(rename = "BOND_EQUITY_2_TAN_BOOK_RATIO")]
    BondEquity2TanBookRatio,
    #[serde(rename = "BOND_INITIAL_SIZE")]
    BondInitialSize,
    #[serde(rename = "BOND_INCREMENT_SIZE")]
    BondIncrementSize,
    #[serde(rename = "BOND_STRUCT_REL")]
    BondStructRel,
    #[serde(rename = "BOND_GOVT_SUBTYPE")]
    BondGovtSubtype,
    #[serde(rename = "UNDCONID")]
    UndConId,
    #[serde(rename = "EMA_20")]
    Ema20,
    #[serde(rename = "EMA_50")]
    Ema50,
    #[serde(rename = "EMA_100")]
    Ema100,
    #[serde(rename = "EMA_200")]
    Ema200,
    #[serde(rename = "PRICE_VS_EMA_20")]
    PriceVsEma20,
    #[serde(rename = "PRICE_VS_EMA_50")]
    PriceVsEma50,
    #[serde(rename = "PRICE_VS_EMA_100")]
    PriceVsEma100,
    #[serde(rename = "PRICE_VS_EMA_200")]
    PriceVsEma200,
    #[serde(rename = "MACD")]
    Macd,
    #[serde(rename = "MACD_SIGNAL")]
    MacdSignal,
    #[serde(rename = "MACD_HISTOGRAM")]
    MacdHistogram,
    #[serde(rename = "PPO")]
    Ppo,
    #[serde(rename = "PPO_SIGNAL")]
    PpoSignal,
    #[serde(rename = "PPO_HISTOGRAM")]
    PpoHistogram,
    #[serde(rename = "ESG_SCORE")]
    EsgScore,
    #[serde(rename = "ESG_COMBINED_SCORE")]
    EsgCombinedScore,
    #[serde(rename = "ESG_CONTROVERSIES_SCORE")]
    EsgControversiesScore,
    #[serde(rename = "ESG_RESOURCE_USE_SCORE")]
    EsgResourceUseScore,
    #[serde(rename = "ESG_EMISSIONS_SCORE")]
    EsgEmissionsScore,
    #[serde(rename = "ESG_ENV_INNOVATION_SCORE")]
    EsgEnvInnovationScore,
    #[serde(rename = "ESG_WORKFORCE_SCORE")]
    EsgWorkforceScore,
    #[serde(rename = "ESG_HUMAN_RIGHTS_SCORE")]
    EsgHumanRightsScore,
    #[serde(rename = "ESG_COMMUNITY_SCORE")]
    EsgCommunityScore,
    #[serde(rename = "ESG_PRODUCT_RESPONSIBILITY_SCORE")]
    EsgProductResponsibilityScore,
    #[serde(rename = "ESG_MANAGEMENT_SCORE")]
    EsgManagementScore,
    #[serde(rename = "ESG_SHAREHOLDERS_SCORE")]
    EsgShareholdersScore,
    #[serde(rename = "ESG_CSR_STRATEGY_SCORE")]
    EsgCsrStrategyScore,
    #[serde(rename = "ESG_ENV_PILLAR_SCORE")]
    EsgEnvPillarScore,
    #[serde(rename = "ESG_SOCIAL_PILLAR_SCORE")]
    EsgSocialPillarScore,
    #[serde(rename = "ESG_CORP_GOV_PILLAR_SCORE")]
    EsgCorpGovPillarScore,
    #[serde(rename = "IV_RANK13")]
    IvRank13,
    #[serde(rename = "IV_RANK26")]
    IvRank26,
    #[serde(rename = "IV_RANK52")]
    IvRank52,
    #[serde(rename = "IV_PERCENTILE13")]
    IvPercentile13,
    #[serde(rename = "IV_PERCENTILE26")]
    IvPercentile26,
    #[serde(rename = "IV_PERCENTILE52")]
    IvPercentile52,
    #[serde(rename = "HV_RANK13")]
    HvRank13,
    #[serde(rename = "HV_RANK26")]
    HvRank26,
    #[serde(rename = "HV_RANK52")]
    HvRank52,
    #[serde(rename = "HV_PERCENTILE13")]
    HvPercentile13,
    #[serde(rename = "HV_PERCENTILE26")]
    HvPercentile26,
    #[serde(rename = "HV_PERCENTILE52")]
    HvPercentile52,
    #[serde(rename = "MF_LDR_TOT_RET_SCR_ALL")]
    MfLdrTotRetScrAll,
    #[serde(rename = "MF_LDR_CONSIS_RET_SCR_ALL")]
    MfLdrConsisRetScrAll,
    #[serde(rename = "MF_LDR_PRESERV_SCR_ALL")]
    MfLdrPreservScrAll,
    #[serde(rename = "MF_LDR_TAX_EFF_SCR_ALL")]
    MfLdrTaxEffScrAll,
    #[serde(rename = "MF_LDR_EXP_SCR_ALL")]
    MfLdrExpScrAll,
    #[serde(rename = "MF_LDR_TOT_RET_SCR_3YR")]
    MfLdrTotRetScr3Yr,
    #[serde(rename = "MF_LDR_CONSIS_RET_SCR_3YR")]
    MfLdrConsisRetScr3Yr,
    #[serde(rename = "MF_LDR_PRESERV_SCR_3YR")]
    MfLdrPreservScr3Yr,
    #[serde(rename = "MF_LDR_TAX_EFF_SCR_3YR")]
    MfLdrTaxEffScr3Yr,
    #[serde(rename = "MF_LDR_EXP_SCR_3YR")]
    MfLdrExpScr3Yr,
    #[serde(rename = "MF_LDR_TOT_RET_SCR_5YR")]
    MfLdrTotRetScr5Yr,
    #[serde(rename = "MF_LDR_CONSIS_RET_SCR_5YR")]
    MfLdrConsisRetScr5Yr,
    #[serde(rename = "MF_LDR_PRESERV_SCR_5YR")]
    MfLdrPreservScr5Yr,
    #[serde(rename = "MF_LDR_TAX_EFF_SCR_5YR")]
    MfLdrTaxEffScr5Yr,
    #[serde(rename = "MF_LDR_EXP_SCR_5YR")]
    MfLdrExpScr5Yr,
    #[serde(rename = "MF_LDR_TOT_RET_SCR_10YR")]
    MfLdrTotRetScr10Yr,
    #[serde(rename = "MF_LDR_CONSIS_RET_SCR_10YR")]
    MfLdrConsisRetScr10Yr,
    #[serde(rename = "MF_LDR_PRESERV_SCR_10YR")]
    MfLdrPreservScr10Yr,
    #[serde(rename = "MF_LDR_TAX_EFF_SCR_10YR")]
    MfLdrTaxEffScr10Yr,
    #[serde(rename = "MF_LDR_EXP_SCR_10YR")]
    MfLdrExpScr10Yr,
    #[serde(rename = "MF_PRICE_CHG_VAL")]
    MfPriceChgVal,
    #[serde(rename = "LIPPER_TOT_NET_ASST")]
    LipperTotNetAsst,
    #[serde(rename = "LIPPER_TOT_EXP_RATIO")]
    LipperTotExpRatio,
    #[serde(rename = "LIPPER_DIST_YLD_1YR")]
    LipperDistYld1Yr,
    #[serde(rename = "LIPPER_GRWTH_CUM")]
    LipperGrwthCum,
    #[serde(rename = "LIPPER_GRWTH_ANN_3YR")]
    LipperGrwthAnn3Yr,
    #[serde(rename = "LIPPER_GRWTH_ANN_5YR")]
    LipperGrwthAnn5Yr,
    #[serde(rename = "LIPPER_GRWTH_ANN_10YR")]
    LipperGrwthAnn10Yr,
    #[serde(rename = "LIPPER_YIELD_1YR")]
    LipperYield1Yr,
    #[serde(rename = "LIPPER_PROJ_YIELD")]
    LipperProjYield,
    #[serde(rename = "LIPPER_PCT_CHANGE")]
    LipperPctChange,
    #[serde(rename = "LIPPER_RSQ_ADJ_1YR")]
    LipperRsqAdj1Yr,
    #[serde(rename = "LIPPER_ALPHA_1YR")]
    LipperAlpha1Yr,
    #[serde(rename = "LIPPER_AVG_LOSS_1YR")]
    LipperAvgLoss1Yr,
    #[serde(rename = "LIPPER_AVG_RETURN_1YR")]
    LipperAvgReturn1Yr,
    #[serde(rename = "LIPPER_BEAR_BETA_1YR")]
    LipperBearBeta1Yr,
    #[serde(rename = "LIPPER_BETA_1YR")]
    LipperBeta1Yr,
    #[serde(rename = "LIPPER_BULL_BETA_1YR")]
    LipperBullBeta1Yr,
    #[serde(rename = "LIPPER_COVAR_1YR")]
    LipperCovar1Yr,
    #[serde(rename = "LIPPER_CORREL_1YR")]
    LipperCorrel1Yr,
    #[serde(rename = "LIPPER_DOWN_DEV_1YR")]
    LipperDownDev1Yr,
    #[serde(rename = "LIPPER_INFO_RATIO_1YR")]
    LipperInfoRatio1Yr,
    #[serde(rename = "LIPPER_MAX_GAIN_1YR")]
    LipperMaxGain1Yr,
    #[serde(rename = "LIPPER_MAX_LOSS_1YR")]
    LipperMaxLoss1Yr,
    #[serde(rename = "LIPPER_MAX_DRAW_1YR")]
    LipperMaxDraw1Yr,
    #[serde(rename = "LIPPER_POS_PERIODS_1YR")]
    LipperPosPeriods1Yr,
    #[serde(rename = "LIPPER_RSQ_1YR")]
    LipperRsq1Yr,
    #[serde(rename = "LIPPER_RET_RISK_RATIO_1YR")]
    LipperRetRiskRatio1Yr,
    #[serde(rename = "LIPPER_SRRI_1YR")]
    LipperSrri1Yr,
    #[serde(rename = "LIPPER_SEMI_DEV_1YR")]
    LipperSemiDev1Yr,
    #[serde(rename = "LIPPER_SEMI_VAR_1YR")]
    LipperSemiVar1Yr,
    #[serde(rename = "LIPPER_SHARPE_1YR")]
    LipperSharpe1Yr,
    #[serde(rename = "LIPPER_SORTINO_1YR")]
    LipperSortino1Yr,
    #[serde(rename = "LIPPER_STD_DEV_1YR")]
    LipperStdDev1Yr,
    #[serde(rename = "LIPPER_TRACKING_ERR_1YR")]
    LipperTrackingErr1Yr,
    #[serde(rename = "LIPPER_TREYNOR_1YR")]
    LipperTreynor1Yr,
    #[serde(rename = "LIPPER_VAR_NORMAL_1YR")]
    LipperVarNormal1Yr,
    #[serde(rename = "LIPPER_VAR_NORMAL_ETL_1YR")]
    LipperVarNormalEtl1Yr,
    #[serde(rename = "LIPPER_VAR_QUANTILE_1YR")]
    LipperVarQuantile1Yr,
    #[serde(rename = "LIPPER_VAR_QUANTILE_ETL_1YR")]
    LipperVarQuantileEtl1Yr,
    #[serde(rename = "LIPPER_VARIANCE_1YR")]
    LipperVariance1Yr,
    #[serde(rename = "LIPPER_NUM_OF_SEC")]
    LipperNumOfSec,
    #[serde(rename = "LIPPER_PAYOUT_RATIO")]
    LipperPayoutRatio,
    #[serde(rename = "LIPPER_PAYOUT_RATIO_5YR")]
    LipperPayoutRatio5Yr,
    #[serde(rename = "LIPPER_DPS_1YR")]
    LipperDps1Yr,
    #[serde(rename = "LIPPER_DPS_3YR")]
    LipperDps3Yr,
    #[serde(rename = "LIPPER_PRICE_2_DIV")]
    LipperPrice2Div,
    #[serde(rename = "LIPPER_DIV_YIELD_WGT_AVG")]
    LipperDivYieldWgtAvg,
    #[serde(rename = "LIPPER_EBIT_2_INT")]
    LipperEbit2Int,
    #[serde(rename = "LIPPER_MKT_CAP_AVG")]
    LipperMktCapAvg,
    #[serde(rename = "LIPPER_OP_CASH_FLOW_GRWTH_RATE_3YR")]
    LipperOpCashFlowGrwthRate3Yr,
    #[serde(rename = "LIPPER_SALES_GRWTH_1YR")]
    LipperSalesGrwth1Yr,
    #[serde(rename = "LIPPER_SALES_GRWTH_3YR")]
    LipperSalesGrwth3Yr,
    #[serde(rename = "LIPPER_SALES_GRWTH_5YR")]
    LipperSalesGrwth5Yr,
    #[serde(rename = "LIPPER_PRICE_2_EARNINGS_LATEST")]
    LipperPrice2EarningsLatest,
    #[serde(rename = "LIPPER_PRICE_2_BOOK_LATEST")]
    LipperPrice2BookLatest,
    #[serde(rename = "LIPPER_PRICE_2_SALES_LATEST")]
    LipperPrice2SalesLatest,
    #[serde(rename = "LIPPER_ROE_WGT_AVG_LATEST")]
    LipperRoeWgtAvgLatest,
    #[serde(rename = "LIPPER_SPS_GRWTH_3YR_LATEST")]
    LipperSpsGrwth3YrLatest,
    #[serde(rename = "LIPPER_LT_DEBT_2_SE")]
    LipperLtDebt2Se,
    #[serde(rename = "LIPPER_EPS_GRWTH_1YR")]
    LipperEpsGrwth1Yr,
    #[serde(rename = "LIPPER_EPS_GRWTH_3YR")]
    LipperEpsGrwth3Yr,
    #[serde(rename = "LIPPER_EPS_GRWTH_5YR")]
    LipperEpsGrwth5Yr,
    #[serde(rename = "LIPPER_PRICE_2_EARNINGS")]
    LipperPrice2Earnings,
    #[serde(rename = "LIPPER_PRICE_2_BOOK")]
    LipperPrice2Book,
    #[serde(rename = "LIPPER_PRICE_2_SALES")]
    LipperPrice2Sales,
    #[serde(rename = "LIPPER_ROA_1YR")]
    LipperRoa1Yr,
    #[serde(rename = "LIPPER_ROA_3YR")]
    LipperRoa3Yr,
    #[serde(rename = "LIPPER_ROE_1YR")]
    LipperRoe1Yr,
    #[serde(rename = "LIPPER_ROE_3YR")]
    LipperRoe3Yr,
    #[serde(rename = "LIPPER_ROI_1YR")]
    LipperRoi1Yr,
    #[serde(rename = "LIPPER_ROI_3YR")]
    LipperRoi3Yr,
    #[serde(rename = "LIPPER_SALES_2_TOTAL_ASSETS")]
    LipperSales2TotalAssets,
    #[serde(rename = "LIPPER_SPS_GRWTH_1YR")]
    LipperSpsGrwth1Yr,
    #[serde(rename = "LIPPER_SPS_GRWTH_3YR")]
    LipperSpsGrwth3Yr,
    #[serde(rename = "LIPPER_TOT_ASSETS_2_TOT_EQ")]
    LipperTotAssets2TotEq,
    #[serde(rename = "LIPPER_TOT_DEBT_2_TOT_CAP")]
    LipperTotDebt2TotCap,
    #[serde(rename = "LIPPER_TOT_DEBT_2_TOT_EQ")]
    LipperTotDebt2TotEq,
    #[serde(rename = "LIPPER_PRICE_2_CASH")]
    LipperPrice2Cash,
    #[serde(rename = "LIPPER_REL_STRENGTH")]
    LipperRelStrength,
    #[serde(rename = "LIPPER_RET_ON_CAPITAL")]
    LipperRetOnCapital,
    #[serde(rename = "LIPPER_RET_ON_CAPITAL_3YR")]
    LipperRetOnCapital3Yr,
    #[serde(rename = "LIPPER_COMP_ZSCORE")]
    LipperCompZscore,
    #[serde(rename = "LIPPER_PE_ZSCORE")]
    LipperPeZscore,
    #[serde(rename = "LIPPER_PB_ZSCORE")]
    LipperPbZscore,
    #[serde(rename = "LIPPER_PS_ZSCORE")]
    LipperPsZscore,
    #[serde(rename = "LIPPER_SPS_GRWTH_ZSCORE")]
    LipperSpsGrwthZscore,
    #[serde(rename = "LIPPER_AVG_FIN_COMP_ZSCORE")]
    LipperAvgFinCompZscore,
    #[serde(rename = "LIPPER_WGT_FIN_COMP_ZSCORE")]
    LipperWgtFinCompZscore,
    #[serde(rename = "LIPPER_YIELD_ZSCORE")]
    LipperYieldZscore,
    #[serde(rename = "LIPPER_ROE_ZSCORE")]
    LipperRoeZscore,
    #[serde(rename = "LIPPER_YIELD_TO_MATURITY")]
    LipperYieldToMaturity,
    #[serde(rename = "LIPPER_NOM_MATURITY")]
    LipperNomMaturity,
    #[serde(rename = "LIPPER_EFF_MATURITY")]
    LipperEffMaturity,
    #[serde(rename = "LIPPER_AVG_COUPON")]
    LipperAvgCoupon,
    #[serde(rename = "LIPPER_CALC_AVG_QUALITY")]
    LipperCalcAvgQuality,
    #[serde(rename = "LIPPER_LEVERAGE_RATIO")]
    LipperLeverageRatio,
    #[serde(rename = "SSCORE")]
    SScore,
    #[serde(rename = "SCHANGE")]
    SChange,
    #[serde(rename = "SVSCORE")]
    SvScore,
    #[serde(rename = "SVCHANGE")]
    SvChange,
}

#[traced_test]
#[tokio::test]
async fn asdasd() {
    //
    let xxx = UsStocks;
}

// MOODY - moodyRatingAbove, moodyRatingBelow
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum MoodyRatingFilters {
    AAA,
    AA1,
    AA2,
    AA3,
    A1,
    A2,
    A3,
    BAA1,
    BAA2,
    BAA3,
    BA1,
    BA2,
    BA3,
    B1,
    B2,
    B3,
    CAA1,
    CAA2,
    CAA3,
    CA,
    C,
    NR,
}

// SP - spRatingAbov, spRatingBelow
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum SnPRatingFilters {
    AAA,

    #[serde(rename = "AA+")]
    AAPlus,
    AA,
    #[serde(rename = "AA-")]
    AAMinus,

    #[serde(rename = "A+")]
    APlus,
    A,
    #[serde(rename = "A-")]
    AMinus,

    #[serde(rename = "BBB+")]
    BBBPlus,
    BBB,
    #[serde(rename = "BBB-")]
    BBBMinus,

    #[serde(rename = "BB+")]
    BBPlus,
    BB,
    #[serde(rename = "BB-")]
    BBMinus,

    #[serde(rename = "B+")]
    BPlus,
    B,
    #[serde(rename = "B-")]
    BMinus,

    #[serde(rename = "CCC+")]
    CCCPlus,
    CCC,
    #[serde(rename = "CCC-")]
    CCCMinus,

    #[serde(rename = "CC+")]
    CCPlus,
    CC,
    #[serde(rename = "CC-")]
    CCMinus,

    #[serde(rename = "C+")]
    CPlus,
    C,
    #[serde(rename = "C-")]
    CMinus,

    D,
    NR,
}

// bondCreditRating -
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum BondCreditRatingFilters {
    #[serde(rename = "highGrade")]
    HighGrade,
    #[serde(rename = "highGrade")]
    HighYield,
    Any,
}

// BONDCREDITRATING - bondCreditRating
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum CurrencyFilters {
    USD,
    CAD,
    GBP,
    EUR,
    CHF,
    BRL,
    HKD,
    Any,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum StockTypesFilters {
    All,
    #[serde(rename = "inc:CORP")]
    IncludeCORP,
    #[serde(rename = "inc:ADR")]
    IncludeADR,
    #[serde(rename = "inc:ETF")]
    IncludeETF,
    #[serde(rename = "inc:ETN")]
    IncludeETN,
    #[serde(rename = "inc:REIT")]
    IncludeREIT,
    #[serde(rename = "inc:CEF")]
    IncludeCEF,
    #[serde(rename = "inc:ETMF")]
    IncludeETMF,
    #[serde(rename = "exc:CORP")]
    ExcludeCORP,
    #[serde(rename = "exc:ADR")]
    ExcludeADR,
    #[serde(rename = "excETF:")]
    ExcludeETF,
    #[serde(rename = "exc:ETN")]
    ExcludeETN,
    #[serde(rename = "exc:REIT")]
    ExcludeREIT,
    #[serde(rename = "exc:CEF")]
    ExcludeCEF,
    #[serde(rename = "exc:ETMF")]
    ExcludeETMF,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum ProductCategoriesFilters {
    All,
    Agriculture,
    #[serde(rename = "Commodity Index")]
    CommodityIndex,
    Dairy,
    Energy,
    Equity,
    #[serde(rename = "Equity Index")]
    EquityIndex,
    #[serde(rename = "Equity Index Volatility")]
    EquityIndexVolatility,
    #[serde(rename = "Fixed Income")]
    FixedIncome,
    #[serde(rename = "Foreign Exchange")]
    ForeignExchange,
    Forest,
    Housing,
    Meat,
    Metal,
    #[serde(rename = "Money Market")]
    MoneyMarket,
    Weather,
}
