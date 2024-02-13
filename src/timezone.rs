use std::fmt::Formatter;
use chrono::{LocalResult, NaiveDate, NaiveDateTime, TimeZone, FixedOffset, Offset};

#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
/// All the possible timezones available in the Tws / IB Gateway software
pub enum IbTimeZone {
    AfricaAbidjan,
    AfricaAccra,
    AfricaAddisAbaba,
    AfricaAlgiers,
    AfricaAsmara,
    AfricaAsmera,
    AfricaBamako,
    AfricaBangui,
    AfricaBanjul,
    AfricaBissau,
    AfricaBlantyre,
    AfricaBrazzaville,
    AfricaBujumbura,
    AfricaCairo,
    AfricaCasablanca,
    AfricaCeuta,
    AfricaConakry,
    AfricaDakar,
    AfricaDarEsSalaam,
    AfricaDjibouti,
    AfricaDouala,
    AfricaElAaiun,
    AfricaFreetown,
    AfricaGaborone,
    AfricaHarare,
    AfricaJohannesburg,
    AfricaJuba,
    AfricaKampala,
    AfricaKhartoum,
    AfricaKigali,
    AfricaKinshasa,
    AfricaLagos,
    AfricaLibreville,
    AfricaLome,
    AfricaLuanda,
    AfricaLubumbashi,
    AfricaLusaka,
    AfricaMalabo,
    AfricaMaputo,
    AfricaMaseru,
    AfricaMbabane,
    AfricaMogadishu,
    AfricaMonrovia,
    AfricaNairobi,
    AfricaNdjamena,
    AfricaNiamey,
    AfricaNouakchott,
    AfricaOuagadougou,
    AfricaPortoNovo,
    AfricaSaoTome,
    AfricaTimbuktu,
    AfricaTripoli,
    AfricaTunis,
    AfricaWindhoek,
    AmericaAdak,
    AmericaAnchorage,
    AmericaAnguilla,
    AmericaAntigua,
    AmericaAraguaina,
    AmericaArgentinaBuenosAires,
    AmericaArgentinaCatamarca,
    AmericaArgentinaCordoba,
    AmericaArgentinaJujuy,
    AmericaArgentinaLaRioja,
    AmericaArgentinaMendoza,
    AmericaArgentinaRioGallegos,
    AmericaArgentinaSalta,
    AmericaArgentinaSanJuan,
    AmericaArgentinaSanLuis,
    AmericaArgentinaTucuman,
    AmericaArgentinaUshuaia,
    AmericaAruba,
    AmericaAsuncion,
    AmericaAtikokan,
    AmericaAtka,
    AmericaBahia,
    AmericaBahiaBanderas,
    AmericaBarbados,
    AmericaBelem,
    AmericaBelize,
    AmericaBlancSablon,
    AmericaBoaVista,
    AmericaBogota,
    AmericaBoise,
    AmericaBuenosAires,
    AmericaCambridgeBay,
    AmericaCampoGrande,
    AmericaCancun,
    AmericaCaracas,
    AmericaCayenne,
    AmericaCayman,
    AmericaChicago,
    AmericaChihuahua,
    AmericaCoralHarbour,
    AmericaCordoba,
    AmericaCostaRica,
    AmericaCreston,
    AmericaCuiaba,
    AmericaCuracao,
    AmericaDanmarkshavn,
    AmericaDawson,
    AmericaDawsonCreek,
    AmericaDenver,
    AmericaDetroit,
    AmericaDominica,
    AmericaEdmonton,
    AmericaEirunepe,
    AmericaElSalvador,
    AmericaEnsenada,
    AmericaFortNelson,
    AmericaFortWayne,
    AmericaFortaleza,
    AmericaGlaceBay,
    AmericaGodthab,
    AmericaGooseBay,
    AmericaGrandTurk,
    AmericaGrenada,
    AmericaGuadeloupe,
    AmericaGuatemala,
    AmericaGuayaquil,
    AmericaGuyana,
    AmericaHalifax,
    AmericaHermosillo,
    AmericaIndianaIndianapolis,
    AmericaIndianaMarengo,
    AmericaIndianaPetersburg,
    AmericaIndianaTellCity,
    AmericaIndianaVevay,
    AmericaIndianaVincennes,
    AmericaIndianaWinamac,
    AmericaIndianapolis,
    AmericaInuvik,
    AmericaIqaluit,
    AmericaJamaica,
    AmericaJuneau,
    AmericaKentuckyLouisville,
    AmericaKentuckyMonticello,
    AmericaKralendijk,
    AmericaLaPaz,
    AmericaLima,
    AmericaLosAngeles,
    AmericaLouisville,
    AmericaLowerPrinces,
    AmericaMaceio,
    AmericaManagua,
    AmericaManaus,
    AmericaMarigot,
    AmericaMartinique,
    AmericaMatamoros,
    AmericaMazatlan,
    AmericaMenominee,
    AmericaMerida,
    AmericaMetlakatla,
    AmericaMexicoCity,
    AmericaMiquelon,
    AmericaMoncton,
    AmericaMonterrey,
    AmericaMontevideo,
    AmericaMontreal,
    AmericaMontserrat,
    AmericaNassau,
    AmericaNewYork,
    AmericaNipigon,
    AmericaNome,
    AmericaNoronha,
    AmericaNorthDakotaBeulah,
    AmericaNorthDakotaCenter,
    AmericaNorthDakotaNewSalem,
    AmericaNuuk,
    AmericaOjinaga,
    AmericaPanama,
    AmericaPangnirtung,
    AmericaParamaribo,
    AmericaPhoenix,
    AmericaPortAuPrince,
    AmericaPortOfSpain,
    AmericaPortoAcre,
    AmericaPortoVelho,
    AmericaPuertoRico,
    AmericaPuntaArenas,
    AmericaRainyRiver,
    AmericaRankinInlet,
    AmericaRecife,
    AmericaRegina,
    AmericaResolute,
    AmericaRioBranco,
    AmericaRosario,
    AmericaSantaIsabel,
    AmericaSantarem,
    AmericaSantiago,
    AmericaSantoDomingo,
    AmericaSaoPaulo,
    AmericaScoresbysund,
    AmericaShiprock,
    AmericaSitka,
    AmericaStBarthelemy,
    AmericaStJohns,
    AmericaStKitts,
    AmericaStLucia,
    AmericaStThomas,
    AmericaStVincent,
    AmericaSwiftCurrent,
    AmericaTegucigalpa,
    AmericaThule,
    AmericaThunderBay,
    AmericaTijuana,
    AmericaToronto,
    AmericaTortola,
    AmericaVancouver,
    AmericaVirgin,
    AmericaWhitehorse,
    AmericaWinnipeg,
    AmericaYakutat,
    AmericaYellowknife,
    AntarcticaCasey,
    AntarcticaDavis,
    AntarcticaDumontdurville,
    AntarcticaMacquarie,
    AntarcticaMawson,
    AntarcticaMcmurdo,
    AntarcticaPalmer,
    AntarcticaRothera,
    AntarcticaSyowa,
    AntarcticaVostok,
    ArcticLongyearbyen,
    AsiaAden,
    AsiaAlmaty,
    AsiaAmman,
    AsiaAnadyr,
    AsiaAqtau,
    AsiaAqtobe,
    AsiaAshgabat,
    AsiaAshkhabad,
    AsiaAtyrau,
    AsiaBaghdad,
    AsiaBahrain,
    AsiaBaku,
    AsiaBangkok,
    AsiaBarnaul,
    AsiaBeirut,
    AsiaBishkek,
    AsiaBrunei,
    AsiaCalcutta,
    AsiaChita,
    AsiaChoibalsan,
    AsiaChongqing,
    AsiaChungking,
    AsiaColombo,
    AsiaDacca,
    AsiaDamascus,
    AsiaDhaka,
    AsiaDili,
    AsiaDubai,
    AsiaDushanbe,
    AsiaFamagusta,
    AsiaGaza,
    AsiaHarbin,
    AsiaHebron,
    AsiaHoChiMinh,
    AsiaHongKong,
    AsiaHovd,
    AsiaIrkutsk,
    AsiaIstanbul,
    AsiaJakarta,
    AsiaJayapura,
    AsiaJerusalem,
    AsiaKabul,
    AsiaKamchatka,
    AsiaKarachi,
    AsiaKashgar,
    AsiaKathmandu,
    AsiaKhandyga,
    AsiaKolkata,
    AsiaKrasnoyarsk,
    AsiaKualaLumpur,
    AsiaKuching,
    AsiaKuwait,
    AsiaMacao,
    AsiaMacau,
    AsiaMagadan,
    AsiaMakassar,
    AsiaManila,
    AsiaMuscat,
    AsiaNicosia,
    AsiaNovokuznetsk,
    AsiaNovosibirsk,
    AsiaOmsk,
    AsiaOral,
    AsiaPhnomPenh,
    AsiaPontianak,
    AsiaPyongyang,
    AsiaQatar,
    AsiaQostanay,
    AsiaQyzylorda,
    AsiaRangoon,
    AsiaRiyadh,
    AsiaSaigon,
    AsiaSakhalin,
    AsiaSamarkand,
    AsiaSeoul,
    AsiaShanghai,
    AsiaSingapore,
    AsiaSrednekolymsk,
    AsiaTaipei,
    AsiaTashkent,
    AsiaTbilisi,
    AsiaTehran,
    AsiaTelAviv,
    AsiaThimbu,
    AsiaThimphu,
    AsiaTokyo,
    AsiaTomsk,
    AsiaUjungPandang,
    AsiaUlaanbaatar,
    AsiaUrumqi,
    AsiaUstNera,
    AsiaVientiane,
    AsiaVladivostok,
    AsiaYangon,
    AsiaYekaterinburg,
    AsiaYerevan,
    AtlanticAzores,
    AtlanticBermuda,
    AtlanticCanary,
    AtlanticCapeVerde,
    AtlanticFaeroe,
    AtlanticFaroe,
    AtlanticJanMayen,
    AtlanticMadeira,
    AtlanticReykjavik,
    AtlanticSouthGeorgia,
    AtlanticStHelena,
    AtlanticStanley,
    AustraliaAct,
    AustraliaAdelaide,
    AustraliaBrisbane,
    AustraliaBrokenHill,
    AustraliaCanberra,
    AustraliaCurrie,
    AustraliaDarwin,
    AustraliaEucla,
    AustraliaHobart,
    AustraliaLhi,
    AustraliaLindeman,
    AustraliaLordHowe,
    AustraliaMelbourne,
    AustraliaNsw,
    AustraliaNorth,
    AustraliaPerth,
    AustraliaQueensland,
    AustraliaSouth,
    AustraliaSydney,
    AustraliaTasmania,
    AustraliaVictoria,
    AustraliaWest,
    BrazilAcre,
    BrazilDenoronha,
    BrazilEast,
    BrazilWest,
    Cet,
    Cst6Cdt,
    CanadaAtlantic,
    CanadaCentral,
    CanadaEastern,
    CanadaMountain,
    CanadaPacific,
    CanadaSaskatchewan,
    CanadaYukon,
    ChileContinental,
    ChileEasterlsland,
    Eet,
    Est5Edt,
    Egypt,
    Eire,
    EuropeAmsterdam,
    EuropeAndorra,
    EuropeAstrakhan,
    EuropeAthens,
    EuropeBelfast,
    EuropeBelgrade,
    EuropeBerlin,
    EuropeBratislava,
    EuropeBrussels,
    EuropeBucharest,
    EuropeBudapest,
    EuropeBusingen,
    EuropeChisinau,
    EuropeCopenhagen,
    EuropeDublin,
    EuropeGibraltar,
    EuropeGuernsey,
    EuropeHelsinki,
    EuropeIsleOfMan,
    EuropeIstanbul,
    EuropeJersey,
    EuropeKaliningrad,
    EuropeKiev,
    EuropeKirov,
    EuropeKyiv,
    EuropeLisbon,
    EuropeLjubljana,
    EuropeLondon,
    EuropeLuxembourg,
    EuropeMadrid,
    EuropeMalta,
    EuropeMariehamn,
    EuropeMinsk,
    EuropeMonaco,
    EuropeMoscow,
    EuropeNicosia,
    EuropeOslo,
    EuropeParis,
    EuropePodgorica,
    EuropePrague,
    EuropeRiga,
    EuropeRome,
    EuropeSamara,
    EuropeSanMarino,
    EuropeSarajevo,
    EuropeSaratov,
    EuropeSimferopol,
    EuropeSkopje,
    EuropeSofia,
    EuropeStockholm,
    EuropeTallinn,
    EuropeTirane,
    EuropeUlyanovsk,
    EuropeUzhgorod,
    EuropeVaduz,
    EuropeVatican,
    EuropeVienna,
    EuropeVilnius,
    EuropeVolgograd,
    EuropeWarsaw,
    EuropeZagreb,
    EuropeZaporozhye,
    EuropeZurich,
    Gb,
    GbEire,
    Greenwich,
    Hongkong,
    Iceland,
    IndianAntananarivo,
    IndianChagos,
    IndianChristmas,
    IndianCocos,
    IndianComoro,
    IndianKerguelen,
    IndianMahe,
    IndianMaldives,
    IndianMauritius,
    IndianMayotte,
    IndianReunion,
    Israel,
    Jamaica,
    Japan,
    Kwajalein,
    Libya,
    Met,
    Mst7Mdt,
    MexicoBajanorte,
    MexicoGeneral,
    Nz,
    NzChat,
    Navajo,
    Prc,
    Pst8Pdt,
    PacificApia,
    PacificAuckland,
    PacificBougainville,
    PacificChatham,
    PacificChuuk,
    PacificEaster,
    PacificEfate,
    PacificEnderbury,
    PacificFiji,
    PacificFunafuti,
    PacificGalapagos,
    PacificGambier,
    PacificGuadalcanal,
    PacificGuam,
    PacificHonolulu,
    PacificJohnston,
    PacificKanton,
    PacificKiritimati,
    PacificKosrae,
    PacificKwajalein,
    PacificMajuro,
    PacificMarquesas,
    PacificMidway,
    PacificNauru,
    PacificNiue,
    PacificNorfolk,
    PacificNoumea,
    PacificPagoPago,
    PacificPalau,
    PacificPitcairn,
    PacificPohnpei,
    PacificPortMoresby,
    PacificRarotonga,
    PacificSaipan,
    PacificSamoa,
    PacificTahiti,
    PacificTarawa,
    PacificTongatapu,
    PacificWallis,
    PacificYap,
    Poland,
    Portugal,
    Rok,
    Singapore,
    Turkey,
    Uct,
    UsAlaska,
    UsAleutian,
    UsArizona,
    UsCentral,
    UsEastIndiana,
    UsEastern,
    UsHawail,
    UsMountain,
    UsPacific,
    UsSamoa,
    Universal,
    WSu,
    Wet,
    Zulu,
    Est,
    Hst,
    Mst,
}

