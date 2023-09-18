use crate::currency::Currency;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
/// Represents a specific account value
pub enum Value {
    /// The account ID number.
    AccountCode(String),
    /// "All" to return account summary data for all accounts, or set to a specific Advisor Account Group name that has already been created in TWS Global Configuration.
    AccountOrGroup(Group, AccountCurrency),
    /// For internal use only.
    AccountReady(bool),
    /// Identifies the IB account structure.
    AccountType(String),
    /// Accrued cash value of stock, commodities and securities.
    AccruedCash(Segment<f64>, AccountCurrency),
    /// Value of dividends accrued.
    AccruedDividend(Segment<f64>, AccountCurrency),
    /// This value tells what you have available for trading.
    AvailableFunds(Segment<f64>, AccountCurrency),
    /// Value of treasury bills.
    Billable(Segment<f64>, AccountCurrency),
    /// Cash Account: Minimum (Equity with Loan Value, Previous Day Equity with Loan Value)-Initial Margin, Standard Margin Account: Minimum (Equity with Loan Value, Previous Day Equity with Loan Value) - Initial Margin *4.
    BuyingPower(f64, AccountCurrency),
    /// Cash recognized at the time of trade + futures PNL.
    CashBalance(f64, AccountCurrency),
    /// Value of non-Government bonds such as corporate bonds and municipal bonds.
    CorporateBondValue(f64, AccountCurrency),
    /// Value of cryptocurrency positions at PAXOS.
    Cryptocurrency(f64, AccountCurrency),
    /// Open positions are grouped by currency.
    Currency(f64, AccountCurrency),
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
    EquityWithLoanValue(Segment<f64>, AccountCurrency),
    /// This value shows your margin cushion, before liquidation.
    ExcessLiquidity(Segment<f64>, AccountCurrency),
    /// The exchange rate of the currency to your base currency.
    ExchangeRate(f64, AccountCurrency),
    /// Available funds of whole portfolio with no discounts or intraday credits.
    FullAvailableFunds(Segment<f64>, AccountCurrency),
    /// Excess liquidity of whole portfolio with no discounts or intraday credits.
    FullExcessLiquidity(Segment<f64>, AccountCurrency),
    /// Initial Margin of whole portfolio with no discounts or intraday credits.
    FullInitMarginReq(Segment<f64>, AccountCurrency),
    /// Maintenance Margin of whole portfolio with no discounts or intraday credits.
    FullMaintMarginReq(Segment<f64>, AccountCurrency),
    /// Value of funds value (money market funds + mutual funds).
    FundValue(f64, AccountCurrency),
    /// Real-time market-to-market value of futures options.
    FutureOptionValue(f64, AccountCurrency),
    /// Real-time changes in futures value since last settlement.
    FuturesPNL(f64, AccountCurrency),
    /// Cash balance in related IB-UKL account.
    FxCashBalance(f64, AccountCurrency),
    /// Gross Position Value in securities segment.
    GrossPositionValue(f64, AccountCurrency),
    /// Long Stock Value + Short Stock Value + Long Option Value + Short Option Value.
    GrossPositionValueSecurity(f64, AccountCurrency),
    /// Guarantee: For internal use only.
    Guarantee(Segment<f64>, AccountCurrency),
    /// Margin rule for IB-IN accounts.
    IndianStockHaircut(Segment<f64>, AccountCurrency),
    /// Initial Margin requirement of whole portfolio.
    InitMarginReq(Segment<f64>, AccountCurrency),
    /// Real-time mark-to-market value of Issued Option.
    IssuerOptionValue(f64, AccountCurrency),
    /// GrossPositionValue / NetLiquidation in security segment.
    LeverageSecurity(f64),
    /// Time when look-ahead values take effect.
    LookAheadNextChange(Option<String>),
    /// This value reflects your available funds at the next margin change.
    LookAheadAvailableFunds(Segment<f64>, AccountCurrency),
    /// This value reflects your excess liquidity at the next margin change.
    LookAheadExcessLiquidity(Segment<f64>, AccountCurrency),
    /// Initial margin requirement of whole portfolio as of next period's margin change.
    LookAheadInitMarginReq(Segment<f64>, AccountCurrency),
    /// Maintenance margin requirement of whole portfolio as of next period's margin change.
    LookAheadMaintMarginReq(Segment<f64>, AccountCurrency),
    /// Maintenance Margin requirement of whole portfolio.
    MaintMarginReq(Segment<f64>, AccountCurrency),
    /// Market value of money market funds excluding mutual funds.
    MoneyMarketFundValue(f64, AccountCurrency),
    /// Market value of mutual funds excluding money market funds.
    MutualFundValue(f64, AccountCurrency),
    /// In review margin: Internal use only
    NLVAndMarginInReview(bool),
    /// The sum of the Dividend Payable/Receivable Values for the securities and commodities segments of the account.
    NetDividend(f64, AccountCurrency),
    /// The basis for determining the price of the assets in your account.
    NetLiquidation(Segment<f64>, AccountCurrency),
    /// Net liquidation for individual currencies.
    NetLiquidationByCurrency(f64, Currency),
    /// Real-time mark-to-market value of options.
    OptionMarketValue(f64, Currency),
    /// Personal Account shares value of whole portfolio.
    PASharesValue(Segment<f64>, AccountCurrency),
    /// Physical certificate value: Internal use only
    PhysicalCertificateValue(Segment<f64>, AccountCurrency),
    /// Total projected "at expiration" excess liquidity.
    PostExpirationExcess(Segment<f64>, AccountCurrency),
    /// Total projected "at expiration" margin.
    PostExpirationMargin(Segment<f64>, AccountCurrency),
    /// Marginable Equity with Loan value as of 16:00 ET the previous day in securities segment.
    PreviousDayEquityWithLoanValue(f64, AccountCurrency),
    /// IMarginable Equity with Loan value as of 16:00 ET the previous day.
    PreviousDayEquityWithLoanValueSecurity(f64, AccountCurrency),
    /// Open positions are grouped by currency.
    RealCurrency(AccountCurrency, AccountCurrency),
    /// Shows your profit on closed positions, which is the difference between your entry execution cost and exit execution costs, or (execution price + commissions to open the positions) - (execution price + commissions to close the position).
    RealizedPnL(f64, AccountCurrency),
    /// Regulation T equity for universal account.
    RegTEquity(f64, AccountCurrency),
    /// Regulation T equity for security segment.
    RegTEquitySecurity(f64, AccountCurrency),
    /// Regulation T margin for universal account.
    RegTMargin(f64, AccountCurrency),
    /// Regulation T margin for security segment.
    RegTMarginSecurity(f64, AccountCurrency),
    /// Line of credit created when the market value of securities in a Regulation T account increase in value.
    Sma(f64, AccountCurrency),
    /// Regulation T Special Memorandum Account balance for security segment.
    SmaSecurity(f64, AccountCurrency),
    /// Account segment name.
    SegmentTitle(Segment<f64>, AccountCurrency),
    /// Real-time mark-to-market value of stock.
    StockMarketValue(f64, AccountCurrency),
    /// Value of treasury bonds.
    TBondValue(f64, AccountCurrency),
    /// Value of treasury bills.
    TBillValue(f64, AccountCurrency),
    /// Total Cash Balance including Future PNL.
    TotalCashBalance(f64, AccountCurrency),
    /// Total cash value of stock, commodities and securities.
    TotalCashValue(Segment<f64>, AccountCurrency),
    /// CashBalance in commodity segment.
    TotalCashValueCommodity(f64, AccountCurrency),
    /// CashBalance in security segment.
    TotalCashValueSecurity(Segment<f64>, AccountCurrency),
    /// Account Type.
    TradingTypeSecurity(String),
    /// The difference between the current market value of your open positions and the average cost, or Value - Average Cost.
    UnrealizedPnL(f64, AccountCurrency),
    /// Value of warrants.
    WarrantValue(f64, AccountCurrency),
    /// To check projected margin requirements under Portfolio Margin model.
    WhatIfPMEnabled(bool),
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Group {
    All,
    Name(String),
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Segment<T> {
    Total(T),
    Commodity(T),
    P(T),
    Security(T),
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Hash)]
pub enum AccountCurrency {
    Base,
    Specific(Currency)
}

impl std::str::FromStr for AccountCurrency {
    type Err = crate::currency::ParseCurrencyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "BASE" => Ok(Self::Base),
            c=> Ok(Self::Specific(c.parse()?))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Hash)]
pub enum RemainingDayTrades {
    Unlimited,
    Count(u32),
}
