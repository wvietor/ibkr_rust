use core::str::FromStr;
use serde::{Deserialize, Serialize};

// === Type definitions ===

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
/// Represents a "routing" exchange where orders and market data requests can be directed.
pub enum Routing {
    #[serde(rename(serialize = "SMART", deserialize = "SMART"))]
    /// IBKR's "SMART" routing destination, which aggregates data from many component exchanges
    /// and intelligently routes orders to minimize overall costs net of rebates.
    Smart,
    /// A physical exchange like NYSE or NASDAQ.
    Primary(Primary),
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// An error type returned when a given exchange code cannot be matched with a valid
/// [`Primary`] or [`Routing`] exchange.
pub struct ParseExchangeError(pub String);

impl std::fmt::Display for ParseExchangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid exchange {}", self.0)
    }
}

impl std::error::Error for ParseExchangeError {
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

// Docs here would be somewhat ridiculous
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
/// Represents all the valid physical trading venues for various contracts.
pub enum Primary {
    #[serde(rename(serialize = "AEB", deserialize = "AEB"))]
    AmsterdamseEffectenbeurs,
    #[serde(rename(serialize = "ALPHA", deserialize = "ALPHA"))]
    AlphaTradingSystems,
    #[serde(rename(serialize = "AMEX", deserialize = "AMEX"))]
    AmericanStockExchange,
    #[serde(rename(serialize = "APEXEN", deserialize = "APEXEN"))]
    ApexEuronext,
    #[serde(rename(serialize = "APEXIT", deserialize = "APEXIT"))]
    ApexItaly,
    #[serde(rename(serialize = "AQEUDE", deserialize = "AQEUDE"))]
    AquisExchangeEuropeGermany,
    #[serde(rename(serialize = "AQEUEN", deserialize = "AQEUEN"))]
    AquisExchangeEuropeEuronext,
    #[serde(rename(serialize = "AQEUES", deserialize = "AQEUES"))]
    AquisExchangeEuropeSpain,
    #[serde(rename(serialize = "AQEUIT", deserialize = "AQEUIT"))]
    AquisExchangeEuropeItaly,
    #[serde(rename(serialize = "AQS", deserialize = "AQS"))]
    Quadriserv,
    #[serde(rename(serialize = "ARCA", deserialize = "ARCA"))]
    Archipelago,
    #[serde(rename(serialize = "ARCAEDGE", deserialize = "ARCAEDGE"))]
    Arcaedge,
    #[serde(rename(serialize = "ASX", deserialize = "ASX"))]
    AustralianStockExchange,
    #[serde(rename(serialize = "ASXCEN", deserialize = "ASXCEN"))]
    AsxCentrePoint,
    #[serde(rename(serialize = "BARCBONDG", deserialize = "BARCBONDG"))]
    BarclaysGovernmentBonds,
    #[serde(rename(serialize = "BATS", deserialize = "BATS"))]
    BatsTradingInc,
    #[serde(rename(serialize = "BELFOX", deserialize = "BELFOX"))]
    BelgianFuturesAmpOptionsExchange,
    #[serde(rename(serialize = "BEX", deserialize = "BEX"))]
    NasdaqOmxBx,
    #[serde(rename(serialize = "BLOOMBERG", deserialize = "BLOOMBERG"))]
    Bloomberg,
    #[serde(rename(serialize = "BM", deserialize = "BM"))]
    BolsaDeMadrid,
    #[serde(rename(serialize = "BOND1G", deserialize = "BOND1G"))]
    Bond1GovernmentBonds,
    #[serde(rename(serialize = "BONDDESK", deserialize = "BONDDESK"))]
    BondDesk,
    #[serde(rename(serialize = "BONDDESKG", deserialize = "BONDDESKG"))]
    BonddeskForUsGovernmentSecurities,
    #[serde(rename(serialize = "BONDDESKM", deserialize = "BONDDESKM"))]
    BondDeskMunicipalBonds,
    #[serde(rename(serialize = "BONDLARGE", deserialize = "BONDLARGE"))]
    GovernmentBondsLargeOrders,
    #[serde(rename(serialize = "BOX", deserialize = "BOX"))]
    BostonOptionExchange,
    #[serde(rename(serialize = "BUX", deserialize = "BUX"))]
    BudapestStockExchange,
    #[serde(rename(serialize = "BVL", deserialize = "BVL"))]
    LisbonStockExchange,
    #[serde(rename(serialize = "BVME", deserialize = "BVME"))]
    BorsaValoriDiMilano,
    #[serde(rename(serialize = "BVME.ETF", deserialize = "BVME.ETF"))]
    BorsaItalianaEtf,
    #[serde(rename(serialize = "BYX", deserialize = "BYX"))]
    BatsYExchange,
    #[serde(rename(serialize = "CBK2FX", deserialize = "CBK2FX"))]
    CommerzbankAgFrankfurtCurrencyDealing2,
    #[serde(rename(serialize = "CBKFX", deserialize = "CBKFX"))]
    CommerzbankAgFrankfurtCurrencyDealing,
    #[serde(rename(serialize = "CBOE", deserialize = "CBOE"))]
    ChicagoBoardOptionsExchange,
    #[serde(rename(serialize = "CBOE.JPN", deserialize = "CBOE.JPN"))]
    CboeJapanLimited,
    #[serde(rename(serialize = "CBOE2", deserialize = "CBOE2"))]
    ChicagoBoardOptionsExchange2,
    #[serde(rename(serialize = "CBOT", deserialize = "CBOT"))]
    ChicagoBoardOfTrade,
    #[serde(rename(serialize = "CDE", deserialize = "CDE"))]
    CanadianDerivativesExchange,
    #[serde(rename(serialize = "CFE", deserialize = "CFE"))]
    CboeFuturesExchange,
    #[serde(rename(serialize = "CFETAS", deserialize = "CFETAS"))]
    ChicagoFuturesExchangeTradingAtSettlement,
    #[serde(rename(serialize = "CHINEXT", deserialize = "CHINEXT"))]
    ChinextSharesOnShenzhenStockExchange,
    #[serde(rename(serialize = "CHIX_CA", deserialize = "CHIX_CA"))]
    ChiXCanadaAtsLimited,
    #[serde(rename(serialize = "CHIXAU", deserialize = "CHIXAU"))]
    ChiXAustralia,
    #[serde(rename(serialize = "CHX", deserialize = "CHX"))]
    ChicagoStockExchange,
    #[serde(rename(serialize = "CITIFX", deserialize = "CITIFX"))]
    CitibankCurrencyDealing,
    #[serde(rename(serialize = "CME", deserialize = "CME"))]
    ChicagoMercantileExchange,
    #[serde(rename(serialize = "COMEX", deserialize = "COMEX"))]
    CommodityExchange,
    #[serde(rename(serialize = "CPH", deserialize = "CPH"))]
    CopenhagenStockExchange,
    #[serde(rename(serialize = "CSBONDG", deserialize = "CSBONDG"))]
    CreditSuisseGovernmentBondsSmallOrders,
    #[serde(rename(serialize = "CSFBALGO", deserialize = "CSFBALGO"))]
    CsfbAlgorithmicEngine,
    #[serde(rename(serialize = "CSFX", deserialize = "CSFX"))]
    CreditSuisseCurrencyDealing,
    #[serde(rename(serialize = "CTDLZERO", deserialize = "CTDLZERO"))]
    CitadelZeroCommission,
    #[serde(rename(serialize = "DRCTEDGE", deserialize = "DRCTEDGE"))]
    DirectEdgeEcnLlc,
    #[serde(rename(serialize = "DXEDE", deserialize = "DXEDE"))]
    CboeGermany,
    #[serde(rename(serialize = "DXEEN", deserialize = "DXEEN"))]
    CboeEuronext,
    #[serde(rename(serialize = "DXEES", deserialize = "DXEES"))]
    CboeSpain,
    #[serde(rename(serialize = "DXEIT", deserialize = "DXEIT"))]
    CboeEuropeBVDxeOrderBookItaly,
    #[serde(rename(serialize = "EBS", deserialize = "EBS"))]
    ElektronischeBoerseSchweiz,
    #[serde(rename(serialize = "EDGEA", deserialize = "EDGEA"))]
    DirectEdgeEcnEdgea,
    #[serde(rename(serialize = "EDGX", deserialize = "EDGX"))]
    BatsTradingEdgx,
    #[serde(rename(serialize = "EMERALD", deserialize = "EMERALD"))]
    MiaxEmeraldExchange,
    #[serde(rename(serialize = "ENDEX", deserialize = "ENDEX"))]
    IceEndexFutures,
    #[serde(rename(serialize = "ENEXT.BE", deserialize = "ENEXT.BE"))]
    EuronextBelgium,
    #[serde(rename(serialize = "EUIBFRSH", deserialize = "EUIBFRSH"))]
    InternalFractionalShareVenueForEuStocksAndEtfs,
    #[serde(rename(serialize = "EUIBSI", deserialize = "EUIBSI"))]
    IbEuropeanSystematicInternaliser,
    #[serde(rename(serialize = "EUREXUK", deserialize = "EUREXUK"))]
    EurexBritishMarketsForLchCrestClearing,
    #[serde(rename(serialize = "FOXRIVER", deserialize = "FOXRIVER"))]
    FoxRiver,
    #[serde(rename(serialize = "FRACSHARE", deserialize = "FRACSHARE"))]
    PartnerFractionalShares,
    #[serde(rename(serialize = "FTA", deserialize = "FTA"))]
    FinancieleTermijnmarktAmsterdam,
    #[serde(rename(serialize = "FINRA", deserialize = "FINRA"))]
    Finra,
    #[serde(rename(serialize = "FUNDSERV", deserialize = "FUNDSERV"))]
    MutualFundHoldingVenue,
    #[serde(rename(serialize = "FWB", deserialize = "FWB"))]
    FrankfurterWertpapierboerse,
    #[serde(rename(serialize = "FXSETTLE", deserialize = "FXSETTLE"))]
    NonStandardSettlementForFx,
    #[serde(rename(serialize = "GEMINI", deserialize = "GEMINI"))]
    IseGemini,
    #[serde(rename(serialize = "GETTEX", deserialize = "GETTEX"))]
    BRseMNchenAg,
    #[serde(rename(serialize = "GETTEX2", deserialize = "GETTEX2"))]
    BRseMNchenAgForCblSettlement,
    #[serde(rename(serialize = "GS2FX", deserialize = "GS2FX"))]
    GoldmanSachsCurrencyDealing2,
    #[serde(rename(serialize = "GSFX", deserialize = "GSFX"))]
    GoldmanSachsCurrencyDealing,
    #[serde(rename(serialize = "HEADLAND", deserialize = "HEADLAND"))]
    HeadlandsTechnologies,
    #[serde(rename(serialize = "HEADLANDM", deserialize = "HEADLANDM"))]
    HeadlandsTechnologiesMunis,
    #[serde(rename(serialize = "HEX", deserialize = "HEX"))]
    HelsinkiStockExchange,
    #[serde(rename(serialize = "HKFE", deserialize = "HKFE"))]
    HongKongFuturesExchange,
    #[serde(rename(serialize = "HSBC2FX", deserialize = "HSBC2FX"))]
    HsbcCurrencyDealing2,
    #[serde(rename(serialize = "HSBCFX", deserialize = "HSBCFX"))]
    HsbcCurrencyDealing,
    #[serde(rename(serialize = "HTD", deserialize = "HTD"))]
    HartfieldTitusAndDonnelly,
    #[serde(rename(serialize = "IBAPCFD", deserialize = "IBAPCFD"))]
    IbCfdDealingAsiaPacific,
    #[serde(rename(serialize = "IBBOND", deserialize = "IBBOND"))]
    InteractiveBrokersBond,
    #[serde(rename(serialize = "IBCMDTY", deserialize = "IBCMDTY"))]
    InteractiveBrokersCommodity,
    #[serde(rename(serialize = "IBDARK", deserialize = "IBDARK"))]
    IbDarkPool,
    #[serde(rename(serialize = "IBEOS", deserialize = "IBEOS"))]
    IbkrOvernightExchange,
    #[serde(rename(serialize = "IBFX", deserialize = "IBFX"))]
    IbCurrencyDealing,
    #[serde(rename(serialize = "IBFXCFD", deserialize = "IBFXCFD"))]
    IbFxCfdDealing,
    #[serde(rename(serialize = "IBIS", deserialize = "IBIS"))]
    IntegriertesBoersenhandelsUndInformationsSystem,
    #[serde(rename(serialize = "IBKRAM", deserialize = "IBKRAM"))]
    InteractiveBrokersAssetManagement,
    #[serde(rename(serialize = "IBKRNOTE", deserialize = "IBKRNOTE"))]
    IbkrNote,
    #[serde(rename(serialize = "IBMETAL", deserialize = "IBMETAL"))]
    InternalizedTradingOfMetals,
    #[serde(rename(serialize = "IBUSCFD", deserialize = "IBUSCFD"))]
    IbCfdDealingUs,
    #[serde(rename(serialize = "IBUSOPT", deserialize = "IBUSOPT"))]
    IbUsOpt,
    #[serde(rename(serialize = "ICECRYPTO", deserialize = "ICECRYPTO"))]
    IceCryptocurrency,
    #[serde(rename(serialize = "ICEUS", deserialize = "ICEUS"))]
    IceFuturesUsInc,
    #[serde(rename(serialize = "IDEAL", deserialize = "IDEAL"))]
    InteractiveBrokersDealingSystem,
    #[serde(rename(serialize = "IDEALPRO", deserialize = "IDEALPRO"))]
    IbForexPro,
    #[serde(rename(serialize = "IDEALFX", deserialize = "IDEALFX"))]
    IdealCurrencyDealing,
    #[serde(rename(serialize = "IDEM", deserialize = "IDEM"))]
    ItalianDerivativesMarketMilano,
    #[serde(rename(serialize = "IEX", deserialize = "IEX"))]
    InvestorsExchange,
    #[serde(rename(serialize = "IPE", deserialize = "IPE"))]
    InternationalPetroleumExchange,
    #[serde(rename(serialize = "IR", deserialize = "IR"))]
    InterestRateRecordingExchange,
    #[serde(rename(serialize = "ISE", deserialize = "ISE"))]
    InternationalSecuritiesExchange,
    #[serde(rename(serialize = "ISLAND", deserialize = "ISLAND"))]
    Island,
    #[serde(rename(serialize = "JANE", deserialize = "JANE"))]
    JaneStreetExecutionServices,
    #[serde(rename(serialize = "JANEZERO", deserialize = "JANEZERO"))]
    JaneStreetZeroCommission,
    #[serde(rename(serialize = "JEFFALGO", deserialize = "JEFFALGO"))]
    JefferiesAlgorithmicEngine,
    #[serde(rename(serialize = "JPMCBOND", deserialize = "JPMCBOND"))]
    JpmcCorporateBonds,
    #[serde(rename(serialize = "JPNNEXT", deserialize = "JPNNEXT"))]
    Japannext,
    #[serde(rename(serialize = "KSE", deserialize = "KSE"))]
    KoreaStockExchange,
    #[serde(rename(serialize = "LTSE", deserialize = "LTSE"))]
    LongTermStockExchange,
    #[serde(rename(serialize = "MATIF", deserialize = "MATIF"))]
    MarcheATermeDInstrumentsFinanciers,
    #[serde(rename(serialize = "MEFFRV", deserialize = "MEFFRV"))]
    MercadoEspanolDeFuturosFinancierosRentaVariableProxy,
    #[serde(rename(serialize = "MEMX", deserialize = "MEMX"))]
    MembersExchange,
    #[serde(rename(serialize = "MERCURY", deserialize = "MERCURY"))]
    IseMercury,
    #[serde(rename(serialize = "MEXDER", deserialize = "MEXDER"))]
    MercadoMexicanoDeDerivados,
    #[serde(rename(serialize = "MEXI", deserialize = "MEXI"))]
    MexicoStockExchange,
    #[serde(rename(serialize = "MIAX", deserialize = "MIAX"))]
    MiamiOptionsExchange,
    #[serde(rename(serialize = "MILLADV", deserialize = "MILLADV"))]
    MillenniumAdvisorsCorporateBonds,
    #[serde(rename(serialize = "MKTAXESS", deserialize = "MKTAXESS"))]
    MarketaxessCorporates,
    #[serde(rename(serialize = "MONEP", deserialize = "MONEP"))]
    MarcheDesOptsNegDeLaBourseDeParis,
    #[serde(rename(serialize = "MSFX", deserialize = "MSFX"))]
    MorganStanleyCurrencyDealing,
    #[serde(rename(serialize = "N.RIGA", deserialize = "N.RIGA"))]
    NasdaqRiga,
    #[serde(rename(serialize = "N.TALLINN", deserialize = "N.TALLINN"))]
    NasdaqTallinn,
    #[serde(rename(serialize = "N.VILNIUS", deserialize = "N.VILNIUS"))]
    AbNasdaqVilnius,
    #[serde(rename(serialize = "NASDAQ", deserialize = "NASDAQ"))]
    NationalAssociationOfSecurityDealers,
    #[serde(rename(serialize = "NASDAQBX", deserialize = "NASDAQBX"))]
    NasdaqOmxBxOptionsExchange,
    #[serde(rename(serialize = "NASDAQOM", deserialize = "NASDAQOM"))]
    NationalAssociationOfSecurityDealersOptionsMarket,
    #[serde(rename(serialize = "NATIXISFX", deserialize = "NATIXISFX"))]
    NatixisCurrencyDealing,
    #[serde(rename(serialize = "NITE", deserialize = "NITE"))]
    KnightTradingOtcbbAndPinkSheets,
    #[serde(rename(serialize = "NITEZERO", deserialize = "NITEZERO"))]
    IbkrRetailZeroCommission,
    #[serde(rename(serialize = "NSE", deserialize = "NSE"))]
    NationalStockExchangeOfIndiaLimited,
    #[serde(rename(serialize = "NYBOT", deserialize = "NYBOT"))]
    NewYorkBoardOfTrade,
    #[serde(rename(serialize = "NYMEX", deserialize = "NYMEX"))]
    NewYorkMercantileExchange,
    #[serde(rename(serialize = "NYSE", deserialize = "NYSE"))]
    NewYorkStockExchange,
    #[serde(rename(serialize = "NYSEFLOOR", deserialize = "NYSEFLOOR"))]
    NyseFloor,
    #[serde(rename(serialize = "NYSELIFFE", deserialize = "NYSELIFFE"))]
    NyseLiffeUs,
    #[serde(rename(serialize = "NYSENAT", deserialize = "NYSENAT"))]
    NyseNational,
    #[serde(rename(serialize = "OMEGA", deserialize = "OMEGA"))]
    OmegaAts,
    #[serde(rename(serialize = "OMS", deserialize = "OMS"))]
    StockholmOptionsMarket,
    #[serde(rename(serialize = "OMXNO", deserialize = "OMXNO"))]
    NorwegianSharesOnOmx,
    #[serde(rename(serialize = "OSE", deserialize = "OSE"))]
    OsloStockExchange,
    #[serde(rename(serialize = "OSE.JPN", deserialize = "OSE.JPN"))]
    OsakaStockExchange,
    #[serde(rename(serialize = "OSL", deserialize = "OSL"))]
    OslCryptoExchange,
    #[serde(rename(serialize = "OTCBB", deserialize = "OTCBB"))]
    OtcBulletinBoard,
    #[serde(rename(serialize = "OTCLNKECN", deserialize = "OTCLNKECN"))]
    OtcLinkEcn,
    #[serde(rename(serialize = "OVERNIGHT", deserialize = "OVERNIGHT"))]
    OvernightTrading,
    #[serde(rename(serialize = "PAXOS", deserialize = "PAXOS"))]
    PaxosCryptoExchange,
    #[serde(rename(serialize = "PEARL", deserialize = "PEARL"))]
    MiaxPearlExchange,
    #[serde(rename(serialize = "PHLX", deserialize = "PHLX"))]
    PhiladelphiaStockExchange,
    #[serde(rename(serialize = "PINK", deserialize = "PINK"))]
    PinkSheets,
    #[serde(rename(serialize = "PRA", deserialize = "PRA"))]
    PraqueStockExchange,
    #[serde(rename(serialize = "PSE", deserialize = "PSE"))]
    PacificStockExchange,
    #[serde(rename(serialize = "PSX", deserialize = "PSX"))]
    NasdaqOmxPsx,
    #[serde(rename(serialize = "PURE", deserialize = "PURE"))]
    PureTrading,
    #[serde(rename(serialize = "RBC2FX", deserialize = "RBC2FX"))]
    RoyalBankOfCanadaCurrencyDealing2,
    #[serde(rename(serialize = "RBCFX", deserialize = "RBCFX"))]
    RoyalBankOfCanadaCurrencyDealing,
    #[serde(rename(serialize = "RBSFX", deserialize = "RBSFX"))]
    RoyalBankOfScotlandCurrencyDealing,
    #[serde(rename(serialize = "RUSSELL", deserialize = "RUSSELL"))]
    ExchangeForRussellIndices,
    #[serde(rename(serialize = "SEHK", deserialize = "SEHK"))]
    StockExchangeOfHongKong,
    #[serde(rename(serialize = "SEHKNTL", deserialize = "SEHKNTL"))]
    StockExchangeHongKongNorthboundTradingLink,
    #[serde(rename(serialize = "SEHKSZSE", deserialize = "SEHKSZSE"))]
    HongKongShenzhenStockExchangeNorthboundTradingLink,
    #[serde(rename(serialize = "SFB", deserialize = "SFB"))]
    StockholmFondbors,
    #[serde(rename(serialize = "SGX", deserialize = "SGX"))]
    SingaporeExchange,
    #[serde(rename(serialize = "SGXCME", deserialize = "SGXCME"))]
    SingaporeExchangeCme,
    #[serde(rename(serialize = "SMFE", deserialize = "SMFE"))]
    TheSmallExchange,
    #[serde(rename(serialize = "SNFE", deserialize = "SNFE"))]
    SydneyFuturesExchange,
    #[serde(rename(serialize = "SUMRIDGE", deserialize = "SUMRIDGE"))]
    SumridgePartners,
    #[serde(rename(serialize = "SUMRIDGEM", deserialize = "SUMRIDGEM"))]
    SumridgePartnersMunicipalBonds,
    #[serde(rename(serialize = "SWB", deserialize = "SWB"))]
    StuttgartWertpapierboerse,
    #[serde(rename(serialize = "TASE", deserialize = "TASE"))]
    TelAvivStockExchange,
    #[serde(rename(serialize = "TGATE", deserialize = "TGATE"))]
    Tradegate,
    #[serde(rename(serialize = "TGHEDE", deserialize = "TGHEDE"))]
    TurquoiseGlobalHoldingsEuropeBVGermany,
    #[serde(rename(serialize = "TGHEEN", deserialize = "TGHEEN"))]
    TurquoiseGlobalHoldingsEuropeBVEuronext,
    #[serde(rename(serialize = "TGHEES", deserialize = "TGHEES"))]
    TurquoiseGlobalHoldingsEuropeBVSpain,
    #[serde(rename(serialize = "TGHEIT", deserialize = "TGHEIT"))]
    TurquoiseGlobalHoldingsBVItaly,
    #[serde(rename(serialize = "THFXCFD", deserialize = "THFXCFD"))]
    ThFxCfdDealing,
    #[serde(rename(serialize = "TPLUS1", deserialize = "TPLUS1"))]
    TPlusOne,
    #[serde(rename(serialize = "TRADEWEB", deserialize = "TRADEWEB"))]
    TradewebCorporate,
    #[serde(rename(serialize = "TRADEWEBG", deserialize = "TRADEWEBG"))]
    TradewebGovernment,
    #[serde(rename(serialize = "TSE", deserialize = "TSE"))]
    TorontoStockExchange,
    #[serde(rename(serialize = "TSEJ", deserialize = "TSEJ"))]
    TokyoStockExchange,
    #[serde(rename(serialize = "UBS2FX", deserialize = "UBS2FX"))]
    UbsCurrencyDealing2,
    #[serde(rename(serialize = "UBSBOND", deserialize = "UBSBOND"))]
    UbsCorporateBond,
    #[serde(rename(serialize = "UBSFX", deserialize = "UBSFX"))]
    UbsCurrencyDealing,
    #[serde(rename(serialize = "VALUBOND", deserialize = "VALUBOND"))]
    KnightValuebondCorporate,
    #[serde(rename(serialize = "VALUBONDG", deserialize = "VALUBONDG"))]
    KnightValuebondGovernment,
    #[serde(rename(serialize = "VALUBONDM", deserialize = "VALUBONDM"))]
    MunicipalBondsOnValuebond,
    #[serde(rename(serialize = "VENTURE", deserialize = "VENTURE"))]
    TsxVentureExchange,
    #[serde(rename(serialize = "VIRTBONDG", deserialize = "VIRTBONDG"))]
    VirtuFinancialGovernmentBonds,
    #[serde(rename(serialize = "VSE", deserialize = "VSE"))]
    ViennaStockExchange,
    #[serde(rename(serialize = "WFFX", deserialize = "WFFX"))]
    WellsFargoForex,
    #[serde(rename(serialize = "WSE", deserialize = "WSE"))]
    WarsawStockExchange,
}

// === Type implementations ===

impl FromStr for Routing {
    type Err = ParseExchangeError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_uppercase().as_str() {
            "SMART" => Self::Smart,
            prim => Self::Primary(prim.parse()?),
        })
    }
}