impl Offset for IbTimeZone {
    // Don't worry about the `unwrap()`. This function can NEVER panic.
    #[allow(clippy::unwrap_used)]
    /// ```
    /// # use chrono::{Offset, FixedOffset};
    /// # use ibapi::timezone::IbTimeZone;
    /// assert_eq!(IbTimeZone::AfricaAbidjan.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::AfricaAccra.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::AfricaAddisAbaba.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AfricaAlgiers.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::AfricaAsmara.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AfricaAsmera.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AfricaBamako.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::AfricaBangui.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::AfricaBanjul.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::AfricaBissau.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::AfricaBlantyre.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::AfricaBrazzaville.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::AfricaBujumbura.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::AfricaCairo.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::AfricaCasablanca.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::AfricaCeuta.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::AfricaConakry.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::AfricaDakar.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::AfricaDarEsSalaam.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AfricaDjibouti.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AfricaDouala.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::AfricaElAaiun.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::AfricaFreetown.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::AfricaGaborone.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::AfricaHarare.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::AfricaJohannesburg.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::AfricaJuba.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::AfricaKampala.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AfricaKhartoum.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::AfricaKigali.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::AfricaKinshasa.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::AfricaLagos.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::AfricaLibreville.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::AfricaLome.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::AfricaLuanda.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::AfricaLubumbashi.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::AfricaLusaka.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::AfricaMalabo.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::AfricaMaputo.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::AfricaMaseru.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::AfricaMbabane.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::AfricaMogadishu.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AfricaMonrovia.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::AfricaNairobi.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AfricaNdjamena.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::AfricaNiamey.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::AfricaNouakchott.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::AfricaOuagadougou.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::AfricaPortoNovo.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::AfricaSaoTome.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::AfricaTimbuktu.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::AfricaTripoli.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::AfricaTunis.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::AfricaWindhoek.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::AmericaAdak.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaAnchorage.fix(), FixedOffset::east_opt(-32400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaAnguilla.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaAntigua.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaAraguaina.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaArgentinaBuenosAires.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaArgentinaCatamarca.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaArgentinaCordoba.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaArgentinaJujuy.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaArgentinaLaRioja.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaArgentinaMendoza.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaArgentinaRioGallegos.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaArgentinaSalta.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaArgentinaSanJuan.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaArgentinaSanLuis.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaArgentinaTucuman.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaArgentinaUshuaia.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaAruba.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaAsuncion.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaAtikokan.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaAtka.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaBahia.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaBahiaBanderas.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AmericaBarbados.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaBelem.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaBelize.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AmericaBlancSablon.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaBoaVista.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaBogota.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaBoise.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AmericaBuenosAires.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaCambridgeBay.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AmericaCampoGrande.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaCancun.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaCaracas.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaCayenne.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaCayman.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaChicago.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AmericaChihuahua.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AmericaCoralHarbour.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaCordoba.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaCostaRica.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AmericaCreston.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AmericaCuiaba.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaCuracao.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaDanmarkshavn.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::AmericaDawson.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AmericaDawsonCreek.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AmericaDenver.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AmericaDetroit.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaDominica.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaEdmonton.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AmericaEirunepe.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaElSalvador.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AmericaEnsenada.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaFortNelson.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AmericaFortWayne.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaFortaleza.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaGlaceBay.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaGodthab.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaGooseBay.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaGrandTurk.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaGrenada.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaGuadeloupe.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaGuatemala.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AmericaGuayaquil.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaGuyana.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaHalifax.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaHermosillo.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AmericaIndianaIndianapolis.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaIndianaMarengo.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaIndianaPetersburg.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaIndianaTellCity.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AmericaIndianaVevay.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaIndianaVincennes.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaIndianaWinamac.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaIndianapolis.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaInuvik.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AmericaIqaluit.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaJamaica.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaJuneau.fix(), FixedOffset::east_opt(-32400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaKentuckyLouisville.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaKentuckyMonticello.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaKralendijk.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaLaPaz.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaLima.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaLosAngeles.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaLouisville.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaLowerPrinces.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaMaceio.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaManagua.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AmericaManaus.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaMarigot.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaMartinique.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaMatamoros.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AmericaMazatlan.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AmericaMenominee.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AmericaMerida.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AmericaMetlakatla.fix(), FixedOffset::east_opt(-32400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaMexicoCity.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AmericaMiquelon.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaMoncton.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaMonterrey.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AmericaMontevideo.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaMontreal.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaMontserrat.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaNassau.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaNewYork.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaNipigon.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaNome.fix(), FixedOffset::east_opt(-32400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaNoronha.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::AmericaNorthDakotaBeulah.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AmericaNorthDakotaCenter.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AmericaNorthDakotaNewSalem.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AmericaNuuk.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaOjinaga.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AmericaPanama.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaPangnirtung.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaParamaribo.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaPhoenix.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AmericaPortAuPrince.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaPortOfSpain.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaPortoAcre.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaPortoVelho.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaPuertoRico.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaPuntaArenas.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaRainyRiver.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AmericaRankinInlet.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AmericaRecife.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaRegina.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AmericaResolute.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AmericaRioBranco.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaRosario.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaSantaIsabel.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaSantarem.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaSantiago.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaSantoDomingo.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaSaoPaulo.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaScoresbysund.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::AmericaShiprock.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AmericaSitka.fix(), FixedOffset::east_opt(-32400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaStBarthelemy.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaStJohns.fix(), FixedOffset::east_opt(-12600).unwrap());
    /// assert_eq!(IbTimeZone::AmericaStKitts.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaStLucia.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaStThomas.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaStVincent.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaSwiftCurrent.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AmericaTegucigalpa.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AmericaThule.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaThunderBay.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaTijuana.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaToronto.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AmericaTortola.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaVancouver.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::AmericaVirgin.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaWhitehorse.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AmericaWinnipeg.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AmericaYakutat.fix(), FixedOffset::east_opt(-32400).unwrap());
    /// assert_eq!(IbTimeZone::AmericaYellowknife.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AntarcticaCasey.fix(), FixedOffset::east_opt(-39600).unwrap());
    /// assert_eq!(IbTimeZone::AntarcticaDavis.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AntarcticaDumontdurville.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::AntarcticaMacquarie.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::AntarcticaMawson.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AntarcticaMcmurdo.fix(), FixedOffset::east_opt(-43200).unwrap());
    /// assert_eq!(IbTimeZone::AntarcticaPalmer.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AntarcticaRothera.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AntarcticaSyowa.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AntarcticaVostok.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::ArcticLongyearbyen.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::AsiaAden.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AsiaAlmaty.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AsiaAmman.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::AsiaAnadyr.fix(), FixedOffset::east_opt(-43200).unwrap());
    /// assert_eq!(IbTimeZone::AsiaAqtau.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AsiaAqtobe.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AsiaAshgabat.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AsiaAshkhabad.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AsiaAtyrau.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AsiaBaghdad.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AsiaBahrain.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AsiaBaku.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AsiaBangkok.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AsiaBarnaul.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AsiaBeirut.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::AsiaBishkek.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AsiaBrunei.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::AsiaCalcutta.fix(), FixedOffset::east_opt(-19800).unwrap());
    /// assert_eq!(IbTimeZone::AsiaChita.fix(), FixedOffset::east_opt(-32400).unwrap());
    /// assert_eq!(IbTimeZone::AsiaChoibalsan.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::AsiaChongqing.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::AsiaChungking.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::AsiaColombo.fix(), FixedOffset::east_opt(-19800).unwrap());
    /// assert_eq!(IbTimeZone::AsiaDacca.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AsiaDamascus.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::AsiaDhaka.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AsiaDili.fix(), FixedOffset::east_opt(-32400).unwrap());
    /// assert_eq!(IbTimeZone::AsiaDubai.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AsiaDushanbe.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AsiaFamagusta.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::AsiaGaza.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::AsiaHarbin.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::AsiaHebron.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::AsiaHoChiMinh.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AsiaHongKong.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::AsiaHovd.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AsiaIrkutsk.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::AsiaIstanbul.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AsiaJakarta.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AsiaJayapura.fix(), FixedOffset::east_opt(-32400).unwrap());
    /// assert_eq!(IbTimeZone::AsiaJerusalem.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::AsiaKabul.fix(), FixedOffset::east_opt(-16200).unwrap());
    /// assert_eq!(IbTimeZone::AsiaKamchatka.fix(), FixedOffset::east_opt(-43200).unwrap());
    /// assert_eq!(IbTimeZone::AsiaKarachi.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AsiaKashgar.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AsiaKathmandu.fix(), FixedOffset::east_opt(-20700).unwrap());
    /// assert_eq!(IbTimeZone::AsiaKhandyga.fix(), FixedOffset::east_opt(-32400).unwrap());
    /// assert_eq!(IbTimeZone::AsiaKolkata.fix(), FixedOffset::east_opt(-19800).unwrap());
    /// assert_eq!(IbTimeZone::AsiaKrasnoyarsk.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AsiaKualaLumpur.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::AsiaKuching.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::AsiaKuwait.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AsiaMacao.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::AsiaMacau.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::AsiaMagadan.fix(), FixedOffset::east_opt(-39600).unwrap());
    /// assert_eq!(IbTimeZone::AsiaMakassar.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::AsiaManila.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::AsiaMuscat.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AsiaNicosia.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::AsiaNovokuznetsk.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AsiaNovosibirsk.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AsiaOmsk.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AsiaOral.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AsiaPhnomPenh.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AsiaPontianak.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AsiaPyongyang.fix(), FixedOffset::east_opt(-32400).unwrap());
    /// assert_eq!(IbTimeZone::AsiaQatar.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AsiaQostanay.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AsiaQyzylorda.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AsiaRangoon.fix(), FixedOffset::east_opt(-23400).unwrap());
    /// assert_eq!(IbTimeZone::AsiaRiyadh.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AsiaSaigon.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AsiaSakhalin.fix(), FixedOffset::east_opt(-39600).unwrap());
    /// assert_eq!(IbTimeZone::AsiaSamarkand.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AsiaSeoul.fix(), FixedOffset::east_opt(-32400).unwrap());
    /// assert_eq!(IbTimeZone::AsiaShanghai.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::AsiaSingapore.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::AsiaSrednekolymsk.fix(), FixedOffset::east_opt(-39600).unwrap());
    /// assert_eq!(IbTimeZone::AsiaTaipei.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::AsiaTashkent.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AsiaTbilisi.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AsiaTehran.fix(), FixedOffset::east_opt(-12600).unwrap());
    /// assert_eq!(IbTimeZone::AsiaTelAviv.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::AsiaThimbu.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AsiaThimphu.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AsiaTokyo.fix(), FixedOffset::east_opt(-32400).unwrap());
    /// assert_eq!(IbTimeZone::AsiaTomsk.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AsiaUjungPandang.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::AsiaUlaanbaatar.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::AsiaUrumqi.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::AsiaUstNera.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::AsiaVientiane.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::AsiaVladivostok.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::AsiaYangon.fix(), FixedOffset::east_opt(-23400).unwrap());
    /// assert_eq!(IbTimeZone::AsiaYekaterinburg.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::AsiaYerevan.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AtlanticAzores.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::AtlanticBermuda.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::AtlanticCanary.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::AtlanticCapeVerde.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::AtlanticFaeroe.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::AtlanticFaroe.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::AtlanticJanMayen.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::AtlanticMadeira.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::AtlanticReykjavik.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::AtlanticSouthGeorgia.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::AtlanticStHelena.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::AtlanticStanley.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::AustraliaAct.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::AustraliaAdelaide.fix(), FixedOffset::east_opt(-34200).unwrap());
    /// assert_eq!(IbTimeZone::AustraliaBrisbane.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::AustraliaBrokenHill.fix(), FixedOffset::east_opt(-34200).unwrap());
    /// assert_eq!(IbTimeZone::AustraliaCanberra.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::AustraliaCurrie.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::AustraliaDarwin.fix(), FixedOffset::east_opt(-34200).unwrap());
    /// assert_eq!(IbTimeZone::AustraliaEucla.fix(), FixedOffset::east_opt(-31500).unwrap());
    /// assert_eq!(IbTimeZone::AustraliaHobart.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::AustraliaLhi.fix(), FixedOffset::east_opt(-37800).unwrap());
    /// assert_eq!(IbTimeZone::AustraliaLindeman.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::AustraliaLordHowe.fix(), FixedOffset::east_opt(-37800).unwrap());
    /// assert_eq!(IbTimeZone::AustraliaMelbourne.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::AustraliaNsw.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::AustraliaNorth.fix(), FixedOffset::east_opt(-34200).unwrap());
    /// assert_eq!(IbTimeZone::AustraliaPerth.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::AustraliaQueensland.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::AustraliaSouth.fix(), FixedOffset::east_opt(-34200).unwrap());
    /// assert_eq!(IbTimeZone::AustraliaSydney.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::AustraliaTasmania.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::AustraliaVictoria.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::AustraliaWest.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::BrazilAcre.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::BrazilDenoronha.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::BrazilEast.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::BrazilWest.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::Cet.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::Cst6Cdt.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::CanadaAtlantic.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::CanadaCentral.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::CanadaEastern.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::CanadaMountain.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::CanadaPacific.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::CanadaSaskatchewan.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::CanadaYukon.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::ChileContinental.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::ChileEasterlsland.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::Eet.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::Est5Edt.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::Egypt.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::Eire.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::EuropeAmsterdam.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropeAndorra.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropeAstrakhan.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::EuropeAthens.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::EuropeBelfast.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::EuropeBelgrade.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropeBerlin.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropeBratislava.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropeBrussels.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropeBucharest.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::EuropeBudapest.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropeBusingen.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropeChisinau.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::EuropeCopenhagen.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropeDublin.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::EuropeGibraltar.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropeGuernsey.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::EuropeHelsinki.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::EuropeIsleOfMan.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::EuropeIstanbul.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::EuropeJersey.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::EuropeKaliningrad.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::EuropeKiev.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::EuropeKirov.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::EuropeKyiv.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::EuropeLisbon.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::EuropeLjubljana.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropeLondon.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::EuropeLuxembourg.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropeMadrid.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropeMalta.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropeMariehamn.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::EuropeMinsk.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::EuropeMonaco.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropeMoscow.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::EuropeNicosia.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::EuropeOslo.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropeParis.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropePodgorica.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropePrague.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropeRiga.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::EuropeRome.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropeSamara.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::EuropeSanMarino.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropeSarajevo.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropeSaratov.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::EuropeSimferopol.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::EuropeSkopje.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropeSofia.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::EuropeStockholm.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropeTallinn.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::EuropeTirane.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropeUlyanovsk.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::EuropeUzhgorod.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::EuropeVaduz.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropeVatican.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropeVienna.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropeVilnius.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::EuropeVolgograd.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::EuropeWarsaw.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropeZagreb.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::EuropeZaporozhye.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::EuropeZurich.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::Gb.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::GbEire.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::Greenwich.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::Hongkong.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::Iceland.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::IndianAntananarivo.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::IndianChagos.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::IndianChristmas.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::IndianCocos.fix(), FixedOffset::east_opt(-23400).unwrap());
    /// assert_eq!(IbTimeZone::IndianComoro.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::IndianKerguelen.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::IndianMahe.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::IndianMaldives.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::IndianMauritius.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::IndianMayotte.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::IndianReunion.fix(), FixedOffset::east_opt(-14400).unwrap());
    /// assert_eq!(IbTimeZone::Israel.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::Jamaica.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::Japan.fix(), FixedOffset::east_opt(-32400).unwrap());
    /// assert_eq!(IbTimeZone::Kwajalein.fix(), FixedOffset::east_opt(-43200).unwrap());
    /// assert_eq!(IbTimeZone::Libya.fix(), FixedOffset::east_opt(-7200).unwrap());
    /// assert_eq!(IbTimeZone::Met.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::Mst7Mdt.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::MexicoBajanorte.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::MexicoGeneral.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::Nz.fix(), FixedOffset::east_opt(-43200).unwrap());
    /// assert_eq!(IbTimeZone::NzChat.fix(), FixedOffset::east_opt(-45900).unwrap());
    /// assert_eq!(IbTimeZone::Navajo.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::Prc.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::Pst8Pdt.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::PacificApia.fix(), FixedOffset::east_opt(-46800).unwrap());
    /// assert_eq!(IbTimeZone::PacificAuckland.fix(), FixedOffset::east_opt(-43200).unwrap());
    /// assert_eq!(IbTimeZone::PacificBougainville.fix(), FixedOffset::east_opt(-39600).unwrap());
    /// assert_eq!(IbTimeZone::PacificChatham.fix(), FixedOffset::east_opt(-45900).unwrap());
    /// assert_eq!(IbTimeZone::PacificChuuk.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::PacificEaster.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::PacificEfate.fix(), FixedOffset::east_opt(-39600).unwrap());
    /// assert_eq!(IbTimeZone::PacificEnderbury.fix(), FixedOffset::east_opt(-46800).unwrap());
    /// assert_eq!(IbTimeZone::PacificFiji.fix(), FixedOffset::east_opt(-43200).unwrap());
    /// assert_eq!(IbTimeZone::PacificFunafuti.fix(), FixedOffset::east_opt(-43200).unwrap());
    /// assert_eq!(IbTimeZone::PacificGalapagos.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::PacificGambier.fix(), FixedOffset::east_opt(-32400).unwrap());
    /// assert_eq!(IbTimeZone::PacificGuadalcanal.fix(), FixedOffset::east_opt(-39600).unwrap());
    /// assert_eq!(IbTimeZone::PacificGuam.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::PacificHonolulu.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::PacificJohnston.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::PacificKanton.fix(), FixedOffset::east_opt(-46800).unwrap());
    /// assert_eq!(IbTimeZone::PacificKiritimati.fix(), FixedOffset::east_opt(-50400).unwrap());
    /// assert_eq!(IbTimeZone::PacificKosrae.fix(), FixedOffset::east_opt(-39600).unwrap());
    /// assert_eq!(IbTimeZone::PacificKwajalein.fix(), FixedOffset::east_opt(-43200).unwrap());
    /// assert_eq!(IbTimeZone::PacificMajuro.fix(), FixedOffset::east_opt(-43200).unwrap());
    /// assert_eq!(IbTimeZone::PacificMarquesas.fix(), FixedOffset::east_opt(-34200).unwrap());
    /// assert_eq!(IbTimeZone::PacificMidway.fix(), FixedOffset::east_opt(-39600).unwrap());
    /// assert_eq!(IbTimeZone::PacificNauru.fix(), FixedOffset::east_opt(-43200).unwrap());
    /// assert_eq!(IbTimeZone::PacificNiue.fix(), FixedOffset::east_opt(-39600).unwrap());
    /// assert_eq!(IbTimeZone::PacificNorfolk.fix(), FixedOffset::east_opt(-39600).unwrap());
    /// assert_eq!(IbTimeZone::PacificNoumea.fix(), FixedOffset::east_opt(-39600).unwrap());
    /// assert_eq!(IbTimeZone::PacificPagoPago.fix(), FixedOffset::east_opt(-39600).unwrap());
    /// assert_eq!(IbTimeZone::PacificPalau.fix(), FixedOffset::east_opt(-32400).unwrap());
    /// assert_eq!(IbTimeZone::PacificPitcairn.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::PacificPohnpei.fix(), FixedOffset::east_opt(-39600).unwrap());
    /// assert_eq!(IbTimeZone::PacificPortMoresby.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::PacificRarotonga.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::PacificSaipan.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::PacificSamoa.fix(), FixedOffset::east_opt(-39600).unwrap());
    /// assert_eq!(IbTimeZone::PacificTahiti.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::PacificTarawa.fix(), FixedOffset::east_opt(-43200).unwrap());
    /// assert_eq!(IbTimeZone::PacificTongatapu.fix(), FixedOffset::east_opt(-46800).unwrap());
    /// assert_eq!(IbTimeZone::PacificWallis.fix(), FixedOffset::east_opt(-43200).unwrap());
    /// assert_eq!(IbTimeZone::PacificYap.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::Poland.fix(), FixedOffset::east_opt(-3600).unwrap());
    /// assert_eq!(IbTimeZone::Portugal.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::Rok.fix(), FixedOffset::east_opt(-32400).unwrap());
    /// assert_eq!(IbTimeZone::Singapore.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::Turkey.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::Uct.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::UsAlaska.fix(), FixedOffset::east_opt(-32400).unwrap());
    /// assert_eq!(IbTimeZone::UsAleutian.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::UsArizona.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::UsCentral.fix(), FixedOffset::east_opt(-21600).unwrap());
    /// assert_eq!(IbTimeZone::UsEastIndiana.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::UsEastern.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::UsHawail.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::UsMountain.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// assert_eq!(IbTimeZone::UsPacific.fix(), FixedOffset::east_opt(-28800).unwrap());
    /// assert_eq!(IbTimeZone::UsSamoa.fix(), FixedOffset::east_opt(-39600).unwrap());
    /// assert_eq!(IbTimeZone::Universal.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::WSu.fix(), FixedOffset::east_opt(-10800).unwrap());
    /// assert_eq!(IbTimeZone::Wet.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::Zulu.fix(), FixedOffset::east_opt(0).unwrap());
    /// assert_eq!(IbTimeZone::Est.fix(), FixedOffset::east_opt(-18000).unwrap());
    /// assert_eq!(IbTimeZone::Hst.fix(), FixedOffset::east_opt(-36000).unwrap());
    /// assert_eq!(IbTimeZone::Mst.fix(), FixedOffset::east_opt(-25200).unwrap());
    /// ```
    fn fix(&self) -> FixedOffset {
        match self {
            Self::AfricaAbidjan => FixedOffset::east_opt(0),
            Self::AfricaAccra => FixedOffset::east_opt(0),
            Self::AfricaAddisAbaba => FixedOffset::east_opt(-10800),
            Self::AfricaAlgiers => FixedOffset::east_opt(-3600),
            Self::AfricaAsmara => FixedOffset::east_opt(-10800),
            Self::AfricaAsmera => FixedOffset::east_opt(-10800),
            Self::AfricaBamako => FixedOffset::east_opt(0),
            Self::AfricaBangui => FixedOffset::east_opt(-3600),
            Self::AfricaBanjul => FixedOffset::east_opt(0),
            Self::AfricaBissau => FixedOffset::east_opt(0),
            Self::AfricaBlantyre => FixedOffset::east_opt(-7200),
            Self::AfricaBrazzaville => FixedOffset::east_opt(-3600),
            Self::AfricaBujumbura => FixedOffset::east_opt(-7200),
            Self::AfricaCairo => FixedOffset::east_opt(-7200),
            Self::AfricaCasablanca => FixedOffset::east_opt(0),
            Self::AfricaCeuta => FixedOffset::east_opt(-3600),
            Self::AfricaConakry => FixedOffset::east_opt(0),
            Self::AfricaDakar => FixedOffset::east_opt(0),
            Self::AfricaDarEsSalaam => FixedOffset::east_opt(-10800),
            Self::AfricaDjibouti => FixedOffset::east_opt(-10800),
            Self::AfricaDouala => FixedOffset::east_opt(-3600),
            Self::AfricaElAaiun => FixedOffset::east_opt(0),
            Self::AfricaFreetown => FixedOffset::east_opt(0),
            Self::AfricaGaborone => FixedOffset::east_opt(-7200),
            Self::AfricaHarare => FixedOffset::east_opt(-7200),
            Self::AfricaJohannesburg => FixedOffset::east_opt(-7200),
            Self::AfricaJuba => FixedOffset::east_opt(-7200),
            Self::AfricaKampala => FixedOffset::east_opt(-10800),
            Self::AfricaKhartoum => FixedOffset::east_opt(-7200),
            Self::AfricaKigali => FixedOffset::east_opt(-7200),
            Self::AfricaKinshasa => FixedOffset::east_opt(-3600),
            Self::AfricaLagos => FixedOffset::east_opt(-3600),
            Self::AfricaLibreville => FixedOffset::east_opt(-3600),
            Self::AfricaLome => FixedOffset::east_opt(0),
            Self::AfricaLuanda => FixedOffset::east_opt(-3600),
            Self::AfricaLubumbashi => FixedOffset::east_opt(-7200),
            Self::AfricaLusaka => FixedOffset::east_opt(-7200),
            Self::AfricaMalabo => FixedOffset::east_opt(-3600),
            Self::AfricaMaputo => FixedOffset::east_opt(-7200),
            Self::AfricaMaseru => FixedOffset::east_opt(-7200),
            Self::AfricaMbabane => FixedOffset::east_opt(-7200),
            Self::AfricaMogadishu => FixedOffset::east_opt(-10800),
            Self::AfricaMonrovia => FixedOffset::east_opt(0),
            Self::AfricaNairobi => FixedOffset::east_opt(-10800),
            Self::AfricaNdjamena => FixedOffset::east_opt(-3600),
            Self::AfricaNiamey => FixedOffset::east_opt(-3600),
            Self::AfricaNouakchott => FixedOffset::east_opt(0),
            Self::AfricaOuagadougou => FixedOffset::east_opt(0),
            Self::AfricaPortoNovo => FixedOffset::east_opt(-3600),
            Self::AfricaSaoTome => FixedOffset::east_opt(0),
            Self::AfricaTimbuktu => FixedOffset::east_opt(0),
            Self::AfricaTripoli => FixedOffset::east_opt(-7200),
            Self::AfricaTunis => FixedOffset::east_opt(-3600),
            Self::AfricaWindhoek => FixedOffset::east_opt(-3600),
            Self::AmericaAdak => FixedOffset::east_opt(-36000),
            Self::AmericaAnchorage => FixedOffset::east_opt(-32400),
            Self::AmericaAnguilla => FixedOffset::east_opt(-14400),
            Self::AmericaAntigua => FixedOffset::east_opt(-14400),
            Self::AmericaAraguaina => FixedOffset::east_opt(-10800),
            Self::AmericaArgentinaBuenosAires => FixedOffset::east_opt(-10800),
            Self::AmericaArgentinaCatamarca => FixedOffset::east_opt(-10800),
            Self::AmericaArgentinaCordoba => FixedOffset::east_opt(-10800),
            Self::AmericaArgentinaJujuy => FixedOffset::east_opt(-10800),
            Self::AmericaArgentinaLaRioja => FixedOffset::east_opt(-10800),
            Self::AmericaArgentinaMendoza => FixedOffset::east_opt(-10800),
            Self::AmericaArgentinaRioGallegos => FixedOffset::east_opt(-10800),
            Self::AmericaArgentinaSalta => FixedOffset::east_opt(-10800),
            Self::AmericaArgentinaSanJuan => FixedOffset::east_opt(-10800),
            Self::AmericaArgentinaSanLuis => FixedOffset::east_opt(-10800),
            Self::AmericaArgentinaTucuman => FixedOffset::east_opt(-10800),
            Self::AmericaArgentinaUshuaia => FixedOffset::east_opt(-10800),
            Self::AmericaAruba => FixedOffset::east_opt(-14400),
            Self::AmericaAsuncion => FixedOffset::east_opt(-14400),
            Self::AmericaAtikokan => FixedOffset::east_opt(-18000),
            Self::AmericaAtka => FixedOffset::east_opt(-36000),
            Self::AmericaBahia => FixedOffset::east_opt(-10800),
            Self::AmericaBahiaBanderas => FixedOffset::east_opt(-21600),
            Self::AmericaBarbados => FixedOffset::east_opt(-14400),
            Self::AmericaBelem => FixedOffset::east_opt(-10800),
            Self::AmericaBelize => FixedOffset::east_opt(-21600),
            Self::AmericaBlancSablon => FixedOffset::east_opt(-14400),
            Self::AmericaBoaVista => FixedOffset::east_opt(-14400),
            Self::AmericaBogota => FixedOffset::east_opt(-18000),
            Self::AmericaBoise => FixedOffset::east_opt(-25200),
            Self::AmericaBuenosAires => FixedOffset::east_opt(-10800),
            Self::AmericaCambridgeBay => FixedOffset::east_opt(-25200),
            Self::AmericaCampoGrande => FixedOffset::east_opt(-14400),
            Self::AmericaCancun => FixedOffset::east_opt(-18000),
            Self::AmericaCaracas => FixedOffset::east_opt(-14400),
            Self::AmericaCayenne => FixedOffset::east_opt(-10800),
            Self::AmericaCayman => FixedOffset::east_opt(-18000),
            Self::AmericaChicago => FixedOffset::east_opt(-21600),
            Self::AmericaChihuahua => FixedOffset::east_opt(-25200),
            Self::AmericaCoralHarbour => FixedOffset::east_opt(-18000),
            Self::AmericaCordoba => FixedOffset::east_opt(-10800),
            Self::AmericaCostaRica => FixedOffset::east_opt(-21600),
            Self::AmericaCreston => FixedOffset::east_opt(-25200),
            Self::AmericaCuiaba => FixedOffset::east_opt(-14400),
            Self::AmericaCuracao => FixedOffset::east_opt(-14400),
            Self::AmericaDanmarkshavn => FixedOffset::east_opt(0),
            Self::AmericaDawson => FixedOffset::east_opt(-25200),
            Self::AmericaDawsonCreek => FixedOffset::east_opt(-25200),
            Self::AmericaDenver => FixedOffset::east_opt(-25200),
            Self::AmericaDetroit => FixedOffset::east_opt(-18000),
            Self::AmericaDominica => FixedOffset::east_opt(-14400),
            Self::AmericaEdmonton => FixedOffset::east_opt(-25200),
            Self::AmericaEirunepe => FixedOffset::east_opt(-18000),
            Self::AmericaElSalvador => FixedOffset::east_opt(-21600),
            Self::AmericaEnsenada => FixedOffset::east_opt(-28800),
            Self::AmericaFortNelson => FixedOffset::east_opt(-25200),
            Self::AmericaFortWayne => FixedOffset::east_opt(-18000),
            Self::AmericaFortaleza => FixedOffset::east_opt(-10800),
            Self::AmericaGlaceBay => FixedOffset::east_opt(-14400),
            Self::AmericaGodthab => FixedOffset::east_opt(-10800),
            Self::AmericaGooseBay => FixedOffset::east_opt(-14400),
            Self::AmericaGrandTurk => FixedOffset::east_opt(-18000),
            Self::AmericaGrenada => FixedOffset::east_opt(-14400),
            Self::AmericaGuadeloupe => FixedOffset::east_opt(-14400),
            Self::AmericaGuatemala => FixedOffset::east_opt(-21600),
            Self::AmericaGuayaquil => FixedOffset::east_opt(-18000),
            Self::AmericaGuyana => FixedOffset::east_opt(-14400),
            Self::AmericaHalifax => FixedOffset::east_opt(-14400),
            Self::AmericaHermosillo => FixedOffset::east_opt(-25200),
            Self::AmericaIndianaIndianapolis => FixedOffset::east_opt(-18000),
            Self::AmericaIndianaMarengo => FixedOffset::east_opt(-18000),
            Self::AmericaIndianaPetersburg => FixedOffset::east_opt(-18000),
            Self::AmericaIndianaTellCity => FixedOffset::east_opt(-21600),
            Self::AmericaIndianaVevay => FixedOffset::east_opt(-18000),
            Self::AmericaIndianaVincennes => FixedOffset::east_opt(-18000),
            Self::AmericaIndianaWinamac => FixedOffset::east_opt(-18000),
            Self::AmericaIndianapolis => FixedOffset::east_opt(-18000),
            Self::AmericaInuvik => FixedOffset::east_opt(-25200),
            Self::AmericaIqaluit => FixedOffset::east_opt(-18000),
            Self::AmericaJamaica => FixedOffset::east_opt(-18000),
            Self::AmericaJuneau => FixedOffset::east_opt(-32400),
            Self::AmericaKentuckyLouisville => FixedOffset::east_opt(-18000),
            Self::AmericaKentuckyMonticello => FixedOffset::east_opt(-18000),
            Self::AmericaKralendijk => FixedOffset::east_opt(-14400),
            Self::AmericaLaPaz => FixedOffset::east_opt(-14400),
            Self::AmericaLima => FixedOffset::east_opt(-18000),
            Self::AmericaLosAngeles => FixedOffset::east_opt(-28800),
            Self::AmericaLouisville => FixedOffset::east_opt(-18000),
            Self::AmericaLowerPrinces => FixedOffset::east_opt(-14400),
            Self::AmericaMaceio => FixedOffset::east_opt(-10800),
            Self::AmericaManagua => FixedOffset::east_opt(-21600),
            Self::AmericaManaus => FixedOffset::east_opt(-14400),
            Self::AmericaMarigot => FixedOffset::east_opt(-14400),
            Self::AmericaMartinique => FixedOffset::east_opt(-14400),
            Self::AmericaMatamoros => FixedOffset::east_opt(-21600),
            Self::AmericaMazatlan => FixedOffset::east_opt(-25200),
            Self::AmericaMenominee => FixedOffset::east_opt(-21600),
            Self::AmericaMerida => FixedOffset::east_opt(-21600),
            Self::AmericaMetlakatla => FixedOffset::east_opt(-32400),
            Self::AmericaMexicoCity => FixedOffset::east_opt(-21600),
            Self::AmericaMiquelon => FixedOffset::east_opt(-10800),
            Self::AmericaMoncton => FixedOffset::east_opt(-14400),
            Self::AmericaMonterrey => FixedOffset::east_opt(-21600),
            Self::AmericaMontevideo => FixedOffset::east_opt(-10800),
            Self::AmericaMontreal => FixedOffset::east_opt(-18000),
            Self::AmericaMontserrat => FixedOffset::east_opt(-14400),
            Self::AmericaNassau => FixedOffset::east_opt(-18000),
            Self::AmericaNewYork => FixedOffset::east_opt(-18000),
            Self::AmericaNipigon => FixedOffset::east_opt(-18000),
            Self::AmericaNome => FixedOffset::east_opt(-32400),
            Self::AmericaNoronha => FixedOffset::east_opt(-7200),
            Self::AmericaNorthDakotaBeulah => FixedOffset::east_opt(-21600),
            Self::AmericaNorthDakotaCenter => FixedOffset::east_opt(-21600),
            Self::AmericaNorthDakotaNewSalem => FixedOffset::east_opt(-21600),
            Self::AmericaNuuk => FixedOffset::east_opt(-10800),
            Self::AmericaOjinaga => FixedOffset::east_opt(-25200),
            Self::AmericaPanama => FixedOffset::east_opt(-18000),
            Self::AmericaPangnirtung => FixedOffset::east_opt(-18000),
            Self::AmericaParamaribo => FixedOffset::east_opt(-10800),
            Self::AmericaPhoenix => FixedOffset::east_opt(-25200),
            Self::AmericaPortAuPrince => FixedOffset::east_opt(-18000),
            Self::AmericaPortOfSpain => FixedOffset::east_opt(-14400),
            Self::AmericaPortoAcre => FixedOffset::east_opt(-18000),
            Self::AmericaPortoVelho => FixedOffset::east_opt(-14400),
            Self::AmericaPuertoRico => FixedOffset::east_opt(-14400),
            Self::AmericaPuntaArenas => FixedOffset::east_opt(-10800),
            Self::AmericaRainyRiver => FixedOffset::east_opt(-21600),
            Self::AmericaRankinInlet => FixedOffset::east_opt(-21600),
            Self::AmericaRecife => FixedOffset::east_opt(-10800),
            Self::AmericaRegina => FixedOffset::east_opt(-21600),
            Self::AmericaResolute => FixedOffset::east_opt(-21600),
            Self::AmericaRioBranco => FixedOffset::east_opt(-18000),
            Self::AmericaRosario => FixedOffset::east_opt(-10800),
            Self::AmericaSantaIsabel => FixedOffset::east_opt(-28800),
            Self::AmericaSantarem => FixedOffset::east_opt(-10800),
            Self::AmericaSantiago => FixedOffset::east_opt(-14400),
            Self::AmericaSantoDomingo => FixedOffset::east_opt(-14400),
            Self::AmericaSaoPaulo => FixedOffset::east_opt(-10800),
            Self::AmericaScoresbysund => FixedOffset::east_opt(-3600),
            Self::AmericaShiprock => FixedOffset::east_opt(-25200),
            Self::AmericaSitka => FixedOffset::east_opt(-32400),
            Self::AmericaStBarthelemy => FixedOffset::east_opt(-14400),
            Self::AmericaStJohns => FixedOffset::east_opt(-12600),
            Self::AmericaStKitts => FixedOffset::east_opt(-14400),
            Self::AmericaStLucia => FixedOffset::east_opt(-14400),
            Self::AmericaStThomas => FixedOffset::east_opt(-14400),
            Self::AmericaStVincent => FixedOffset::east_opt(-14400),
            Self::AmericaSwiftCurrent => FixedOffset::east_opt(-21600),
            Self::AmericaTegucigalpa => FixedOffset::east_opt(-21600),
            Self::AmericaThule => FixedOffset::east_opt(-14400),
            Self::AmericaThunderBay => FixedOffset::east_opt(-18000),
            Self::AmericaTijuana => FixedOffset::east_opt(-28800),
            Self::AmericaToronto => FixedOffset::east_opt(-18000),
            Self::AmericaTortola => FixedOffset::east_opt(-14400),
            Self::AmericaVancouver => FixedOffset::east_opt(-28800),
            Self::AmericaVirgin => FixedOffset::east_opt(-14400),
            Self::AmericaWhitehorse => FixedOffset::east_opt(-25200),
            Self::AmericaWinnipeg => FixedOffset::east_opt(-21600),
            Self::AmericaYakutat => FixedOffset::east_opt(-32400),
            Self::AmericaYellowknife => FixedOffset::east_opt(-25200),
            Self::AntarcticaCasey => FixedOffset::east_opt(-39600),
            Self::AntarcticaDavis => FixedOffset::east_opt(-25200),
            Self::AntarcticaDumontdurville => FixedOffset::east_opt(-36000),
            Self::AntarcticaMacquarie => FixedOffset::east_opt(-36000),
            Self::AntarcticaMawson => FixedOffset::east_opt(-18000),
            Self::AntarcticaMcmurdo => FixedOffset::east_opt(-43200),
            Self::AntarcticaPalmer => FixedOffset::east_opt(-10800),
            Self::AntarcticaRothera => FixedOffset::east_opt(-10800),
            Self::AntarcticaSyowa => FixedOffset::east_opt(-10800),
            Self::AntarcticaVostok => FixedOffset::east_opt(-21600),
            Self::ArcticLongyearbyen => FixedOffset::east_opt(-3600),
            Self::AsiaAden => FixedOffset::east_opt(-10800),
            Self::AsiaAlmaty => FixedOffset::east_opt(-21600),
            Self::AsiaAmman => FixedOffset::east_opt(-7200),
            Self::AsiaAnadyr => FixedOffset::east_opt(-43200),
            Self::AsiaAqtau => FixedOffset::east_opt(-18000),
            Self::AsiaAqtobe => FixedOffset::east_opt(-18000),
            Self::AsiaAshgabat => FixedOffset::east_opt(-18000),
            Self::AsiaAshkhabad => FixedOffset::east_opt(-18000),
            Self::AsiaAtyrau => FixedOffset::east_opt(-18000),
            Self::AsiaBaghdad => FixedOffset::east_opt(-10800),
            Self::AsiaBahrain => FixedOffset::east_opt(-10800),
            Self::AsiaBaku => FixedOffset::east_opt(-14400),
            Self::AsiaBangkok => FixedOffset::east_opt(-25200),
            Self::AsiaBarnaul => FixedOffset::east_opt(-25200),
            Self::AsiaBeirut => FixedOffset::east_opt(-7200),
            Self::AsiaBishkek => FixedOffset::east_opt(-21600),
            Self::AsiaBrunei => FixedOffset::east_opt(-28800),
            Self::AsiaCalcutta => FixedOffset::east_opt(-19800),
            Self::AsiaChita => FixedOffset::east_opt(-32400),
            Self::AsiaChoibalsan => FixedOffset::east_opt(-28800),
            Self::AsiaChongqing => FixedOffset::east_opt(-28800),
            Self::AsiaChungking => FixedOffset::east_opt(-28800),
            Self::AsiaColombo => FixedOffset::east_opt(-19800),
            Self::AsiaDacca => FixedOffset::east_opt(-21600),
            Self::AsiaDamascus => FixedOffset::east_opt(-7200),
            Self::AsiaDhaka => FixedOffset::east_opt(-21600),
            Self::AsiaDili => FixedOffset::east_opt(-32400),
            Self::AsiaDubai => FixedOffset::east_opt(-14400),
            Self::AsiaDushanbe => FixedOffset::east_opt(-18000),
            Self::AsiaFamagusta => FixedOffset::east_opt(-7200),
            Self::AsiaGaza => FixedOffset::east_opt(-7200),
            Self::AsiaHarbin => FixedOffset::east_opt(-28800),
            Self::AsiaHebron => FixedOffset::east_opt(-7200),
            Self::AsiaHoChiMinh => FixedOffset::east_opt(-25200),
            Self::AsiaHongKong => FixedOffset::east_opt(-28800),
            Self::AsiaHovd => FixedOffset::east_opt(-25200),
            Self::AsiaIrkutsk => FixedOffset::east_opt(-28800),
            Self::AsiaIstanbul => FixedOffset::east_opt(-10800),
            Self::AsiaJakarta => FixedOffset::east_opt(-25200),
            Self::AsiaJayapura => FixedOffset::east_opt(-32400),
            Self::AsiaJerusalem => FixedOffset::east_opt(-7200),
            Self::AsiaKabul => FixedOffset::east_opt(-16200),
            Self::AsiaKamchatka => FixedOffset::east_opt(-43200),
            Self::AsiaKarachi => FixedOffset::east_opt(-18000),
            Self::AsiaKashgar => FixedOffset::east_opt(-21600),
            Self::AsiaKathmandu => FixedOffset::east_opt(-20700),
            Self::AsiaKhandyga => FixedOffset::east_opt(-32400),
            Self::AsiaKolkata => FixedOffset::east_opt(-19800),
            Self::AsiaKrasnoyarsk => FixedOffset::east_opt(-25200),
            Self::AsiaKualaLumpur => FixedOffset::east_opt(-28800),
            Self::AsiaKuching => FixedOffset::east_opt(-28800),
            Self::AsiaKuwait => FixedOffset::east_opt(-10800),
            Self::AsiaMacao => FixedOffset::east_opt(-28800),
            Self::AsiaMacau => FixedOffset::east_opt(-28800),
            Self::AsiaMagadan => FixedOffset::east_opt(-39600),
            Self::AsiaMakassar => FixedOffset::east_opt(-28800),
            Self::AsiaManila => FixedOffset::east_opt(-28800),
            Self::AsiaMuscat => FixedOffset::east_opt(-14400),
            Self::AsiaNicosia => FixedOffset::east_opt(-7200),
            Self::AsiaNovokuznetsk => FixedOffset::east_opt(-25200),
            Self::AsiaNovosibirsk => FixedOffset::east_opt(-25200),
            Self::AsiaOmsk => FixedOffset::east_opt(-21600),
            Self::AsiaOral => FixedOffset::east_opt(-18000),
            Self::AsiaPhnomPenh => FixedOffset::east_opt(-25200),
            Self::AsiaPontianak => FixedOffset::east_opt(-25200),
            Self::AsiaPyongyang => FixedOffset::east_opt(-32400),
            Self::AsiaQatar => FixedOffset::east_opt(-10800),
            Self::AsiaQostanay => FixedOffset::east_opt(-21600),
            Self::AsiaQyzylorda => FixedOffset::east_opt(-18000),
            Self::AsiaRangoon => FixedOffset::east_opt(-23400),
            Self::AsiaRiyadh => FixedOffset::east_opt(-10800),
            Self::AsiaSaigon => FixedOffset::east_opt(-25200),
            Self::AsiaSakhalin => FixedOffset::east_opt(-39600),
            Self::AsiaSamarkand => FixedOffset::east_opt(-18000),
            Self::AsiaSeoul => FixedOffset::east_opt(-32400),
            Self::AsiaShanghai => FixedOffset::east_opt(-28800),
            Self::AsiaSingapore => FixedOffset::east_opt(-28800),
            Self::AsiaSrednekolymsk => FixedOffset::east_opt(-39600),
            Self::AsiaTaipei => FixedOffset::east_opt(-28800),
            Self::AsiaTashkent => FixedOffset::east_opt(-18000),
            Self::AsiaTbilisi => FixedOffset::east_opt(-14400),
            Self::AsiaTehran => FixedOffset::east_opt(-12600),
            Self::AsiaTelAviv => FixedOffset::east_opt(-7200),
            Self::AsiaThimbu => FixedOffset::east_opt(-21600),
            Self::AsiaThimphu => FixedOffset::east_opt(-21600),
            Self::AsiaTokyo => FixedOffset::east_opt(-32400),
            Self::AsiaTomsk => FixedOffset::east_opt(-25200),
            Self::AsiaUjungPandang => FixedOffset::east_opt(-28800),
            Self::AsiaUlaanbaatar => FixedOffset::east_opt(-28800),
            Self::AsiaUrumqi => FixedOffset::east_opt(-21600),
            Self::AsiaUstNera => FixedOffset::east_opt(-36000),
            Self::AsiaVientiane => FixedOffset::east_opt(-25200),
            Self::AsiaVladivostok => FixedOffset::east_opt(-36000),
            Self::AsiaYangon => FixedOffset::east_opt(-23400),
            Self::AsiaYekaterinburg => FixedOffset::east_opt(-18000),
            Self::AsiaYerevan => FixedOffset::east_opt(-14400),
            Self::AtlanticAzores => FixedOffset::east_opt(-3600),
            Self::AtlanticBermuda => FixedOffset::east_opt(-14400),
            Self::AtlanticCanary => FixedOffset::east_opt(0),
            Self::AtlanticCapeVerde => FixedOffset::east_opt(-3600),
            Self::AtlanticFaeroe => FixedOffset::east_opt(0),
            Self::AtlanticFaroe => FixedOffset::east_opt(0),
            Self::AtlanticJanMayen => FixedOffset::east_opt(-3600),
            Self::AtlanticMadeira => FixedOffset::east_opt(0),
            Self::AtlanticReykjavik => FixedOffset::east_opt(0),
            Self::AtlanticSouthGeorgia => FixedOffset::east_opt(-7200),
            Self::AtlanticStHelena => FixedOffset::east_opt(0),
            Self::AtlanticStanley => FixedOffset::east_opt(-10800),
            Self::AustraliaAct => FixedOffset::east_opt(-36000),
            Self::AustraliaAdelaide => FixedOffset::east_opt(-34200),
            Self::AustraliaBrisbane => FixedOffset::east_opt(-36000),
            Self::AustraliaBrokenHill => FixedOffset::east_opt(-34200),
            Self::AustraliaCanberra => FixedOffset::east_opt(-36000),
            Self::AustraliaCurrie => FixedOffset::east_opt(-36000),
            Self::AustraliaDarwin => FixedOffset::east_opt(-34200),
            Self::AustraliaEucla => FixedOffset::east_opt(-31500),
            Self::AustraliaHobart => FixedOffset::east_opt(-36000),
            Self::AustraliaLhi => FixedOffset::east_opt(-37800),
            Self::AustraliaLindeman => FixedOffset::east_opt(-36000),
            Self::AustraliaLordHowe => FixedOffset::east_opt(-37800),
            Self::AustraliaMelbourne => FixedOffset::east_opt(-36000),
            Self::AustraliaNsw => FixedOffset::east_opt(-36000),
            Self::AustraliaNorth => FixedOffset::east_opt(-34200),
            Self::AustraliaPerth => FixedOffset::east_opt(-28800),
            Self::AustraliaQueensland => FixedOffset::east_opt(-36000),
            Self::AustraliaSouth => FixedOffset::east_opt(-34200),
            Self::AustraliaSydney => FixedOffset::east_opt(-36000),
            Self::AustraliaTasmania => FixedOffset::east_opt(-36000),
            Self::AustraliaVictoria => FixedOffset::east_opt(-36000),
            Self::AustraliaWest => FixedOffset::east_opt(-28800),
            Self::BrazilAcre => FixedOffset::east_opt(-18000),
            Self::BrazilDenoronha => FixedOffset::east_opt(-7200),
            Self::BrazilEast => FixedOffset::east_opt(-10800),
            Self::BrazilWest => FixedOffset::east_opt(-14400),
            Self::Cet => FixedOffset::east_opt(-3600),
            Self::Cst6Cdt => FixedOffset::east_opt(-21600),
            Self::CanadaAtlantic => FixedOffset::east_opt(-14400),
            Self::CanadaCentral => FixedOffset::east_opt(-21600),
            Self::CanadaEastern => FixedOffset::east_opt(-18000),
            Self::CanadaMountain => FixedOffset::east_opt(-25200),
            Self::CanadaPacific => FixedOffset::east_opt(-28800),
            Self::CanadaSaskatchewan => FixedOffset::east_opt(-21600),
            Self::CanadaYukon => FixedOffset::east_opt(-25200),
            Self::ChileContinental => FixedOffset::east_opt(-14400),
            Self::ChileEasterlsland => FixedOffset::east_opt(-21600),
            Self::Eet => FixedOffset::east_opt(-7200),
            Self::Est5Edt => FixedOffset::east_opt(-18000),
            Self::Egypt => FixedOffset::east_opt(-7200),
            Self::Eire => FixedOffset::east_opt(0),
            Self::EuropeAmsterdam => FixedOffset::east_opt(-3600),
            Self::EuropeAndorra => FixedOffset::east_opt(-3600),
            Self::EuropeAstrakhan => FixedOffset::east_opt(-14400),
            Self::EuropeAthens => FixedOffset::east_opt(-7200),
            Self::EuropeBelfast => FixedOffset::east_opt(0),
            Self::EuropeBelgrade => FixedOffset::east_opt(-3600),
            Self::EuropeBerlin => FixedOffset::east_opt(-3600),
            Self::EuropeBratislava => FixedOffset::east_opt(-3600),
            Self::EuropeBrussels => FixedOffset::east_opt(-3600),
            Self::EuropeBucharest => FixedOffset::east_opt(-7200),
            Self::EuropeBudapest => FixedOffset::east_opt(-3600),
            Self::EuropeBusingen => FixedOffset::east_opt(-3600),
            Self::EuropeChisinau => FixedOffset::east_opt(-7200),
            Self::EuropeCopenhagen => FixedOffset::east_opt(-3600),
            Self::EuropeDublin => FixedOffset::east_opt(0),
            Self::EuropeGibraltar => FixedOffset::east_opt(-3600),
            Self::EuropeGuernsey => FixedOffset::east_opt(0),
            Self::EuropeHelsinki => FixedOffset::east_opt(-7200),
            Self::EuropeIsleOfMan => FixedOffset::east_opt(0),
            Self::EuropeIstanbul => FixedOffset::east_opt(-10800),
            Self::EuropeJersey => FixedOffset::east_opt(0),
            Self::EuropeKaliningrad => FixedOffset::east_opt(-7200),
            Self::EuropeKiev => FixedOffset::east_opt(-7200),
            Self::EuropeKirov => FixedOffset::east_opt(-10800),
            Self::EuropeKyiv => FixedOffset::east_opt(-7200),
            Self::EuropeLisbon => FixedOffset::east_opt(0),
            Self::EuropeLjubljana => FixedOffset::east_opt(-3600),
            Self::EuropeLondon => FixedOffset::east_opt(0),
            Self::EuropeLuxembourg => FixedOffset::east_opt(-3600),
            Self::EuropeMadrid => FixedOffset::east_opt(-3600),
            Self::EuropeMalta => FixedOffset::east_opt(-3600),
            Self::EuropeMariehamn => FixedOffset::east_opt(-7200),
            Self::EuropeMinsk => FixedOffset::east_opt(-10800),
            Self::EuropeMonaco => FixedOffset::east_opt(-3600),
            Self::EuropeMoscow => FixedOffset::east_opt(-10800),
            Self::EuropeNicosia => FixedOffset::east_opt(-7200),
            Self::EuropeOslo => FixedOffset::east_opt(-3600),
            Self::EuropeParis => FixedOffset::east_opt(-3600),
            Self::EuropePodgorica => FixedOffset::east_opt(-3600),
            Self::EuropePrague => FixedOffset::east_opt(-3600),
            Self::EuropeRiga => FixedOffset::east_opt(-7200),
            Self::EuropeRome => FixedOffset::east_opt(-3600),
            Self::EuropeSamara => FixedOffset::east_opt(-14400),
            Self::EuropeSanMarino => FixedOffset::east_opt(-3600),
            Self::EuropeSarajevo => FixedOffset::east_opt(-3600),
            Self::EuropeSaratov => FixedOffset::east_opt(-14400),
            Self::EuropeSimferopol => FixedOffset::east_opt(-10800),
            Self::EuropeSkopje => FixedOffset::east_opt(-3600),
            Self::EuropeSofia => FixedOffset::east_opt(-7200),
            Self::EuropeStockholm => FixedOffset::east_opt(-3600),
            Self::EuropeTallinn => FixedOffset::east_opt(-7200),
            Self::EuropeTirane => FixedOffset::east_opt(-3600),
            Self::EuropeUlyanovsk => FixedOffset::east_opt(-14400),
            Self::EuropeUzhgorod => FixedOffset::east_opt(-7200),
            Self::EuropeVaduz => FixedOffset::east_opt(-3600),
            Self::EuropeVatican => FixedOffset::east_opt(-3600),
            Self::EuropeVienna => FixedOffset::east_opt(-3600),
            Self::EuropeVilnius => FixedOffset::east_opt(-7200),
            Self::EuropeVolgograd => FixedOffset::east_opt(-10800),
            Self::EuropeWarsaw => FixedOffset::east_opt(-3600),
            Self::EuropeZagreb => FixedOffset::east_opt(-3600),
            Self::EuropeZaporozhye => FixedOffset::east_opt(-7200),
            Self::EuropeZurich => FixedOffset::east_opt(-3600),
            Self::Gb => FixedOffset::east_opt(0),
            Self::GbEire => FixedOffset::east_opt(0),
            Self::Greenwich => FixedOffset::east_opt(0),
            Self::Hongkong => FixedOffset::east_opt(-28800),
            Self::Iceland => FixedOffset::east_opt(0),
            Self::IndianAntananarivo => FixedOffset::east_opt(-10800),
            Self::IndianChagos => FixedOffset::east_opt(-21600),
            Self::IndianChristmas => FixedOffset::east_opt(-25200),
            Self::IndianCocos => FixedOffset::east_opt(-23400),
            Self::IndianComoro => FixedOffset::east_opt(-10800),
            Self::IndianKerguelen => FixedOffset::east_opt(-18000),
            Self::IndianMahe => FixedOffset::east_opt(-14400),
            Self::IndianMaldives => FixedOffset::east_opt(-18000),
            Self::IndianMauritius => FixedOffset::east_opt(-14400),
            Self::IndianMayotte => FixedOffset::east_opt(-10800),
            Self::IndianReunion => FixedOffset::east_opt(-14400),
            Self::Israel => FixedOffset::east_opt(-7200),
            Self::Jamaica => FixedOffset::east_opt(-18000),
            Self::Japan => FixedOffset::east_opt(-32400),
            Self::Kwajalein => FixedOffset::east_opt(-43200),
            Self::Libya => FixedOffset::east_opt(-7200),
            Self::Met => FixedOffset::east_opt(-3600),
            Self::Mst7Mdt => FixedOffset::east_opt(-25200),
            Self::MexicoBajanorte => FixedOffset::east_opt(-28800),
            Self::MexicoGeneral => FixedOffset::east_opt(-21600),
            Self::Nz => FixedOffset::east_opt(-43200),
            Self::NzChat => FixedOffset::east_opt(-45900),
            Self::Navajo => FixedOffset::east_opt(-25200),
            Self::Prc => FixedOffset::east_opt(-28800),
            Self::Pst8Pdt => FixedOffset::east_opt(-28800),
            Self::PacificApia => FixedOffset::east_opt(-46800),
            Self::PacificAuckland => FixedOffset::east_opt(-43200),
            Self::PacificBougainville => FixedOffset::east_opt(-39600),
            Self::PacificChatham => FixedOffset::east_opt(-45900),
            Self::PacificChuuk => FixedOffset::east_opt(-36000),
            Self::PacificEaster => FixedOffset::east_opt(-21600),
            Self::PacificEfate => FixedOffset::east_opt(-39600),
            Self::PacificEnderbury => FixedOffset::east_opt(-46800),
            Self::PacificFiji => FixedOffset::east_opt(-43200),
            Self::PacificFunafuti => FixedOffset::east_opt(-43200),
            Self::PacificGalapagos => FixedOffset::east_opt(-21600),
            Self::PacificGambier => FixedOffset::east_opt(-32400),
            Self::PacificGuadalcanal => FixedOffset::east_opt(-39600),
            Self::PacificGuam => FixedOffset::east_opt(-36000),
            Self::PacificHonolulu => FixedOffset::east_opt(-36000),
            Self::PacificJohnston => FixedOffset::east_opt(-36000),
            Self::PacificKanton => FixedOffset::east_opt(-46800),
            Self::PacificKiritimati => FixedOffset::east_opt(-50400),
            Self::PacificKosrae => FixedOffset::east_opt(-39600),
            Self::PacificKwajalein => FixedOffset::east_opt(-43200),
            Self::PacificMajuro => FixedOffset::east_opt(-43200),
            Self::PacificMarquesas => FixedOffset::east_opt(-34200),
            Self::PacificMidway => FixedOffset::east_opt(-39600),
            Self::PacificNauru => FixedOffset::east_opt(-43200),
            Self::PacificNiue => FixedOffset::east_opt(-39600),
            Self::PacificNorfolk => FixedOffset::east_opt(-39600),
            Self::PacificNoumea => FixedOffset::east_opt(-39600),
            Self::PacificPagoPago => FixedOffset::east_opt(-39600),
            Self::PacificPalau => FixedOffset::east_opt(-32400),
            Self::PacificPitcairn => FixedOffset::east_opt(-28800),
            Self::PacificPohnpei => FixedOffset::east_opt(-39600),
            Self::PacificPortMoresby => FixedOffset::east_opt(-36000),
            Self::PacificRarotonga => FixedOffset::east_opt(-36000),
            Self::PacificSaipan => FixedOffset::east_opt(-36000),
            Self::PacificSamoa => FixedOffset::east_opt(-39600),
            Self::PacificTahiti => FixedOffset::east_opt(-36000),
            Self::PacificTarawa => FixedOffset::east_opt(-43200),
            Self::PacificTongatapu => FixedOffset::east_opt(-46800),
            Self::PacificWallis => FixedOffset::east_opt(-43200),
            Self::PacificYap => FixedOffset::east_opt(-36000),
            Self::Poland => FixedOffset::east_opt(-3600),
            Self::Portugal => FixedOffset::east_opt(0),
            Self::Rok => FixedOffset::east_opt(-32400),
            Self::Singapore => FixedOffset::east_opt(-28800),
            Self::Turkey => FixedOffset::east_opt(-10800),
            Self::Uct => FixedOffset::east_opt(0),
            Self::UsAlaska => FixedOffset::east_opt(-32400),
            Self::UsAleutian => FixedOffset::east_opt(-36000),
            Self::UsArizona => FixedOffset::east_opt(-25200),
            Self::UsCentral => FixedOffset::east_opt(-21600),
            Self::UsEastIndiana => FixedOffset::east_opt(-18000),
            Self::UsEastern => FixedOffset::east_opt(-18000),
            Self::UsHawail => FixedOffset::east_opt(-36000),
            Self::UsMountain => FixedOffset::east_opt(-25200),
            Self::UsPacific => FixedOffset::east_opt(-28800),
            Self::UsSamoa => FixedOffset::east_opt(-39600),
            Self::Universal => FixedOffset::east_opt(0),
            Self::WSu => FixedOffset::east_opt(-10800),
            Self::Wet => FixedOffset::east_opt(0),
            Self::Zulu => FixedOffset::east_opt(0),
            Self::Est => FixedOffset::east_opt(-18000),
            Self::Hst => FixedOffset::east_opt(-36000),
            Self::Mst => FixedOffset::east_opt(-25200),
        }.unwrap()
    }
}

