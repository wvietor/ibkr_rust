use core::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

// === Type definitions ===
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
/// Represents all the possible currencies available for trading at IBKR.
pub enum Currency {
    #[serde(rename = "AUD")]
    /// The Australian Dollar (AUD) is the currency of Australia.
    AustralianDollar,
    #[serde(rename = "GBP")]
    /// The Pound Sterling (GBP) is the currency of the United Kingdom.
    BritishPound,
    #[serde(rename = "CAD")]
    /// The Canadian Dollar (CAD) is the currency of Canada.
    CanadianDollar,
    #[serde(rename = "CNH")]
    /// The Chinese Renminbi (RMB / CNH) is the currency of The People's Republic of China. The
    /// Yuan is the basic unit of the Renminbi.
    ChineseYuan,
    #[serde(rename = "DKK")]
    /// The Danish Krone (DKK) is the currency of Denmark.
    DanishKrone,
    #[serde(rename = "CZK")]
    /// The Czech Koruny is the currency of most countries in the Czech Republic.
    CzechKoruny,
    #[serde(rename = "EUR")]
    /// The Euro (EUR) is the currency of most countries in the European Union
    Euro,
    #[serde(rename = "SGD")]
    /// The Singapore Dollars (SGD) is the currency of Singapore.
    SingaporeDollars,
    #[serde(rename = "TWD")]
    /// The New Taiwan Dollar (TWD) is the currency of Taiwan.
    NewTaiwanDollar,
    #[serde(rename = "HKD")]
    /// The Hong Kong Dollar (HKD) is the currency of Hong Kong.
    HongKongDollar,
    #[serde(rename = "INR")]
    /// The Indian Rupee (INR) is the currency of the Republic of India.
    IndianRupee,
    #[serde(rename = "ILS")]
    /// The Israeli New Shekel (ILS / NIS) is the currency of Israel.
    IsraeliNewShekel,
    #[serde(rename = "JPY")]
    /// The Japanese Yen (JPY) is the currency of Japan.
    JapaneseYen,
    #[serde(rename = "KRW")]
    /// The Korean Won (KRW) is the currency of South Korea.
    KoreanWon,
    #[serde(rename = "MXN")]
    /// The Mexican Peso (MXN) is the currency of Mexico.
    MexicanPeso,
    #[serde(rename = "NZD")]
    /// The New Zealand Dollar (NZD) is the currency of New Zealand.
    NewZealandDollar,
    #[serde(rename = "NOK")]
    /// The Norwegian Krone (NOK) is the currency of Norway.
    NorwegianKrone,
    #[serde(rename = "SEK")]
    /// The Swedish Króna (SEK) is the currency of Sweden.
    SwedishKrona,
    #[serde(rename = "CHF")]
    /// The Swiss Franc (CHF) is the currency of Switzerland.
    SwissFranc,
    #[serde(rename = "USD")]
    /// The US Dollar (USD) is the currency of the United States of America.
    UsDollar,
}

#[derive(Error, Default, Debug, Clone)]
#[error(
    "Invalid value encountered when attempting to parse currency. No such currency symbol: {0}"
)]
/// An error returned when parsing a [`Currency`] fails.
pub struct ParseCurrencyError(pub String);

// === Type implementations ===

impl FromStr for Currency {
    type Err = ParseCurrencyError;

    #[allow(clippy::too_many_lines)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_uppercase().as_str() {
            "AUD" => Self::AustralianDollar,
            "GBP" => Self::BritishPound,
            "CAD" => Self::CanadianDollar,
            "CNH" => Self::ChineseYuan,
            "DKK" => Self::DanishKrone,
            "CZK" => Self::CzechKoruny,
            "EUR" => Self::Euro,
            "SGD" => Self::SingaporeDollars,
            "TWD" => Self::NewTaiwanDollar,
            "HKD" => Self::HongKongDollar,
            "INR" => Self::IndianRupee,
            "ILS" => Self::IsraeliNewShekel,
            "JPY" => Self::JapaneseYen,
            "KRW" => Self::KoreanWon,
            "MXN" => Self::MexicanPeso,
            "NZD" => Self::NewZealandDollar,
            "NOK" => Self::NorwegianKrone,
            "SEK" => Self::SwedishKrona,
            "CHF" => Self::SwissFranc,
            "USD" => Self::UsDollar,
            s => return Err(ParseCurrencyError(s.to_owned())),
        })
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::AustralianDollar => "AUD",
            Self::BritishPound => "GBP",
            Self::CanadianDollar => "CAD",
            Self::ChineseYuan => "CNH",
            Self::DanishKrone => "DKK",
            Self::Euro => "EUR",
            Self::HongKongDollar => "HKD",
            Self::IndianRupee => "INR",
            Self::IsraeliNewShekel => "ILS",
            Self::JapaneseYen => "JPY",
            Self::KoreanWon => "KRW",
            Self::MexicanPeso => "MXN",
            Self::NewZealandDollar => "NZD",
            Self::NorwegianKrone => "NOK",
            Self::SwedishKrona => "SEK",
            Self::SwissFranc => "CHF",
            Self::UsDollar => "USD",
        };
        write!(f, "{s}")
    }
}