impl FromStr for Primary {
    type Err = ParseExchangeError;

    #[allow(clippy::too_many_lines)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_uppercase().as_str() {
            "AEB" => Self::AmsterdamseEffectenbeurs,
            "ALPHA" => Self::AlphaTradingSystems,
            "AMEX" => Self::AmericanStockExchange,
            "APEXEN" => Self::ApexEuronext,
            "APEXIT" => Self::ApexItaly,
            "AQEUDE" => Self::AquisExchangeEuropeGermany,
            "AQEUEN" => Self::AquisExchangeEuropeEuronext,
            "AQEUES" => Self::AquisExchangeEuropeSpain,
            "AQEUIT" => Self::AquisExchangeEuropeItaly,
            "AQS" => Self::Quadriserv,
            "ARCA" => Self::Archipelago,
            "ARCAEDGE" => Self::Arcaedge,
            "ASX" => Self::AustralianStockExchange,
            "ASXCEN" => Self::AsxCentrePoint,
            "BARCBONDG" => Self::BarclaysGovernmentBonds,
            "BATS" => Self::BatsTradingInc,
            "BELFOX" => Self::BelgianFuturesAmpOptionsExchange,
            "BEX" => Self::NasdaqOmxBx,
            "BLOOMBERG" => Self::Bloomberg,
            "BM" => Self::BolsaDeMadrid,
            "BOND1G" => Self::Bond1GovernmentBonds,
            "BONDDESK" => Self::BondDesk,
            "BONDDESKG" => Self::BonddeskForUsGovernmentSecurities,
            "BONDDESKM" => Self::BondDeskMunicipalBonds,
            "BONDLARGE" => Self::GovernmentBondsLargeOrders,
            "BOX" => Self::BostonOptionExchange,
            "BUX" => Self::BudapestStockExchange,
            "BVL" => Self::LisbonStockExchange,
            "BVME" => Self::BorsaValoriDiMilano,
            "BVME.ETF" => Self::BorsaItalianaEtf,
            "BYX" => Self::BatsYExchange,
            "CBK2FX" => Self::CommerzbankAgFrankfurtCurrencyDealing2,
            "CBKFX" => Self::CommerzbankAgFrankfurtCurrencyDealing,
            "CBOE" => Self::ChicagoBoardOptionsExchange,
            "CBOE.JPN" => Self::CboeJapanLimited,
            "CBOE2" => Self::ChicagoBoardOptionsExchange2,
            "CBOT" => Self::ChicagoBoardOfTrade,
            "CDE" => Self::CanadianDerivativesExchange,
            "CFE" => Self::CboeFuturesExchange,
            "CFETAS" => Self::ChicagoFuturesExchangeTradingAtSettlement,
            "CHINEXT" => Self::ChinextSharesOnShenzhenStockExchange,
            "CHIX_CA" => Self::ChiXCanadaAtsLimited,
            "CHIXAU" => Self::ChiXAustralia,
            "CHX" => Self::ChicagoStockExchange,
            "CITIFX" => Self::CitibankCurrencyDealing,
            "CME" => Self::ChicagoMercantileExchange,
            "COMEX" => Self::CommodityExchange,
            "CPH" => Self::CopenhagenStockExchange,
            "CSBONDG" => Self::CreditSuisseGovernmentBondsSmallOrders,
            "CSFBALGO" => Self::CsfbAlgorithmicEngine,
            "CSFX" => Self::CreditSuisseCurrencyDealing,
            "CTDLZERO" => Self::CitadelZeroCommission,
            "DRCTEDGE" => Self::DirectEdgeEcnLlc,
            "DXEDE" => Self::CboeGermany,
            "DXEEN" => Self::CboeEuronext,
            "DXEES" => Self::CboeSpain,
            "DXEIT" => Self::CboeEuropeBVDxeOrderBookItaly,
            "EBS" => Self::ElektronischeBoerseSchweiz,
            "EDGEA" => Self::DirectEdgeEcnEdgea,
            "EDGX" => Self::BatsTradingEdgx,
            "EMERALD" => Self::MiaxEmeraldExchange,
            "ENDEX" => Self::IceEndexFutures,
            "ENEXT.BE" => Self::EuronextBelgium,
            "EUIBFRSH" => Self::InternalFractionalShareVenueForEuStocksAndEtfs,
            "EUIBSI" => Self::IbEuropeanSystematicInternaliser,
            "EUREXUK" => Self::EurexBritishMarketsForLchCrestClearing,
            "FOXRIVER" => Self::FoxRiver,
            "FRACSHARE" => Self::PartnerFractionalShares,
            "FTA" => Self::FinancieleTermijnmarktAmsterdam,
            "FINRA" => Self::Finra,
            "FUNDSERV" => Self::MutualFundHoldingVenue,
            "FWB" => Self::FrankfurterWertpapierboerse,
            "FXSETTLE" => Self::NonStandardSettlementForFx,
            "GEMINI" => Self::IseGemini,
            "GETTEX" => Self::BRseMNchenAg,
            "GETTEX2" => Self::BRseMNchenAgForCblSettlement,
            "GS2FX" => Self::GoldmanSachsCurrencyDealing2,
            "GSFX" => Self::GoldmanSachsCurrencyDealing,
            "HEADLAND" => Self::HeadlandsTechnologies,
            "HEADLANDM" => Self::HeadlandsTechnologiesMunis,
            "HEX" => Self::HelsinkiStockExchange,
            "HKFE" => Self::HongKongFuturesExchange,
            "HSBC2FX" => Self::HsbcCurrencyDealing2,
            "HSBCFX" => Self::HsbcCurrencyDealing,
            "HTD" => Self::HartfieldTitusAndDonnelly,
            "IBAPCFD" => Self::IbCfdDealingAsiaPacific,
            "IBBOND" => Self::InteractiveBrokersBond,
            "IBCMDTY" => Self::InteractiveBrokersCommodity,
            "IBDARK" => Self::IbDarkPool,
            "IBEOS" => Self::IbkrOvernightExchange,
            "IBFX" => Self::IbCurrencyDealing,
            "IBFXCFD" => Self::IbFxCfdDealing,
            "IBIS" => Self::IntegriertesBoersenhandelsUndInformationsSystem,
            "IBKRAM" => Self::InteractiveBrokersAssetManagement,
            "IBKRNOTE" => Self::IbkrNote,
            "IBMETAL" => Self::InternalizedTradingOfMetals,
            "IBUSCFD" => Self::IbCfdDealingUs,
            "IBUSOPT" => Self::IbUsOpt,
            "ICECRYPTO" => Self::IceCryptocurrency,
            "ICEUS" => Self::IceFuturesUsInc,
            "IDEAL" => Self::InteractiveBrokersDealingSystem,
            "IDEALPRO" => Self::IbForexPro,
            "IDEALFX" => Self::IdealCurrencyDealing,
            "IDEM" => Self::ItalianDerivativesMarketMilano,
            "IEX" => Self::InvestorsExchange,
            "IPE" => Self::InternationalPetroleumExchange,
            "IR" => Self::InterestRateRecordingExchange,
            "ISE" => Self::InternationalSecuritiesExchange,
            "ISLAND" => Self::Island,
            "JANE" => Self::JaneStreetExecutionServices,
            "JANEZERO" => Self::JaneStreetZeroCommission,
            "JEFFALGO" => Self::JefferiesAlgorithmicEngine,
            "JPMCBOND" => Self::JpmcCorporateBonds,
            "JPNNEXT" => Self::Japannext,
            "KSE" => Self::KoreaStockExchange,
            "LTSE" => Self::LongTermStockExchange,
            "MATIF" => Self::MarcheATermeDInstrumentsFinanciers,
            "MEFFRV" => Self::MercadoEspanolDeFuturosFinancierosRentaVariableProxy,
            "MEMX" => Self::MembersExchange,
            "MERCURY" => Self::IseMercury,
            "MEXDER" => Self::MercadoMexicanoDeDerivados,
            "MEXI" => Self::MexicoStockExchange,
            "MIAX" => Self::MiamiOptionsExchange,
            "MILLADV" => Self::MillenniumAdvisorsCorporateBonds,
            "MKTAXESS" => Self::MarketaxessCorporates,
            "MONEP" => Self::MarcheDesOptsNegDeLaBourseDeParis,
            "MSFX" => Self::MorganStanleyCurrencyDealing,
            "N.RIGA" => Self::NasdaqRiga,
            "N.TALLINN" => Self::NasdaqTallinn,
            "N.VILNIUS" => Self::AbNasdaqVilnius,
            "NASDAQ" => Self::NationalAssociationOfSecurityDealers,
            "NASDAQBX" => Self::NasdaqOmxBxOptionsExchange,
            "NASDAQOM" => Self::NationalAssociationOfSecurityDealersOptionsMarket,
            "NATIXISFX" => Self::NatixisCurrencyDealing,
            "NITE" => Self::KnightTradingOtcbbAndPinkSheets,
            "NITEZERO" => Self::IbkrRetailZeroCommission,
            "NSE" => Self::NationalStockExchangeOfIndiaLimited,
            "NYBOT" => Self::NewYorkBoardOfTrade,
            "NYMEX" => Self::NewYorkMercantileExchange,
            "NYSE" => Self::NewYorkStockExchange,
            "NYSEFLOOR" => Self::NyseFloor,
            "NYSELIFFE" => Self::NyseLiffeUs,
            "NYSENAT" => Self::NyseNational,
            "OMEGA" => Self::OmegaAts,
            "OMS" => Self::StockholmOptionsMarket,
            "OMXNO" => Self::NorwegianSharesOnOmx,
            "OSE" => Self::OsloStockExchange,
            "OSE.JPN" => Self::OsakaStockExchange,
            "OSL" => Self::OslCryptoExchange,
            "OTCBB" => Self::OtcBulletinBoard,
            "OTCLNKECN" => Self::OtcLinkEcn,
            "OVERNIGHT" => Self::OvernightTrading,
            "PAXOS" => Self::PaxosCryptoExchange,
            "PEARL" => Self::MiaxPearlExchange,
            "PHLX" => Self::PhiladelphiaStockExchange,
            "PINK" => Self::PinkSheets,
            "PRA" => Self::PraqueStockExchange,
            "PSE" => Self::PacificStockExchange,
            "PSX" => Self::NasdaqOmxPsx,
            "PURE" => Self::PureTrading,
            "RBC2FX" => Self::RoyalBankOfCanadaCurrencyDealing2,
            "RBCFX" => Self::RoyalBankOfCanadaCurrencyDealing,
            "RBSFX" => Self::RoyalBankOfScotlandCurrencyDealing,
            "RUSSELL" => Self::ExchangeForRussellIndices,
            "SEHK" => Self::StockExchangeOfHongKong,
            "SEHKNTL" => Self::StockExchangeHongKongNorthboundTradingLink,
            "SEHKSZSE" => Self::HongKongShenzhenStockExchangeNorthboundTradingLink,
            "SFB" => Self::StockholmFondbors,
            "SGX" => Self::SingaporeExchange,
            "SGXCME" => Self::SingaporeExchangeCme,
            "SMFE" => Self::TheSmallExchange,
            "SNFE" => Self::SydneyFuturesExchange,
            "SUMRIDGE" => Self::SumridgePartners,
            "SUMRIDGEM" => Self::SumridgePartnersMunicipalBonds,
            "SWB" => Self::StuttgartWertpapierboerse,
            "TASE" => Self::TelAvivStockExchange,
            "TGATE" => Self::Tradegate,
            "TGHEDE" => Self::TurquoiseGlobalHoldingsEuropeBVGermany,
            "TGHEEN" => Self::TurquoiseGlobalHoldingsEuropeBVEuronext,
            "TGHEES" => Self::TurquoiseGlobalHoldingsEuropeBVSpain,
            "TGHEIT" => Self::TurquoiseGlobalHoldingsBVItaly,
            "THFXCFD" => Self::ThFxCfdDealing,
            "TPLUS1" => Self::TPlusOne,
            "TRADEWEB" => Self::TradewebCorporate,
            "TRADEWEBG" => Self::TradewebGovernment,
            "TSE" => Self::TorontoStockExchange,
            "TSEJ" => Self::TokyoStockExchange,
            "UBS2FX" => Self::UbsCurrencyDealing2,
            "UBSBOND" => Self::UbsCorporateBond,
            "UBSFX" => Self::UbsCurrencyDealing,
            "VALUBOND" => Self::KnightValuebondCorporate,
            "VALUBONDG" => Self::KnightValuebondGovernment,
            "VALUBONDM" => Self::MunicipalBondsOnValuebond,
            "VENTURE" => Self::TsxVentureExchange,
            "VIRTBONDG" => Self::VirtuFinancialGovernmentBonds,
            "VSE" => Self::ViennaStockExchange,
            "WFFX" => Self::WellsFargoForex,
            "WSE" => Self::WarsawStockExchange,
            s => return Err(ParseExchangeError(s.to_owned())),
        })
    }
}