impl TimeZone for IbTimeZone {
    type Offset = Self;

    fn from_offset(offset: &Self::Offset) -> Self {
        *offset
    }

    fn offset_from_local_date(&self, _local: &NaiveDate) -> LocalResult<Self::Offset> {
        LocalResult::None
    }

    fn offset_from_local_datetime(&self, _local: &NaiveDateTime) -> LocalResult<Self::Offset> {
        LocalResult::None
    }

    fn offset_from_utc_date(&self, _utc: &NaiveDate) -> Self::Offset {
        Self::Universal
    }

    fn offset_from_utc_datetime(&self, _utc: &NaiveDateTime) -> Self::Offset {
        Self::Universal
    }
}

impl From<IbTimeZone> for FixedOffset {
    fn from(val: IbTimeZone) -> Self {
        val.fix()
    }
}

impl std::str::FromStr for IbTimeZone {
    type Err = InvalidTimeZone;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Africa/Abidjan" => Self::AfricaAbidjan,
            "Africa/Accra" => Self::AfricaAccra,
            "Africa/Addis Ababa" => Self::AfricaAddisAbaba,
            "Africa/Algiers" => Self::AfricaAlgiers,
            "Africa/Asmara" => Self::AfricaAsmara,
            "Africa/Asmera" => Self::AfricaAsmera,
            "Africa/Bamako" => Self::AfricaBamako,
            "Africa/Bangui" => Self::AfricaBangui,
            "Africa/Banjul" => Self::AfricaBanjul,
            "Africa/Bissau" => Self::AfricaBissau,
            "Africa/Blantyre" => Self::AfricaBlantyre,
            "Africa/Brazzaville" => Self::AfricaBrazzaville,
            "Africa/Bujumbura" => Self::AfricaBujumbura,
            "Africa/Cairo" => Self::AfricaCairo,
            "Africa/Casablanca" => Self::AfricaCasablanca,
            "Africa/Ceuta" => Self::AfricaCeuta,
            "Africa/Conakry" => Self::AfricaConakry,
            "Africa/Dakar" => Self::AfricaDakar,
            "Africa/Dar es Salaam" => Self::AfricaDarEsSalaam,
            "Africa/Djibouti" => Self::AfricaDjibouti,
            "Africa/Douala" => Self::AfricaDouala,
            "Africa/El Aaiun" => Self::AfricaElAaiun,
            "Africa/Freetown" => Self::AfricaFreetown,
            "Africa/Gaborone" => Self::AfricaGaborone,
            "Africa/Harare" => Self::AfricaHarare,
            "Africa/Johannesburg" => Self::AfricaJohannesburg,
            "Africa/Juba" => Self::AfricaJuba,
            "Africa/Kampala" => Self::AfricaKampala,
            "Africa/Khartoum" => Self::AfricaKhartoum,
            "Africa/Kigali" => Self::AfricaKigali,
            "Africa/Kinshasa" => Self::AfricaKinshasa,
            "Africa/Lagos" => Self::AfricaLagos,
            "Africa/Libreville" => Self::AfricaLibreville,
            "Africa/Lome" => Self::AfricaLome,
            "Africa/Luanda" => Self::AfricaLuanda,
            "Africa/Lubumbashi" => Self::AfricaLubumbashi,
            "Africa/Lusaka" => Self::AfricaLusaka,
            "Africa/Malabo" => Self::AfricaMalabo,
            "Africa/Maputo" => Self::AfricaMaputo,
            "Africa/Maseru" => Self::AfricaMaseru,
            "Africa/Mbabane" => Self::AfricaMbabane,
            "Africa/Mogadishu" => Self::AfricaMogadishu,
            "Africa/Monrovia" => Self::AfricaMonrovia,
            "Africa/Nairobi" => Self::AfricaNairobi,
            "Africa/Ndjamena" => Self::AfricaNdjamena,
            "Africa/Niamey" => Self::AfricaNiamey,
            "Africa/Nouakchott" => Self::AfricaNouakchott,
            "Africa/Ouagadougou" => Self::AfricaOuagadougou,
            "Africa/Porto-Novo" => Self::AfricaPortoNovo,
            "Africa/Sao Tome" => Self::AfricaSaoTome,
            "Africa/Timbuktu" => Self::AfricaTimbuktu,
            "Africa/Tripoli" => Self::AfricaTripoli,
            "Africa/Tunis" => Self::AfricaTunis,
            "Africa/Windhoek" => Self::AfricaWindhoek,
            "America/Adak" => Self::AmericaAdak,
            "America/Anchorage" => Self::AmericaAnchorage,
            "America/Anguilla" => Self::AmericaAnguilla,
            "America/Antigua" => Self::AmericaAntigua,
            "America/Araguaina" => Self::AmericaAraguaina,
            "America/Argentina/Buenos Aires" => Self::AmericaArgentinaBuenosAires,
            "America/Argentina/Catamarca" => Self::AmericaArgentinaCatamarca,
            "America/Argentina/Cordoba" => Self::AmericaArgentinaCordoba,
            "America/Argentina/Jujuy" => Self::AmericaArgentinaJujuy,
            "America/Argentina/La Rioja" => Self::AmericaArgentinaLaRioja,
            "America/Argentina/Mendoza" => Self::AmericaArgentinaMendoza,
            "America/Argentina/Rio Gallegos" => Self::AmericaArgentinaRioGallegos,
            "America/Argentina/Salta" => Self::AmericaArgentinaSalta,
            "America/Argentina/San Juan" => Self::AmericaArgentinaSanJuan,
            "America/Argentina/San Luis" => Self::AmericaArgentinaSanLuis,
            "America/Argentina/Tucuman" => Self::AmericaArgentinaTucuman,
            "America/Argentina/Ushuaia" => Self::AmericaArgentinaUshuaia,
            "America/Aruba" => Self::AmericaAruba,
            "America/Asuncion" => Self::AmericaAsuncion,
            "America/Atikokan" => Self::AmericaAtikokan,
            "America/Atka" => Self::AmericaAtka,
            "America/Bahia" => Self::AmericaBahia,
            "America/Bahia Banderas" => Self::AmericaBahiaBanderas,
            "America/Barbados" => Self::AmericaBarbados,
            "America/Belem" => Self::AmericaBelem,
            "America/Belize" => Self::AmericaBelize,
            "America/Blanc-Sablon" => Self::AmericaBlancSablon,
            "America/Boa Vista" => Self::AmericaBoaVista,
            "America/Bogota" => Self::AmericaBogota,
            "America/Boise" => Self::AmericaBoise,
            "America/Buenos Aires" => Self::AmericaBuenosAires,
            "America/Cambridge Bay" => Self::AmericaCambridgeBay,
            "America/Campo Grande" => Self::AmericaCampoGrande,
            "America/Cancun" => Self::AmericaCancun,
            "America/Caracas" => Self::AmericaCaracas,
            "America/Cayenne" => Self::AmericaCayenne,
            "America/Cayman" => Self::AmericaCayman,
            "America/Chicago" => Self::AmericaChicago,
            "America/Chihuahua" => Self::AmericaChihuahua,
            "America/Coral Harbour" => Self::AmericaCoralHarbour,
            "America/Cordoba" => Self::AmericaCordoba,
            "America/Costa Rica" => Self::AmericaCostaRica,
            "America/Creston" => Self::AmericaCreston,
            "America/Cuiaba" => Self::AmericaCuiaba,
            "America/Curacao" => Self::AmericaCuracao,
            "America/Danmarkshavn" => Self::AmericaDanmarkshavn,
            "America/Dawson" => Self::AmericaDawson,
            "America/Dawson Creek" => Self::AmericaDawsonCreek,
            "America/Denver" => Self::AmericaDenver,
            "America/Detroit" => Self::AmericaDetroit,
            "America/Dominica" => Self::AmericaDominica,
            "America/Edmonton" => Self::AmericaEdmonton,
            "America/Eirunepe" => Self::AmericaEirunepe,
            "America/El Salvador" => Self::AmericaElSalvador,
            "America/Ensenada" => Self::AmericaEnsenada,
            "America/Fort Nelson" => Self::AmericaFortNelson,
            "America/Fort Wayne" => Self::AmericaFortWayne,
            "America/Fortaleza" => Self::AmericaFortaleza,
            "America/Glace Bay" => Self::AmericaGlaceBay,
            "America/Godthab" => Self::AmericaGodthab,
            "America/Goose Bay" => Self::AmericaGooseBay,
            "America/Grand Turk" => Self::AmericaGrandTurk,
            "America/Grenada" => Self::AmericaGrenada,
            "America/Guadeloupe" => Self::AmericaGuadeloupe,
            "America/Guatemala" => Self::AmericaGuatemala,
            "America/Guayaquil" => Self::AmericaGuayaquil,
            "America/Guyana" => Self::AmericaGuyana,
            "America/Halifax" => Self::AmericaHalifax,
            "America/Hermosillo" => Self::AmericaHermosillo,
            "America/Indiana/Indianapolis" => Self::AmericaIndianaIndianapolis,
            "America/Indiana/Marengo" => Self::AmericaIndianaMarengo,
            "America/Indiana/Petersburg" => Self::AmericaIndianaPetersburg,
            "America/Indiana/Tell City" => Self::AmericaIndianaTellCity,
            "America/Indiana/Vevay" => Self::AmericaIndianaVevay,
            "America/Indiana/Vincennes" => Self::AmericaIndianaVincennes,
            "America/Indiana/Winamac" => Self::AmericaIndianaWinamac,
            "America/Indianapolis" => Self::AmericaIndianapolis,
            "America/Inuvik" => Self::AmericaInuvik,
            "America/Iqaluit" => Self::AmericaIqaluit,
            "America/Jamaica" => Self::AmericaJamaica,
            "America/Juneau" => Self::AmericaJuneau,
            "America/Kentucky/Louisville" => Self::AmericaKentuckyLouisville,
            "America/Kentucky/Monticello" => Self::AmericaKentuckyMonticello,
            "America/Kralendijk" => Self::AmericaKralendijk,
            "America/La Paz" => Self::AmericaLaPaz,
            "America/Lima" => Self::AmericaLima,
            "America/Los Angeles" => Self::AmericaLosAngeles,
            "America/Louisville" => Self::AmericaLouisville,
            "America/Lower Princes" => Self::AmericaLowerPrinces,
            "America/Maceio" => Self::AmericaMaceio,
            "America/Managua" => Self::AmericaManagua,
            "America/Manaus" => Self::AmericaManaus,
            "America/Marigot" => Self::AmericaMarigot,
            "America/Martinique" => Self::AmericaMartinique,
            "America/Matamoros" => Self::AmericaMatamoros,
            "America/Mazatlan" => Self::AmericaMazatlan,
            "America/Menominee" => Self::AmericaMenominee,
            "America/Merida" => Self::AmericaMerida,
            "America/Metlakatla" => Self::AmericaMetlakatla,
            "America/Mexico City" => Self::AmericaMexicoCity,
            "America/Miquelon" => Self::AmericaMiquelon,
            "America/Moncton" => Self::AmericaMoncton,
            "America/Monterrey" => Self::AmericaMonterrey,
            "America/Montevideo" => Self::AmericaMontevideo,
            "America/Montreal" => Self::AmericaMontreal,
            "America/Montserrat" => Self::AmericaMontserrat,
            "America/Nassau" => Self::AmericaNassau,
            "America/New York" => Self::AmericaNewYork,
            "America/Nipigon" => Self::AmericaNipigon,
            "America/Nome" => Self::AmericaNome,
            "America/Noronha" => Self::AmericaNoronha,
            "America/North Dakota/Beulah" => Self::AmericaNorthDakotaBeulah,
            "America/North Dakota/Center" => Self::AmericaNorthDakotaCenter,
            "America/North Dakota/New Salem" => Self::AmericaNorthDakotaNewSalem,
            "America/Nuuk" => Self::AmericaNuuk,
            "America/Ojinaga" => Self::AmericaOjinaga,
            "America/Panama" => Self::AmericaPanama,
            "America/Pangnirtung" => Self::AmericaPangnirtung,
            "America/Paramaribo" => Self::AmericaParamaribo,
            "America/Phoenix" => Self::AmericaPhoenix,
            "America/Port-au-Prince" => Self::AmericaPortAuPrince,
            "America/Port of Spain" => Self::AmericaPortOfSpain,
            "America/Porto Acre" => Self::AmericaPortoAcre,
            "America/Porto Velho" => Self::AmericaPortoVelho,
            "America/Puerto Rico" => Self::AmericaPuertoRico,
            "America/Punta Arenas" => Self::AmericaPuntaArenas,
            "America/Rainy River" => Self::AmericaRainyRiver,
            "America/Rankin Inlet" => Self::AmericaRankinInlet,
            "America/Recife" => Self::AmericaRecife,
            "America/Regina" => Self::AmericaRegina,
            "America/Resolute" => Self::AmericaResolute,
            "America/Rio Branco" => Self::AmericaRioBranco,
            "America/Rosario" => Self::AmericaRosario,
            "America/Santa Isabel" => Self::AmericaSantaIsabel,
            "America/Santarem" => Self::AmericaSantarem,
            "America/Santiago" => Self::AmericaSantiago,
            "America/Santo Domingo" => Self::AmericaSantoDomingo,
            "America/Sao Paulo" => Self::AmericaSaoPaulo,
            "America/Scoresbysund" => Self::AmericaScoresbysund,
            "America/Shiprock" => Self::AmericaShiprock,
            "America/Sitka" => Self::AmericaSitka,
            "America/St Barthelemy" => Self::AmericaStBarthelemy,
            "America/St Johns" => Self::AmericaStJohns,
            "America/St Kitts" => Self::AmericaStKitts,
            "America/St Lucia" => Self::AmericaStLucia,
            "America/St Thomas" => Self::AmericaStThomas,
            "America/St Vincent" => Self::AmericaStVincent,
            "America/Swift Current" => Self::AmericaSwiftCurrent,
            "America/Tegucigalpa" => Self::AmericaTegucigalpa,
            "America/Thule" => Self::AmericaThule,
            "America/Thunder Bay" => Self::AmericaThunderBay,
            "America/Tijuana" => Self::AmericaTijuana,
            "America/Toronto" => Self::AmericaToronto,
            "America/Tortola" => Self::AmericaTortola,
            "America/Vancouver" => Self::AmericaVancouver,
            "America/Virgin" => Self::AmericaVirgin,
            "America/Whitehorse" => Self::AmericaWhitehorse,
            "America/Winnipeg" => Self::AmericaWinnipeg,
            "America/Yakutat" => Self::AmericaYakutat,
            "America/Yellowknife" => Self::AmericaYellowknife,
            "Antarctica/Casey" => Self::AntarcticaCasey,
            "Antarctica/Davis" => Self::AntarcticaDavis,
            "Antarctica/DumontDUrville" => Self::AntarcticaDumontdurville,
            "Antarctica/Macquarie" => Self::AntarcticaMacquarie,
            "Antarctica/Mawson" => Self::AntarcticaMawson,
            "Antarctica/McMurdo" => Self::AntarcticaMcmurdo,
            "Antarctica/Palmer" => Self::AntarcticaPalmer,
            "Antarctica/Rothera" => Self::AntarcticaRothera,
            "Antarctica/Syowa" => Self::AntarcticaSyowa,
            "Antarctica/Vostok" => Self::AntarcticaVostok,
            "Arctic/Longyearbyen" => Self::ArcticLongyearbyen,
            "Asia/Aden" => Self::AsiaAden,
            "Asia/Almaty" => Self::AsiaAlmaty,
            "Asia/Amman" => Self::AsiaAmman,
            "Asia/Anadyr" => Self::AsiaAnadyr,
            "Asia/Aqtau" => Self::AsiaAqtau,
            "Asia/Aqtobe" => Self::AsiaAqtobe,
            "Asia/Ashgabat" => Self::AsiaAshgabat,
            "Asia/Ashkhabad" => Self::AsiaAshkhabad,
            "Asia/Atyrau" => Self::AsiaAtyrau,
            "Asia/Baghdad" => Self::AsiaBaghdad,
            "Asia/Bahrain" => Self::AsiaBahrain,
            "Asia/Baku" => Self::AsiaBaku,
            "Asia/Bangkok" => Self::AsiaBangkok,
            "Asia/Barnaul" => Self::AsiaBarnaul,
            "Asia/Beirut" => Self::AsiaBeirut,
            "Asia/Bishkek" => Self::AsiaBishkek,
            "Asia/Brunei" => Self::AsiaBrunei,
            "Asia/Calcutta" => Self::AsiaCalcutta,
            "Asia/Chita" => Self::AsiaChita,
            "Asia/Choibalsan" => Self::AsiaChoibalsan,
            "Asia/Chongqing" => Self::AsiaChongqing,
            "Asia/Chungking" => Self::AsiaChungking,
            "Asia/Colombo" => Self::AsiaColombo,
            "Asia/Dacca" => Self::AsiaDacca,
            "Asia/Damascus" => Self::AsiaDamascus,
            "Asia/Dhaka" => Self::AsiaDhaka,
            "Asia/Dili" => Self::AsiaDili,
            "Asia/Dubai" => Self::AsiaDubai,
            "Asia/Dushanbe" => Self::AsiaDushanbe,
            "Asia/Famagusta" => Self::AsiaFamagusta,
            "Asia/Gaza" => Self::AsiaGaza,
            "Asia/Harbin" => Self::AsiaHarbin,
            "Asia/Hebron" => Self::AsiaHebron,
            "Asia/Ho Chi Minh" => Self::AsiaHoChiMinh,
            "Asia/Hong Kong" => Self::AsiaHongKong,
            "Asia/Hovd" => Self::AsiaHovd,
            "Asia/Irkutsk" => Self::AsiaIrkutsk,
            "Asia/Istanbul" => Self::AsiaIstanbul,
            "Asia/Jakarta" => Self::AsiaJakarta,
            "Asia/Jayapura" => Self::AsiaJayapura,
            "Asia/Jerusalem" => Self::AsiaJerusalem,
            "Asia/Kabul" => Self::AsiaKabul,
            "Asia/Kamchatka" => Self::AsiaKamchatka,
            "Asia/Karachi" => Self::AsiaKarachi,
            "Asia/Kashgar" => Self::AsiaKashgar,
            "Asia/Kathmandu" => Self::AsiaKathmandu,
            "Asia/Khandyga" => Self::AsiaKhandyga,
            "Asia/Kolkata" => Self::AsiaKolkata,
            "Asia/Krasnoyarsk" => Self::AsiaKrasnoyarsk,
            "Asia/Kuala Lumpur" => Self::AsiaKualaLumpur,
            "Asia/Kuching" => Self::AsiaKuching,
            "Asia/Kuwait" => Self::AsiaKuwait,
            "Asia/Macao" => Self::AsiaMacao,
            "Asia/Macau" => Self::AsiaMacau,
            "Asia/Magadan" => Self::AsiaMagadan,
            "Asia/Makassar" => Self::AsiaMakassar,
            "Asia/Manila" => Self::AsiaManila,
            "Asia/Muscat" => Self::AsiaMuscat,
            "Asia/Nicosia" => Self::AsiaNicosia,
            "Asia/Novokuznetsk" => Self::AsiaNovokuznetsk,
            "Asia/Novosibirsk" => Self::AsiaNovosibirsk,
            "Asia/Omsk" => Self::AsiaOmsk,
            "Asia/Oral" => Self::AsiaOral,
            "Asia/Phnom Penh" => Self::AsiaPhnomPenh,
            "Asia/Pontianak" => Self::AsiaPontianak,
            "Asia/Pyongyang" => Self::AsiaPyongyang,
            "Asia/Qatar" => Self::AsiaQatar,
            "Asia/Qostanay" => Self::AsiaQostanay,
            "Asia/Qyzylorda" => Self::AsiaQyzylorda,
            "Asia/Rangoon" => Self::AsiaRangoon,
            "Asia/Riyadh" => Self::AsiaRiyadh,
            "Asia/Saigon" => Self::AsiaSaigon,
            "Asia/Sakhalin" => Self::AsiaSakhalin,
            "Asia/Samarkand" => Self::AsiaSamarkand,
            "Asia/Seoul" => Self::AsiaSeoul,
            "Asia/Shanghai" => Self::AsiaShanghai,
            "Asia/Singapore" => Self::AsiaSingapore,
            "Asia/Srednekolymsk" => Self::AsiaSrednekolymsk,
            "Asia/Taipei" => Self::AsiaTaipei,
            "Asia/Tashkent" => Self::AsiaTashkent,
            "Asia Tbilisi" => Self::AsiaTbilisi,
            "Asia/Tehran" => Self::AsiaTehran,
            "Asia/Tel Aviv" => Self::AsiaTelAviv,
            "Asia/Thimbu" => Self::AsiaThimbu,
            "Asia/Thimphu" => Self::AsiaThimphu,
            "Asia/Tokyo" => Self::AsiaTokyo,
            "Asia/Tomsk" => Self::AsiaTomsk,
            "Asia/Ujung Pandang" => Self::AsiaUjungPandang,
            "Asia/Ulaanbaatar" => Self::AsiaUlaanbaatar,
            "Asia/Urumqi" => Self::AsiaUrumqi,
            "Asia/Ust-Nera" => Self::AsiaUstNera,
            "Asia/Vientiane" => Self::AsiaVientiane,
            "Asia/Vladivostok" => Self::AsiaVladivostok,
            "Asia/Yangon" => Self::AsiaYangon,
            "Asia/Yekaterinburg" => Self::AsiaYekaterinburg,
            "Asia/Yerevan" => Self::AsiaYerevan,
            "Atlantic/Azores" => Self::AtlanticAzores,
            "Atlantic/Bermuda" => Self::AtlanticBermuda,
            "Atlantic/Canary" => Self::AtlanticCanary,
            "Atlantic/Cape Verde" => Self::AtlanticCapeVerde,
            "Atlantic/Faeroe" => Self::AtlanticFaeroe,
            "Atlantic/Faroe" => Self::AtlanticFaroe,
            "Atlantic/Jan Mayen" => Self::AtlanticJanMayen,
            "Atlantic/Madeira" => Self::AtlanticMadeira,
            "Atlantic/Reykjavik" => Self::AtlanticReykjavik,
            "Atlantic/South Georgia" => Self::AtlanticSouthGeorgia,
            "Atlantic/St Helena" => Self::AtlanticStHelena,
            "Atlantic/Stanley" => Self::AtlanticStanley,
            "Australia/ACT" => Self::AustraliaAct,
            "Australia/Adelaide" => Self::AustraliaAdelaide,
            "Australia/Brisbane" => Self::AustraliaBrisbane,
            "Australia/Broken Hill" => Self::AustraliaBrokenHill,
            "Australia/Canberra" => Self::AustraliaCanberra,
            "Australia/Currie" => Self::AustraliaCurrie,
            "Australia/Darwin" => Self::AustraliaDarwin,
            "Australia/Eucla" => Self::AustraliaEucla,
            "Australia/Hobart" => Self::AustraliaHobart,
            "Australia/LHI" => Self::AustraliaLhi,
            "Australia/Lindeman" => Self::AustraliaLindeman,
            "Australia/Lord Howe" => Self::AustraliaLordHowe,
            "Australia/Melbourne" => Self::AustraliaMelbourne,
            "Australia/NSW" => Self::AustraliaNsw,
            "Australia/North" => Self::AustraliaNorth,
            "Australia/Perth" => Self::AustraliaPerth,
            "Australia/Queensland" => Self::AustraliaQueensland,
            "Australia/South" => Self::AustraliaSouth,
            "Australia/Sydney" => Self::AustraliaSydney,
            "Australia/Tasmania" => Self::AustraliaTasmania,
            "Australia/Victoria" => Self::AustraliaVictoria,
            "Australia/West" => Self::AustraliaWest,
            "Brazil/Acre" => Self::BrazilAcre,
            "Brazil/DeNoronha" => Self::BrazilDenoronha,
            "Brazil/East" => Self::BrazilEast,
            "Brazil/West" => Self::BrazilWest,
            "CET" => Self::Cet,
            "CST6CDT" => Self::Cst6Cdt,
            "Canada/Atlantic" => Self::CanadaAtlantic,
            "Canada/Central" => Self::CanadaCentral,
            "Canada/Eastern" => Self::CanadaEastern,
            "Canada/Mountain" => Self::CanadaMountain,
            "Canada/Pacific" => Self::CanadaPacific,
            "Canada/Saskatchewan" => Self::CanadaSaskatchewan,
            "Canada/Yukon" => Self::CanadaYukon,
            "Chile/Continental" => Self::ChileContinental,
            "Chile/Easterlsland" => Self::ChileEasterlsland,
            "EET" => Self::Eet,
            "EST5EDT" => Self::Est5Edt,
            "Egypt" => Self::Egypt,
            "Eire" => Self::Eire,
            "Europe/Amsterdam" => Self::EuropeAmsterdam,
            "Europe/Andorra" => Self::EuropeAndorra,
            "Europe/Astrakhan" => Self::EuropeAstrakhan,
            "Europe/Athens" => Self::EuropeAthens,
            "Europe/Belfast" => Self::EuropeBelfast,
            "Europe/Belgrade" => Self::EuropeBelgrade,
            "Europe/Berlin" => Self::EuropeBerlin,
            "Europe/Bratislava" => Self::EuropeBratislava,
            "Europe/Brussels" => Self::EuropeBrussels,
            "Europe/Bucharest" => Self::EuropeBucharest,
            "Europe/Budapest" => Self::EuropeBudapest,
            "Europe/Busingen" => Self::EuropeBusingen,
            "Europe/Chisinau" => Self::EuropeChisinau,
            "Europe/Copenhagen" => Self::EuropeCopenhagen,
            "Europe/Dublin" => Self::EuropeDublin,
            "Europe/Gibraltar" => Self::EuropeGibraltar,
            "Europe/Guernsey" => Self::EuropeGuernsey,
            "Europe/Helsinki" => Self::EuropeHelsinki,
            "Europe/Isle of Man" => Self::EuropeIsleOfMan,
            "Europe/Istanbul" => Self::EuropeIstanbul,
            "Europe/Jersey" => Self::EuropeJersey,
            "Europe/Kaliningrad" => Self::EuropeKaliningrad,
            "Europe/Kiev" => Self::EuropeKiev,
            "Europe/Kirov" => Self::EuropeKirov,
            "Europe/Kyiv" => Self::EuropeKyiv,
            "Europe/Lisbon" => Self::EuropeLisbon,
            "Europe/Ljubljana" => Self::EuropeLjubljana,
            "Europe/London" => Self::EuropeLondon,
            "Europe/Luxembourg" => Self::EuropeLuxembourg,
            "Europe/Madrid" => Self::EuropeMadrid,
            "Europe/Malta" => Self::EuropeMalta,
            "Europe/Mariehamn" => Self::EuropeMariehamn,
            "Europe/Minsk" => Self::EuropeMinsk,
            "Europe/Monaco" => Self::EuropeMonaco,
            "Europe/Moscow" => Self::EuropeMoscow,
            "Europe/Nicosia" => Self::EuropeNicosia,
            "Europe/Oslo" => Self::EuropeOslo,
            "Europe/Paris" => Self::EuropeParis,
            "Europe/Podgorica" => Self::EuropePodgorica,
            "Europe/Prague" => Self::EuropePrague,
            "Europe/Riga" => Self::EuropeRiga,
            "Europe/Rome" => Self::EuropeRome,
            "Europe/Samara" => Self::EuropeSamara,
            "Europe/San Marino" => Self::EuropeSanMarino,
            "Europe/Sarajevo" => Self::EuropeSarajevo,
            "Europe/Saratov" => Self::EuropeSaratov,
            "Europe/Simferopol" => Self::EuropeSimferopol,
            "Europe/Skopje" => Self::EuropeSkopje,
            "Europe/Sofia" => Self::EuropeSofia,
            "Europe/Stockholm" => Self::EuropeStockholm,
            "Europe/Tallinn" => Self::EuropeTallinn,
            "Europe/Tirane" => Self::EuropeTirane,
            "Europe/Ulyanovsk" => Self::EuropeUlyanovsk,
            "Europe/Uzhgorod" => Self::EuropeUzhgorod,
            "Europe/Vaduz" => Self::EuropeVaduz,
            "Europe/Vatican" => Self::EuropeVatican,
            "Europe/Vienna" => Self::EuropeVienna,
            "Europe/Vilnius" => Self::EuropeVilnius,
            "Europe/Volgograd" => Self::EuropeVolgograd,
            "Europe/Warsaw" => Self::EuropeWarsaw,
            "Europe/Zagreb" => Self::EuropeZagreb,
            "Europe/Zaporozhye" => Self::EuropeZaporozhye,
            "Europe/Zurich" => Self::EuropeZurich,
            "GB" => Self::Gb,
            "GB-Eire" => Self::GbEire,
            "Greenwich" => Self::Greenwich,
            "Hongkong" => Self::Hongkong,
            "Iceland" => Self::Iceland,
            "Indian/Antananarivo" => Self::IndianAntananarivo,
            "Indian/Chagos" => Self::IndianChagos,
            "Indian/Christmas" => Self::IndianChristmas,
            "Indian/Cocos" => Self::IndianCocos,
            "Indian/Comoro" => Self::IndianComoro,
            "Indian/Kerguelen" => Self::IndianKerguelen,
            "Indian/Mahe" => Self::IndianMahe,
            "Indian/Maldives" => Self::IndianMaldives,
            "Indian/Mauritius" => Self::IndianMauritius,
            "Indian/Mayotte" => Self::IndianMayotte,
            "Indian/Reunion" => Self::IndianReunion,
            "Israel" => Self::Israel,
            "Jamaica" => Self::Jamaica,
            "Japan" => Self::Japan,
            "Kwajalein" => Self::Kwajalein,
            "Libya" => Self::Libya,
            "MET" => Self::Met,
            "MST7MDT" => Self::Mst7Mdt,
            "Mexico/BajaNorte" => Self::MexicoBajanorte,
            "Mexico/General" => Self::MexicoGeneral,
            "NZ" => Self::Nz,
            "NZ-CHAT" => Self::NzChat,
            "Navajo" => Self::Navajo,
            "PRC" => Self::Prc,
            "PST8PDT" => Self::Pst8Pdt,
            "Pacific/Apia" => Self::PacificApia,
            "Pacific/Auckland" => Self::PacificAuckland,
            "Pacific/Bougainville" => Self::PacificBougainville,
            "Pacific/Chatham" => Self::PacificChatham,
            "Pacific/Chuuk" => Self::PacificChuuk,
            "Pacific/Easter" => Self::PacificEaster,
            "Pacific/Efate" => Self::PacificEfate,
            "Pacific/Enderbury" => Self::PacificEnderbury,
            "Pacific/Fiji" => Self::PacificFiji,
            "Pacific/Funafuti" => Self::PacificFunafuti,
            "Pacific/Galapagos" => Self::PacificGalapagos,
            "Pacific/Gambier" => Self::PacificGambier,
            "Pacific/Guadalcanal" => Self::PacificGuadalcanal,
            "Pacific/Guam" => Self::PacificGuam,
            "Pacific/Honolulu" => Self::PacificHonolulu,
            "Pacific/Johnston" => Self::PacificJohnston,
            "Pacific/Kanton" => Self::PacificKanton,
            "Pacific/Kiritimati" => Self::PacificKiritimati,
            "Pacific/Kosrae" => Self::PacificKosrae,
            "Pacific/Kwajalein" => Self::PacificKwajalein,
            "Pacific/Majuro" => Self::PacificMajuro,
            "Pacific/Marquesas" => Self::PacificMarquesas,
            "Pacific/Midway" => Self::PacificMidway,
            "Pacific/Nauru" => Self::PacificNauru,
            "Pacific/Niue" => Self::PacificNiue,
            "Pacific/Norfolk" => Self::PacificNorfolk,
            "Pacific/Noumea" => Self::PacificNoumea,
            "Pacific/Pago Pago" => Self::PacificPagoPago,
            "Pacific/Palau" => Self::PacificPalau,
            "Pacific/Pitcairn" => Self::PacificPitcairn,
            "Pacific/Pohnpei" => Self::PacificPohnpei,
            "Pacific/Port Moresby" => Self::PacificPortMoresby,
            "Pacific/Rarotonga" => Self::PacificRarotonga,
            "Pacific/Saipan" => Self::PacificSaipan,
            "Pacific/Samoa" => Self::PacificSamoa,
            "Pacific/Tahiti" => Self::PacificTahiti,
            "Pacific/Tarawa" => Self::PacificTarawa,
            "Pacific/Tongatapu" => Self::PacificTongatapu,
            "Pacific/Wallis" => Self::PacificWallis,
            "Pacific/Yap" => Self::PacificYap,
            "Poland" => Self::Poland,
            "Portugal" => Self::Portugal,
            "ROK" => Self::Rok,
            "Singapore" => Self::Singapore,
            "Turkey" => Self::Turkey,
            "UCT" => Self::Uct,
            "US/Alaska" => Self::UsAlaska,
            "US/Aleutian" => Self::UsAleutian,
            "US/Arizona" => Self::UsArizona,
            "US/Central" => Self::UsCentral,
            "US/East-Indiana" => Self::UsEastIndiana,
            "US/Eastern" => Self::UsEastern,
            "US/Hawail" => Self::UsHawail,
            "US/Mountain" => Self::UsMountain,
            "US/Pacific" => Self::UsPacific,
            "US/Samoa" => Self::UsSamoa,
            "Universal" => Self::Universal,
            "W-SU" => Self::WSu,
            "WET" => Self::Wet,
            "Zulu" => Self::Zulu,
            "EST" => Self::Est,
            "HST" => Self::Hst,
            "MST" => Self::Mst,
            s => return Err(InvalidTimeZone(s.to_owned())),
        })
    }
}

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
/// An error from attempting to parse an ['IbTimeZone']
pub struct InvalidTimeZone(String);

