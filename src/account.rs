use std::fmt::Formatter;
use std::num::{ParseFloatError, ParseIntError};
use std::str::{FromStr, ParseBoolError};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::currency::{Currency, ParseCurrencyError};

#[derive(Debug, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
#[serde(tag = "attribute", content = "data")]
/// Represents a specific account value
pub enum Attribute {
    /// The account ID number.
    AccountCode(String),
    /// "All" to return account summary data for all accounts, or set to a specific Advisor Account Group name that has already been created in TWS Global Configuration.
    AccountOrGroup(Group, Denomination),
    /// For internal use only.
    AccountReady(bool),
    /// Identifies the IB account structure.
    AccountType(String),
    /// Accrued cash value of stock, commodities and securities.
    AccruedCash(Segment<f64>, Denomination),
    /// Value of dividends accrued.
    AccruedDividend(Segment<f64>, Denomination),
    /// This value tells what you have available for trading.
    AvailableFunds(Segment<f64>, Denomination),
    /// Value of treasury bills.
    Billable(Segment<f64>, Denomination),
    /// Cash Account: Minimum (Equity with Loan Value, Previous Day Equity with Loan Value)-Initial Margin, Standard Margin Account: Minimum (Equity with Loan Value, Previous Day Equity with Loan Value) - Initial Margin *4.
    BuyingPower(f64, Denomination),
    /// Cash recognized at the time of trade + futures PNL.
    CashBalance(f64, Denomination),
    /// Unknown.
    ColumnPrio(Segment<i64>),
    /// Value of non-Government bonds such as corporate bonds and municipal bonds.
    CorporateBondValue(f64, Denomination),
    /// Value of cryptocurrency positions at PAXOS.
    Cryptocurrency(f64, Denomination),
    /// Open positions are grouped by currency.
    Currency(Denomination),
    /// Excess liquidity as a percentage of net liquidation value.
    Cushion(f64),
    /// Number of Open/Close trades one could do before Pattern Day Trading is detected.
    DayTradesRemaining(RemainingDayTrades),
    /// Number of Open/Close trades one could do tomorrow before Pattern Day Trading is detected.
    DayTradesRemainingTPlus1(RemainingDayTrades),
    /// Number of Open/Close trades one could do two days from today before Pattern Day Trading is detected.
    DayTradesRemainingTPlus2(RemainingDayTrades),
    /// Number of Open/Close trades one could do three days from today before Pattern Day Trading is detected.
    DayTradesRemainingTPlus3(RemainingDayTrades),
    /// Number of Open/Close trades one could do four days from today before Pattern Day Trading is detected.
    DayTradesRemainingTPlus4(RemainingDayTrades),
    /// Day trading status: For internal use only.
    DayTradingStatus(String),
    /// Forms the basis for determining whether a client has the necessary assets to either initiate or maintain security positions.
    EquityWithLoanValue(Segment<f64>, Denomination),
    /// This value shows your margin cushion, before liquidation.
    ExcessLiquidity(Segment<f64>, Denomination),
    /// The exchange rate of the currency to your base currency.
    ExchangeRate(f64, Denomination),
    /// Available funds of whole portfolio with no discounts or intraday credits.
    FullAvailableFunds(Segment<f64>, Denomination),
    /// Excess liquidity of whole portfolio with no discounts or intraday credits.
    FullExcessLiquidity(Segment<f64>, Denomination),
    /// Initial Margin of whole portfolio with no discounts or intraday credits.
    FullInitMarginReq(Segment<f64>, Denomination),
    /// Maintenance Margin of whole portfolio with no discounts or intraday credits.
    FullMaintenanceMarginReq(Segment<f64>, Denomination),
    /// Value of funds value (money market funds + mutual funds).
    FundValue(f64, Denomination),
    /// Real-time market-to-market value of futures options.
    FutureOptionValue(f64, Denomination),
    /// Real-time changes in futures value since last settlement.
    FuturesPnl(f64, Denomination),
    /// Cash balance in related IB-UKL account.
    FxCashBalance(f64, Denomination),
    /// Gross Position Value in securities segment.
    GrossPositionValue(f64, Denomination),
    /// Long Stock Value + Short Stock Value + Long Option Value + Short Option Value.
    GrossPositionValueSecurity(f64, Denomination),
    /// Guarantee: For internal use only.
    Guarantee(Segment<f64>, Denomination),
    /// Incentive coupon.
    IncentiveCoupons(Segment<f64>, Denomination),
    /// Margin rule for IB-IN accounts.
    IndianStockHaircut(Segment<f64>, Denomination),
    /// Initial Margin requirement of whole portfolio.
    InitMarginReq(Segment<f64>, Denomination),
    /// Real-time mark-to-market value of Issued Option.
    IssuerOptionValue(f64, Denomination),
    /// Quotient of `GrossPositionValue` and `NetLiquidation` in security segment.
    LeverageSecurity(f64),
    /// Time when look-ahead values take effect.
    LookAheadNextChange(i32),
    /// This value reflects your available funds at the next margin change.
    LookAheadAvailableFunds(Segment<f64>, Denomination),
    /// This value reflects your excess liquidity at the next margin change.
    LookAheadExcessLiquidity(Segment<f64>, Denomination),
    /// Initial margin requirement of whole portfolio as of next period's margin change.
    LookAheadInitMarginReq(Segment<f64>, Denomination),
    /// Maintenance margin requirement of whole portfolio as of next period's margin change.
    LookAheadMaintenanceMarginReq(Segment<f64>, Denomination),
    /// Maintenance Margin requirement of whole portfolio.
    MaintenanceMarginReq(Segment<f64>, Denomination),
    /// Market value of money market funds excluding mutual funds.
    MoneyMarketFundValue(f64, Denomination),
    /// Market value of mutual funds excluding money market funds.
    MutualFundValue(f64, Denomination),
    /// In review margin: Internal use only
    NlvAndMarginInReview(bool),
    /// The sum of the Dividend Payable/Receivable Values for the securities and commodities segments of the account.
    NetDividend(f64, Denomination),
    /// The basis for determining the price of the assets in your account.
    NetLiquidation(Segment<f64>, Denomination),
    /// Net liquidation for individual currencies.
    NetLiquidationByCurrency(f64, Denomination),
    /// Net liquidation uncertainty.
    NetLiquidationUncertainty(f64, Currency),
    /// Real-time mark-to-market value of options.
    OptionMarketValue(f64, Denomination),
    /// Personal Account shares value of whole portfolio.
    PaSharesValue(Segment<f64>, Denomination),
    /// Physical certificate value: Internal use only
    PhysicalCertificateValue(Segment<f64>, Denomination),
    /// Total projected "at expiration" excess liquidity.
    PostExpirationExcess(Segment<f64>, Denomination),
    /// Total projected "at expiration" margin.
    PostExpirationMargin(Segment<f64>, Denomination),
    /// Marginable Equity with Loan value as of 16:00 ET the previous day in securities segment.
    PreviousDayEquityWithLoanValue(f64, Denomination),
    /// `IMarginable` Equity with Loan value as of 16:00 ET the previous day.
    PreviousDayEquityWithLoanValueSecurity(f64, Denomination),
    /// Open positions are grouped by currency.
    RealCurrency(Denomination),
    /// Shows your profit on closed positions, which is the difference between your entry execution cost and exit execution costs, or (execution price + commissions to open the positions) - (execution price + commissions to close the position).
    RealizedPnL(f64, Denomination),
    /// Regulation T equity for universal account.
    RegTEquity(f64, Denomination),
    /// Regulation T equity for security segment.
    RegTEquitySecurity(f64, Denomination),
    /// Regulation T margin for universal account.
    RegTMargin(f64, Denomination),
    /// Regulation T margin for security segment.
    RegTMarginSecurity(f64, Denomination),
    /// Line of credit created when the market value of securities in a Regulation T account increase in value.
    Sma(f64, Denomination),
    /// Regulation T Special Memorandum Account balance for security segment.
    SmaSecurity(f64, Denomination),
    /// Account segment name.
    SegmentTitle(Segment<f64>, Denomination),
    /// Real-time mark-to-market value of stock.
    StockMarketValue(f64, Denomination),
    /// Value of treasury bonds.
    TBondValue(f64, Denomination),
    /// Value of treasury bills.
    TBillValue(f64, Denomination),
    /// Total Cash Balance including Future PNL.
    TotalCashBalance(f64, Denomination),
    /// Total cash value of stock, commodities and securities.
    TotalCashValue(Segment<f64>, Denomination),
    /// Total debit card pending charges.
    TotalDebitCardPendingCharges(Segment<f64>, Denomination),
    /// Account Type.
    TradingTypeSecurity(String),
    /// The difference between the current market value of your open positions and the average cost, or Value - Average Cost.
    UnrealizedPnL(f64, Denomination),
    /// Value of warrants.
    WarrantValue(f64, Denomination),
    /// To check projected margin requirements under Portfolio Margin model.
    WhatIfPMEnabled(bool),
}

