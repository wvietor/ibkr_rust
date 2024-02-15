use chrono::{FixedOffset, LocalResult, NaiveDate, NaiveDateTime, Offset, ParseError, TimeZone};

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
    #[allow(clippy::unwrap_used, clippy::too_many_lines)]
    fn fix(&self) -> FixedOffset {
        match self {
            Self::AfricaAbidjan
            | Self::AfricaAccra
            | Self::AfricaBamako
            | Self::AfricaBanjul
            | Self::AfricaBissau
            | Self::AfricaCasablanca
            | Self::AfricaConakry
            | Self::AfricaDakar
            | Self::AfricaElAaiun
            | Self::AfricaFreetown
            | Self::AfricaLome
            | Self::AfricaMonrovia
            | Self::AfricaNouakchott
            | Self::AfricaOuagadougou
            | Self::AfricaSaoTome
            | Self::AfricaTimbuktu
            | Self::AmericaDanmarkshavn
            | Self::AtlanticCanary
            | Self::AtlanticFaeroe
            | Self::AtlanticFaroe
            | Self::AtlanticMadeira
            | Self::AtlanticReykjavik
            | Self::AtlanticStHelena
            | Self::Eire
            | Self::EuropeBelfast
            | Self::EuropeDublin
            | Self::EuropeGuernsey
            | Self::EuropeIsleOfMan
            | Self::EuropeJersey
            | Self::EuropeLisbon
            | Self::EuropeLondon
            | Self::Gb
            | Self::GbEire
            | Self::Greenwich
            | Self::Iceland
            | Self::Portugal
            | Self::Uct
            | Self::Universal
            | Self::Wet
            | Self::Zulu => FixedOffset::east_opt(0),
            Self::AfricaAddisAbaba
            | Self::AfricaAsmara
            | Self::AfricaAsmera
            | Self::AfricaDarEsSalaam
            | Self::AfricaDjibouti
            | Self::AfricaKampala
            | Self::AfricaMogadishu
            | Self::AfricaNairobi
            | Self::AntarcticaSyowa
            | Self::AsiaAden
            | Self::AsiaBaghdad
            | Self::AsiaBahrain
            | Self::AsiaIstanbul
            | Self::AsiaKuwait
            | Self::AsiaQatar
            | Self::AsiaRiyadh
            | Self::EuropeIstanbul
            | Self::EuropeKirov
            | Self::EuropeMinsk
            | Self::EuropeMoscow
            | Self::EuropeSimferopol
            | Self::EuropeVolgograd
            | Self::IndianAntananarivo
            | Self::IndianComoro
            | Self::IndianMayotte
            | Self::Turkey
            | Self::WSu => FixedOffset::east_opt(10800),
            Self::AfricaAlgiers
            | Self::AfricaBangui
            | Self::AfricaBrazzaville
            | Self::AfricaCeuta
            | Self::AfricaDouala
            | Self::AfricaKinshasa
            | Self::AfricaLagos
            | Self::AfricaLibreville
            | Self::AfricaLuanda
            | Self::AfricaMalabo
            | Self::AfricaNdjamena
            | Self::AfricaNiamey
            | Self::AfricaPortoNovo
            | Self::AfricaTunis
            | Self::AfricaWindhoek
            | Self::ArcticLongyearbyen
            | Self::AtlanticJanMayen
            | Self::Cet
            | Self::EuropeAmsterdam
            | Self::EuropeAndorra
            | Self::EuropeBelgrade
            | Self::EuropeBerlin
            | Self::EuropeBratislava
            | Self::EuropeBrussels
            | Self::EuropeBudapest
            | Self::EuropeBusingen
            | Self::EuropeCopenhagen
            | Self::EuropeGibraltar
            | Self::EuropeLjubljana
            | Self::EuropeLuxembourg
            | Self::EuropeMadrid
            | Self::EuropeMalta
            | Self::EuropeMonaco
            | Self::EuropeOslo
            | Self::EuropeParis
            | Self::EuropePodgorica
            | Self::EuropePrague
            | Self::EuropeRome
            | Self::EuropeSanMarino
            | Self::EuropeSarajevo
            | Self::EuropeSkopje
            | Self::EuropeStockholm
            | Self::EuropeTirane
            | Self::EuropeVaduz
            | Self::EuropeVatican
            | Self::EuropeVienna
            | Self::EuropeWarsaw
            | Self::EuropeZagreb
            | Self::EuropeZurich
            | Self::Met
            | Self::Poland => FixedOffset::east_opt(3600),
            Self::AfricaBlantyre
            | Self::AfricaBujumbura
            | Self::AfricaCairo
            | Self::AfricaGaborone
            | Self::AfricaHarare
            | Self::AfricaJohannesburg
            | Self::AfricaJuba
            | Self::AfricaKhartoum
            | Self::AfricaKigali
            | Self::AfricaLubumbashi
            | Self::AfricaLusaka
            | Self::AfricaMaputo
            | Self::AfricaMaseru
            | Self::AfricaMbabane
            | Self::AfricaTripoli
            | Self::AsiaAmman
            | Self::AsiaBeirut
            | Self::AsiaDamascus
            | Self::AsiaFamagusta
            | Self::AsiaGaza
            | Self::AsiaHebron
            | Self::AsiaJerusalem
            | Self::AsiaNicosia
            | Self::AsiaTelAviv
            | Self::Eet
            | Self::Egypt
            | Self::EuropeAthens
            | Self::EuropeBucharest
            | Self::EuropeChisinau
            | Self::EuropeHelsinki
            | Self::EuropeKaliningrad
            | Self::EuropeKiev
            | Self::EuropeKyiv
            | Self::EuropeMariehamn
            | Self::EuropeNicosia
            | Self::EuropeRiga
            | Self::EuropeSofia
            | Self::EuropeTallinn
            | Self::EuropeUzhgorod
            | Self::EuropeVilnius
            | Self::EuropeZaporozhye
            | Self::Israel
            | Self::Libya => FixedOffset::east_opt(7200),
            Self::AmericaAdak
            | Self::AmericaAtka
            | Self::PacificHonolulu
            | Self::PacificJohnston
            | Self::PacificRarotonga
            | Self::PacificTahiti
            | Self::UsAleutian
            | Self::UsHawail
            | Self::Hst => FixedOffset::east_opt(-36000),
            Self::AmericaAnchorage
            | Self::AmericaJuneau
            | Self::AmericaMetlakatla
            | Self::AmericaNome
            | Self::AmericaSitka
            | Self::AmericaYakutat
            | Self::PacificGambier
            | Self::UsAlaska => FixedOffset::east_opt(-32400),
            Self::AmericaAnguilla
            | Self::AmericaAntigua
            | Self::AmericaAruba
            | Self::AmericaAsuncion
            | Self::AmericaBarbados
            | Self::AmericaBlancSablon
            | Self::AmericaBoaVista
            | Self::AmericaCampoGrande
            | Self::AmericaCaracas
            | Self::AmericaCuiaba
            | Self::AmericaCuracao
            | Self::AmericaDominica
            | Self::AmericaGlaceBay
            | Self::AmericaGooseBay
            | Self::AmericaGrenada
            | Self::AmericaGuadeloupe
            | Self::AmericaGuyana
            | Self::AmericaHalifax
            | Self::AmericaKralendijk
            | Self::AmericaLaPaz
            | Self::AmericaLowerPrinces
            | Self::AmericaManaus
            | Self::AmericaMarigot
            | Self::AmericaMartinique
            | Self::AmericaMoncton
            | Self::AmericaMontserrat
            | Self::AmericaPortOfSpain
            | Self::AmericaPortoVelho
            | Self::AmericaPuertoRico
            | Self::AmericaSantiago
            | Self::AmericaSantoDomingo
            | Self::AmericaStBarthelemy
            | Self::AmericaStKitts
            | Self::AmericaStLucia
            | Self::AmericaStThomas
            | Self::AmericaStVincent
            | Self::AmericaThule
            | Self::AmericaTortola
            | Self::AmericaVirgin
            | Self::AtlanticBermuda
            | Self::BrazilWest
            | Self::CanadaAtlantic
            | Self::ChileContinental => FixedOffset::east_opt(-14400),
            Self::AmericaAraguaina
            | Self::AmericaArgentinaBuenosAires
            | Self::AmericaArgentinaCatamarca
            | Self::AmericaArgentinaCordoba
            | Self::AmericaArgentinaJujuy
            | Self::AmericaArgentinaLaRioja
            | Self::AmericaArgentinaMendoza
            | Self::AmericaArgentinaRioGallegos
            | Self::AmericaArgentinaSalta
            | Self::AmericaArgentinaSanJuan
            | Self::AmericaArgentinaSanLuis
            | Self::AmericaArgentinaTucuman
            | Self::AmericaArgentinaUshuaia
            | Self::AmericaBahia
            | Self::AmericaBelem
            | Self::AmericaBuenosAires
            | Self::AmericaCayenne
            | Self::AmericaCordoba
            | Self::AmericaFortaleza
            | Self::AmericaGodthab
            | Self::AmericaMaceio
            | Self::AmericaMiquelon
            | Self::AmericaMontevideo
            | Self::AmericaNuuk
            | Self::AmericaParamaribo
            | Self::AmericaPuntaArenas
            | Self::AmericaRecife
            | Self::AmericaRosario
            | Self::AmericaSantarem
            | Self::AmericaSaoPaulo
            | Self::AntarcticaPalmer
            | Self::AntarcticaRothera
            | Self::AtlanticStanley
            | Self::BrazilEast => FixedOffset::east_opt(-10800),
            Self::AmericaAtikokan
            | Self::AmericaBogota
            | Self::AmericaCancun
            | Self::AmericaCayman
            | Self::AmericaCoralHarbour
            | Self::AmericaDetroit
            | Self::AmericaEirunepe
            | Self::AmericaFortWayne
            | Self::AmericaGrandTurk
            | Self::AmericaGuayaquil
            | Self::AmericaIndianaIndianapolis
            | Self::AmericaIndianaMarengo
            | Self::AmericaIndianaPetersburg
            | Self::AmericaIndianaVevay
            | Self::AmericaIndianaVincennes
            | Self::AmericaIndianaWinamac
            | Self::AmericaIndianapolis
            | Self::AmericaIqaluit
            | Self::AmericaJamaica
            | Self::AmericaKentuckyLouisville
            | Self::AmericaKentuckyMonticello
            | Self::AmericaLima
            | Self::AmericaLouisville
            | Self::AmericaMontreal
            | Self::AmericaNassau
            | Self::AmericaNewYork
            | Self::AmericaNipigon
            | Self::AmericaPanama
            | Self::AmericaPangnirtung
            | Self::AmericaPortAuPrince
            | Self::AmericaPortoAcre
            | Self::AmericaRioBranco
            | Self::AmericaThunderBay
            | Self::AmericaToronto
            | Self::BrazilAcre
            | Self::CanadaEastern
            | Self::Est5Edt
            | Self::Jamaica
            | Self::UsEastIndiana
            | Self::UsEastern
            | Self::Est => FixedOffset::east_opt(-18000),
            Self::AmericaBahiaBanderas
            | Self::AmericaBelize
            | Self::AmericaChicago
            | Self::AmericaCostaRica
            | Self::AmericaElSalvador
            | Self::AmericaGuatemala
            | Self::AmericaIndianaTellCity
            | Self::AmericaManagua
            | Self::AmericaMatamoros
            | Self::AmericaMenominee
            | Self::AmericaMerida
            | Self::AmericaMexicoCity
            | Self::AmericaMonterrey
            | Self::AmericaNorthDakotaBeulah
            | Self::AmericaNorthDakotaCenter
            | Self::AmericaNorthDakotaNewSalem
            | Self::AmericaRainyRiver
            | Self::AmericaRankinInlet
            | Self::AmericaRegina
            | Self::AmericaResolute
            | Self::AmericaSwiftCurrent
            | Self::AmericaTegucigalpa
            | Self::AmericaWinnipeg
            | Self::Cst6Cdt
            | Self::CanadaCentral
            | Self::CanadaSaskatchewan
            | Self::ChileEasterlsland
            | Self::MexicoGeneral
            | Self::PacificEaster
            | Self::PacificGalapagos
            | Self::UsCentral => FixedOffset::east_opt(-21600),
            Self::AmericaBoise
            | Self::AmericaCambridgeBay
            | Self::AmericaChihuahua
            | Self::AmericaCreston
            | Self::AmericaDawson
            | Self::AmericaDawsonCreek
            | Self::AmericaDenver
            | Self::AmericaEdmonton
            | Self::AmericaFortNelson
            | Self::AmericaHermosillo
            | Self::AmericaInuvik
            | Self::AmericaMazatlan
            | Self::AmericaOjinaga
            | Self::AmericaPhoenix
            | Self::AmericaShiprock
            | Self::AmericaWhitehorse
            | Self::AmericaYellowknife
            | Self::CanadaMountain
            | Self::CanadaYukon
            | Self::Mst7Mdt
            | Self::Navajo
            | Self::UsArizona
            | Self::UsMountain
            | Self::Mst => FixedOffset::east_opt(-25200),
            Self::AmericaEnsenada
            | Self::AmericaLosAngeles
            | Self::AmericaSantaIsabel
            | Self::AmericaTijuana
            | Self::AmericaVancouver
            | Self::CanadaPacific
            | Self::MexicoBajanorte
            | Self::Pst8Pdt
            | Self::PacificPitcairn
            | Self::UsPacific => FixedOffset::east_opt(-28800),
            Self::AmericaNoronha | Self::AtlanticSouthGeorgia | Self::BrazilDenoronha => {
                FixedOffset::east_opt(-7200)
            }
            Self::AmericaScoresbysund | Self::AtlanticAzores | Self::AtlanticCapeVerde => {
                FixedOffset::east_opt(-3600)
            }
            Self::AmericaStJohns => FixedOffset::east_opt(-9000),
            Self::AntarcticaCasey
            | Self::AsiaMagadan
            | Self::AsiaSakhalin
            | Self::AsiaSrednekolymsk
            | Self::PacificBougainville
            | Self::PacificEfate
            | Self::PacificGuadalcanal
            | Self::PacificKosrae
            | Self::PacificNorfolk
            | Self::PacificNoumea
            | Self::PacificPohnpei => FixedOffset::east_opt(39600),
            Self::AntarcticaDavis
            | Self::AsiaBangkok
            | Self::AsiaBarnaul
            | Self::AsiaHoChiMinh
            | Self::AsiaHovd
            | Self::AsiaJakarta
            | Self::AsiaKrasnoyarsk
            | Self::AsiaNovokuznetsk
            | Self::AsiaNovosibirsk
            | Self::AsiaPhnomPenh
            | Self::AsiaPontianak
            | Self::AsiaSaigon
            | Self::AsiaTomsk
            | Self::AsiaVientiane
            | Self::IndianChristmas => FixedOffset::east_opt(25200),
            Self::AntarcticaDumontdurville
            | Self::AntarcticaMacquarie
            | Self::AsiaUstNera
            | Self::AsiaVladivostok
            | Self::AustraliaAct
            | Self::AustraliaBrisbane
            | Self::AustraliaCanberra
            | Self::AustraliaCurrie
            | Self::AustraliaHobart
            | Self::AustraliaLindeman
            | Self::AustraliaMelbourne
            | Self::AustraliaNsw
            | Self::AustraliaQueensland
            | Self::AustraliaSydney
            | Self::AustraliaTasmania
            | Self::AustraliaVictoria
            | Self::PacificChuuk
            | Self::PacificGuam
            | Self::PacificPortMoresby
            | Self::PacificSaipan
            | Self::PacificYap => FixedOffset::east_opt(36000),
            Self::AntarcticaMawson
            | Self::AsiaAqtau
            | Self::AsiaAqtobe
            | Self::AsiaAshgabat
            | Self::AsiaAshkhabad
            | Self::AsiaAtyrau
            | Self::AsiaDushanbe
            | Self::AsiaKarachi
            | Self::AsiaOral
            | Self::AsiaQyzylorda
            | Self::AsiaSamarkand
            | Self::AsiaTashkent
            | Self::AsiaYekaterinburg
            | Self::IndianKerguelen
            | Self::IndianMaldives => FixedOffset::east_opt(18000),
            Self::AntarcticaMcmurdo
            | Self::AsiaAnadyr
            | Self::AsiaKamchatka
            | Self::Kwajalein
            | Self::Nz
            | Self::PacificAuckland
            | Self::PacificFiji
            | Self::PacificFunafuti
            | Self::PacificKwajalein
            | Self::PacificMajuro
            | Self::PacificNauru
            | Self::PacificTarawa
            | Self::PacificWallis => FixedOffset::east_opt(43200),
            Self::AntarcticaVostok
            | Self::AsiaAlmaty
            | Self::AsiaBishkek
            | Self::AsiaDacca
            | Self::AsiaDhaka
            | Self::AsiaKashgar
            | Self::AsiaOmsk
            | Self::AsiaQostanay
            | Self::AsiaThimbu
            | Self::AsiaThimphu
            | Self::AsiaUrumqi
            | Self::IndianChagos => FixedOffset::east_opt(21600),
            Self::AsiaBaku
            | Self::AsiaDubai
            | Self::AsiaMuscat
            | Self::AsiaTbilisi
            | Self::AsiaYerevan
            | Self::EuropeAstrakhan
            | Self::EuropeSamara
            | Self::EuropeSaratov
            | Self::EuropeUlyanovsk
            | Self::IndianMahe
            | Self::IndianMauritius
            | Self::IndianReunion => FixedOffset::east_opt(14400),
            Self::AsiaBrunei
            | Self::AsiaChoibalsan
            | Self::AsiaChongqing
            | Self::AsiaChungking
            | Self::AsiaHarbin
            | Self::AsiaHongKong
            | Self::AsiaIrkutsk
            | Self::AsiaKualaLumpur
            | Self::AsiaKuching
            | Self::AsiaMacao
            | Self::AsiaMacau
            | Self::AsiaMakassar
            | Self::AsiaManila
            | Self::AsiaShanghai
            | Self::AsiaSingapore
            | Self::AsiaTaipei
            | Self::AsiaUjungPandang
            | Self::AsiaUlaanbaatar
            | Self::AustraliaPerth
            | Self::AustraliaWest
            | Self::Hongkong
            | Self::Prc
            | Self::Singapore => FixedOffset::east_opt(28800),
            Self::AsiaCalcutta | Self::AsiaColombo | Self::AsiaKolkata => {
                FixedOffset::east_opt(19800)
            }
            Self::AsiaChita
            | Self::AsiaDili
            | Self::AsiaJayapura
            | Self::AsiaKhandyga
            | Self::AsiaPyongyang
            | Self::AsiaSeoul
            | Self::AsiaTokyo
            | Self::Japan
            | Self::PacificPalau
            | Self::Rok => FixedOffset::east_opt(32400),
            Self::AsiaKabul => FixedOffset::east_opt(16200),
            Self::AsiaKathmandu => FixedOffset::east_opt(20700),
            Self::AsiaRangoon | Self::AsiaYangon | Self::IndianCocos => {
                FixedOffset::east_opt(23400)
            }
            Self::AsiaTehran => FixedOffset::east_opt(12600),
            Self::AustraliaAdelaide
            | Self::AustraliaBrokenHill
            | Self::AustraliaDarwin
            | Self::AustraliaNorth
            | Self::AustraliaSouth => FixedOffset::east_opt(34200),
            Self::AustraliaEucla => FixedOffset::east_opt(31500),
            Self::AustraliaLhi | Self::AustraliaLordHowe => FixedOffset::east_opt(37800),
            Self::NzChat | Self::PacificChatham => FixedOffset::east_opt(45900),
            Self::PacificApia
            | Self::PacificEnderbury
            | Self::PacificKanton
            | Self::PacificTongatapu => FixedOffset::east_opt(46800),
            Self::PacificKiritimati => FixedOffset::east_opt(50400),
            Self::PacificMarquesas => FixedOffset::east_opt(-30600),
            Self::PacificMidway
            | Self::PacificNiue
            | Self::PacificPagoPago
            | Self::PacificSamoa
            | Self::UsSamoa => FixedOffset::east_opt(-39600),
        }
        .unwrap()
    }
}