impl std::fmt::Display for InvalidTimeZone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid time zone encountered: {}", self.0)
    }
}

impl std::error::Error for InvalidTimeZone {}

// let FixedOffset { local_minus_utc } = offset;
// match local_minus_utc {
//     0 => Self::Universal,
//     -12600 => Self::AmericaStJohns,
//     39600 => Self::AsiaMagadan,
//     10800 => Self::AsiaAden,
//     28800 => Self::AsiaBrunei,
//     -28800 => Self::UsPacific,
//     -39600 => Self::UsSamoa,
//     31500 => Self::AustraliaEucla,
//     46800 => Self::PacificApia,
//     25200 => Self::AsiaBangkok,
//     19800 => Self::AsiaCalcutta,
//     43200 => Self::AsiaAnadyr,
//     20700 => Self::AsiaKathmandu,
//     7200 => Self::AsiaAmman,
//     -25200 => Self::UsMountain,
//     23400 => Self::AsiaRangoon,
//     12600 => Self::AsiaTehran,
//     16200 => Self::AsiaKabul,
//     32400 => Self::AsiaChita,
//     21600 => Self::AsiaAlmaty,
//     -18000 => Self::UsEastern,
//     -36000 => Self::AmericaAdak,
//     36000 => Self::AustraliaBrisbane,
//     -32400 => Self::UsAlaska,
//     45900 => Self::PacificChatham,
//     34200 => Self::AustraliaAdelaide,
//     -34200 => Self::PacificMarquesas,
//     18000 => Self::AsiaAqtau,
//     37800 => Self::AustraliaLhi,
//     -10800 => Self::AmericaSaoPaulo,
//     -7200 => Self::BrazilDenoronha,
//     50400 => Self::PacificKiritimati,
//     -3600 => Self::AtlanticAzores,
//     14400 => Self::AsiaDubai,
//     3600 => Self::EuropeAmsterdam,
//     -21600 => Self::UsCentral,
//     -14400 => Self::AmericaAntigua,
//     _ => Self::Unrecognized(*offset),
// }