#[derive(Debug, Clone, Error)]
#[error("Invalid value encountered when attempting to parse attribute. Cause: {0}")]
/// An error returned when parsing an [`Attribute`] fails.
pub enum ParseAttributeError {
    #[error("Failed to parse floating point attribute {attribute_name}. Cause: {float_error}")]
    /// Failed to parse float attribute
    Float {
        /// The name of the attribute
        attribute_name: &'static str,
        /// The underlying error
        float_error: ParseFloatError,
    },
    #[error("Failed to parse integer attribute {attribute_name}. Cause: {int_error}")]
    /// Failed to parse int attribute
    Int {
        /// The name of the attribute
        attribute_name: &'static str,
        /// The underlying error
        int_error: ParseIntError,
    },
    #[error("Failed to parse day trades attribute {attribute_name}. Cause: {day_trades_error}")]
    /// Failed to parse [`RemainingDayTrades`] attribute
    DayTrades {
        /// The name of the attribute
        attribute_name: &'static str,
        /// The underlying error
        day_trades_error: ParseDayTradesError,
    },
    #[error("Failed to parse boolean attribute {attribute_name}. Cause: {bool_error}")]
    /// Failed to parse [`bool`] attribute
    Bool {
        /// The name of the attribute
        attribute_name: &'static str,
        /// The underlying error
        bool_error: ParseBoolError,
    },
    #[error(
        "Failed to parse denomination attribute {attribute_name}. Cause: {denomination_error}"
    )]
    /// Failed to parse [`Denomination`] attribute
    Denomination {
        /// The name of the attribute
        attribute_name: &'static str,
        /// The underlying error
        denomination_error: ParseCurrencyError,
    },
    #[error("No such attribute {0}")]
    /// No such attribute exists
    NoSuchAttribute(String),
}