impl TimeZone for IbTimeZone {
    type Offset = Self;

    fn from_offset(offset: &Self::Offset) -> Self {
        *offset
    }

    fn offset_from_local_date(&self, _local: &NaiveDate) -> LocalResult<Self::Offset> {
        LocalResult::Single(*self)
    }

    fn offset_from_local_datetime(&self, _local: &NaiveDateTime) -> LocalResult<Self::Offset> {
        LocalResult::Single(*self)
    }

    fn offset_from_utc_date(&self, _utc: &NaiveDate) -> Self::Offset {
        *self
    }

    fn offset_from_utc_datetime(&self, _utc: &NaiveDateTime) -> Self::Offset {
        *self
    }
}

impl From<IbTimeZone> for FixedOffset {
    fn from(val: IbTimeZone) -> Self {
        val.fix()
    }
}

impl std::str::FromStr for IbTimeZone {
    type Err = InvalidTimeZone;

    #[allow(clippy::too_many_lines)]
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
/// An error from attempting to parse an [`IbTimeZone`]
pub struct InvalidTimeZone(pub String);

impl From<ParseError> for InvalidTimeZone {
    fn from(value: ParseError) -> Self {
        Self(format!("{value}"))
    }
}

impl std::fmt::Display for InvalidTimeZone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