// (UTC 00:00) Africa/Abidjan
// (UTC 00:00) Africa/Accra
// (UTC+03:00) Africa/Addis Ababa
// (UTC+01:00) Africa/Algiers
// (UTC+03:00) Africa/Asmara
// (UTC+03:00) Africa/Asmera
// (UTC 00:00) Africa/Bamako
// (UTC+01:00) Africa/Bangui
// (UTC 00:00) Africa/Banjul
// (UTC 00:00) Africa/Bissau
// (UTC+02:00) Africa/Blantyre
// (UTC+01:00) Africa/Brazzaville
// (UTC+02:00) Africa/Bujumbura
// (UTC+02:00) Africa/Cairo
// (UTC 00:00) Africa/Casablanca
// (UTC+01:00) Africa/Ceuta
// (UTC 00:00) Africa/Conakry
// (UTC 00:00) Africa/Dakar
// (UTC+03:00) Africa/Dar es Salaam
// (UTC+03:00) Africa/Djibouti
// (UTC+01:00) Africa/Douala
// (UTC 00:00) Africa/El Aaiun
// (UTC 00:00) Africa/Freetown
// (UTC+02:00) Africa/Gaborone
// (UTC+02:00) Africa/Harare
// (UTC+02:00) Africa/Johannesburg
// (UTC+02:00) Africa/Juba
// (UTC+03:00) Africa/Kampala
// (UTC+02:00) Africa/Khartoum
// (UTC+02:00) Africa/Kigali
// (UTC+01:00) Africa/Kinshasa
// (UTC+01:00) Africa/Lagos
// (UTC+01:00) Africa/Libreville
// (UTC 00:00) Africa/Lome
// (UTC+01:00) Africa/Luanda
// (UTC+02:00) Africa/Lubumbashi
// (UTC+02:00) Africa/Lusaka
// (UTC+01:00) Africa/Malabo
// (UTC+02:00) Africa/Maputo
// (UTC+02:00) Africa/Maseru
// (UTC+02:00) Africa/Mbabane
// (UTC+03:00) Africa/Mogadishu
// (UTC 00:00) Africa/Monrovia
// (UTC+03:00) Africa/Nairobi
// (UTC+01:00) Africa/Ndjamena
// (UTC+01:00) Africa/Niamey
// (UTC 00:00) Africa/Nouakchott
// (UTC 00:00) Africa/Ouagadougou
// (UTC+01:00) Africa/Porto-Novo
// (UTC 00:00) Africa/Sao Tome
// (UTC 00:00) Africa/Timbuktu
// (UTC+02:00) Africa/Tripoli
// (UTC+01:00) Africa/Tunis
// (UTC+01:00) Africa/Windhoek
// (UTC-10:00) America/Adak
// (UTC-09:00) America/Anchorage
// (UTC-04:00) America/Anguilla
// (UTC-04:00) America/Antigua
// (UTC-03:00) America/Araguaina
// (UTC-03:00) America/Argentina/Buenos Aires
// (UTC-03:00) America/Argentina/Catamarca
// (UTC-03:00) America/Argentina/Cordoba
// (UTC-03:00) America/Argentina/Jujuy
// (UTC-03:00) America/Argentina/La Rioja
// (UTC-03:00) America/Argentina/Mendoza
// (UTC-03:00) America/Argentina/Rio Gallegos
// (UTC-03:00) America/Argentina/Salta
// (UTC-03:00) America/Argentina/San Juan
// (UTC-03:00) America/Argentina/San Luis
// (UTC-03:00) America/Argentina/Tucuman
// (UTC-03:00) America/Argentina/Ushuaia
// (UTC-04:00) America/Aruba
// (UTC-04:00) America/Asuncion
// (UTC-05:00) America/Atikokan
// (UTC-10:00) America/Atka
// (UTC-03:00) America/Bahia
// (UTC-06:00) America/Bahia Banderas
// (UTC-04:00) America/Barbados
// (UTC-03:00) America/Belem
// (UTC-06:00) America/Belize
// (UTC-04:00) America/Blanc-Sablon
// (UTC-04:00) America/Boa Vista
// (UTC-05:00) America/Bogota
// (UTC-07:00) America/Boise
// (UTC-03:00) America/Buenos Aires
// (UTC-07:00) America/Cambridge Bay
// (UTC-04:00) America/Campo Grande
// (UTC-05:00) America/Cancun
// (UTC-04:00) America/Caracas
// (UTC-03:00) America/Cayenne
// (UTC-05:00) America/Cayman
// (UTC-06:00) America/Chicago
// (UTC-07:00) America/Chihuahua
// (UTC-05:00) America/Coral Harbour
// (UTC-03:00) America/Cordoba
// (UTC-06:00) America/Costa Rica
// (UTC-07:00) America/Creston
// (UTC-04:00) America/Cuiaba
// (UTC-04:00) America/Curacao
// (UTC 00:00) America/Danmarkshavn
// (UTC-07:00) America/Dawson
// (UTC-07:00) America/Dawson Creek
// (UTC-07:00) America/Denver
// (UTC-05:00) America/Detroit
// (UTC-04:00) America/Dominica
// (UTC-07:00) America/Edmonton
// (UTC-05:00) America/Eirunepe
// (UTC-06:00) America/El Salvador
// (UTC-08:00) America/Ensenada
// (UTC-07:00) America/Fort Nelson
// (UTC-05:00) America/Fort Wayne
// (UTC-03:00) America/Fortaleza
// (UTC-04:00) America/Glace Bay
// (UTC-03:00) America/Godthab
// (UTC-04:00) America/Goose Bay
// (UTC-05:00) America/Grand Turk
// (UTC-04:00) America/Grenada
// (UTC-04:00) America/Guadeloupe
// (UTC-06:00) America/Guatemala
// (UTC-05:00) America/Guayaquil
// (UTC-04:00) America/Guyana
// (UTC-04:00) America/Halifax
// (UTC-07:00) America/Hermosillo
// (UTC-05:00) America/Indiana/Indianapolis
// (UTC-05:00) America/Indiana/Marengo
// (UTC-05:00) America/Indiana/Petersburg
// (UTC-06:00) America/Indiana/Tell City
// (UTC-05:00) America/Indiana/Vevay
// (UTC-05:00) America/Indiana/Vincennes
// (UTC-05:00) America/Indiana/Winamac
// (UTC-05:00) America/Indianapolis
// (UTC-07:00) America/Inuvik
// (UTC-05:00) America/Iqaluit
// (UTC-05:00) America/Jamaica
// (UTC-09:00) America/Juneau
// (UTC-05:00) America/Kentucky/Louisville
// (UTC-05:00) America/Kentucky/Monticello
// (UTC-04:00) America/Kralendijk
// (UTC-04:00) America/La Paz
// (UTC-05:00) America/Lima
// (UTC-08:00) America/Los Angeles
// (UTC-05:00) America/Louisville
// (UTC-04:00) America/Lower Princes
// (UTC-03:00) America/Maceio
// (UTC-06:00) America/Managua
// (UTC-04:00) America/Manaus
// (UTC-04:00) America/Marigot
// (UTC-04:00) America/Martinique
// (UTC-06:00) America/Matamoros
// (UTC-07:00) America/Mazatlan
// (UTC-06:00) America/Menominee
// (UTC-06:00) America/Merida
// (UTC-09:00) America/Metlakatla
// (UTC-06:00) America/Mexico City
// (UTC-03:00) America/Miquelon
// (UTC-04:00) America/Moncton
// (UTC-06:00) America/Monterrey
// (UTC-03:00) America/Montevideo
// (UTC-05:00) America/Montreal
// (UTC-04:00) America/Montserrat
// (UTC-05:00) America/Nassau
// (UTC-05:00) America/New York
// (UTC-05:00) America/Nipigon
// (UTC-09:00) America/Nome
// (UTC-02:00) America/Noronha
// (UTC-06:00) America/North Dakota/Beulah
// (UTC-06:00) America/North Dakota/Center
// (UTC-06:00) America/North Dakota/New Salem
// (UTC-03:00) America/Nuuk
// (UTC-07:00) America/Ojinaga
// (UTC-05:00) America/Panama
// (UTC-05:00) America/Pangnirtung
// (UTC-03:00) America/Paramaribo
// (UTC-07:00) America/Phoenix
// (UTC-05:00) America/Port-au-Prince
// (UTC-04:00) America/Port of Spain
// (UTC-05:00) America/Porto Acre
// (UTC-04:00) America/Porto Velho
// (UTC-04:00) America/Puerto Rico
// (UTC-03:00) America/Punta Arenas
// (UTC-06:00) America/Rainy River
// (UTC-06:00) America/Rankin Inlet
// (UTC-03:00) America/Recife
// (UTC-06:00) America/Regina
// (UTC-06:00) America/Resolute
// (UTC-05:00) America/Rio Branco
// (UTC-03:00) America/Rosario
// (UTC-08:00) America/Santa Isabel
// (UTC-03:00) America/Santarem
// (UTC-04:00) America/Santiago
// (UTC-04:00) America/Santo Domingo
// (UTC-03:00) America/Sao Paulo
// (UTC-01:00) America/Scoresbysund
// (UTC-07:00) America/Shiprock
// (UTC-09:00) America/Sitka
// (UTC-04:00) America/St Barthelemy
// (UTC-03:30) America/St Johns
// (UTC-04:00) America/St Kitts
// (UTC-04:00) America/St Lucia
// (UTC-04:00) America/St Thomas
// (UTC-04:00) America/St Vincent
// (UTC-06:00) America/Swift Current
// (UTC-06:00) America/Tegucigalpa
// (UTC-04:00) America/Thule
// (UTC-05:00) America/Thunder Bay
// (UTC-08:00) America/Tijuana
// (UTC-05:00) America/Toronto
// (UTC-04:00) America/Tortola
// (UTC-08:00) America/Vancouver
// (UTC-04:00) America/Virgin
// (UTC-07:00) America/Whitehorse
// (UTC-06:00) America/Winnipeg
// (UTC-09:00) America/Yakutat
// (UTC-07:00) America/Yellowknife
// (UTC+11:00) Antarctica/Casey
// (UTC+07:00) Antarctica/Davis
// (UTC+10:00) Antarctica/DumontDUrville
// (UTC+10:00) Antarctica/Macquarie
// (UTC+05:00) Antarctica/Mawson
// (UTC+12:00) Antarctica/McMurdo
// (UTC-03:00) Antarctica/Palmer
// (UTC-03:00) Antarctica/Rothera
// (UTC+03:00) Antarctica/Syowa
// (UTC+06:00) Antarctica/Vostok
// (UTC+01:00) Arctic/Longyearbyen
// (UTC+03:00) Asia/Aden
// (UTC+06:00) Asia/Almaty
// (UTC+02:00) Asia/Amman
// (UTC+12:00) Asia/Anadyr
// (UTC+05:00) Asia/Aqtau
// (UTC+05:00) Asia/Aqtobe
// (UTC+05:00) Asia/Ashgabat
// (UTC+05:00) Asia/Ashkhabad
// (UTC+05:00) Asia/Atyrau
// (UTC+03:00) Asia/Baghdad
// (UTC+03:00) Asia/Bahrain
// (UTC+04:00) Asia/Baku
// (UTC+07:00) Asia/Bangkok
// (UTC+07:00) Asia/Barnaul
// (UTC+02:00) Asia/Beirut
// (UTC+06:00) Asia/Bishkek
// (UTC+08:00) Asia/Brunei
// (UTC+05:30) Asia/Calcutta
// (UTC+09:00) Asia/Chita
// (UTC+08:00) Asia/Choibalsan
// (UTC+08:00) Asia/Chongqing
// (UTC+08:00) Asia/Chungking
// (UTC+05:30) Asia/Colombo
// (UTC+06:00) Asia/Dacca
// (UTC+02:00) Asia/Damascus
// (UTC+06:00) Asia/Dhaka
// (UTC+09:00) Asia/Dili
// (UTC+04:00) Asia/Dubai
// (UTC+05:00) Asia/Dushanbe
// (UTC+02:00) Asia/Famagusta
// (UTC+02:00) Asia/Gaza
// (UTC+08:00) Asia/Harbin
// (UTC+02:00) Asia/Hebron
// (UTC+07:00) Asia/Ho Chi Minh
// (UTC+08:00) Asia/Hong Kong
// (UTC+07:00) Asia/Hovd
// (UTC+08:00) Asia/Irkutsk
// (UTC+03:00) Asia/Istanbul
// (UTC+07:00) Asia/Jakarta
// (UTC+09:00) Asia/Jayapura
// (UTC+02:00) Asia/Jerusalem
// (UTC+04:30) Asia/Kabul
// (UTC+12:00) Asia/Kamchatka
// (UTC+05:00) Asia/Karachi
// (UTC+06:00) Asia/Kashgar
// (UTC+05:45) Asia/Kathmandu
// (UTC+09:00) Asia/Khandyga
// (UTC+05:30) Asia/Kolkata
// (UTC+07:00) Asia/Krasnoyarsk
// (UTC+08:00) Asia/Kuala Lumpur
// (UTC+08:00) Asia/Kuching
// (UTC+03:00) Asia/Kuwait
// (UTC+08:00) Asia/Macao
// (UTC+08:00) Asia/Macau
// (UTC+11:00) Asia/Magadan
// (UTC+08:00) Asia/Makassar
// (UTC+08:00) Asia/Manila
// (UTC+04:00) Asia/Muscat
// (UTC+02:00) Asia/Nicosia
// (UTC+07:00) Asia/Novokuznetsk
// (UTC+07:00) Asia/Novosibirsk
// (UTC+06:00) Asia/Omsk
// (UTC+05:00) Asia/Oral
// (UTC+07:00) Asia/Phnom Penh
// (UTC+07:00) Asia/Pontianak
// (UTC+09:00) Asia/Pyongyang
// (UTC+03:00) Asia/Qatar
// (UTC+06:00) Asia/Qostanay
// (UTC+05:00) Asia/Qyzylorda
// (UTC+06:30) Asia/Rangoon
// (UTC+03:00) Asia/Riyadh
// (UTC+07:00) Asia/Saigon
// (UTC+11:00) Asia/Sakhalin
// (UTC+05:00) Asia/Samarkand
// (UTC+09:00) Asia/Seoul
// (UTC+08:00) Asia/Shanghai
// (UTC+08:00) Asia/Singapore
// (UTC+11:00) Asia/Srednekolymsk
// (UTC+08:00) Asia/Taipei
// (UTC+05:00) Asia/Tashkent
// (UTC+04:00) Asia Tbilisi
// (UTC+03:30) Asia/Tehran
// (UTC+02:00) Asia/Tel Aviv
// (UTC+06:00) Asia/Thimbu
// (UTC+06:00) Asia/Thimphu
// (UTC+09:00) Asia/Tokyo
// (UTC+07:00) Asia/Tomsk
// (UTC+08:00) Asia/Ujung Pandang
// (UTC+08:00) Asia/Ulaanbaatar
// (UTC+06:00) Asia/Urumqi
// (UTC+10:00) Asia/Ust-Nera
// (UTC+07:00) Asia/Vientiane
// (UTC+10:00) Asia/Vladivostok
// (UTC+06:30) Asia/Yangon
// (UTC+05:00) Asia/Yekaterinburg
// (UTC+04:00) Asia/Yerevan
// (UTC-01:00) Atlantic/Azores
// (UTC-04:00) Atlantic/Bermuda
// (UTC 00:00) Atlantic/Canary
// (UTC-01:00) Atlantic/Cape Verde
// (UTC 00:00) Atlantic/Faeroe
// (UTC 00:00) Atlantic/Faroe
// (UTC+01:00) Atlantic/Jan Mayen
// (UTC 00:00) Atlantic/Madeira
// (UTC 00:00) Atlantic/Reykjavik
// (UTC-02:00) Atlantic/South Georgia
// (UTC 00:00) Atlantic/St Helena
// (UTC-03:00) Atlantic/Stanley
// (UTC+10:00) Australia/ACT
// (UTC+09:30) Australia/Adelaide
// (UTC+10:00) Australia/Brisbane
// (UTC+09:30) Australia/Broken Hill
// (UTC+10:00) Australia/Canberra
// (UTC+10:00) Australia/Currie
// (UTC+09:30) Australia/Darwin
// (UTC+08:45) Australia/Eucla
// (UTC+10:00) Australia/Hobart
// (UTC+10:30) Australia/LHI
// (UTC+10:00) Australia/Lindeman
// (UTC+10:30) Australia/Lord Howe
// (UTC+10:00) Australia/Melbourne
// (UTC+10:00) Australia/NSW
// (UTC+09:30) Australia/North
// (UTC+08:00) Australia/Perth
// (UTC+10:00) Australia/Queensland
// (UTC+09:30) Australia/South
// (UTC+10:00) Australia/Sydney
// (UTC+10:00) Australia/Tasmania
// (UTC+10:00) Australia/Victoria
// (UTC+08:00) Australia/West
// (UTC-05:00) Brazil/Acre
// (UTC-02:00) Brazil/DeNoronha
// (UTC-03:00) Brazil/East
// (UTC-04:00) Brazil/West
// (UTC+01:00) CET
// (UTC-06:00) CST6CDT
// (UTC-04:00) Canada/Atlantic
// (UTC-06:00) Canada/Central
// (UTC-05:00) Canada/Eastern
// (UTC-07:00) Canada/Mountain
// (UTC-08:00) Canada/Pacific
// (UTC-06:00) Canada/Saskatchewan
// (UTC-07:00) Canada/Yukon
// (UTC-04:00) Chile/Continental
// (UTC-06:00) Chile/Easterlsland
// (UTC+02:00) EET
// (UTC-05:00) EST5EDT
// (UTC+02:00) Egypt
// (UTC 00:00) Eire
// (UTC+01:00) Europe/Amsterdam
// (UTC+01:00) Europe/Andorra
// (UTC+04:00) Europe/Astrakhan
// (UTC+02:00) Europe/Athens
// (UTC 00:00) Europe/Belfast
// (UTC+01:00) Europe/Belgrade
// (UTC+01:00) Europe/Berlin
// (UTC+01:00) Europe/Bratislava
// (UTC+01:00) Europe/Brussels
// (UTC+02:00) Europe/Bucharest
// (UTC+01:00) Europe/Budapest
// (UTC+01:00) Europe/Busingen
// (UTC+02:00) Europe/Chisinau
// (UTC+01:00) Europe/Copenhagen
// (UTC 00:00) Europe/Dublin
// (UTC+01:00) Europe/Gibraltar
// (UTC 00:00) Europe/Guernsey
// (UTC+02:00) Europe/Helsinki
// (UTC 00:00) Europe/Isle of Man
// (UTC+03:00) Europe/Istanbul
// (UTC 00:00) Europe/Jersey
// (UTC+02:00) Europe/Kaliningrad
// (UTC+02:00) Europe/Kiev
// (UTC+03:00) Europe/Kirov
// (UTC+02:00) Europe/Kyiv
// (UTC 00:00) Europe/Lisbon
// (UTC+01:00) Europe/Ljubljana
// (UTC 00:00) Europe/London
// (UTC+01:00) Europe/Luxembourg
// (UTC+01:00) Europe/Madrid
// (UTC+01:00) Europe/Malta
// (UTC+02:00) Europe/Mariehamn
// (UTC+03:00) Europe/Minsk
// (UTC+01:00) Europe/Monaco
// (UTC+03:00) Europe/Moscow
// (UTC+02:00) Europe/Nicosia
// (UTC+01:00) Europe/Oslo
// (UTC+01:00) Europe/Paris
// (UTC+01:00) Europe/Podgorica
// (UTC+01:00) Europe/Prague
// (UTC+02:00) Europe/Riga
// (UTC+01:00) Europe/Rome
// (UTC+04:00) Europe/Samara
// (UTC+01:00) Europe/San Marino
// (UTC+01:00) Europe/Sarajevo
// (UTC+04:00) Europe/Saratov
// (UTC+03:00) Europe/Simferopol
// (UTC+01:00) Europe/Skopje
// (UTC+02:00) Europe/Sofia
// (UTC+01:00) Europe/Stockholm
// (UTC+02:00) Europe/Tallinn
// (UTC+01:00) Europe/Tirane
// (UTC+04:00) Europe/Ulyanovsk
// (UTC+02:00) Europe/Uzhgorod
// (UTC+01:00) Europe/Vaduz
// (UTC+01:00) Europe/Vatican
// (UTC+01:00) Europe/Vienna
// (UTC+02:00) Europe/Vilnius
// (UTC+03:00) Europe/Volgograd
// (UTC+01:00) Europe/Warsaw
// (UTC+01:00) Europe/Zagreb
// (UTC+02:00) Europe/Zaporozhye
// (UTC+01:00) Europe/Zurich
// (UTC 00:00) GB
// (UTC 00:00) GB-Eire
// (UTC 00:00) Greenwich
// (UTC+08:00) Hongkong
// (UTC 00:00) Iceland
// (UTC+03:00) Indian/Antananarivo
// (UTC+06:00) Indian/Chagos
// (UTC+07:00) Indian/Christmas
// (UTC+06:30) Indian/Cocos
// (UTC+03:00) Indian/Comoro
// (UTC+05:00) Indian/Kerguelen
// (UTC+04:00) Indian/Mahe
// (UTC+05:00) Indian/Maldives
// (UTC+04:00) Indian/Mauritius
// (UTC+03:00) Indian/Mayotte
// (UTC+04:00) Indian/Reunion
// (UTC+02:00) Israel
// (UTC-05:00) Jamaica
// (UTC+09:00) Japan
// (UTC+12:00) Kwajalein
// (UTC+02:00) Libya
// (UTC+01:00) MET
// (UTC-07:00) MST7MDT
// (UTC-08:00) Mexico/BajaNorte
// (UTC-06:00) Mexico/General
// (UTC+12:00) NZ
// (UTC+12:45) NZ-CHAT
// (UTC-07:00) Navajo
// (UTC+08:00) PRC
// (UTC-08:00) PST8PDT
// (UTC+13:00) Pacific/Apia
// (UTC+12:00) Pacific/Auckland
// (UTC+11:00) Pacific/Bougainville
// (UTC+12:45) Pacific/Chatham
// (UTC+10:00) Pacific/Chuuk
// (UTC-06:00) Pacific/Easter
// (UTC+11:00) Pacific/Efate
// (UTC+13:00) Pacific/Enderbury
// (UTC+12:00) Pacific/Fiji
// (UTC+12:00) Pacific/Funafuti
// (UTC-06:00) Pacific/Galapagos
// (UTC-09:00) Pacific/Gambier
// (UTC+11:00) Pacific/Guadalcanal
// (UTC+10:00) Pacific/Guam
// (UTC-10:00) Pacific/Honolulu
// (UTC-10:00) Pacific/Johnston
// (UTC+13:00) Pacific/Kanton
// (UTC+14:00) Pacific/Kiritimati
// (UTC+11:00) Pacific/Kosrae
// (UTC+12:00) Pacific/Kwajalein
// (UTC+12:00) Pacific/Majuro
// (UTC-09:30) Pacific/Marquesas
// (UTC-11:00) Pacific/Midway
// (UTC+12:00) Pacific/Nauru
// (UTC-11:00) Pacific/Niue
// (UTC+11:00) Pacific/Norfolk
// (UTC+11:00) Pacific/Noumea
// (UTC-11:00) Pacific/Pago Pago
// (UTC+09:00) Pacific/Palau
// (UTC-08:00) Pacific/Pitcairn
// (UTC+11:00) Pacific/Pohnpei
// (UTC+10:00) Pacific/Port Moresby
// (UTC-10:00) Pacific/Rarotonga
// (UTC+10:00) Pacific/Saipan
// (UTC-11:00) Pacific/Samoa
// (UTC-10:00) Pacific/Tahiti
// (UTC+12:00) Pacific/Tarawa
// (UTC+13:00) Pacific/Tongatapu
// (UTC+12:00) Pacific/Wallis
// (UTC+10:00) Pacific/Yap
// (UTC+01:00) Poland
// (UTC 00:00) Portugal
// (UTC+09:00) ROK
// (UTC+08:00) Singapore
// (UTC+03:00) Turkey
// (UTC 00:00) UCT
// (UTC-09:00) US/Alaska
// (UTC-10:00) US/Aleutian
// (UTC-07:00) US/Arizona
// (UTC-06:00) US/Central
// (UTC-05:00) US/East-Indiana
// (UTC-05:00) US/Eastern
// (UTC-10:00) US/Hawail
// (UTC-07:00) US/Mountain
// (UTC-08:00) US/Pacific
// (UTC-11:00) US/Samoa
// (UTC 00:00) Universal
// (UTC+03:00) W-SU
// (UTC 00:00) WET
// (UTC 00:00) Zulu
// (UTC-05:00) EST
// (UTC-10:00) HST
// (UTC-07:00) MST