impl From<(&'static str, ParseFloatError)> for ParseAttributeError {
    fn from(value: (&'static str, ParseFloatError)) -> Self {
        Self::Float {
            attribute_name: value.0,
            float_error: value.1,
        }
    }
}

impl From<(&'static str, ParseIntError)> for ParseAttributeError {
    fn from(value: (&'static str, ParseIntError)) -> Self {
        Self::Int {
            attribute_name: value.0,
            int_error: value.1,
        }
    }
}

impl From<(&'static str, ParseDayTradesError)> for ParseAttributeError {
    fn from(value: (&'static str, ParseDayTradesError)) -> Self {
        Self::DayTrades {
            attribute_name: value.0,
            day_trades_error: value.1,
        }
    }
}

impl From<(&'static str, ParseBoolError)> for ParseAttributeError {
    fn from(value: (&'static str, ParseBoolError)) -> Self {
        Self::Bool {
            attribute_name: value.0,
            bool_error: value.1,
        }
    }
}

impl From<(&'static str, ParseCurrencyError)> for ParseAttributeError {
    fn from(value: (&'static str, ParseCurrencyError)) -> Self {
        Self::Denomination {
            attribute_name: value.0,
            denomination_error: value.1,
        }
    }
}

impl From<(&'static str, std::convert::Infallible)> for ParseAttributeError {
    fn from(_value: (&'static str, std::convert::Infallible)) -> Self {
        unreachable!()
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
/// The particular account groups managed by a given client.
pub enum Group {
    /// All accounts to which a given user has access.
    All,
    /// A specific account.
    Name(String),
}

impl FromStr for Group {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "All" => Self::All,
            _ => Self::Name(s.to_owned()),
        })
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "segment")]
/// The intra-account segments of various values.
pub enum Segment<T> {
    /// The total value across an entire account.
    Total(T),
    /// The value for US Commodities.
    Commodity(T),
    /// The value for Crypto at Paxos.
    Paxos(T),
    /// The value for US Securities.
    Security(T),
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
/// The denomination of a given value.
pub enum Denomination {
    /// The base currency for the corresponding account.
    Base,
    /// A specific [`Currency`]
    Specific(Currency),
}

impl FromStr for Denomination {
    type Err = ParseCurrencyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "BASE" => Ok(Self::Base),
            c => Ok(Self::Specific(c.parse()?)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
/// Represents the possible numbers of day trades before a regulatory breach of pattern day-trading
/// rules is committed.
pub enum RemainingDayTrades {
    /// No limits on the number of day trades.
    Unlimited,
    /// A specified number of day trades are remaining.
    Count(u32),
}

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
/// An error type that represents an invalid value encountered while parsing the numer of remaining
/// day trades.
pub struct ParseDayTradesError(String);

impl std::fmt::Display for ParseDayTradesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Unable to parse day trades information. Unexpected count {}",
            self.0
        )
    }
}

impl std::error::Error for ParseDayTradesError {}

impl FromStr for RemainingDayTrades {
    type Err = ParseDayTradesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-1" => Ok(Self::Unlimited),
            u => Ok(Self::Count(
                u.parse().map_err(|_| ParseDayTradesError(u.to_owned()))?,
            )),
        }
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
/// Represents the different tag and value pairs in an account summary callback.
pub enum TagValue {
    /// A tag whose value is a String
    String(Tag, String),
    /// A tag whose value is an integer (i64)
    Int(Tag, i64),
    /// A tag whose valued is a float (f64)
    Float(Tag, f64),
    /// A tag whose value is a float (f64), Currency pair
    Currency(Tag, f64, Currency),
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Represents the different types of account information available for a
/// [`crate::client::Client::req_account_summary`] request.
pub enum Tag {
    /// Identifies the IB account structure
    AccountType,
    /// The basis for determining the price of the assets in your account. Total cash value + stock value + options value + bond value.
    NetLiquidation,
    /// Total cash balance recognized at the time of trade + futures PNL.
    TotalCashValue,
    /// Cash recognized at the time of settlement - purchases at the time of trade - commissions - taxes - fees.
    SettledCash,
    /// Total accrued cash value of stock, commodities and securities.
    AccruedCash,
    /// Buying power serves as a measurement of the dollar value of securities that one may purchase in a securities account without depositing additional funds.
    BuyingPower,
    /// Forms the basis for determining whether a client has the necessary assets to either initiate or maintain security positions. Cash + stocks + bonds + mutual funds.
    EquityWithLoanValue,
    /// Marginable Equity with Loan value as of 16:00 ET the previous day.
    PreviousEquityWithLoanValue,
    /// The sum of the absolute value of all stock and equity option positions.
    GrossPositionValue,
    /// Regulation T equity for universal account.
    RegTEquity,
    /// Regulation T margin for universal account.
    RegTMargin,
    #[serde(rename(serialize = "SMA"))]
    /// Special Memorandum Account: Line of credit created when the market value of securities in a Regulation T account increase in value.
    Sma,
    /// Initial Margin requirement of whole portfolio.
    InitMarginReq,
    #[serde(rename(serialize = "MaintMarginReq"))]
    /// Maintenance Margin requirement of whole portfolio.
    MaintenanceMarginReq,
    /// This value tells what you have available for trading.
    AvailableFunds,
    /// This value shows your margin cushion, before liquidation.
    ExcessLiquidity,
    /// Excess liquidity as a percentage of net liquidation value.
    Cushion,
    /// Initial Margin of whole portfolio with no discounts or intraday credits.
    FullInitMarginReq,
    #[serde(rename(serialize = "FullMaintMarginReq"))]
    /// Maintenance Margin of whole portfolio with no discounts or intraday credits.
    FullMaintenanceMarginReq,
    /// Available funds of whole portfolio with no discounts or intraday credits.
    FullAvailableFunds,
    /// Excess liquidity of whole portfolio with no discounts or intraday credits.
    FullExcessLiquidity,
    /// Time when look-ahead values take effect.
    LookAheadNextChange,
    /// Initial Margin requirement of whole portfolio as of next period's margin change.
    LookAheadInitMarginReq,
    #[serde(rename(serialize = "LookAheadMaintMarginReq"))]
    /// Maintenance Margin requirement of whole portfolio as of next period's margin change.
    LookAheadMaintenanceMarginReq,
    /// This value reflects your available funds at the next margin change.
    LookAheadAvailableFunds,
    /// This value reflects your excess liquidity at the next margin change.
    LookAheadExcessLiquidity,
    /// A measure of how close the account is to liquidation.
    HighestSeverity,
    /// The Number of Open/Close trades a user could put on before Pattern Day Trading is detected. A value of "-1" means that the user can put on unlimited day trades.
    DayTradesRemaining,
    /// Quotient of `GrossPositionValue` and `NetLiquidation`.
    Leverage,
}

#[derive(Debug, Clone, Copy, Error)]
#[error("Invalid value encountered when parsing tag.")]
/// An error returned when attempting to parse a [`Tag`]
pub struct ParseTagError;

impl FromStr for Tag {
    type Err = ParseTagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "AccountType" => Self::AccountType,
            "NetLiquidation" => Self::NetLiquidation,
            "TotalCashValue" => Self::TotalCashValue,
            "SettledCash" => Self::SettledCash,
            "AccruedCash" => Self::AccruedCash,
            "BuyingPower" => Self::BuyingPower,
            "EquityWithLoanValue" => Self::EquityWithLoanValue,
            "PreviousEquityWithLoanValue" => Self::PreviousEquityWithLoanValue,
            "GrossPositionValue" => Self::GrossPositionValue,
            "RegTEquity" => Self::RegTEquity,
            "RegTMargin" => Self::RegTMargin,
            "SMA" => Self::Sma,
            "InitMarginReq" => Self::InitMarginReq,
            "MaintMarginReq" => Self::MaintenanceMarginReq,
            "AvailableFunds" => Self::AvailableFunds,
            "ExcessLiquidity" => Self::ExcessLiquidity,
            "Cushion" => Self::Cushion,
            "FullInitMarginReq" => Self::FullInitMarginReq,
            "FullMaintMarginReq" => Self::FullMaintenanceMarginReq,
            "FullAvailableFunds" => Self::FullAvailableFunds,
            "FullExcessLiquidity" => Self::FullExcessLiquidity,
            "LookAheadNextChange" => Self::LookAheadNextChange,
            "LookAheadInitMarginReq" => Self::LookAheadInitMarginReq,
            "LookAheadMaintMarginReq" => Self::LookAheadMaintenanceMarginReq,
            "LookAheadAvailableFunds" => Self::LookAheadAvailableFunds,
            "LookAheadExcessLiquidity" => Self::LookAheadExcessLiquidity,
            "HighestSeverity" => Self::HighestSeverity,
            "DayTradesRemaining" => Self::DayTradesRemaining,
            "Leverage" => Self::Leverage,
            _ => return Err(ParseTagError),
        })
    }
}
