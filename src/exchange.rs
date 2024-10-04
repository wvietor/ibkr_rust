use core::str::FromStr;

use serde::{Deserialize, Serialize};

// === Type definitions ===

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Represents a "routing" exchange where orders and market data requests can be directed.
pub enum Routing {
    #[serde(rename = "SMART")]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Represents all the valid physical trading venues for various contracts.
pub enum Primary {
    #[serde(rename = "AEB")]
    AmsterdamseEffectenbeurs,
    #[serde(rename = "ALPHA")]
    AlphaTradingSystems,
    #[serde(rename = "AMEX")]
    AmericanStockExchange,
    #[serde(rename = "APEXEN")]
    ApexEuronext,
    #[serde(rename = "APEXIT")]
    ApexItaly,
    #[serde(rename = "AQEUDE")]
    AquisExchangeEuropeGermany,
    #[serde(rename = "AQEUEN")]
    AquisExchangeEuropeEuronext,
    #[serde(rename = "AQEUES")]
    AquisExchangeEuropeSpain,
    #[serde(rename = "AQEUIT")]
    AquisExchangeEuropeItaly,
    #[serde(rename = "AQS")]
    Quadriserv,
    #[serde(rename = "ARCA")]
    Archipelago,
    #[serde(rename = "ARCAEDGE")]
    Arcaedge,
    #[serde(rename = "ASX")]
    AustralianStockExchange,
    #[serde(rename = "ASXCEN")]
    AsxCentrePoint,
    #[serde(rename = "BARCBONDG")]
    BarclaysGovernmentBonds,
    #[serde(rename = "BATS")]
    BatsTradingInc,
    #[serde(rename = "BELFOX")]
    BelgianFuturesAmpOptionsExchange,
    #[serde(rename = "BEX")]
    NasdaqOmxBx,
    #[serde(rename = "BLOOMBERG")]
    Bloomberg,
    #[serde(rename = "BM")]
    BolsaDeMadrid,
    #[serde(rename = "BOND1G")]
    Bond1GovernmentBonds,
    #[serde(rename = "BONDDESK")]
    BondDesk,
    #[serde(rename = "BONDDESKG")]
    BonddeskForUsGovernmentSecurities,
    #[serde(rename = "BONDDESKM")]
    BondDeskMunicipalBonds,
    #[serde(rename = "BONDLARGE")]
    GovernmentBondsLargeOrders,
    #[serde(rename = "BOX")]
    BostonOptionExchange,
    #[serde(rename = "BUX")]
    BudapestStockExchange,
    #[serde(rename = "BVL")]
    LisbonStockExchange,
    #[serde(rename = "BVME")]
    BorsaValoriDiMilano,
    #[serde(rename = "BVME.ETF")]
    BorsaItalianaEtf,
    #[serde(rename = "BYX")]
    BatsYExchange,
    #[serde(rename = "CBK2FX")]
    CommerzbankAgFrankfurtCurrencyDealing2,
    #[serde(rename = "CBKFX")]
    CommerzbankAgFrankfurtCurrencyDealing,
    #[serde(rename = "CBOE")]
    ChicagoBoardOptionsExchange,
    #[serde(rename = "CBOE.JPN")]
    CboeJapanLimited,
    #[serde(rename = "CBOE2")]
    ChicagoBoardOptionsExchange2,
    #[serde(rename = "CBOT")]
    ChicagoBoardOfTrade,
    #[serde(rename = "CDE")]
    CanadianDerivativesExchange,
    #[serde(rename = "CFE")]
    CboeFuturesExchange,
    #[serde(rename = "CFETAS")]
    ChicagoFuturesExchangeTradingAtSettlement,
    #[serde(rename = "CHINEXT")]
    ChinextSharesOnShenzhenStockExchange,
    #[serde(rename = "CHIX_CA")]
    ChiXCanadaAtsLimited,
    #[serde(rename = "CHIXAU")]
    ChiXAustralia,
    #[serde(rename = "CHX")]
    ChicagoStockExchange,
    #[serde(rename = "CITIFX")]
    CitibankCurrencyDealing,
    #[serde(rename = "CME")]
    ChicagoMercantileExchange,
    #[serde(rename = "COMEX")]
    CommodityExchange,
    #[serde(rename = "CPH")]
    CopenhagenStockExchange,
    #[serde(rename = "CSBONDG")]
    CreditSuisseGovernmentBondsSmallOrders,
    #[serde(rename = "CSFBALGO")]
    CsfbAlgorithmicEngine,
    #[serde(rename = "CSFX")]
    CreditSuisseCurrencyDealing,
    #[serde(rename = "CTDLZERO")]
    CitadelZeroCommission,
    #[serde(rename = "DRCTEDGE")]
    DirectEdgeEcnLlc,
    #[serde(rename = "DXEDE")]
    CboeGermany,
    #[serde(rename = "DXEEN")]
    CboeEuronext,
    #[serde(rename = "DXEES")]
    CboeSpain,
    #[serde(rename = "DXEIT")]
    CboeEuropeBVDxeOrderBookItaly,
    #[serde(rename = "EBS")]
    ElektronischeBoerseSchweiz,
    #[serde(rename = "EDGEA")]
    DirectEdgeEcnEdgea,
    #[serde(rename = "EDGX")]
    BatsTradingEdgx,
    #[serde(rename = "EMERALD")]
    MiaxEmeraldExchange,
    #[serde(rename = "ENDEX")]
    IceEndexFutures,
    #[serde(rename = "ENEXT.BE")]
    EuronextBelgium,
    #[serde(rename = "EUIBFRSH")]
    InternalFractionalShareVenueForEuStocksAndEtfs,
    #[serde(rename = "EUIBSI")]
    IbEuropeanSystematicInternaliser,
    #[serde(rename = "EUREXUK")]
    EurexBritishMarketsForLchCrestClearing,
    #[serde(rename = "FOXRIVER")]
    FoxRiver,
    #[serde(rename = "FRACSHARE")]
    PartnerFractionalShares,
    #[serde(rename = "FTA")]
    FinancieleTermijnmarktAmsterdam,
    #[serde(rename = "FINRA")]
    Finra,
    #[serde(rename = "FUNDSERV")]
    MutualFundHoldingVenue,
    #[serde(rename = "FWB")]
    FrankfurterWertpapierboerse,
    #[serde(rename = "FXSETTLE")]
    NonStandardSettlementForFx,
    #[serde(rename = "GEMINI")]
    IseGemini,
    #[serde(rename = "GETTEX")]
    BRseMNchenAg,
    #[serde(rename = "GETTEX2")]
    BRseMNchenAgForCblSettlement,
    #[serde(rename = "GS2FX")]
    GoldmanSachsCurrencyDealing2,
    #[serde(rename = "GSFX")]
    GoldmanSachsCurrencyDealing,
    #[serde(rename = "HEADLAND")]
    HeadlandsTechnologies,
    #[serde(rename = "HEADLANDM")]
    HeadlandsTechnologiesMunis,
    #[serde(rename = "HEX")]
    HelsinkiStockExchange,
    #[serde(rename = "HKFE-IND")]
    HangSengIndices,
    #[serde(rename = "HKFE")]
    HongKongFuturesExchange,
    #[serde(rename = "HSBC2FX")]
    HsbcCurrencyDealing2,
    #[serde(rename = "HSBCFX")]
    HsbcCurrencyDealing,
    #[serde(rename = "HTD")]
    HartfieldTitusAndDonnelly,
    #[serde(rename = "IBAPCFD")]
    IbCfdDealingAsiaPacific,
    #[serde(rename = "IBBOND")]
    InteractiveBrokersBond,
    #[serde(rename = "IBCMDTY")]
    InteractiveBrokersCommodity,
    #[serde(rename = "IBDARK")]
    IbDarkPool,
    #[serde(rename = "IBEOS")]
    IbkrOvernightExchange,
    #[serde(rename = "IBFX")]
    IbCurrencyDealing,
    #[serde(rename = "IBFXCFD")]
    IbFxCfdDealing,
    #[serde(rename = "IBIS")]
    IntegriertesBoersenhandelsUndInformationsSystem,
    #[serde(rename = "IBKRAM")]
    InteractiveBrokersAssetManagement,
    #[serde(rename = "IBKRNOTE")]
    IbkrNote,
    #[serde(rename = "IBMETAL")]
    InternalizedTradingOfMetals,
    #[serde(rename = "IBUSCFD")]
    IbCfdDealingUs,
    #[serde(rename = "IBUSOPT")]
    IbUsOpt,
    #[serde(rename = "ICECRYPTO")]
    IceCryptocurrency,
    #[serde(rename = "ICEUS")]
    IceFuturesUsInc,
    #[serde(rename = "IDEAL")]
    InteractiveBrokersDealingSystem,
    #[serde(rename = "IDEALPRO")]
    IbForexPro,
    #[serde(rename = "IDEALFX")]
    IdealCurrencyDealing,
    #[serde(rename = "IDEM")]
    ItalianDerivativesMarketMilano,
    #[serde(rename = "IEX")]
    InvestorsExchange,
    #[serde(rename = "IPE")]
    InternationalPetroleumExchange,
    #[serde(rename = "IR")]
    InterestRateRecordingExchange,
    #[serde(rename = "ISE")]
    InternationalSecuritiesExchange,
    #[serde(rename = "ISLAND")]
    Island,
    #[serde(rename = "JANE")]
    JaneStreetExecutionServices,
    #[serde(rename = "JANEZERO")]
    JaneStreetZeroCommission,
    #[serde(rename = "JEFFALGO")]
    JefferiesAlgorithmicEngine,
    #[serde(rename = "JPMCBOND")]
    JpmcCorporateBonds,
    #[serde(rename = "JPNNEXT")]
    Japannext,
    #[serde(rename = "KSE")]
    KoreaStockExchange,
    #[serde(rename = "LTSE")]
    LongTermStockExchange,
    #[serde(rename = "TWSE")]
    TaiwanStockExchange,
    #[serde(rename = "MATIF")]
    MarcheATermeDInstrumentsFinanciers,
    #[serde(rename = "MEFFRV")]
    MercadoEspanolDeFuturosFinancierosRentaVariableProxy,
    #[serde(rename = "MEMX")]
    MembersExchange,
    #[serde(rename = "MERCURY")]
    IseMercury,
    #[serde(rename = "MEXDER")]
    MercadoMexicanoDeDerivados,
    #[serde(rename = "MEXI")]
    MexicoStockExchange,
    #[serde(rename = "MIAX")]
    MiamiOptionsExchange,
    #[serde(rename = "MILLADV")]
    MillenniumAdvisorsCorporateBonds,
    #[serde(rename = "MKTAXESS")]
    MarketaxessCorporates,
    #[serde(rename = "MONEP")]
    MarcheDesOptsNegDeLaBourseDeParis,
    #[serde(rename = "MSFX")]
    MorganStanleyCurrencyDealing,
    #[serde(rename = "N.RIGA")]
    NasdaqRiga,
    #[serde(rename = "N.TALLINN")]
    NasdaqTallinn,
    #[serde(rename = "N.VILNIUS")]
    AbNasdaqVilnius,
    #[serde(rename = "NASDAQ")]
    NationalAssociationOfSecurityDealers,
    #[serde(rename = "NASDAQBX")]
    NasdaqOmxBxOptionsExchange,
    #[serde(rename = "NASDAQOM")]
    NationalAssociationOfSecurityDealersOptionsMarket,
    #[serde(rename = "NATIXISFX")]
    NatixisCurrencyDealing,
    #[serde(rename = "NITE")]
    KnightTradingOtcbbAndPinkSheets,
    #[serde(rename = "NITEZERO")]
    IbkrRetailZeroCommission,
    #[serde(rename = "NSE")]
    NationalStockExchangeOfIndiaLimited,
    #[serde(rename = "NYBOT")]
    NewYorkBoardOfTrade,
    #[serde(rename = "NYMEX")]
    NewYorkMercantileExchange,
    #[serde(rename = "NYSE")]
    NewYorkStockExchange,
    #[serde(rename = "NYSEFLOOR")]
    NyseFloor,
    #[serde(rename = "NYSELIFFE")]
    NyseLiffeUs,
    #[serde(rename = "NYSENAT")]
    NyseNational,
    #[serde(rename = "OMEGA")]
    OmegaAts,
    #[serde(rename = "OMS")]
    StockholmOptionsMarket,
    #[serde(rename = "OMXNO")]
    NorwegianSharesOnOmx,
    #[serde(rename = "OSE")]
    OsloStockExchange,
    #[serde(rename = "OSE.JPN")]
    OsakaStockExchange,
    #[serde(rename = "OSL")]
    OslCryptoExchange,
    #[serde(rename = "OTCBB")]
    OtcBulletinBoard,
    #[serde(rename = "OTCLNKECN")]
    OtcLinkEcn,
    #[serde(rename = "OVERNIGHT")]
    OvernightTrading,
    #[serde(rename = "PAXOS")]
    PaxosCryptoExchange,
    #[serde(rename = "PEARL")]
    MiaxPearlExchange,
    #[serde(rename = "PHLX")]
    PhiladelphiaStockExchange,
    #[serde(rename = "PINK")]
    PinkSheets,
    #[serde(rename = "PRA")]
    PraqueStockExchange,
    #[serde(rename = "PSE")]
    PacificStockExchange,
    #[serde(rename = "PSX")]
    NasdaqOmxPsx,
    #[serde(rename = "PURE")]
    PureTrading,
    #[serde(rename = "RBC2FX")]
    RoyalBankOfCanadaCurrencyDealing2,
    #[serde(rename = "RBCFX")]
    RoyalBankOfCanadaCurrencyDealing,
    #[serde(rename = "RBSFX")]
    RoyalBankOfScotlandCurrencyDealing,
    #[serde(rename = "RUSSELL")]
    ExchangeForRussellIndices,
    #[serde(rename = "SAPPHIRE")]
    Sapphire,
    #[serde(rename = "SEHK")]
    StockExchangeOfHongKong,
    #[serde(rename = "SEHKNTL")]
    StockExchangeHongKongNorthboundTradingLink,
    #[serde(rename = "SEHKSTAR")]
    ShanghaiStockExchangeSTARMarket,
    #[serde(rename = "SEHKSZSE")]
    HongKongShenzhenStockExchangeNorthboundTradingLink,
    #[serde(rename = "SFB")]
    StockholmFondbors,
    #[serde(rename = "SGX")]
    SingaporeExchange,
    #[serde(rename = "SGXCME")]
    SingaporeExchangeCme,
    #[serde(rename = "SMFE")]
    TheSmallExchange,
    #[serde(rename = "SNFE")]
    SydneyFuturesExchange,
    #[serde(rename = "SUMRIDGE")]
    SumridgePartners,
    #[serde(rename = "SUMRIDGEM")]
    SumridgePartnersMunicipalBonds,
    #[serde(rename = "SWB")]
    StuttgartWertpapierboerse,
    #[serde(rename = "TASE")]
    TelAvivStockExchange,
    #[serde(rename = "TGATE")]
    Tradegate,
    #[serde(rename = "TGHEDE")]
    TurquoiseGlobalHoldingsEuropeBVGermany,
    #[serde(rename = "TGHEEN")]
    TurquoiseGlobalHoldingsEuropeBVEuronext,
    #[serde(rename = "TGHEES")]
    TurquoiseGlobalHoldingsEuropeBVSpain,
    #[serde(rename = "TGHEIT")]
    TurquoiseGlobalHoldingsBVItaly,
    #[serde(rename = "THFXCFD")]
    ThFxCfdDealing,
    #[serde(rename = "TPLUS1")]
    TPlusOne,
    #[serde(rename = "TPLUS0")]
    TplusZero,
    #[serde(rename = "TRADEWEB")]
    TradewebCorporate,
    #[serde(rename = "TRADEWEBG")]
    TradewebGovernment,
    #[serde(rename = "TSE")]
    TorontoStockExchange,
    #[serde(rename = "TSEJ")]
    TokyoStockExchange,
    #[serde(rename = "UBS2FX")]
    UbsCurrencyDealing2,
    #[serde(rename = "UBSBOND")]
    UbsCorporateBond,
    #[serde(rename = "UBSFX")]
    UbsCurrencyDealing,
    #[serde(rename = "VALUBOND")]
    KnightValuebondCorporate,
    #[serde(rename = "VALUBONDG")]
    KnightValuebondGovernment,
    #[serde(rename = "VALUBONDM")]
    MunicipalBondsOnValuebond,
    #[serde(rename = "VENTURE")]
    TsxVentureExchange,
    #[serde(rename = "VIRTBONDG")]
    VirtuFinancialGovernmentBonds,
    #[serde(rename = "VSE")]
    ViennaStockExchange,
    #[serde(rename = "VALUE")]
    /// A holding exchange used for clients to close positions on a contract that is no longer listed
    Value,
    #[serde(rename = "WFFX")]
    WellsFargoForex,
    #[serde(rename = "WSE")]
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
            "HKFE-IND" => Self::HangSengIndices,
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
            "TWSE" => Self::TaiwanStockExchange,
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
            "SAPPHIRE" => Self::Sapphire,
            "SEHK" => Self::StockExchangeOfHongKong,
            "SEHKNTL" => Self::StockExchangeHongKongNorthboundTradingLink,
            "SEHKSTAR" => Self::ShanghaiStockExchangeSTARMarket,
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
            "TPLUS0" => Self::TplusZero,
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
            "VALUE" => Self::Value,
            "VENTURE" => Self::TsxVentureExchange,
            "VIRTBONDG" => Self::VirtuFinancialGovernmentBonds,
            "VSE" => Self::ViennaStockExchange,
            "WFFX" => Self::WellsFargoForex,
            "WSE" => Self::WarsawStockExchange,
            s => return Err(ParseExchangeError(s.to_owned())),
        })
    }
}

