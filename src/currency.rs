use core::str::FromStr;
use serde::{Deserialize, Serialize};

// === Type definitions ===
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
/// Represents all the possible currencies available for trading at IBKR.
pub enum Currency {
    #[serde(rename(serialize = "AUD"))]
    /// The Australian Dollar (AUD) is the currency of Australia.
    AustralianDollar,
    #[serde(rename(serialize = "GBP"))]
    /// The Pound Sterling (GBP) is the currency of the United Kingdom.
    BritishPound,
    #[serde(rename(serialize = "CAD"))]
    /// The Canadian Dollar (CAD) is the currency of Canada.
    CanadianDollar,
    #[serde(rename(serialize = "CNH"))]
    /// The Chinese Renminbi (RMB / CNH) is the currency of The People's Republic of China. The
    /// Yuan is the basic unit of the Renminbi.
    ChineseYuan,
    #[serde(rename(serialize = "DKK"))]
    /// The Danish Krone (DKK) is the currency of Denmark.
    DanishKrone,
    #[serde(rename(serialize = "EUR"))]
    /// The Euro (EUR) is the currency of most countries in the European Union
    Euro,
    #[serde(rename(serialize = "HKD"))]
    /// The Hong Kong Dollar (HKD) is the currency of Hong Kong.
    HongKongDollar,
    #[serde(rename(serialize = "INR"))]
    /// The Indian Rupee (INR) is the currency of the Republic of India.
    IndianRupee,
    #[serde(rename(serialize = "ILS"))]
    /// The Israeli New Shekel (ILS / NIS) is the currency of Israel.
    IsraeliNewShekel,
    #[serde(rename(serialize = "JPY"))]
    /// The Japanese Yen (JPY) is the currency of Japan.
    JapaneseYen,
    #[serde(rename(serialize = "KRW"))]
    /// The Korean Won (KRW) is the currency of South Korea.
    KoreanWon,
    #[serde(rename(serialize = "MXN"))]
    /// The Mexican Peso (MXN) is the currency of Mexico.
    MexicanPeso,
    #[serde(rename(serialize = "NZD"))]
    /// The New Zealand Dollar (NZD) is the currency of New Zealand.
    NewZealandDollar,
    #[serde(rename(serialize = "NOK"))]
    /// The Norwegian Krone (NOK) is the currency of Norway.
    NorwegianKrone,
    #[serde(rename(serialize = "SEK"))]
    /// The Swedish Krona (SEK) is the currency of Sweden.
    SwedishKrona,
    #[serde(rename(serialize = "CHF"))]
    /// The Swiss Franc (CHF) is the currency of Switzerland.
    SwissFranc,
    #[serde(rename(serialize = "USD"))]
    /// The US Dollar (USD) is the currency of the United States of America.
    USDollar,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// An error type returned when a given currency code cannot be matched with a valid [`Currency`]
pub struct ParseCurrencyError(pub String);

impl std::fmt::Display for ParseCurrencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid currency {}", self.0)
    }
}

impl std::error::Error for ParseCurrencyError {
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

// === Type implementations ===

impl ToString for Currency {
    fn to_string(&self) -> String {
        match *self {
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
            Self::USDollar => "USD",
        }
        .to_owned()
    }
}

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
            "EUR" => Self::Euro,
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
            "USD" => Self::USDollar,
            s => return Err(ParseCurrencyError(s.to_owned())),
        })
    }
}
