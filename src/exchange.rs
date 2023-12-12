use core::str::FromStr;

// === Type definitions ===

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
/// Represents a "routing" exchange where orders and market data requests can be directed.
pub enum Routing {
    #[serde(rename(serialize = "SMART"))]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
/// Represents all the valid physical trading venues for various contracts.
pub enum Primary {
    #[serde(rename(serialize = "AEB"))]
    AmsterdamseEffectenbeurs,
    #[serde(rename(serialize = "ALPHA"))]
    AlphaTradingSystems,
    #[serde(rename(serialize = "AMEX"))]
    AmericanStockExchange,
    #[serde(rename(serialize = "APEXEN"))]
    ApexEuronext,
    #[serde(rename(serialize = "APEXIT"))]
    ApexItaly,
    #[serde(rename(serialize = "AQEUDE"))]
    AquisExchangeEuropeGermany,
    #[serde(rename(serialize = "AQEUEN"))]
    AquisExchangeEuropeEuronext,
    #[serde(rename(serialize = "AQEUES"))]
    AquisExchangeEuropeSpain,
    #[serde(rename(serialize = "AQEUIT"))]
    AquisExchangeEuropeItaly,
    #[serde(rename(serialize = "AQS"))]
    Quadriserv,
    #[serde(rename(serialize = "ARCA"))]
    Archipelago,
    #[serde(rename(serialize = "ARCAEDGE"))]
    Arcaedge,
    #[serde(rename(serialize = "ASX"))]
    AustralianStockExchange,
    #[serde(rename(serialize = "ASXCEN"))]
    AsxCentrePoint,
    #[serde(rename(serialize = "BARCBONDG"))]
    BarclaysGovernmentBonds,
    #[serde(rename(serialize = "BATS"))]
    BatsTradingInc,
    #[serde(rename(serialize = "BELFOX"))]
    BelgianFuturesAmpOptionsExchange,
    #[serde(rename(serialize = "BEX"))]
    NasdaqOmxBx,
    #[serde(rename(serialize = "BLOOMBERG"))]
    Bloomberg,
    #[serde(rename(serialize = "BM"))]
    BolsaDeMadrid,
    #[serde(rename(serialize = "BOND1G"))]
    Bond1GovernmentBonds,
    #[serde(rename(serialize = "BONDDESK"))]
    BondDesk,
    #[serde(rename(serialize = "BONDDESKG"))]
    BonddeskForUsGovernmentSecurities,
    #[serde(rename(serialize = "BONDDESKM"))]
    BondDeskMunicipalBonds,
    #[serde(rename(serialize = "BONDLARGE"))]
    GovernmentBondsLargeOrders,
    #[serde(rename(serialize = "BOX"))]
    BostonOptionExchange,
    #[serde(rename(serialize = "BUX"))]
    BudapestStockExchange,
    #[serde(rename(serialize = "BVL"))]
    LisbonStockExchange,
    #[serde(rename(serialize = "BVME"))]
    BorsaValoriDiMilano,
    #[serde(rename(serialize = "BVME.ETF"))]
    BorsaItalianaEtf,
    #[serde(rename(serialize = "BYX"))]
    BatsYExchange,
    #[serde(rename(serialize = "CBK2FX"))]
    CommerzbankAgFrankfurtCurrencyDealing2,
    #[serde(rename(serialize = "CBKFX"))]
    CommerzbankAgFrankfurtCurrencyDealing,
    #[serde(rename(serialize = "CBOE"))]
    ChicagoBoardOptionsExchange,
    #[serde(rename(serialize = "CBOE.JPN"))]
    CboeJapanLimited,
    #[serde(rename(serialize = "CBOE2"))]
    ChicagoBoardOptionsExchange2,
    #[serde(rename(serialize = "CBOT"))]
    ChicagoBoardOfTrade,
    #[serde(rename(serialize = "CDE"))]
    CanadianDerivativesExchange,
    #[serde(rename(serialize = "CFE"))]
    CboeFuturesExchange,
    #[serde(rename(serialize = "CFETAS"))]
    ChicagoFuturesExchangeTradingAtSettlement,
    #[serde(rename(serialize = "CHINEXT"))]
    ChinextSharesOnShenzhenStockExchange,
    #[serde(rename(serialize = "CHIX_CA"))]
    ChiXCanadaAtsLimited,
    #[serde(rename(serialize = "CHIXAU"))]
    ChiXAustralia,
    #[serde(rename(serialize = "CHX"))]
    ChicagoStockExchange,
    #[serde(rename(serialize = "CITIFX"))]
    CitibankCurrencyDealing,
    #[serde(rename(serialize = "CME"))]
    ChicagoMercantileExchange,
    #[serde(rename(serialize = "COMEX"))]
    CommodityExchange,
    #[serde(rename(serialize = "CPH"))]
    CopenhagenStockExchange,
    #[serde(rename(serialize = "CSBONDG"))]
    CreditSuisseGovernmentBondsSmallOrders,
    #[serde(rename(serialize = "CSFBALGO"))]
    CsfbAlgorithmicEngine,
    #[serde(rename(serialize = "CSFX"))]
    CreditSuisseCurrencyDealing,
    #[serde(rename(serialize = "CTDLZERO"))]
    CitadelZeroCommission,
    #[serde(rename(serialize = "DRCTEDGE"))]
    DirectEdgeEcnLlc,
    #[serde(rename(serialize = "DXEDE"))]
    CboeGermany,
    #[serde(rename(serialize = "DXEEN"))]
    CboeEuronext,
    #[serde(rename(serialize = "DXEES"))]
    CboeSpain,
    #[serde(rename(serialize = "DXEIT"))]
    CboeEuropeBVDxeOrderBookItaly,
    #[serde(rename(serialize = "EBS"))]
    ElektronischeBoerseSchweiz,
    #[serde(rename(serialize = "EDGEA"))]
    DirectEdgeEcnEdgea,
    #[serde(rename(serialize = "EDGX"))]
    BatsTradingEdgx,
    #[serde(rename(serialize = "EMERALD"))]
    MiaxEmeraldExchange,
    #[serde(rename(serialize = "ENDEX"))]
    IceEndexFutures,
    #[serde(rename(serialize = "ENEXT.BE"))]
    EuronextBelgium,
    #[serde(rename(serialize = "EUIBFRSH"))]
    InternalFractionalShareVenueForEuStocksAndEtfs,
    #[serde(rename(serialize = "EUIBSI"))]
    IbEuropeanSystematicInternaliser,
    #[serde(rename(serialize = "EUREXUK"))]
    EurexBritishMarketsForLchCrestClearing,
    #[serde(rename(serialize = "FOXRIVER"))]
    FoxRiver,
    #[serde(rename(serialize = "FRACSHARE"))]
    PartnerFractionalShares,
    #[serde(rename(serialize = "FTA"))]
    FinancieleTermijnmarktAmsterdam,
    #[serde(rename(serialize = "FINRA"))]
    Finra,
    #[serde(rename(serialize = "FUNDSERV"))]
    MutualFundHoldingVenue,
    #[serde(rename(serialize = "FWB"))]
    FrankfurterWertpapierboerse,
    #[serde(rename(serialize = "FXSETTLE"))]
    NonStandardSettlementForFx,
    #[serde(rename(serialize = "GEMINI"))]
    IseGemini,
    #[serde(rename(serialize = "GETTEX"))]
    BRseMNchenAg,
    #[serde(rename(serialize = "GETTEX2"))]
    BRseMNchenAgForCblSettlement,
    #[serde(rename(serialize = "GS2FX"))]
    GoldmanSachsCurrencyDealing2,
    #[serde(rename(serialize = "GSFX"))]
    GoldmanSachsCurrencyDealing,
    #[serde(rename(serialize = "HEADLAND"))]
    HeadlandsTechnologies,
    #[serde(rename(serialize = "HEADLANDM"))]
    HeadlandsTechnologiesMunis,
    #[serde(rename(serialize = "HEX"))]
    HelsinkiStockExchange,
    #[serde(rename(serialize = "HKFE"))]
    HongKongFuturesExchange,
    #[serde(rename(serialize = "HSBC2FX"))]
    HsbcCurrencyDealing2,
    #[serde(rename(serialize = "HSBCFX"))]
    HsbcCurrencyDealing,
    #[serde(rename(serialize = "HTD"))]
    HartfieldTitusAndDonnelly,
    #[serde(rename(serialize = "IBAPCFD"))]
    IbCfdDealingAsiaPacific,
    #[serde(rename(serialize = "IBBOND"))]
    InteractiveBrokersBond,
    #[serde(rename(serialize = "IBCMDTY"))]
    InteractiveBrokersCommodity,
    #[serde(rename(serialize = "IBDARK"))]
    IbDarkPool,
    #[serde(rename(serialize = "IBEOS"))]
    IbkrOvernightExchange,
    #[serde(rename(serialize = "IBFX"))]
    IbCurrencyDealing,
    #[serde(rename(serialize = "IBFXCFD"))]
    IbFxCfdDealing,
    #[serde(rename(serialize = "IBIS"))]
    IntegriertesBoersenhandelsUndInformationsSystem,
    #[serde(rename(serialize = "IBKRAM"))]
    InteractiveBrokersAssetManagement,
    #[serde(rename(serialize = "IBKRNOTE"))]
    IbkrNote,
    #[serde(rename(serialize = "IBMETAL"))]
    InternalizedTradingOfMetals,
    #[serde(rename(serialize = "IBUSCFD"))]
    IbCfdDealingUs,
    #[serde(rename(serialize = "IBUSOPT"))]
    IbUsOpt,
    #[serde(rename(serialize = "ICECRYPTO"))]
    IceCryptocurrency,
    #[serde(rename(serialize = "ICEUS"))]
    IceFuturesUsInc,
    #[serde(rename(serialize = "IDEAL"))]
    InteractiveBrokersDealingSystem,
    #[serde(rename(serialize = "IDEALPRO"))]
    IbForexPro,
    #[serde(rename(serialize = "IDEALFX"))]
    IdealCurrencyDealing,
    #[serde(rename(serialize = "IDEM"))]
    ItalianDerivativesMarketMilano,
    #[serde(rename(serialize = "IEX"))]
    InvestorsExchange,
    #[serde(rename(serialize = "IPE"))]
    InternationalPetroleumExchange,
    #[serde(rename(serialize = "IR"))]
    InterestRateRecordingExchange,
    #[serde(rename(serialize = "ISE"))]
    InternationalSecuritiesExchange,
    #[serde(rename(serialize = "ISLAND"))]
    Island,
    #[serde(rename(serialize = "JANE"))]
    JaneStreetExecutionServices,
    #[serde(rename(serialize = "JANEZERO"))]
    JaneStreetZeroCommission,
    #[serde(rename(serialize = "JEFFALGO"))]
    JefferiesAlgorithmicEngine,
    #[serde(rename(serialize = "JPMCBOND"))]
    JpmcCorporateBonds,
    #[serde(rename(serialize = "JPNNEXT"))]
    Japannext,
    #[serde(rename(serialize = "KSE"))]
    KoreaStockExchange,
    #[serde(rename(serialize = "LTSE"))]
    LongTermStockExchange,
    #[serde(rename(serialize = "MATIF"))]
    MarcheATermeDInstrumentsFinanciers,
    #[serde(rename(serialize = "MEFFRV"))]
    MercadoEspanolDeFuturosFinancierosRentaVariableProxy,
    #[serde(rename(serialize = "MEMX"))]
    MembersExchange,
    #[serde(rename(serialize = "MERCURY"))]
    IseMercury,
    #[serde(rename(serialize = "MEXDER"))]
    MercadoMexicanoDeDerivados,
    #[serde(rename(serialize = "MEXI"))]
    MexicoStockExchange,
    #[serde(rename(serialize = "MIAX"))]
    MiamiOptionsExchange,
    #[serde(rename(serialize = "MILLADV"))]
    MillenniumAdvisorsCorporateBonds,
    #[serde(rename(serialize = "MKTAXESS"))]
    MarketaxessCorporates,
    #[serde(rename(serialize = "MONEP"))]
    MarcheDesOptsNegDeLaBourseDeParis,
    #[serde(rename(serialize = "MSFX"))]
    MorganStanleyCurrencyDealing,
    #[serde(rename(serialize = "N.RIGA"))]
    NasdaqRiga,
    #[serde(rename(serialize = "N.TALLINN"))]
    NasdaqTallinn,
    #[serde(rename(serialize = "N.VILNIUS"))]
    AbNasdaqVilnius,
    #[serde(rename(serialize = "NASDAQ"))]
    NationalAssociationOfSecurityDealers,
    #[serde(rename(serialize = "NASDAQBX"))]
    NasdaqOmxBxOptionsExchange,
    #[serde(rename(serialize = "NASDAQOM"))]
    NationalAssociationOfSecurityDealersOptionsMarket,
    #[serde(rename(serialize = "NATIXISFX"))]
    NatixisCurrencyDealing,
    #[serde(rename(serialize = "NITE"))]
    KnightTradingOtcbbAndPinkSheets,
    #[serde(rename(serialize = "NITEZERO"))]
    IbkrRetailZeroCommission,
    #[serde(rename(serialize = "NSE"))]
    NationalStockExchangeOfIndiaLimited,
    #[serde(rename(serialize = "NYBOT"))]
    NewYorkBoardOfTrade,
    #[serde(rename(serialize = "NYMEX"))]
    NewYorkMercantileExchange,
    #[serde(rename(serialize = "NYSE"))]
    NewYorkStockExchange,
    #[serde(rename(serialize = "NYSEFLOOR"))]
    NyseFloor,
    #[serde(rename(serialize = "NYSELIFFE"))]
    NyseLiffeUs,
    #[serde(rename(serialize = "NYSENAT"))]
    NyseNational,
    #[serde(rename(serialize = "OMEGA"))]
    OmegaAts,
    #[serde(rename(serialize = "OMS"))]
    StockholmOptionsMarket,
    #[serde(rename(serialize = "OMXNO"))]
    NorwegianSharesOnOmx,
    #[serde(rename(serialize = "OSE"))]
    OsloStockExchange,
    #[serde(rename(serialize = "OSE.JPN"))]
    OsakaStockExchange,
    #[serde(rename(serialize = "OSL"))]
    OslCryptoExchange,
    #[serde(rename(serialize = "OTCBB"))]
    OtcBulletinBoard,
    #[serde(rename(serialize = "OTCLNKECN"))]
    OtcLinkEcn,
    #[serde(rename(serialize = "OVERNIGHT"))]
    OvernightTrading,
    #[serde(rename(serialize = "PAXOS"))]
    PaxosCryptoExchange,
    #[serde(rename(serialize = "PEARL"))]
    MiaxPearlExchange,
    #[serde(rename(serialize = "PHLX"))]
    PhiladelphiaStockExchange,
    #[serde(rename(serialize = "PINK"))]
    PinkSheets,
    #[serde(rename(serialize = "PRA"))]
    PraqueStockExchange,
    #[serde(rename(serialize = "PSE"))]
    PacificStockExchange,
    #[serde(rename(serialize = "PSX"))]
    NasdaqOmxPsx,
    #[serde(rename(serialize = "PURE"))]
    PureTrading,
    #[serde(rename(serialize = "RBC2FX"))]
    RoyalBankOfCanadaCurrencyDealing2,
    #[serde(rename(serialize = "RBCFX"))]
    RoyalBankOfCanadaCurrencyDealing,
    #[serde(rename(serialize = "RBSFX"))]
    RoyalBankOfScotlandCurrencyDealing,
    #[serde(rename(serialize = "RUSSELL"))]
    ExchangeForRussellIndices,
    #[serde(rename(serialize = "SEHK"))]
    StockExchangeOfHongKong,
    #[serde(rename(serialize = "SEHKNTL"))]
    StockExchangeHongKongNorthboundTradingLink,
    #[serde(rename(serialize = "SEHKSZSE"))]
    HongKongShenzhenStockExchangeNorthboundTradingLink,
    #[serde(rename(serialize = "SFB"))]
    StockholmFondbors,
    #[serde(rename(serialize = "SGX"))]
    SingaporeExchange,
    #[serde(rename(serialize = "SGXCME"))]
    SingaporeExchangeCme,
    #[serde(rename(serialize = "SMFE"))]
    TheSmallExchange,
    #[serde(rename(serialize = "SNFE"))]
    SydneyFuturesExchange,
    #[serde(rename(serialize = "SUMRIDGE"))]
    SumridgePartners,
    #[serde(rename(serialize = "SUMRIDGEM"))]
    SumridgePartnersMunicipalBonds,
    #[serde(rename(serialize = "SWB"))]
    StuttgartWertpapierboerse,
    #[serde(rename(serialize = "TASE"))]
    TelAvivStockExchange,
    #[serde(rename(serialize = "TGATE"))]
    Tradegate,
    #[serde(rename(serialize = "TGHEDE"))]
    TurquoiseGlobalHoldingsEuropeBVGermany,
    #[serde(rename(serialize = "TGHEEN"))]
    TurquoiseGlobalHoldingsEuropeBVEuronext,
    #[serde(rename(serialize = "TGHEES"))]
    TurquoiseGlobalHoldingsEuropeBVSpain,
    #[serde(rename(serialize = "TGHEIT"))]
    TurquoiseGlobalHoldingsBVItaly,
    #[serde(rename(serialize = "THFXCFD"))]
    ThFxCfdDealing,
    #[serde(rename(serialize = "TPLUS1"))]
    TPlusOne,
    #[serde(rename(serialize = "TRADEWEB"))]
    TradewebCorporate,
    #[serde(rename(serialize = "TRADEWEBG"))]
    TradewebGovernment,
    #[serde(rename(serialize = "TSE"))]
    TorontoStockExchange,
    #[serde(rename(serialize = "TSEJ"))]
    TokyoStockExchange,
    #[serde(rename(serialize = "UBS2FX"))]
    UbsCurrencyDealing2,
    #[serde(rename(serialize = "UBSBOND"))]
    UbsCorporateBond,
    #[serde(rename(serialize = "UBSFX"))]
    UbsCurrencyDealing,
    #[serde(rename(serialize = "VALUBOND"))]
    KnightValuebondCorporate,
    #[serde(rename(serialize = "VALUBONDG"))]
    KnightValuebondGovernment,
    #[serde(rename(serialize = "VALUBONDM"))]
    MunicipalBondsOnValuebond,
    #[serde(rename(serialize = "VENTURE"))]
    TsxVentureExchange,
    #[serde(rename(serialize = "VIRTBONDG"))]
    VirtuFinancialGovernmentBonds,
    #[serde(rename(serialize = "VSE"))]
    ViennaStockExchange,
    #[serde(rename(serialize = "WFFX"))]
    WellsFargoForex,
    #[serde(rename(serialize = "WSE"))]
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