impl std::fmt::Display for Primary {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::AmsterdamseEffectenbeurs => "AEB",
            Self::AlphaTradingSystems => "ALPHA",
            Self::AmericanStockExchange => "AMEX",
            Self::ApexEuronext => "APEXEN",
            Self::ApexItaly => "APEXIT",
            Self::AquisExchangeEuropeGermany => "AQEUDE",
            Self::AquisExchangeEuropeEuronext => "AQEUEN",
            Self::AquisExchangeEuropeSpain => "AQEUES",
            Self::AquisExchangeEuropeItaly => "AQEUIT",
            Self::Quadriserv => "AQS",
            Self::Archipelago => "ARCA",
            Self::Arcaedge => "ARCAEDGE",
            Self::AustralianStockExchange => "ASX",
            Self::AsxCentrePoint => "ASXCEN",
            Self::BarclaysGovernmentBonds => "BARCBONDG",
            Self::BatsTradingInc => "BATS",
            Self::BelgianFuturesAmpOptionsExchange => "BELFOX",
            Self::NasdaqOmxBx => "BEX",
            Self::Bloomberg => "BLOOMBERG",
            Self::BolsaDeMadrid => "BM",
            Self::Bond1GovernmentBonds => "BOND1G",
            Self::BondDesk => "BONDDESK",
            Self::BonddeskForUsGovernmentSecurities => "BONDDESKG",
            Self::BondDeskMunicipalBonds => "BONDDESKM",
            Self::GovernmentBondsLargeOrders => "BONDLARGE",
            Self::BostonOptionExchange => "BOX",
            Self::BudapestStockExchange => "BUX",
            Self::LisbonStockExchange => "BVL",
            Self::BorsaValoriDiMilano => "BVME",
            Self::BorsaItalianaEtf => "BVME.ETF",
            Self::BatsYExchange => "BYX",
            Self::CommerzbankAgFrankfurtCurrencyDealing2 => "CBK2FX",
            Self::CommerzbankAgFrankfurtCurrencyDealing => "CBKFX",
            Self::ChicagoBoardOptionsExchange => "CBOE",
            Self::CboeJapanLimited => "CBOE.JPN",
            Self::ChicagoBoardOptionsExchange2 => "CBOE2",
            Self::ChicagoBoardOfTrade => "CBOT",
            Self::CanadianDerivativesExchange => "CDE",
            Self::CboeFuturesExchange => "CFE",
            Self::ChicagoFuturesExchangeTradingAtSettlement => "CFETAS",
            Self::ChinextSharesOnShenzhenStockExchange => "CHINEXT",
            Self::ChiXCanadaAtsLimited => "CHIX_CA",
            Self::ChiXAustralia => "CHIXAU",
            Self::ChicagoStockExchange => "CHX",
            Self::CitibankCurrencyDealing => "CITIFX",
            Self::ChicagoMercantileExchange => "CME",
            Self::CommodityExchange => "COMEX",
            Self::CopenhagenStockExchange => "CPH",
            Self::CreditSuisseGovernmentBondsSmallOrders => "CSBONDG",
            Self::CsfbAlgorithmicEngine => "CSFBALGO",
            Self::CreditSuisseCurrencyDealing => "CSFX",
            Self::CitadelZeroCommission => "CTDLZERO",
            Self::DirectEdgeEcnLlc => "DRCTEDGE",
            Self::CboeGermany => "DXEDE",
            Self::CboeEuronext => "DXEEN",
            Self::CboeSpain => "DXEES",
            Self::CboeEuropeBVDxeOrderBookItaly => "DXEIT",
            Self::ElektronischeBoerseSchweiz => "EBS",
            Self::DirectEdgeEcnEdgea => "EDGEA",
            Self::BatsTradingEdgx => "EDGX",
            Self::MiaxEmeraldExchange => "EMERALD",
            Self::IceEndexFutures => "ENDEX",
            Self::EuronextBelgium => "ENEXT.BE",
            Self::InternalFractionalShareVenueForEuStocksAndEtfs => "EUIBFRSH",
            Self::IbEuropeanSystematicInternaliser => "EUIBSI",
            Self::EurexBritishMarketsForLchCrestClearing => "EUREXUK",
            Self::FoxRiver => "FOXRIVER",
            Self::PartnerFractionalShares => "FRACSHARE",
            Self::FinancieleTermijnmarktAmsterdam => "FTA",
            Self::Finra => "FINRA",
            Self::MutualFundHoldingVenue => "FUNDSERV",
            Self::FrankfurterWertpapierboerse => "FWB",
            Self::NonStandardSettlementForFx => "FXSETTLE",
            Self::IseGemini => "GEMINI",
            Self::BRseMNchenAg => "GETTEX",
            Self::BRseMNchenAgForCblSettlement => "GETTEX2",
            Self::GoldmanSachsCurrencyDealing2 => "GS2FX",
            Self::GoldmanSachsCurrencyDealing => "GSFX",
            Self::HeadlandsTechnologies => "HEADLAND",
            Self::HeadlandsTechnologiesMunis => "HEADLANDM",
            Self::HelsinkiStockExchange => "HEX",
            Self::HangSengIndices => "HKFE-IND",
            Self::HongKongFuturesExchange => "HKFE",
            Self::HsbcCurrencyDealing2 => "HSBC2FX",
            Self::HsbcCurrencyDealing => "HSBCFX",
            Self::HartfieldTitusAndDonnelly => "HTD",
            Self::IbCfdDealingAsiaPacific => "IBAPCFD",
            Self::InteractiveBrokersBond => "IBBOND",
            Self::InteractiveBrokersCommodity => "IBCMDTY",
            Self::IbDarkPool => "IBDARK",
            Self::IbkrOvernightExchange => "IBEOS",
            Self::IbCurrencyDealing => "IBFX",
            Self::IbFxCfdDealing => "IBFXCFD",
            Self::IntegriertesBoersenhandelsUndInformationsSystem => "IBIS",
            Self::InteractiveBrokersAssetManagement => "IBKRAM",
            Self::IbkrNote => "IBKRNOTE",
            Self::InternalizedTradingOfMetals => "IBMETAL",
            Self::IbCfdDealingUs => "IBUSCFD",
            Self::IbUsOpt => "IBUSOPT",
            Self::IceCryptocurrency => "ICECRYPTO",
            Self::IceFuturesUsInc => "ICEUS",
            Self::InteractiveBrokersDealingSystem => "IDEAL",
            Self::IbForexPro => "IDEALPRO",
            Self::IdealCurrencyDealing => "IDEALFX",
            Self::ItalianDerivativesMarketMilano => "IDEM",
            Self::InvestorsExchange => "IEX",
            Self::InternationalPetroleumExchange => "IPE",
            Self::InterestRateRecordingExchange => "IR",
            Self::InternationalSecuritiesExchange => "ISE",
            Self::Island => "ISLAND",
            Self::JaneStreetExecutionServices => "JANE",
            Self::JaneStreetZeroCommission => "JANEZERO",
            Self::JefferiesAlgorithmicEngine => "JEFFALGO",
            Self::JpmcCorporateBonds => "JPMCBOND",
            Self::Japannext => "JPNNEXT",
            Self::KoreaStockExchange => "KSE",
            Self::LongTermStockExchange => "LTSE",
            Self::TaiwanStockExchange => "TWSE",
            Self::MarcheATermeDInstrumentsFinanciers => "MATIF",
            Self::MercadoEspanolDeFuturosFinancierosRentaVariableProxy => "MEFFRV",
            Self::MembersExchange => "MEMX",
            Self::IseMercury => "MERCURY",
            Self::MercadoMexicanoDeDerivados => "MEXDER",
            Self::MexicoStockExchange => "MEXI",
            Self::MiamiOptionsExchange => "MIAX",
            Self::MillenniumAdvisorsCorporateBonds => "MILLADV",
            Self::MarketaxessCorporates => "MKTAXESS",
            Self::MarcheDesOptsNegDeLaBourseDeParis => "MONEP",
            Self::MorganStanleyCurrencyDealing => "MSFX",
            Self::NasdaqRiga => "N.RIGA",
            Self::NasdaqTallinn => "N.TALLINN",
            Self::AbNasdaqVilnius => "N.VILNIUS",
            Self::NationalAssociationOfSecurityDealers => "NASDAQ",
            Self::NasdaqOmxBxOptionsExchange => "NASDAQBX",
            Self::NationalAssociationOfSecurityDealersOptionsMarket => "NASDAQOM",
            Self::NatixisCurrencyDealing => "NATIXISFX",
            Self::KnightTradingOtcbbAndPinkSheets => "NITE",
            Self::IbkrRetailZeroCommission => "NITEZERO",
            Self::NationalStockExchangeOfIndiaLimited => "NSE",
            Self::NewYorkBoardOfTrade => "NYBOT",
            Self::NewYorkMercantileExchange => "NYMEX",
            Self::NewYorkStockExchange => "NYSE",
            Self::NyseFloor => "NYSEFLOOR",
            Self::NyseLiffeUs => "NYSELIFFE",
            Self::NyseNational => "NYSENAT",
            Self::OmegaAts => "OMEGA",
            Self::StockholmOptionsMarket => "OMS",
            Self::NorwegianSharesOnOmx => "OMXNO",
            Self::OsloStockExchange => "OSE",
            Self::OsakaStockExchange => "OSE.JPN",
            Self::OslCryptoExchange => "OSL",
            Self::OtcBulletinBoard => "OTCBB",
            Self::OtcLinkEcn => "OTCLNKECN",
            Self::OvernightTrading => "OVERNIGHT",
            Self::PaxosCryptoExchange => "PAXOS",
            Self::MiaxPearlExchange => "PEARL",
            Self::PhiladelphiaStockExchange => "PHLX",
            Self::PinkSheets => "PINK",
            Self::PraqueStockExchange => "PRA",
            Self::PacificStockExchange => "PSE",
            Self::NasdaqOmxPsx => "PSX",
            Self::PureTrading => "PURE",
            Self::RoyalBankOfCanadaCurrencyDealing2 => "RBC2FX",
            Self::RoyalBankOfCanadaCurrencyDealing => "RBCFX",
            Self::RoyalBankOfScotlandCurrencyDealing => "RBSFX",
            Self::ExchangeForRussellIndices => "RUSSELL",
            Self::Sapphire => "SAPPHIRE",
            Self::StockExchangeOfHongKong => "SEHK",
            Self::StockExchangeHongKongNorthboundTradingLink => "SEHKNTL",
            Self::ShanghaiStockExchangeSTARMarket => "SEHKSTAR",
            Self::HongKongShenzhenStockExchangeNorthboundTradingLink => "SEHKSZSE",
            Self::StockholmFondbors => "SFB",
            Self::SingaporeExchange => "SGX",
            Self::SingaporeExchangeCme => "SGXCME",
            Self::TheSmallExchange => "SMFE",
            Self::SydneyFuturesExchange => "SNFE",
            Self::SumridgePartners => "SUMRIDGE",
            Self::SumridgePartnersMunicipalBonds => "SUMRIDGEM",
            Self::StuttgartWertpapierboerse => "SWB",
            Self::TelAvivStockExchange => "TASE",
            Self::Tradegate => "TGATE",
            Self::TurquoiseGlobalHoldingsEuropeBVGermany => "TGHEDE",
            Self::TurquoiseGlobalHoldingsEuropeBVEuronext => "TGHEEN",
            Self::TurquoiseGlobalHoldingsEuropeBVSpain => "TGHEES",
            Self::TurquoiseGlobalHoldingsBVItaly => "TGHEIT",
            Self::ThFxCfdDealing => "THFXCFD",
            Self::TplusZero => "TPLUS0",
            Self::TPlusOne => "TPLUS1",
            Self::TradewebCorporate => "TRADEWEB",
            Self::TradewebGovernment => "TRADEWEBG",
            Self::TorontoStockExchange => "TSE",
            Self::TokyoStockExchange => "TSEJ",
            Self::UbsCurrencyDealing2 => "UBS2FX",
            Self::UbsCorporateBond => "UBSBOND",
            Self::UbsCurrencyDealing => "UBSFX",
            Self::KnightValuebondCorporate => "VALUBOND",
            Self::KnightValuebondGovernment => "VALUBONDG",
            Self::MunicipalBondsOnValuebond => "VALUBONDM",
            Self::Value => "VALUE",
            Self::TsxVentureExchange => "VENTURE",
            Self::VirtuFinancialGovernmentBonds => "VIRTBONDG",
            Self::ViennaStockExchange => "VSE",
            Self::WellsFargoForex => "WFFX",
            Self::WarsawStockExchange => "WSE",
        };
        write!(f, "{s}")
    }
}

impl std::fmt::Display for Routing {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Routing::Smart => write!(f, "SMART"),
            Routing::Primary(prim) => write!(f, "{prim}"),
        }
    }
}
