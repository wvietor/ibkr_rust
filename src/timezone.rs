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
    type Err = ParseTimezoneError;

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
            "Greenwich" | "GMT" => Self::Greenwich,
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
            "Universal" | "UTC" => Self::Universal,
            "W-SU" => Self::WSu,
            "WET" => Self::Wet,
            "Zulu" => Self::Zulu,
            "EST" => Self::Est,
            "HST" => Self::Hst,
            "MST" => Self::Mst,
            s => return Err(ParseTimezoneError(s.to_owned())),
        })
    }
}

impl std::fmt::Display for IbTimeZone {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::AfricaAbidjan => "Africa/Abidjan",
            Self::AfricaAccra => "Africa/Accra",
            Self::AfricaAddisAbaba => "Africa/Addis Ababa",
            Self::AfricaAlgiers => "Africa/Algiers",
            Self::AfricaAsmara => "Africa/Asmara",
            Self::AfricaAsmera => "Africa/Asmera",
            Self::AfricaBamako => "Africa/Bamako",
            Self::AfricaBangui => "Africa/Bangui",
            Self::AfricaBanjul => "Africa/Banjul",
            Self::AfricaBissau => "Africa/Bissau",
            Self::AfricaBlantyre => "Africa/Blantyre",
            Self::AfricaBrazzaville => "Africa/Brazzaville",
            Self::AfricaBujumbura => "Africa/Bujumbura",
            Self::AfricaCairo => "Africa/Cairo",
            Self::AfricaCasablanca => "Africa/Casablanca",
            Self::AfricaCeuta => "Africa/Ceuta",
            Self::AfricaConakry => "Africa/Conakry",
            Self::AfricaDakar => "Africa/Dakar",
            Self::AfricaDarEsSalaam => "Africa/Dar es Salaam",
            Self::AfricaDjibouti => "Africa/Djibouti",
            Self::AfricaDouala => "Africa/Douala",
            Self::AfricaElAaiun => "Africa/El Aaiun",
            Self::AfricaFreetown => "Africa/Freetown",
            Self::AfricaGaborone => "Africa/Gaborone",
            Self::AfricaHarare => "Africa/Harare",
            Self::AfricaJohannesburg => "Africa/Johannesburg",
            Self::AfricaJuba => "Africa/Juba",
            Self::AfricaKampala => "Africa/Kampala",
            Self::AfricaKhartoum => "Africa/Khartoum",
            Self::AfricaKigali => "Africa/Kigali",
            Self::AfricaKinshasa => "Africa/Kinshasa",
            Self::AfricaLagos => "Africa/Lagos",
            Self::AfricaLibreville => "Africa/Libreville",
            Self::AfricaLome => "Africa/Lome",
            Self::AfricaLuanda => "Africa/Luanda",
            Self::AfricaLubumbashi => "Africa/Lubumbashi",
            Self::AfricaLusaka => "Africa/Lusaka",
            Self::AfricaMalabo => "Africa/Malabo",
            Self::AfricaMaputo => "Africa/Maputo",
            Self::AfricaMaseru => "Africa/Maseru",
            Self::AfricaMbabane => "Africa/Mbabane",
            Self::AfricaMogadishu => "Africa/Mogadishu",
            Self::AfricaMonrovia => "Africa/Monrovia",
            Self::AfricaNairobi => "Africa/Nairobi",
            Self::AfricaNdjamena => "Africa/Ndjamena",
            Self::AfricaNiamey => "Africa/Niamey",
            Self::AfricaNouakchott => "Africa/Nouakchott",
            Self::AfricaOuagadougou => "Africa/Ouagadougou",
            Self::AfricaPortoNovo => "Africa/Porto-Novo",
            Self::AfricaSaoTome => "Africa/Sao Tome",
            Self::AfricaTimbuktu => "Africa/Timbuktu",
            Self::AfricaTripoli => "Africa/Tripoli",
            Self::AfricaTunis => "Africa/Tunis",
            Self::AfricaWindhoek => "Africa/Windhoek",
            Self::AmericaAdak => "America/Adak",
            Self::AmericaAnchorage => "America/Anchorage",
            Self::AmericaAnguilla => "America/Anguilla",
            Self::AmericaAntigua => "America/Antigua",
            Self::AmericaAraguaina => "America/Araguaina",
            Self::AmericaArgentinaBuenosAires => "America/Argentina/Buenos Aires",
            Self::AmericaArgentinaCatamarca => "America/Argentina/Catamarca",
            Self::AmericaArgentinaCordoba => "America/Argentina/Cordoba",
            Self::AmericaArgentinaJujuy => "America/Argentina/Jujuy",
            Self::AmericaArgentinaLaRioja => "America/Argentina/La Rioja",
            Self::AmericaArgentinaMendoza => "America/Argentina/Mendoza",
            Self::AmericaArgentinaRioGallegos => "America/Argentina/Rio Gallegos",
            Self::AmericaArgentinaSalta => "America/Argentina/Salta",
            Self::AmericaArgentinaSanJuan => "America/Argentina/San Juan",
            Self::AmericaArgentinaSanLuis => "America/Argentina/San Luis",
            Self::AmericaArgentinaTucuman => "America/Argentina/Tucuman",
            Self::AmericaArgentinaUshuaia => "America/Argentina/Ushuaia",
            Self::AmericaAruba => "America/Aruba",
            Self::AmericaAsuncion => "America/Asuncion",
            Self::AmericaAtikokan => "America/Atikokan",
            Self::AmericaAtka => "America/Atka",
            Self::AmericaBahia => "America/Bahia",
            Self::AmericaBahiaBanderas => "America/Bahia Banderas",
            Self::AmericaBarbados => "America/Barbados",
            Self::AmericaBelem => "America/Belem",
            Self::AmericaBelize => "America/Belize",
            Self::AmericaBlancSablon => "America/Blanc-Sablon",
            Self::AmericaBoaVista => "America/Boa Vista",
            Self::AmericaBogota => "America/Bogota",
            Self::AmericaBoise => "America/Boise",
            Self::AmericaBuenosAires => "America/Buenos Aires",
            Self::AmericaCambridgeBay => "America/Cambridge Bay",
            Self::AmericaCampoGrande => "America/Campo Grande",
            Self::AmericaCancun => "America/Cancun",
            Self::AmericaCaracas => "America/Caracas",
            Self::AmericaCayenne => "America/Cayenne",
            Self::AmericaCayman => "America/Cayman",
            Self::AmericaChicago => "America/Chicago",
            Self::AmericaChihuahua => "America/Chihuahua",
            Self::AmericaCoralHarbour => "America/Coral Harbour",
            Self::AmericaCordoba => "America/Cordoba",
            Self::AmericaCostaRica => "America/Costa Rica",
            Self::AmericaCreston => "America/Creston",
            Self::AmericaCuiaba => "America/Cuiaba",
            Self::AmericaCuracao => "America/Curacao",
            Self::AmericaDanmarkshavn => "America/Danmarkshavn",
            Self::AmericaDawson => "America/Dawson",
            Self::AmericaDawsonCreek => "America/Dawson Creek",
            Self::AmericaDenver => "America/Denver",
            Self::AmericaDetroit => "America/Detroit",
            Self::AmericaDominica => "America/Dominica",
            Self::AmericaEdmonton => "America/Edmonton",
            Self::AmericaEirunepe => "America/Eirunepe",
            Self::AmericaElSalvador => "America/El Salvador",
            Self::AmericaEnsenada => "America/Ensenada",
            Self::AmericaFortNelson => "America/Fort Nelson",
            Self::AmericaFortWayne => "America/Fort Wayne",
            Self::AmericaFortaleza => "America/Fortaleza",
            Self::AmericaGlaceBay => "America/Glace Bay",
            Self::AmericaGodthab => "America/Godthab",
            Self::AmericaGooseBay => "America/Goose Bay",
            Self::AmericaGrandTurk => "America/Grand Turk",
            Self::AmericaGrenada => "America/Grenada",
            Self::AmericaGuadeloupe => "America/Guadeloupe",
            Self::AmericaGuatemala => "America/Guatemala",
            Self::AmericaGuayaquil => "America/Guayaquil",
            Self::AmericaGuyana => "America/Guyana",
            Self::AmericaHalifax => "America/Halifax",
            Self::AmericaHermosillo => "America/Hermosillo",
            Self::AmericaIndianaIndianapolis => "America/Indiana/Indianapolis",
            Self::AmericaIndianaMarengo => "America/Indiana/Marengo",
            Self::AmericaIndianaPetersburg => "America/Indiana/Petersburg",
            Self::AmericaIndianaTellCity => "America/Indiana/Tell City",
            Self::AmericaIndianaVevay => "America/Indiana/Vevay",
            Self::AmericaIndianaVincennes => "America/Indiana/Vincennes",
            Self::AmericaIndianaWinamac => "America/Indiana/Winamac",
            Self::AmericaIndianapolis => "America/Indianapolis",
            Self::AmericaInuvik => "America/Inuvik",
            Self::AmericaIqaluit => "America/Iqaluit",
            Self::AmericaJamaica => "America/Jamaica",
            Self::AmericaJuneau => "America/Juneau",
            Self::AmericaKentuckyLouisville => "America/Kentucky/Louisville",
            Self::AmericaKentuckyMonticello => "America/Kentucky/Monticello",
            Self::AmericaKralendijk => "America/Kralendijk",
            Self::AmericaLaPaz => "America/La Paz",
            Self::AmericaLima => "America/Lima",
            Self::AmericaLosAngeles => "America/Los Angeles",
            Self::AmericaLouisville => "America/Louisville",
            Self::AmericaLowerPrinces => "America/Lower Princes",
            Self::AmericaMaceio => "America/Maceio",
            Self::AmericaManagua => "America/Managua",
            Self::AmericaManaus => "America/Manaus",
            Self::AmericaMarigot => "America/Marigot",
            Self::AmericaMartinique => "America/Martinique",
            Self::AmericaMatamoros => "America/Matamoros",
            Self::AmericaMazatlan => "America/Mazatlan",
            Self::AmericaMenominee => "America/Menominee",
            Self::AmericaMerida => "America/Merida",
            Self::AmericaMetlakatla => "America/Metlakatla",
            Self::AmericaMexicoCity => "America/Mexico City",
            Self::AmericaMiquelon => "America/Miquelon",
            Self::AmericaMoncton => "America/Moncton",
            Self::AmericaMonterrey => "America/Monterrey",
            Self::AmericaMontevideo => "America/Montevideo",
            Self::AmericaMontreal => "America/Montreal",
            Self::AmericaMontserrat => "America/Montserrat",
            Self::AmericaNassau => "America/Nassau",
            Self::AmericaNewYork => "America/New York",
            Self::AmericaNipigon => "America/Nipigon",
            Self::AmericaNome => "America/Nome",
            Self::AmericaNoronha => "America/Noronha",
            Self::AmericaNorthDakotaBeulah => "America/North Dakota/Beulah",
            Self::AmericaNorthDakotaCenter => "America/North Dakota/Center",
            Self::AmericaNorthDakotaNewSalem => "America/North Dakota/New Salem",
            Self::AmericaNuuk => "America/Nuuk",
            Self::AmericaOjinaga => "America/Ojinaga",
            Self::AmericaPanama => "America/Panama",
            Self::AmericaPangnirtung => "America/Pangnirtung",
            Self::AmericaParamaribo => "America/Paramaribo",
            Self::AmericaPhoenix => "America/Phoenix",
            Self::AmericaPortAuPrince => "America/Port-au-Prince",
            Self::AmericaPortOfSpain => "America/Port of Spain",
            Self::AmericaPortoAcre => "America/Porto Acre",
            Self::AmericaPortoVelho => "America/Porto Velho",
            Self::AmericaPuertoRico => "America/Puerto Rico",
            Self::AmericaPuntaArenas => "America/Punta Arenas",
            Self::AmericaRainyRiver => "America/Rainy River",
            Self::AmericaRankinInlet => "America/Rankin Inlet",
            Self::AmericaRecife => "America/Recife",
            Self::AmericaRegina => "America/Regina",
            Self::AmericaResolute => "America/Resolute",
            Self::AmericaRioBranco => "America/Rio Branco",
            Self::AmericaRosario => "America/Rosario",
            Self::AmericaSantaIsabel => "America/Santa Isabel",
            Self::AmericaSantarem => "America/Santarem",
            Self::AmericaSantiago => "America/Santiago",
            Self::AmericaSantoDomingo => "America/Santo Domingo",
            Self::AmericaSaoPaulo => "America/Sao Paulo",
            Self::AmericaScoresbysund => "America/Scoresbysund",
            Self::AmericaShiprock => "America/Shiprock",
            Self::AmericaSitka => "America/Sitka",
            Self::AmericaStBarthelemy => "America/St Barthelemy",
            Self::AmericaStJohns => "America/St Johns",
            Self::AmericaStKitts => "America/St Kitts",
            Self::AmericaStLucia => "America/St Lucia",
            Self::AmericaStThomas => "America/St Thomas",
            Self::AmericaStVincent => "America/St Vincent",
            Self::AmericaSwiftCurrent => "America/Swift Current",
            Self::AmericaTegucigalpa => "America/Tegucigalpa",
            Self::AmericaThule => "America/Thule",
            Self::AmericaThunderBay => "America/Thunder Bay",
            Self::AmericaTijuana => "America/Tijuana",
            Self::AmericaToronto => "America/Toronto",
            Self::AmericaTortola => "America/Tortola",
            Self::AmericaVancouver => "America/Vancouver",
            Self::AmericaVirgin => "America/Virgin",
            Self::AmericaWhitehorse => "America/Whitehorse",
            Self::AmericaWinnipeg => "America/Winnipeg",
            Self::AmericaYakutat => "America/Yakutat",
            Self::AmericaYellowknife => "America/Yellowknife",
            Self::AntarcticaCasey => "Antarctica/Casey",
            Self::AntarcticaDavis => "Antarctica/Davis",
            Self::AntarcticaDumontdurville => "Antarctica/DumontDUrville",
            Self::AntarcticaMacquarie => "Antarctica/Macquarie",
            Self::AntarcticaMawson => "Antarctica/Mawson",
            Self::AntarcticaMcmurdo => "Antarctica/McMurdo",
            Self::AntarcticaPalmer => "Antarctica/Palmer",
            Self::AntarcticaRothera => "Antarctica/Rothera",
            Self::AntarcticaSyowa => "Antarctica/Syowa",
            Self::AntarcticaVostok => "Antarctica/Vostok",
            Self::ArcticLongyearbyen => "Arctic/Longyearbyen",
            Self::AsiaAden => "Asia/Aden",
            Self::AsiaAlmaty => "Asia/Almaty",
            Self::AsiaAmman => "Asia/Amman",
            Self::AsiaAnadyr => "Asia/Anadyr",
            Self::AsiaAqtau => "Asia/Aqtau",
            Self::AsiaAqtobe => "Asia/Aqtobe",
            Self::AsiaAshgabat => "Asia/Ashgabat",
            Self::AsiaAshkhabad => "Asia/Ashkhabad",
            Self::AsiaAtyrau => "Asia/Atyrau",
            Self::AsiaBaghdad => "Asia/Baghdad",
            Self::AsiaBahrain => "Asia/Bahrain",
            Self::AsiaBaku => "Asia/Baku",
            Self::AsiaBangkok => "Asia/Bangkok",
            Self::AsiaBarnaul => "Asia/Barnaul",
            Self::AsiaBeirut => "Asia/Beirut",
            Self::AsiaBishkek => "Asia/Bishkek",
            Self::AsiaBrunei => "Asia/Brunei",
            Self::AsiaCalcutta => "Asia/Calcutta",
            Self::AsiaChita => "Asia/Chita",
            Self::AsiaChoibalsan => "Asia/Choibalsan",
            Self::AsiaChongqing => "Asia/Chongqing",
            Self::AsiaChungking => "Asia/Chungking",
            Self::AsiaColombo => "Asia/Colombo",
            Self::AsiaDacca => "Asia/Dacca",
            Self::AsiaDamascus => "Asia/Damascus",
            Self::AsiaDhaka => "Asia/Dhaka",
            Self::AsiaDili => "Asia/Dili",
            Self::AsiaDubai => "Asia/Dubai",
            Self::AsiaDushanbe => "Asia/Dushanbe",
            Self::AsiaFamagusta => "Asia/Famagusta",
            Self::AsiaGaza => "Asia/Gaza",
            Self::AsiaHarbin => "Asia/Harbin",
            Self::AsiaHebron => "Asia/Hebron",
            Self::AsiaHoChiMinh => "Asia/Ho Chi Minh",
            Self::AsiaHongKong => "Asia/Hong Kong",
            Self::AsiaHovd => "Asia/Hovd",
            Self::AsiaIrkutsk => "Asia/Irkutsk",
            Self::AsiaIstanbul => "Asia/Istanbul",
            Self::AsiaJakarta => "Asia/Jakarta",
            Self::AsiaJayapura => "Asia/Jayapura",
            Self::AsiaJerusalem => "Asia/Jerusalem",
            Self::AsiaKabul => "Asia/Kabul",
            Self::AsiaKamchatka => "Asia/Kamchatka",
            Self::AsiaKarachi => "Asia/Karachi",
            Self::AsiaKashgar => "Asia/Kashgar",
            Self::AsiaKathmandu => "Asia/Kathmandu",
            Self::AsiaKhandyga => "Asia/Khandyga",
            Self::AsiaKolkata => "Asia/Kolkata",
            Self::AsiaKrasnoyarsk => "Asia/Krasnoyarsk",
            Self::AsiaKualaLumpur => "Asia/Kuala Lumpur",
            Self::AsiaKuching => "Asia/Kuching",
            Self::AsiaKuwait => "Asia/Kuwait",
            Self::AsiaMacao => "Asia/Macao",
            Self::AsiaMacau => "Asia/Macau",
            Self::AsiaMagadan => "Asia/Magadan",
            Self::AsiaMakassar => "Asia/Makassar",
            Self::AsiaManila => "Asia/Manila",
            Self::AsiaMuscat => "Asia/Muscat",
            Self::AsiaNicosia => "Asia/Nicosia",
            Self::AsiaNovokuznetsk => "Asia/Novokuznetsk",
            Self::AsiaNovosibirsk => "Asia/Novosibirsk",
            Self::AsiaOmsk => "Asia/Omsk",
            Self::AsiaOral => "Asia/Oral",
            Self::AsiaPhnomPenh => "Asia/Phnom Penh",
            Self::AsiaPontianak => "Asia/Pontianak",
            Self::AsiaPyongyang => "Asia/Pyongyang",
            Self::AsiaQatar => "Asia/Qatar",
            Self::AsiaQostanay => "Asia/Qostanay",
            Self::AsiaQyzylorda => "Asia/Qyzylorda",
            Self::AsiaRangoon => "Asia/Rangoon",
            Self::AsiaRiyadh => "Asia/Riyadh",
            Self::AsiaSaigon => "Asia/Saigon",
            Self::AsiaSakhalin => "Asia/Sakhalin",
            Self::AsiaSamarkand => "Asia/Samarkand",
            Self::AsiaSeoul => "Asia/Seoul",
            Self::AsiaShanghai => "Asia/Shanghai",
            Self::AsiaSingapore => "Asia/Singapore",
            Self::AsiaSrednekolymsk => "Asia/Srednekolymsk",
            Self::AsiaTaipei => "Asia/Taipei",
            Self::AsiaTashkent => "Asia/Tashkent",
            Self::AsiaTbilisi => "Asia Tbilisi",
            Self::AsiaTehran => "Asia/Tehran",
            Self::AsiaTelAviv => "Asia/Tel Aviv",
            Self::AsiaThimbu => "Asia/Thimbu",
            Self::AsiaThimphu => "Asia/Thimphu",
            Self::AsiaTokyo => "Asia/Tokyo",
            Self::AsiaTomsk => "Asia/Tomsk",
            Self::AsiaUjungPandang => "Asia/Ujung Pandang",
            Self::AsiaUlaanbaatar => "Asia/Ulaanbaatar",
            Self::AsiaUrumqi => "Asia/Urumqi",
            Self::AsiaUstNera => "Asia/Ust-Nera",
            Self::AsiaVientiane => "Asia/Vientiane",
            Self::AsiaVladivostok => "Asia/Vladivostok",
            Self::AsiaYangon => "Asia/Yangon",
            Self::AsiaYekaterinburg => "Asia/Yekaterinburg",
            Self::AsiaYerevan => "Asia/Yerevan",
            Self::AtlanticAzores => "Atlantic/Azores",
            Self::AtlanticBermuda => "Atlantic/Bermuda",
            Self::AtlanticCanary => "Atlantic/Canary",
            Self::AtlanticCapeVerde => "Atlantic/Cape Verde",
            Self::AtlanticFaeroe => "Atlantic/Faeroe",
            Self::AtlanticFaroe => "Atlantic/Faroe",
            Self::AtlanticJanMayen => "Atlantic/Jan Mayen",
            Self::AtlanticMadeira => "Atlantic/Madeira",
            Self::AtlanticReykjavik => "Atlantic/Reykjavik",
            Self::AtlanticSouthGeorgia => "Atlantic/South Georgia",
            Self::AtlanticStHelena => "Atlantic/St Helena",
            Self::AtlanticStanley => "Atlantic/Stanley",
            Self::AustraliaAct => "Australia/ACT",
            Self::AustraliaAdelaide => "Australia/Adelaide",
            Self::AustraliaBrisbane => "Australia/Brisbane",
            Self::AustraliaBrokenHill => "Australia/Broken Hill",
            Self::AustraliaCanberra => "Australia/Canberra",
            Self::AustraliaCurrie => "Australia/Currie",
            Self::AustraliaDarwin => "Australia/Darwin",
            Self::AustraliaEucla => "Australia/Eucla",
            Self::AustraliaHobart => "Australia/Hobart",
            Self::AustraliaLhi => "Australia/LHI",
            Self::AustraliaLindeman => "Australia/Lindeman",
            Self::AustraliaLordHowe => "Australia/Lord Howe",
            Self::AustraliaMelbourne => "Australia/Melbourne",
            Self::AustraliaNsw => "Australia/NSW",
            Self::AustraliaNorth => "Australia/North",
            Self::AustraliaPerth => "Australia/Perth",
            Self::AustraliaQueensland => "Australia/Queensland",
            Self::AustraliaSouth => "Australia/South",
            Self::AustraliaSydney => "Australia/Sydney",
            Self::AustraliaTasmania => "Australia/Tasmania",
            Self::AustraliaVictoria => "Australia/Victoria",
            Self::AustraliaWest => "Australia/West",
            Self::BrazilAcre => "Brazil/Acre",
            Self::BrazilDenoronha => "Brazil/DeNoronha",
            Self::BrazilEast => "Brazil/East",
            Self::BrazilWest => "Brazil/West",
            Self::Cet => "CET",
            Self::Cst6Cdt => "CST6CDT",
            Self::CanadaAtlantic => "Canada/Atlantic",
            Self::CanadaCentral => "Canada/Central",
            Self::CanadaEastern => "Canada/Eastern",
            Self::CanadaMountain => "Canada/Mountain",
            Self::CanadaPacific => "Canada/Pacific",
            Self::CanadaSaskatchewan => "Canada/Saskatchewan",
            Self::CanadaYukon => "Canada/Yukon",
            Self::ChileContinental => "Chile/Continental",
            Self::ChileEasterlsland => "Chile/Easterlsland",
            Self::Eet => "EET",
            Self::Est5Edt => "EST5EDT",
            Self::Egypt => "Egypt",
            Self::Eire => "Eire",
            Self::EuropeAmsterdam => "Europe/Amsterdam",
            Self::EuropeAndorra => "Europe/Andorra",
            Self::EuropeAstrakhan => "Europe/Astrakhan",
            Self::EuropeAthens => "Europe/Athens",
            Self::EuropeBelfast => "Europe/Belfast",
            Self::EuropeBelgrade => "Europe/Belgrade",
            Self::EuropeBerlin => "Europe/Berlin",
            Self::EuropeBratislava => "Europe/Bratislava",
            Self::EuropeBrussels => "Europe/Brussels",
            Self::EuropeBucharest => "Europe/Bucharest",
            Self::EuropeBudapest => "Europe/Budapest",
            Self::EuropeBusingen => "Europe/Busingen",
            Self::EuropeChisinau => "Europe/Chisinau",
            Self::EuropeCopenhagen => "Europe/Copenhagen",
            Self::EuropeDublin => "Europe/Dublin",
            Self::EuropeGibraltar => "Europe/Gibraltar",
            Self::EuropeGuernsey => "Europe/Guernsey",
            Self::EuropeHelsinki => "Europe/Helsinki",
            Self::EuropeIsleOfMan => "Europe/Isle of Man",
            Self::EuropeIstanbul => "Europe/Istanbul",
            Self::EuropeJersey => "Europe/Jersey",
            Self::EuropeKaliningrad => "Europe/Kaliningrad",
            Self::EuropeKiev => "Europe/Kiev",
            Self::EuropeKirov => "Europe/Kirov",
            Self::EuropeKyiv => "Europe/Kyiv",
            Self::EuropeLisbon => "Europe/Lisbon",
            Self::EuropeLjubljana => "Europe/Ljubljana",
            Self::EuropeLondon => "Europe/London",
            Self::EuropeLuxembourg => "Europe/Luxembourg",
            Self::EuropeMadrid => "Europe/Madrid",
            Self::EuropeMalta => "Europe/Malta",
            Self::EuropeMariehamn => "Europe/Mariehamn",
            Self::EuropeMinsk => "Europe/Minsk",
            Self::EuropeMonaco => "Europe/Monaco",
            Self::EuropeMoscow => "Europe/Moscow",
            Self::EuropeNicosia => "Europe/Nicosia",
            Self::EuropeOslo => "Europe/Oslo",
            Self::EuropeParis => "Europe/Paris",
            Self::EuropePodgorica => "Europe/Podgorica",
            Self::EuropePrague => "Europe/Prague",
            Self::EuropeRiga => "Europe/Riga",
            Self::EuropeRome => "Europe/Rome",
            Self::EuropeSamara => "Europe/Samara",
            Self::EuropeSanMarino => "Europe/San Marino",
            Self::EuropeSarajevo => "Europe/Sarajevo",
            Self::EuropeSaratov => "Europe/Saratov",
            Self::EuropeSimferopol => "Europe/Simferopol",
            Self::EuropeSkopje => "Europe/Skopje",
            Self::EuropeSofia => "Europe/Sofia",
            Self::EuropeStockholm => "Europe/Stockholm",
            Self::EuropeTallinn => "Europe/Tallinn",
            Self::EuropeTirane => "Europe/Tirane",
            Self::EuropeUlyanovsk => "Europe/Ulyanovsk",
            Self::EuropeUzhgorod => "Europe/Uzhgorod",
            Self::EuropeVaduz => "Europe/Vaduz",
            Self::EuropeVatican => "Europe/Vatican",
            Self::EuropeVienna => "Europe/Vienna",
            Self::EuropeVilnius => "Europe/Vilnius",
            Self::EuropeVolgograd => "Europe/Volgograd",
            Self::EuropeWarsaw => "Europe/Warsaw",
            Self::EuropeZagreb => "Europe/Zagreb",
            Self::EuropeZaporozhye => "Europe/Zaporozhye",
            Self::EuropeZurich => "Europe/Zurich",
            Self::Gb => "GB",
            Self::GbEire => "GB-Eire",
            Self::Greenwich => "Greenwich",
            Self::Hongkong => "Hongkong",
            Self::Iceland => "Iceland",
            Self::IndianAntananarivo => "Indian/Antananarivo",
            Self::IndianChagos => "Indian/Chagos",
            Self::IndianChristmas => "Indian/Christmas",
            Self::IndianCocos => "Indian/Cocos",
            Self::IndianComoro => "Indian/Comoro",
            Self::IndianKerguelen => "Indian/Kerguelen",
            Self::IndianMahe => "Indian/Mahe",
            Self::IndianMaldives => "Indian/Maldives",
            Self::IndianMauritius => "Indian/Mauritius",
            Self::IndianMayotte => "Indian/Mayotte",
            Self::IndianReunion => "Indian/Reunion",
            Self::Israel => "Israel",
            Self::Jamaica => "Jamaica",
            Self::Japan => "Japan",
            Self::Kwajalein => "Kwajalein",
            Self::Libya => "Libya",
            Self::Met => "MET",
            Self::Mst7Mdt => "MST7MDT",
            Self::MexicoBajanorte => "Mexico/BajaNorte",
            Self::MexicoGeneral => "Mexico/General",
            Self::Nz => "NZ",
            Self::NzChat => "NZ-CHAT",
            Self::Navajo => "Navajo",
            Self::Prc => "PRC",
            Self::Pst8Pdt => "PST8PDT",
            Self::PacificApia => "Pacific/Apia",
            Self::PacificAuckland => "Pacific/Auckland",
            Self::PacificBougainville => "Pacific/Bougainville",
            Self::PacificChatham => "Pacific/Chatham",
            Self::PacificChuuk => "Pacific/Chuuk",
            Self::PacificEaster => "Pacific/Easter",
            Self::PacificEfate => "Pacific/Efate",
            Self::PacificEnderbury => "Pacific/Enderbury",
            Self::PacificFiji => "Pacific/Fiji",
            Self::PacificFunafuti => "Pacific/Funafuti",
            Self::PacificGalapagos => "Pacific/Galapagos",
            Self::PacificGambier => "Pacific/Gambier",
            Self::PacificGuadalcanal => "Pacific/Guadalcanal",
            Self::PacificGuam => "Pacific/Guam",
            Self::PacificHonolulu => "Pacific/Honolulu",
            Self::PacificJohnston => "Pacific/Johnston",
            Self::PacificKanton => "Pacific/Kanton",
            Self::PacificKiritimati => "Pacific/Kiritimati",
            Self::PacificKosrae => "Pacific/Kosrae",
            Self::PacificKwajalein => "Pacific/Kwajalein",
            Self::PacificMajuro => "Pacific/Majuro",
            Self::PacificMarquesas => "Pacific/Marquesas",
            Self::PacificMidway => "Pacific/Midway",
            Self::PacificNauru => "Pacific/Nauru",
            Self::PacificNiue => "Pacific/Niue",
            Self::PacificNorfolk => "Pacific/Norfolk",
            Self::PacificNoumea => "Pacific/Noumea",
            Self::PacificPagoPago => "Pacific/Pago Pago",
            Self::PacificPalau => "Pacific/Palau",
            Self::PacificPitcairn => "Pacific/Pitcairn",
            Self::PacificPohnpei => "Pacific/Pohnpei",
            Self::PacificPortMoresby => "Pacific/Port Moresby",
            Self::PacificRarotonga => "Pacific/Rarotonga",
            Self::PacificSaipan => "Pacific/Saipan",
            Self::PacificSamoa => "Pacific/Samoa",
            Self::PacificTahiti => "Pacific/Tahiti",
            Self::PacificTarawa => "Pacific/Tarawa",
            Self::PacificTongatapu => "Pacific/Tongatapu",
            Self::PacificWallis => "Pacific/Wallis",
            Self::PacificYap => "Pacific/Yap",
            Self::Poland => "Poland",
            Self::Portugal => "Portugal",
            Self::Rok => "ROK",
            Self::Singapore => "Singapore",
            Self::Turkey => "Turkey",
            Self::Uct => "UCT",
            Self::UsAlaska => "US/Alaska",
            Self::UsAleutian => "US/Aleutian",
            Self::UsArizona => "US/Arizona",
            Self::UsCentral => "US/Central",
            Self::UsEastIndiana => "US/East-Indiana",
            Self::UsEastern => "US/Eastern",
            Self::UsHawail => "US/Hawail",
            Self::UsMountain => "US/Mountain",
            Self::UsPacific => "US/Pacific",
            Self::UsSamoa => "US/Samoa",
            Self::Universal => "Universal",
            Self::WSu => "W-SU",
            Self::Wet => "WET",
            Self::Zulu => "Zulu",
            Self::Est => "EST",
            Self::Hst => "HST",
            Self::Mst => "MST",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
/// An error from attempting to parse an [`IbTimeZone`]
pub struct ParseTimezoneError(pub String);

impl From<ParseError> for ParseTimezoneError {
    fn from(value: ParseError) -> Self {
        Self(format!("{value}"))
    }
}

impl std::fmt::Display for ParseTimezoneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid time zone encountered: {}", self.0)
    }
}

impl std::error::Error for ParseTimezoneError {}
