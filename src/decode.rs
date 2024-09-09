use core::future::Future;

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use thiserror::Error;

use crate::{
    currency::Currency,
    exchange::Routing,
    message::{ToClient, ToWrapper},
    timezone, wrapper,
};
use crate::account::{self, ParseAttributeError, Tag, TagValue};
use crate::contract::{
    Commodity, Contract, ContractId, ContractType, Crypto, Forex, Index, Proxy, SecFuture,
    SecOption, SecOptionInner, SecurityId, Stock,
};
use crate::exchange::Primary;
use crate::execution::{Exec, Execution, OrderSide, ParseOrderSideError};
use crate::payload::{
    Bar,
    BarCore, BidAsk, ExchangeId, Fill, HistogramEntry, Last, market_depth::{CompleteEntry, Entry, Operation}, MarketDataClass, Midpoint,
    ParsePayloadError, Pnl, PnlSingle, Position, PositionSummary, TickData, Trade,
};
use crate::tick::{
    Accessibility, AuctionData, CalculationResult, Class, Dividends, EtfNav, ExtremeValue, Ipo,
    MarkPrice, OpenInterest, Period, Price, PriceFactor, QuotingExchanges, Rate, RealTimeVolume,
    RealTimeVolumeBase, SecOptionCalculationResults, SecOptionCalculations,
    SecOptionCalculationSource, SecOptionVolume, Size, SummaryVolume, TimeStamp, Volatility, Yield,
};
use crate::timezone::ParseTimezoneError;

type Tx = tokio::sync::mpsc::Sender<ToClient>;
type Rx = tokio::sync::mpsc::Receiver<ToWrapper>;
type Fields = std::vec::IntoIter<String>;
type DecodeResult = Result<(), DecodeError>;

macro_rules! decode_fields {
    ($fields: expr => $f_name: ident @ $ind: literal: String) => {
        let $f_name = nth($fields, $ind, stringify!($f_name))?;
    };
    ($fields: expr => $f_name: ident @ $ind: literal: Option<$op_f_type: ty>) => {
        let $f_name = match nth($fields, $ind, stringify!($f_name))?.as_str() {
            "" => None::<$op_f_type>,
            s => Some(s.parse::<$op_f_type>().map_err(|e| DecodeError::from((stringify!($f_name), e)))?)
        };
    };
    ($fields: expr => $f_name: ident @ $ind: literal: $f_type: ty) => {
        let $f_name = nth($fields, $ind, stringify!($f_name))?
            .parse::<$f_type>().map_err(|e| DecodeError::from((stringify!($f_name), e)))?;
    };
    ($fields: expr => $($f_name: ident @ $ind: literal: $f_type: ty ),* $(,)?) => {
        $(
            decode_fields!($fields => $f_name @ $ind: $f_type);
        )*
    };
}

macro_rules! decode_account_attr {
    ($attr_var: ident, $value: expr, $currency: expr) => {
        account::Attribute::$attr_var(
            $value
                .parse()
                .map_err(|e| ParseAttributeError::from((stringify!($attr_var), e)))?,
            $currency
                .parse()
                .map_err(|e| ParseAttributeError::from((stringify!($attr_var), e)))?,
        )
    };
    ($attr_var: ident, $value: expr) => {
        account::Attribute::$attr_var(
            $value
                .parse()
                .map_err(|e| ParseAttributeError::from((stringify!($attr_var), e)))?,
        )
    };
}

macro_rules! expand_seg_variants {
    ($root_name: literal) => {
        $root_name
            | concat!($root_name, "-C")
            | concat!($root_name, "-P")
            | concat!($root_name, "-S")
    };
}

macro_rules! impl_seg_variants {
    ($root_name: literal, $attr_var: ident, $name: expr, $value: expr, $currency: expr) => {{
        match $name.as_str() {
            concat!($root_name) => account::Attribute::$attr_var(
                account::Segment::Total(
                    $value
                        .parse()
                        .map_err(|e| ParseAttributeError::from((stringify!($attr_var), e)))?,
                ),
                $currency
                    .parse()
                    .map_err(|e| ParseAttributeError::from((stringify!($attr_var), e)))?,
            ),
            concat!($root_name, "-C") => account::Attribute::$attr_var(
                account::Segment::Commodity(
                    $value
                        .parse()
                        .map_err(|e| ParseAttributeError::from((stringify!($attr_var), e)))?,
                ),
                $currency
                    .parse()
                    .map_err(|e| ParseAttributeError::from((stringify!($attr_var), e)))?,
            ),
            concat!($root_name, "-P") => account::Attribute::$attr_var(
                account::Segment::Paxos(
                    $value
                        .parse()
                        .map_err(|e| ParseAttributeError::from((stringify!($attr_var), e)))?,
                ),
                $currency
                    .parse()
                    .map_err(|e| ParseAttributeError::from((stringify!($attr_var), e)))?,
            ),
            concat!($root_name, "-S") => account::Attribute::$attr_var(
                account::Segment::Security(
                    $value
                        .parse()
                        .map_err(|e| ParseAttributeError::from((stringify!($attr_var), e)))?,
                ),
                $currency
                    .parse()
                    .map_err(|e| ParseAttributeError::from((stringify!($attr_var), e)))?,
            ),
            _ => unreachable!(),
        }
    }};
    ($root_name: literal, $attr_var: ident, $name: expr, $value: expr) => {{
        match $name.as_str() {
            stringify!($attr_var) => account::Attribute::$attr_var(account::Segment::Total(
                $value
                    .parse()
                    .map_err(|e| ParseAttributeError::from((stringify!($attr_var), e)))?,
            )),
            concat!(stringify!($attr_var), "-C") => {
                account::Attribute::$attr_var(account::Segment::Commodity(
                    $value
                        .parse()
                        .map_err(|e| ParseAttributeError::from((stringify!($attr_var), e)))?,
                ))
            }
            concat!(stringify!($attr_var), "-P") => {
                account::Attribute::$attr_var(account::Segment::Paxos(
                    $value
                        .parse()
                        .map_err(|e| ParseAttributeError::from((stringify!($attr_var), e)))?,
                ))
            }
            concat!(stringify!($attr_var), "-S") => {
                account::Attribute::$attr_var(account::Segment::Security(
                    $value
                        .parse()
                        .map_err(|e| ParseAttributeError::from((stringify!($attr_var), e)))?,
                ))
            }
            _ => unreachable!(),
        }
    }};
}

#[ibapi_macros::make_send(Remote(Send): wrapper::Wrapper)]
pub trait Local: wrapper::LocalWrapper {
    #[inline]
    fn tick_price_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
            fields =>
                req_id @ 2: i64,
                tick_type @ 0: u16,
                price @ 0: f64
            );
            decode_fields!(
                fields => size @ 0: Option<f64>
            );
            decode_fields!(
                fields => _attr_mask @ 0: u8
            );

            if (price + 1.0).abs() < f64::EPSILON || (price == 0.0 && size == Some(0.0)) {
                return Ok(());
            }

            match tick_type {
                1 | 2 | 4 | 6 | 7 | 9 | 14 => {
                    let (price, size) = match (tick_type, size) {
                        (1, Some(sz)) => (Price::Bid(price), Some(Size::Bid(sz))),
                        (1, None) => (Price::Bid(price), None),
                        (2, Some(sz)) => (Price::Ask(price), Some(Size::Ask(sz))),
                        (2, None) => (Price::Ask(price), None),
                        (4, Some(sz)) => (Price::Last(price), Some(Size::Last(sz))),
                        (4, None) => (Price::Last(price), None),
                        (6, _) => (Price::High(price), None),
                        (7, _) => (Price::Low(price), None),
                        (9, _) => (Price::Close(price), None),
                        (14, _) => (Price::Open(price), None),
                        _ => unreachable!(),
                    };
                    wrapper.price_data(req_id, Class::Live(price)).await;
                    if let Some(sz) = size {
                        wrapper.size_data(req_id, Class::Live(sz)).await;
                    }
                }
                15..=20 => {
                    let value = match tick_type {
                        15 => ExtremeValue::Low(Period::ThirteenWeek(price)),
                        16 => ExtremeValue::High(Period::ThirteenWeek(price)),
                        17 => ExtremeValue::Low(Period::TwentySixWeek(price)),
                        18 => ExtremeValue::High(Period::TwentySixWeek(price)),
                        19 => ExtremeValue::Low(Period::FiftyTwoWeek(price)),
                        20 => ExtremeValue::High(Period::FiftyTwoWeek(price)),
                        _ => unreachable!(),
                    };
                    wrapper.extreme_data(req_id, value).await;
                }
                35 => {
                    wrapper.auction(req_id, AuctionData::Price(price)).await;
                }
                37 | 79 => {
                    let mark = match tick_type {
                        37 => MarkPrice::Standard(price),
                        79 => MarkPrice::Slow(price),
                        _ => unreachable!(),
                    };
                    wrapper.mark_price(req_id, mark).await;
                }
                50..=52 => {
                    let yld = match tick_type {
                        50 => Yield::Bid(price),
                        51 => Yield::Ask(price),
                        52 => Yield::Last(price),
                        _ => unreachable!(),
                    };
                    wrapper.yield_data(req_id, yld).await;
                }
                57 => {
                    wrapper
                        .price_data(req_id, Class::Live(Price::LastRthTrade(price)))
                        .await;
                }
                66..=68 | 72 | 73 | 75 | 76 => {
                    let (price, size) = match (tick_type, size) {
                        (66, Some(sz)) => (Price::Bid(price), Some(Size::Bid(sz))),
                        (66, None) => (Price::Bid(price), None),
                        (67, Some(sz)) => (Price::Ask(price), Some(Size::Ask(sz))),
                        (67, None) => (Price::Ask(price), None),
                        (68, Some(sz)) => (Price::Last(price), Some(Size::Last(sz))),
                        (68, None) => (Price::Last(price), None),
                        (72, _) => (Price::High(price), None),
                        (73, _) => (Price::Low(price), None),
                        (75, _) => (Price::Close(price), None),
                        (76, _) => (Price::Open(price), None),
                        _ => unreachable!(),
                    };
                    wrapper.price_data(req_id, Class::Delayed(price)).await;
                    if let Some(sz) = size {
                        wrapper.size_data(req_id, Class::Delayed(sz)).await;
                    }
                }
                92..=99 => {
                    let nav = match tick_type {
                        92 => EtfNav::Close(price),
                        93 => EtfNav::PriorClose(price),
                        94 => EtfNav::Bid(price),
                        95 => EtfNav::Ask(price),
                        96 => EtfNav::Last(price),
                        97 => EtfNav::FrozenLast(price),
                        98 => EtfNav::High(price),
                        99 => EtfNav::Low(price),
                        _ => unreachable!(),
                    };
                    wrapper.etf_nav(req_id, nav).await;
                }
                t => {
                    return Err(DecodeError::Other(format!(
                        "Unexpected price market data request: {t}"
                    )))
                }
            };
            Ok(())
        }
    }

    #[inline]
    fn tick_size_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields =>
                    req_id @ 2: i64,
                    tick_type @ 0: u16,
                    value @ 0: f64
            );
            Self::decode_generic_tick_msg(req_id, tick_type, value, wrapper).await
        }
    }

    #[inline]
    fn order_status_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
            fields =>
                order_id @ 1: i64,
                status @ 0: String,
                filled @ 0: f64,
                remaining @ 0: f64,
                average_price @ 0: f64,
                permanent_id @ 0: i64,
                parent_id @ 0: i64,
                last_price @ 0: f64,
                client_id @ 0: i64
            );
            decode_fields!(
                fields => why_held @ 0: Option<crate::payload::Locate>
            );
            decode_fields!(
                fields => market_cap_price @ 0: f64
            );

            let market_cap_price = if market_cap_price == 0.0 {
                None
            } else {
                Some(market_cap_price)
            };
            let fill = if filled == 0.0 && average_price == 0.0 && last_price == 0.0 {
                None
            } else {
                Some(Fill {
                    filled,
                    average_price,
                    last_price,
                })
            };
            let parent_id = if parent_id == 0 {
                None
            } else {
                Some(parent_id)
            };
            let core = crate::payload::OrderStatusCore {
                order_id,
                fill,
                remaining,
                permanent_id,
                parent_id,
                client_id,
                why_held,
                market_cap_price,
            };

            wrapper
                .order_status(
                    (status.as_str(), core)
                        .try_into()
                        .map_err(|e| ("order_status", e))?,
                )
                .await;

            Ok(())
        }
    }

    #[inline]
    // todo: Implement a proper Error Enum
    fn err_msg_msg(fields: &mut Fields, wrapper: &mut Self) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields =>
                    req_id @ 2: i64,
                    error_code @ 0: i64,
                    error_string @ 0: String,
                    advanced_order_reject_json @ 0: String
            );
            wrapper
                .error(req_id, error_code, error_string, advanced_order_reject_json)
                .await;
            Ok(())
        }
    }

    #[inline]
    fn open_order_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields =>
                    order_id @ 1: i64,
            );
            let proxy = deserialize_contract_proxy::<crate::contract::proxy_indicators::HasExchange>(
                fields,
            )?;
            decode_fields!(
                fields =>
                    client_id @ 10: i64,
                    permanent_id @ 0: i64,
                    parent_id @ 32: i64,
            );
            let parent_id = if parent_id == 0 {
                None
            } else {
                Some(parent_id)
            };
            wrapper
                .open_order(order_id, proxy, client_id, parent_id, permanent_id)
                .await;

            Ok(())
        }
    }

    #[inline]
    fn acct_value_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields =>
                    name @ 2: String,
                    value @ 0: String,
                    currency @ 0: String,
                    account_number @ 0: String
            );
            let attribute = match name.as_str() {
                "AccountCode" => account::Attribute::AccountCode(value),
                "AccountOrGroup" => decode_account_attr!(AccountOrGroup, value, currency),
                "AccountReady" => decode_account_attr!(AccountReady, value),
                "AccountType" => account::Attribute::AccountType(value),
                expand_seg_variants!("AccruedCash") => {
                    impl_seg_variants!("AccruedCash", AccruedCash, name, value, currency)
                }
                expand_seg_variants!("AccruedDividend") => {
                    impl_seg_variants!("AccruedDividend", AccruedDividend, name, value, currency)
                }
                expand_seg_variants!("AvailableFunds") => {
                    impl_seg_variants!("AvailableFunds", AvailableFunds, name, value, currency)
                }
                expand_seg_variants!("Billable") => {
                    impl_seg_variants!("Billable", Billable, name, value, currency)
                }
                "BuyingPower" => decode_account_attr!(BuyingPower, value, currency),
                "CashBalance" => decode_account_attr!(CashBalance, value, currency),
                expand_seg_variants!("ColumnPrio") => {
                    impl_seg_variants!("ColumnPrio", ColumnPrio, name, value)
                }
                "CorporateBondValue" => decode_account_attr!(CorporateBondValue, value, currency),
                "Cryptocurrency" => decode_account_attr!(Cryptocurrency, value, currency),
                "Currency" => decode_account_attr!(Currency, value),
                "Cushion" => decode_account_attr!(Cushion, value),
                "DayTradesRemaining" => decode_account_attr!(DayTradesRemaining, value),
                "DayTradesRemainingT+1" => decode_account_attr!(DayTradesRemainingTPlus1, value),
                "DayTradesRemainingT+2" => decode_account_attr!(DayTradesRemainingTPlus2, value),
                "DayTradesRemainingT+3" => decode_account_attr!(DayTradesRemainingTPlus3, value),
                "DayTradesRemainingT+4" => decode_account_attr!(DayTradesRemainingTPlus4, value),
                "DayTradingStatus-S" => account::Attribute::DayTradingStatus(value),
                expand_seg_variants!("EquityWithLoanValue") => impl_seg_variants!(
                    "EquityWithLoanValue",
                    EquityWithLoanValue,
                    name,
                    value,
                    currency
                ),
                expand_seg_variants!("ExcessLiquidity") => {
                    impl_seg_variants!("ExcessLiquidity", ExcessLiquidity, name, value, currency)
                }
                "ExchangeRate" => decode_account_attr!(ExchangeRate, value, currency),
                expand_seg_variants!("FullAvailableFunds") => impl_seg_variants!(
                    "FullAvailableFunds",
                    FullAvailableFunds,
                    name,
                    value,
                    currency
                ),
                expand_seg_variants!("FullExcessLiquidity") => impl_seg_variants!(
                    "FullExcessLiquidity",
                    FullExcessLiquidity,
                    name,
                    value,
                    currency
                ),
                expand_seg_variants!("FullInitMarginReq") => impl_seg_variants!(
                    "FullInitMarginReq",
                    FullInitMarginReq,
                    name,
                    value,
                    currency
                ),
                expand_seg_variants!("FullMaintMarginReq") => impl_seg_variants!(
                    "FullMaintMarginReq",
                    FullMaintenanceMarginReq,
                    name,
                    value,
                    currency
                ),
                "FundValue" => decode_account_attr!(FundValue, value, currency),
                "FutureOptionValue" => decode_account_attr!(FutureOptionValue, value, currency),
                "FuturesPNL" => decode_account_attr!(FuturesPnl, value, currency),
                "FxCashBalance" => decode_account_attr!(FxCashBalance, value, currency),
                "GrossPositionValue" => decode_account_attr!(GrossPositionValue, value, currency),
                "GrossPositionValue-S" => {
                    decode_account_attr!(GrossPositionValueSecurity, value, currency)
                }
                expand_seg_variants!("Guarantee") => {
                    impl_seg_variants!("Guarantee", Guarantee, name, value, currency)
                }
                expand_seg_variants!("IncentiveCoupons") => {
                    impl_seg_variants!("IncentiveCoupons", IncentiveCoupons, name, value, currency)
                }
                expand_seg_variants!("IndianStockHaircut") => impl_seg_variants!(
                    "IndianStockHaircut",
                    IndianStockHaircut,
                    name,
                    value,
                    currency
                ),
                expand_seg_variants!("InitMarginReq") => {
                    impl_seg_variants!("InitMarginReq", InitMarginReq, name, value, currency)
                }
                "IssuerOptionValue" => decode_account_attr!(IssuerOptionValue, value, currency),
                "Leverage-S" => decode_account_attr!(LeverageSecurity, value),
                "LookAheadNextChange" => decode_account_attr!(LookAheadNextChange, value),
                expand_seg_variants!("LookAheadAvailableFunds") => impl_seg_variants!(
                    "LookAheadAvailableFunds",
                    LookAheadAvailableFunds,
                    name,
                    value,
                    currency
                ),
                expand_seg_variants!("LookAheadExcessLiquidity") => impl_seg_variants!(
                    "LookAheadExcessLiquidity",
                    LookAheadExcessLiquidity,
                    name,
                    value,
                    currency
                ),
                expand_seg_variants!("LookAheadInitMarginReq") => impl_seg_variants!(
                    "LookAheadInitMarginReq",
                    LookAheadInitMarginReq,
                    name,
                    value,
                    currency
                ),
                expand_seg_variants!("LookAheadMaintMarginReq") => impl_seg_variants!(
                    "LookAheadMaintMarginReq",
                    LookAheadMaintenanceMarginReq,
                    name,
                    value,
                    currency
                ),
                expand_seg_variants!("MaintMarginReq") => impl_seg_variants!(
                    "MaintMarginReq",
                    MaintenanceMarginReq,
                    name,
                    value,
                    currency
                ),
                "MoneyMarketFundValue" => {
                    decode_account_attr!(MoneyMarketFundValue, value, currency)
                }
                "MutualFundValue" => decode_account_attr!(MutualFundValue, value, currency),
                "NLVAndMarginInReview" => decode_account_attr!(NlvAndMarginInReview, value),
                "NetDividend" => decode_account_attr!(NetDividend, value, currency),
                expand_seg_variants!("NetLiquidation") => {
                    impl_seg_variants!("NetLiquidation", NetLiquidation, name, value, currency)
                }
                "NetLiquidationByCurrency" => {
                    decode_account_attr!(NetLiquidationByCurrency, value, currency)
                }
                "NetLiquidationUncertainty" => {
                    decode_account_attr!(NetLiquidationUncertainty, value, currency)
                }
                "OptionMarketValue" => decode_account_attr!(OptionMarketValue, value, currency),
                expand_seg_variants!("PASharesValue") => {
                    impl_seg_variants!("PASharesValue", PaSharesValue, name, value, currency)
                }
                expand_seg_variants!("PhysicalCertificateValue") => impl_seg_variants!(
                    "PhysicalCertificateValue",
                    PhysicalCertificateValue,
                    name,
                    value,
                    currency
                ),
                expand_seg_variants!("PostExpirationExcess") => impl_seg_variants!(
                    "PostExpirationExcess",
                    PostExpirationExcess,
                    name,
                    value,
                    currency
                ),
                expand_seg_variants!("PostExpirationMargin") => impl_seg_variants!(
                    "PostExpirationMargin",
                    PostExpirationMargin,
                    name,
                    value,
                    currency
                ),
                "PreviousDayEquityWithLoanValue" => {
                    decode_account_attr!(PreviousDayEquityWithLoanValue, value, currency)
                }
                "PreviousDayEquityWithLoanValue-S" => {
                    decode_account_attr!(PreviousDayEquityWithLoanValueSecurity, value, currency)
                }
                "RealCurrency" => decode_account_attr!(RealCurrency, value),
                "RealizedPnL" => decode_account_attr!(RealizedPnL, value, currency),
                "RegTEquity" => decode_account_attr!(RegTEquity, value, currency),
                "RegTEquity-S" => decode_account_attr!(RegTEquitySecurity, value, currency),
                "RegTMargin" => decode_account_attr!(RegTMargin, value, currency),
                "RegTMargin-S" => decode_account_attr!(RegTMarginSecurity, value, currency),
                "SMA" => decode_account_attr!(Sma, value, currency),
                "SMA-S" => decode_account_attr!(SmaSecurity, value, currency),
                "StockMarketValue" => decode_account_attr!(StockMarketValue, value, currency),
                "TBillValue" => decode_account_attr!(TBillValue, value, currency),
                "TBondValue" => decode_account_attr!(TBondValue, value, currency),
                "TotalCashBalance" => decode_account_attr!(TotalCashBalance, value, currency),
                expand_seg_variants!("TotalCashValue") => {
                    impl_seg_variants!("TotalCashValue", TotalCashValue, name, value, currency)
                }
                expand_seg_variants!("TotalDebitCardPendingCharges") => impl_seg_variants!(
                    "TotalDebitCardPendingCharges",
                    TotalDebitCardPendingCharges,
                    name,
                    value,
                    currency
                ),
                "TradingType-S" => account::Attribute::TradingTypeSecurity(value),
                "UnrealizedPnL" => decode_account_attr!(UnrealizedPnL, value, currency),
                "WarrantValue" => decode_account_attr!(WarrantValue, value, currency),
                "WhatIfPMEnabled" => decode_account_attr!(WhatIfPMEnabled, value),
                expand_seg_variants!("SegmentTitle") => {
                    if name.ends_with('C') || name.ends_with('P') || name.ends_with('S') {
                        return Ok(());
                    }
                    return Err(ParseAttributeError::NoSuchAttribute(format!("Unexpected segment title \"{name}\" encountered. This may mandate an API update: currently-supported values are C, P, and S as outlined in the account::Segment type.")).into());
                }
                _ => return Err(ParseAttributeError::NoSuchAttribute(name).into()),
            };
            wrapper.account_attribute(attribute, account_number).await;
            Ok(())
        }
    }

    #[inline]
    fn portfolio_value_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            let _ = fields.nth(1);
            let proxy = deserialize_contract_proxy(fields)?;
            decode_fields!(
                fields =>
                    position @ 0: f64,
                    market_price @ 0: f64,
                    market_value @ 0: f64,
                    average_cost @ 0: f64,
                    unrealized_pnl @ 0: f64,
                    realized_pnl @ 0: f64,
                    account_name @ 0: String
            );
            wrapper
                .portfolio_value(Position {
                    contract: proxy,
                    position,
                    market_price,
                    market_value,
                    average_cost,
                    unrealized_pnl,
                    realized_pnl,
                    account_number: account_name,
                })
                .await;
            Ok(())
        }
    }

    #[inline]
    fn acct_update_time_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields =>
                    timestamp @ 2: String
            );
            wrapper
                .account_attribute_time(
                    NaiveTime::parse_from_str(timestamp.as_str(), "%H:%M")
                        .map_err(|e| ("timestamp", ParseDateTimeError::Timestamp))?,
                )
                .await;
            Ok(())
        }
    }

    #[inline]
    fn next_valid_id_msg(
        _fields: &mut Fields,
        _wrapper: &mut Self,
        _tx: &mut Tx,
        _rx: &mut Rx,
    ) -> impl Future<Output = DecodeResult> {
        async move { Ok(()) }
    }

    #[inline]
    fn contract_data_msg(
        fields: &mut Fields,
        _wrapper: &mut Self,
        tx: &mut Tx,
        rx: &mut Rx,
    ) -> impl Future<Output = DecodeResult> {
        async move { decode_contract_no_wrapper(fields, tx, rx).await }
    }

    #[inline]
    fn execution_data_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields =>
                    req_id @ 1: i64,
                    order_id @ 0: i64
            );
            let contract = deserialize_contract_proxy(fields)?;
            decode_fields!(
                fields =>
                    execution_id @ 0: String,
                    datetime @ 0: String,
                    account_number @ 0: String,
                    exchange @ 0: Primary,
                    side @ 0: OrderSide,
                    quantity @ 0: f64,
                    price @ 0: f64,
                    perm_id @ 0: i64,
                    client_id @ 0: i64,
                    liquidation @ 0: u8,
                    cumulative_quantity @ 0: f64,
                    average_price @ 0: f64,
                    pending_price_revision @ 5: u8
            );

            let (dt, tz) = NaiveDateTime::parse_and_remainder(datetime.as_str(), "%Y%m%d %T ")
                .map_err(|_| ("datetime", ParseDateTimeError::Timestamp))?;
            let datetime = dt
                .and_local_timezone(
                    tz.parse::<timezone::IbTimeZone>()
                        .map_err(|e| ("datetime", ParseDateTimeError::Timezone(e)))?,
                )
                .single()
                .ok_or(("datetime", ParseDateTimeError::Single))?
                .to_utc();
            let exec = Execution::from((
                Exec {
                    contract,
                    order_id,
                    execution_id,
                    datetime,
                    account_number,
                    exchange,
                    quantity,
                    price,
                    perm_id,
                    client_id,
                    liquidation: liquidation.ne(&0),
                    cumulative_quantity,
                    average_price,
                    pending_price_revision: pending_price_revision.ne(&0),
                },
                side,
            ));
            wrapper.execution(req_id, exec).await;

            Ok(())
        }
    }

    #[inline]
    fn market_depth_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields =>
                    req_id @ 1: i64,
                    position @ 0: u64,
                    operation @ 0: i64,
                    side @ 0: u32,
                    price @ 0: f64,
                    size @ 0: f64
            );

            let entry = CompleteEntry::Ordinary(
                Entry::try_from((side, position, price, size)).map_err(|e| ("entry", e))?,
            );
            let operation =
                Operation::try_from((operation, entry)).map_err(|e| ("operation", e))?;

            wrapper.update_market_depth(req_id, operation).await;
            Ok(())
        }
    }

    #[inline]
    fn market_depth_l2_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields =>
                    req_id @ 2: i64,
                    position  @ 0: u64,
                    market_maker @ 0: String,
                    operation @ 0: i64,
                    side @ 0: u32,
                    price @ 0: f64,
                    size @ 0: f64,
                    is_smart @ 0: i32
            );
            let entry = Entry::try_from((side, position, price, size)).map_err(|e| ("entry", e))?;
            let entry = match is_smart {
                0 => CompleteEntry::MarketMaker {
                    market_maker: market_maker
                        .chars()
                        .take(4)
                        .collect::<Vec<char>>()
                        .try_into()
                        .map_err(|_| ("market_maker", ParsePayloadError::Mpid))?,
                    entry,
                },
                _ => CompleteEntry::SmartDepth {
                    exchange: market_maker.parse().map_err(|e| ("exchange", e))?,
                    entry,
                },
            };
            let operation =
                Operation::try_from((operation, entry)).map_err(|e| ("operation", e))?;

            wrapper.update_market_depth(req_id, operation).await;
            Ok(())
        }
    }

    #[inline]
    fn news_bulletins_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }
    #[inline]
    fn managed_accts_msg(
        _fields: &mut Fields,
        _wrapper: &mut Self,
        _tx: &mut Tx,
        _rx: &mut Rx,
    ) -> impl Future<Output = DecodeResult> {
        async move { Ok(()) }
    }

    #[inline]
    fn receive_fa_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn historical_data_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields =>
                    req_id @ 1: i64,
                    _start_date_str @ 0: String,
                    _end_date_str @ 0: String,
                    count @ 0: usize
            );
            let mut bars = Vec::with_capacity(count);
            for chunk in fields.collect::<Vec<String>>().chunks(8) {
                if let [datetime_str, open, high, low, close, volume, wap, trade_count] = chunk {
                    let (date, rem) = NaiveDate::parse_and_remainder(datetime_str, "%Y%m%d")
                        .map_err(|_| ("date", ParseDateTimeError::Timestamp))?;
                    let time = if rem.is_empty() {
                        NaiveTime::default()
                    } else {
                        NaiveTime::parse_and_remainder(rem, " %T")
                            .map_err(|_| ("time", ParseDateTimeError::Timestamp))?
                            .0
                    };
                    let core = BarCore {
                        datetime: NaiveDateTime::new(date, time).and_utc(),
                        open: open.parse().map_err(|e| ("open", e))?,
                        high: high.parse().map_err(|e| ("high", e))?,
                        low: low.parse().map_err(|e| ("low", e))?,
                        close: close.parse().map_err(|e| ("close", e))?,
                    };
                    let (volume, wap, trade_count) = (
                        volume.parse().map_err(|e| ("volume", e))?,
                        wap.parse().map_err(|e| ("wap", e))?,
                        trade_count.parse::<i64>().map_err(|e| ("trade_count", e))?,
                    );
                    let bar = if volume > 0. && wap > 0. && trade_count > 0 {
                        Bar::Trades(Trade {
                            bar: core,
                            volume,
                            wap,
                            trade_count: trade_count.try_into().map_err(|_| {
                                DecodeError::UnexpectedData(
                                    "trade_count could not be converted to unsigned integer.",
                                )
                            })?,
                        })
                    } else {
                        Bar::Ordinary(core)
                    };
                    bars.push(bar);
                }
            }
            wrapper.historical_bars(req_id, bars).await;
            Ok(())
        }
    }

    #[inline]
    fn bond_contract_data_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn scanner_parameters_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn scanner_data_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn tick_option_computation_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields =>
                    req_id @ 1: i64,
                    tick_type @ 0: u16,
                    base @ 0: u8,
                    implied_volatility @ 0: CalculationResult,
                    delta @ 0: CalculationResult,
                    price @ 0: CalculationResult,
                    pv_dividend @ 0: CalculationResult,
                    gamma @ 0: CalculationResult,
                    vega @ 0: CalculationResult,
                    theta @ 0: CalculationResult,
                    underlying_price @ 0: CalculationResult
            );
            let calc = SecOptionCalculationResults {
                implied_volatility,
                delta,
                price,
                dividend_present_value: pv_dividend,
                gamma,
                vega,
                theta,
                underlying_price,
            };
            let calc = match base {
                0 => SecOptionCalculations::ReturnBased(calc),
                1 => SecOptionCalculations::PriceBased(calc),
                _ => {
                    return Err(DecodeError::UnexpectedData(
                        "Unexpected option calculation base.",
                    ))
                }
            };
            let calc = match tick_type {
                10..=13 | 53 => Class::Live(match tick_type {
                    10 => SecOptionCalculationSource::Bid(calc),
                    11 => SecOptionCalculationSource::Ask(calc),
                    12 => SecOptionCalculationSource::Last(calc),
                    13 => SecOptionCalculationSource::Model(calc),
                    53 => SecOptionCalculationSource::Custom(calc),
                    _ => unreachable!(),
                }),
                80..=83 => Class::Delayed(match tick_type {
                    80 => SecOptionCalculationSource::Bid(calc),
                    81 => SecOptionCalculationSource::Ask(calc),
                    82 => SecOptionCalculationSource::Last(calc),
                    83 => SecOptionCalculationSource::Model(calc),
                    _ => unreachable!(),
                }),
                _ => unreachable!(),
            };
            wrapper.sec_option_computation(req_id, calc).await;

            Ok(())
        }
    }

    #[inline]
    fn tick_generic_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields =>
                    req_id @ 2: i64,
                    tick_type @ 0: u16,
                    value @ 0: f64
            );
            Self::decode_generic_tick_msg(req_id, tick_type, value, wrapper).await
        }
    }

    #[inline]
    fn tick_string_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields =>
                    req_id @ 2: i64,
                    tick_type @ 0: u16,
                    value @ 0: String
            );
            match tick_type {
                32 | 33 | 84 => {
                    let quoting_exchanges = match tick_type {
                        32 => QuotingExchanges::Bid(value.chars().collect()),
                        33 => QuotingExchanges::Ask(value.chars().collect()),
                        84 => QuotingExchanges::Last(value.chars().collect()),
                        _ => unreachable!(),
                    };
                    wrapper.quoting_exchanges(req_id, quoting_exchanges).await;
                }
                45 | 85 | 88 => {
                    let value = value.parse().map_err(|e| ("value", e))?;
                    if value == 0 {
                        return Ok(());
                    }
                    let timestamp = match tick_type {
                        45 | 88 => DateTime::from_timestamp(value, 0),
                        85 => DateTime::from_timestamp_millis(value),
                        _ => unreachable!(),
                    }
                    .ok_or(("timestamp", ParseDateTimeError::Timestamp))?;
                    let timestamp = match tick_type {
                        45 => Class::Live(TimeStamp::Last(timestamp)),
                        85 => Class::Live(TimeStamp::Regulatory(timestamp)),
                        88 => Class::Delayed(TimeStamp::Last(timestamp)),
                        _ => unreachable!(),
                    };
                    wrapper.timestamp(req_id, timestamp).await;
                }
                48 | 77 => {
                    let mut vols = value.split(';');
                    let base = RealTimeVolumeBase {
                        last_price: vols
                            .next()
                            .ok_or(DecodeError::MissingData {
                                field_name: "last_price",
                            })?
                            .parse()
                            .map_err(|e| ("last_price", e))?,
                        last_size: vols
                            .next()
                            .ok_or(DecodeError::MissingData {
                                field_name: "last_size",
                            })?
                            .parse()
                            .map_err(|e| ("last_size", e))?,
                        last_time: DateTime::from_timestamp(
                            vols.next()
                                .ok_or(DecodeError::MissingData {
                                    field_name: "last_time",
                                })?
                                .parse()
                                .map_err(|e| ("last_time", e))?,
                            0,
                        )
                        .ok_or(("last_time", ParseDateTimeError::Timestamp))?,
                        day_volume: vols
                            .next()
                            .ok_or(DecodeError::MissingData {
                                field_name: "day_volume",
                            })?
                            .parse()
                            .map_err(|e| ("day_volume", e))?,
                        vwap: vols
                            .next()
                            .ok_or(DecodeError::MissingData { field_name: "vwap" })?
                            .parse()
                            .map_err(|e| ("vwap", e))?,
                        single_mm: vols
                            .next()
                            .ok_or(DecodeError::MissingData {
                                field_name: "single_mm",
                            })?
                            .parse()
                            .map_err(|e| ("single_mm", e))?,
                    };
                    let volume = match tick_type {
                        48 => RealTimeVolume::All(base),
                        77 => RealTimeVolume::Trades(base),
                        _ => unreachable!(),
                    };
                    wrapper.real_time_volume(req_id, volume).await;
                }
                59 => {
                    let mut divs = value.split(',');
                    let dividends = Dividends {
                        trailing_year: divs
                            .next()
                            .ok_or(DecodeError::MissingData {
                                field_name: "trailing_year",
                            })?
                            .parse()
                            .map_err(|e| ("trailing_year", e))?,
                        forward_year: divs
                            .next()
                            .ok_or(DecodeError::MissingData {
                                field_name: "forward_year",
                            })?
                            .parse()
                            .map_err(|e| ("forward_year", e))?,
                        next_dividend: (
                            NaiveDate::parse_and_remainder(
                                divs.next().ok_or(DecodeError::MissingData {
                                    field_name: "next_dividend",
                                })?,
                                "%Y%m%d",
                            )
                            .map_err(|_| ("next_dividend", ParseDateTimeError::Timestamp))?
                            .0,
                            divs.next()
                                .ok_or(DecodeError::MissingData {
                                    field_name: "next_price",
                                })?
                                .parse()
                                .map_err(|e| ("next_dividend", e))?,
                        ),
                    };
                    wrapper.dividends(req_id, dividends).await;
                }
                62 => {
                    wrapper.news(req_id, value).await;
                }
                t => {
                    return Err(DecodeError::Other(format!(
                        "unexpected price market data request: {t}."
                    )))
                }
            };
            Ok(())
        }
    }

    #[inline]
    fn tick_efp_msg(
        _fields: &mut Fields,
        _wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            unimplemented!();
        }
    }

    #[inline]
    fn current_time_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields =>
                    req_id @ 1: i64,
                    datetime @ 0: i64
            );

            wrapper
                .current_time(
                    req_id,
                    DateTime::from_timestamp(datetime, 0)
                        .ok_or(("datetime", ParseDateTimeError::Timestamp))?,
                )
                .await;
            Ok(())
        }
    }

    #[inline]
    fn real_time_bars_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields =>
                    req_id @ 2: i64,
                    date_time @ 0: i64,
                    open @ 0: f64,
                    high @ 0: f64,
                    low @ 0: f64,
                    close @ 0: f64,
                    volume @ 0: f64,
                    wap @ 0: f64,
                    trade_count @ 0: i64
            );
            let core = BarCore {
                datetime: DateTime::from_timestamp(date_time, 0)
                    .ok_or(("datetime", ParseDateTimeError::Timestamp))?,
                open,
                high,
                low,
                close,
            };
            let bar = if trade_count > 0 && wap > 0. && volume > 0. {
                Bar::Trades(Trade {
                    bar: core,
                    volume,
                    wap,
                    trade_count: trade_count.try_into().map_err(|_| {
                        DecodeError::UnexpectedData(
                            "trade_count could not be converted to unsigned integer.",
                        )
                    })?,
                })
            } else {
                Bar::Ordinary(core)
            };
            wrapper.real_time_bar(req_id, bar).await;
            Ok(())
        }
    }

    #[inline]
    fn fundamental_data_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn contract_data_end_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(fields => req_id @ 2: i64);
            wrapper.contract_data_end(req_id).await;
            Ok(())
        }
    }

    #[inline]
    fn open_order_end_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            wrapper.open_order_end().await;
            Ok(())
        }
    }

    #[inline]
    fn acct_download_end_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields => account_number @ 2: String
            );
            wrapper.account_download_end(account_number).await;
            Ok(())
        }
    }

    #[inline]
    fn execution_data_end_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);

            Ok(())
        }
    }

    #[inline]
    fn delta_neutral_validation_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn tick_snapshot_end_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn market_data_type_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields =>
                    req_id @ 2: i64,
                    class @ 0: MarketDataClass
            );
            wrapper.market_data_class(req_id, class).await;
            Ok(())
        }
    }

    #[inline]
    fn commission_report_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn position_data_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields =>
                    account_number @ 2: String
            );
            let contract = deserialize_contract_proxy(fields)?;
            decode_fields!(
                fields =>
                    position @ 0: f64,
                    average_cost @ 0: f64
            );
            wrapper
                .position_summary(PositionSummary {
                    contract,
                    position,
                    average_cost,
                    account_number,
                })
                .await;
            Ok(())
        }
    }

    #[inline]
    fn position_end_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            wrapper.position_end().await;
            Ok(())
        }
    }

    #[inline]
    fn account_summary_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields =>
                    req_id @ 2: i64,
                    account_number @ 0: String,
                    tag @ 0: Tag,
                    value @ 0: String,
                    currency @ 0: String
            );
            let summary = match tag {
                Tag::AccountType => TagValue::String(Tag::AccountType, value),
                Tag::Cushion => {
                    TagValue::Float(Tag::Cushion, value.parse().map_err(|e| ("summary", e))?)
                }
                Tag::LookAheadNextChange => TagValue::Int(
                    Tag::LookAheadNextChange,
                    value.parse().map_err(|e| ("summary", e))?,
                ),
                Tag::HighestSeverity => TagValue::String(Tag::HighestSeverity, value),
                Tag::DayTradesRemaining => TagValue::Int(
                    Tag::DayTradesRemaining,
                    value.parse().map_err(|e| ("summary", e))?,
                ),
                Tag::Leverage => {
                    TagValue::Float(Tag::Leverage, value.parse().map_err(|e| ("summary", e))?)
                }
                t => TagValue::Currency(
                    t,
                    value.parse().map_err(|e| ("summary", e))?,
                    currency.parse().map_err(|e| ("summary", e))?,
                ),
            };
            wrapper
                .account_summary(req_id, account_number, summary)
                .await;
            Ok(())
        }
    }

    #[inline]
    fn account_summary_end_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields => req_id @ 2: i64
            );
            wrapper.account_summary_end(req_id).await;
            Ok(())
        }
    }

    #[inline]
    fn verify_message_api_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn verify_completed_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn display_group_list_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn display_group_updated_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn verify_and_auth_message_api_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn verify_and_auth_completed_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn position_multi_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn position_multi_end_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn account_update_multi_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn account_update_multi_end_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn security_definition_option_parameter_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn security_definition_option_parameter_end_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn soft_dollar_tiers_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn family_codes_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn symbol_samples_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn mkt_depth_exchanges_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn tick_req_params_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields =>
                    req_id @ 1: i64,
                    min_tick @ 0: f64,
                    exchange_id @ 0: ExchangeId,
                    snapshot_permissions @ 0: u32
            );
            wrapper
                .tick_params(req_id, min_tick, exchange_id, snapshot_permissions)
                .await;
            Ok(())
        }
    }

    #[inline]
    fn smart_components_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn news_article_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn tick_news_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn news_providers_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn historical_news_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn historical_news_end_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn head_timestamp_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields =>
                    req_id @ 1: i64,
                    timestamp @ 0: i64
            );
            wrapper
                .head_timestamp(
                    req_id,
                    DateTime::from_timestamp(timestamp, 0)
                        .ok_or(("timestamp", ParseDateTimeError::Timestamp))?,
                )
                .await;
            Ok(())
        }
    }

    #[inline]
    fn histogram_data_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields =>
                    req_id @ 1: i64,
                    num_points @ 0: usize
            );
            let mut hist = std::collections::HashMap::with_capacity(num_points);
            for (bin, chunk) in fields
                .take(num_points * 2)
                .map(|v| v.parse())
                .collect::<Result<Vec<f64>, _>>()
                .map_err(|e| ("chunk", e))?
                .chunks_exact(2)
                .enumerate()
            {
                if let [price, size] = *chunk {
                    hist.insert(bin, HistogramEntry { price, size });
                }
            }
            wrapper.histogram(req_id, hist).await;
            Ok(())
        }
    }

    #[inline]
    fn historical_data_update_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields =>
                    req_id @ 1: i64,
                    trade_count @ 0: i64,
                    datetime_str @ 0: String,
                    open @ 0: f64,
                    high @ 0: f64,
                    low @ 0: f64,
                    close @ 0: f64,
                    wap @ 0: f64,
                    volume @ 0: f64
            );
            let core = BarCore {
                datetime: NaiveDateTime::parse_and_remainder(datetime_str.as_str(), "%Y%m%d %T")
                    .map_err(|_| ("datetime", ParseDateTimeError::Timestamp))?
                    .0
                    .and_utc(),
                open,
                high,
                low,
                close,
            };
            let bar = if trade_count > 0 && wap > 0. && volume > 0. {
                Bar::Trades(Trade {
                    bar: core,
                    volume,
                    wap,
                    trade_count: trade_count.try_into().map_err(|_| {
                        DecodeError::UnexpectedData(
                            "trade_count could not be converted to unsigned integer.",
                        )
                    })?,
                })
            } else {
                Bar::Ordinary(core)
            };
            wrapper.updating_historical_bar(req_id, bar).await;
            Ok(())
        }
    }

    #[inline]
    fn reroute_mkt_data_req_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn reroute_mkt_depth_req_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn market_rule_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn pnl_msg(fields: &mut Fields, wrapper: &mut Self) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields =>
                    req_id @ 1: i64,
                    daily_pnl @ 0: f64,
                    unrealized_pnl @ 0: f64,
                    realized_pnl @ 0: f64
            );
            let pnl = Pnl {
                daily: daily_pnl,
                unrealized: unrealized_pnl,
                realized: realized_pnl,
            };
            wrapper.pnl(req_id, pnl).await;
            Ok(())
        }
    }

    #[inline]
    fn pnl_single_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields =>
                    req_id @ 1: i64,
                    position_size @ 0: f64,
                    daily @ 0: f64,
                    unrealized @ 0: f64,
                    realized @ 0: f64,
                    market_value @ 0: f64
            );
            let pnl = PnlSingle {
                daily,
                unrealized,
                realized,
                position_size,
                market_value,
            };
            wrapper.single_position_pnl(req_id, pnl).await;
            Ok(())
        }
    }

    #[inline]
    fn historical_ticks_midpoint_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields =>
                    req_id @ 1: i64,
                    tick_count @ 0: usize
            );
            let mut ticks = Vec::with_capacity(tick_count);
            for chunk in fields
                .take(tick_count * 4)
                .collect::<Vec<String>>()
                .chunks_exact(4)
            {
                if let [time, _, price, size] = chunk {
                    ticks.push(TickData::Midpoint(Midpoint {
                        datetime: DateTime::from_timestamp(
                            time.parse().map_err(|e| ("datetime", e))?,
                            0,
                        )
                        .ok_or(("datetime", ParseDateTimeError::Timestamp))?,
                        price: price.parse().map_err(|e| ("price", e))?,
                    }));
                }
            }
            wrapper.historical_ticks(req_id, ticks).await;
            Ok(())
        }
    }

    #[inline]
    fn historical_ticks_bid_ask_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields =>
                    req_id @ 1: i64,
                    tick_count @ 0: usize
            );
            let mut ticks = Vec::with_capacity(tick_count);
            for chunk in fields
                .take(tick_count * 6)
                .collect::<Vec<String>>()
                .chunks_exact(6)
            {
                if let [time, _, bid_price, ask_price, bid_size, ask_size] = chunk {
                    ticks.push(TickData::BidAsk(BidAsk {
                        datetime: DateTime::from_timestamp(
                            time.parse().map_err(|e| ("datetime", e))?,
                            0,
                        )
                        .ok_or(("datetime", ParseDateTimeError::Timestamp))?,
                        bid_price: bid_price.parse().map_err(|e| ("bid_price", e))?,
                        ask_price: ask_price.parse().map_err(|e| ("ask_price", e))?,
                        bid_size: bid_size.parse().map_err(|e| ("bid_size", e))?,
                        ask_size: ask_size.parse().map_err(|e| ("ask_size", e))?,
                    }));
                }
            }
            wrapper.historical_ticks(req_id, ticks).await;
            Ok(())
        }
    }

    #[inline]
    fn historical_ticks_last_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields =>
                    req_id @ 1: i64,
                    tick_count @ 0: usize
            );
            let mut ticks = Vec::with_capacity(tick_count);
            for chunk in fields
                .take(tick_count * 6)
                .collect::<Vec<String>>()
                .chunks_exact(6)
            {
                if let [time, _, price, size, exchange, _] = chunk {
                    ticks.push(TickData::Last(Last {
                        datetime: DateTime::from_timestamp(
                            time.parse().map_err(|e| ("datetime", e))?,
                            0,
                        )
                        .ok_or(("datetime", ParseDateTimeError::Timestamp))?,
                        price: price.parse().map_err(|e| ("price", e))?,
                        size: size.parse().map_err(|e| ("size", e))?,
                        exchange: exchange.parse().map_err(|e| ("exchange", e))?,
                    }));
                }
            }
            wrapper.historical_ticks(req_id, ticks).await;
            Ok(())
        }
    }

    #[inline]
    fn tick_by_tick_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields =>
                    req_id @ 1: i64,
                    tick_type @ 0: u8,
                    timestamp @ 0: i64
            );
            let datetime = DateTime::from_timestamp(timestamp, 0)
                .ok_or(("datetime", ParseDateTimeError::Timestamp))?;
            let tick = match tick_type {
                1 | 2 => TickData::Last(Last {
                    datetime,
                    price: nth(fields, 0, "price")?.parse().map_err(|e| ("price", e))?,
                    size: nth(fields, 0, "size")?.parse().map_err(|e| ("size", e))?,
                    exchange: nth(fields, 1, "exchange")?
                        .parse()
                        .map_err(|e| ("exchange", e))?,
                }),
                3 => {
                    decode_fields!(
                        fields =>
                            bid_price @ 0: f64,
                            ask_price @ 0: f64,
                            bid_size @ 0: f64,
                            ask_size @ 0: f64
                    );
                    TickData::BidAsk(BidAsk {
                        datetime,
                        bid_price,
                        ask_price,
                        bid_size,
                        ask_size,
                    })
                }
                4 => TickData::Midpoint(Midpoint {
                    datetime,
                    price: nth(fields, 0, "price")?.parse().map_err(|e| ("price", e))?,
                }),
                _ => Err(DecodeError::UnexpectedData("Unexpected tick type"))?,
            };
            wrapper.live_tick(req_id, tick).await;
            Ok(())
        }
    }

    #[inline]
    fn order_bound_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn completed_order_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn completed_orders_end_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn replace_fa_end_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn wsh_meta_data_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn wsh_event_data_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn historical_schedule_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn user_info_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn decode_generic_tick_msg(
        req_id: i64,
        tick_type: u16,
        value: f64,
        wrapper: &mut Self,
    ) -> impl Future<Output = DecodeResult> {
        async move {
            match tick_type {
                0 | 3 | 5 => {
                    let size = Class::Live(match tick_type {
                        0 => Size::Bid(value),
                        3 => Size::Ask(value),
                        5 => Size::Last(value),
                        _ => unreachable!(),
                    });
                    wrapper.size_data(req_id, size).await;
                }
                8 | 74 => {
                    let volume = match tick_type {
                        8 => Class::Live(value),
                        74 => Class::Delayed(value),
                        _ => unreachable!(),
                    };
                    wrapper.volume(req_id, volume).await;
                }
                21 | 63 | 64 | 65 => {
                    let volume = match tick_type {
                        21 => SummaryVolume::NinetyDayAverage(value),
                        63 => SummaryVolume::ThreeMinutes(value),
                        64 => SummaryVolume::FiveMinutes(value),
                        65 => SummaryVolume::TenMinutes(value),
                        _ => unreachable!(),
                    };
                    wrapper.summary_volume(req_id, volume).await;
                }
                23 | 24 | 58 => {
                    let vol = match tick_type {
                        23 => Volatility::SecOptionHistorical(value),
                        24 => Volatility::SecOptionImplied(value),
                        58 => Volatility::RealTimeHistorical(value),
                        _ => unreachable!(),
                    };
                    wrapper.volatility(req_id, vol).await;
                }
                29 | 30 | 87 => {
                    let volume = match tick_type {
                        29 => SecOptionVolume::Call(value),
                        30 => SecOptionVolume::Put(value),
                        87 => SecOptionVolume::Average(value),
                        _ => unreachable!(),
                    };
                    wrapper.sec_option_volume(req_id, volume).await;
                }
                34 | 36 | 61 => {
                    let auction = match tick_type {
                        34 => AuctionData::Volume(value),
                        36 => AuctionData::Imbalance(value),
                        61 => AuctionData::Regulatory(value),
                        _ => unreachable!(),
                    };
                    wrapper.auction(req_id, auction).await;
                }
                27 | 28 | 86 => {
                    let open_interest = match tick_type {
                        27 => OpenInterest::SecOptionCall(value),
                        28 => OpenInterest::SecOptionPut(value),
                        86 => OpenInterest::SecFuture(value),
                        _ => unreachable!(),
                    };
                    wrapper.open_interest(req_id, open_interest).await;
                }
                31 | 60 => {
                    let factor = match tick_type {
                        31 => PriceFactor::IndexFuturePremium(value),
                        60 => PriceFactor::BondFactorMultiplier(value),
                        _ => unreachable!(),
                    };
                    wrapper.price_factor(req_id, factor).await;
                }
                46 | 49 | 89 => {
                    let access = match tick_type {
                        46 => Accessibility::Shortable(value),
                        49 => Accessibility::Halted(value),
                        89 => Accessibility::ShortableShares(value),
                        _ => unreachable!(),
                    };
                    wrapper.accessibility(req_id, access).await;
                }
                54 => {
                    wrapper.trade_count(req_id, value).await;
                }
                55 | 56 => {
                    let rate = match tick_type {
                        55 => Rate::Trade(value),
                        56 => Rate::Volume(value),
                        _ => unreachable!(),
                    };
                    wrapper.rate(req_id, rate).await;
                }
                69..=71 => {
                    let size = Class::Delayed(match tick_type {
                        69 => Size::Bid(value),
                        70 => Size::Ask(value),
                        71 => Size::Last(value),
                        _ => unreachable!(),
                    });
                    wrapper.size_data(req_id, size).await;
                }
                101 | 102 => {
                    let ipo = match tick_type {
                        101 => Ipo::Estimated(value),
                        102 => Ipo::Final(value),
                        _ => unreachable!(),
                    };
                    wrapper.ipo(req_id, ipo).await;
                }
                _ => {
                    return Err(DecodeError::UnexpectedData(
                        "Unexpected generic market data request",
                    ))
                }
            };

            Ok(())
        }
    }
}

impl<W: wrapper::LocalWrapper> Local for W {}

impl<W: wrapper::Wrapper> Remote for W {}

#[inline]
pub(crate) fn nth(
    fields: &mut Fields,
    n: usize,
    field_name: &'static str,
) -> Result<String, DecodeError> {
    fields.nth(n).ok_or(DecodeError::MissingData { field_name })
}

#[inline]
pub(crate) async fn decode_contract_no_wrapper(
    fields: &mut Fields,
    tx: &mut Tx,
    rx: &mut Rx,
) -> DecodeResult {
    decode_fields!(
        fields =>
            req_id @ 1: i64,
            symbol @ 0: String,
            sec_type @ 0: ContractType,
            expiration_date @ 0: String,
            strike @ 0: f64,
            class @ 0: String,
            exchange @ 0: Routing,
            currency @ 0: Currency,
            local_symbol @ 0: String,
            trading_class @ 1: String,
            contract_id @ 0: ContractId,
            min_tick @ 0: f64,
            multiplier @ 0: String,
            order_types @ 0: String,
            valid_exchanges @ 0: String,
            underlying_contract_id @ 1: ContractId,
            long_name @ 0: String,
            primary_exchange @ 0: String,
            sector @ 1: String,
            security_id_count @ 7: usize
    );

    let order_types = order_types
        .split(',')
        .map(std::borrow::ToOwned::to_owned)
        .collect();
    let valid_exchanges = valid_exchanges
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()
        .map_err(|e| ("valid_exchanges", e))?;
    let security_ids = (0..security_id_count)
        .map(
            |_| match nth(fields, 0, "security_ids")?.to_uppercase().as_str() {
                "CUSIP" => Ok(SecurityId::Cusip(nth(fields, 0, "security_id")?)),
                "SEDOL" => Ok(SecurityId::Sedol(nth(fields, 0, "security_id")?)),
                "ISIN" => Ok(SecurityId::Isin(nth(fields, 0, "security_id")?)),
                "RIC" => Ok(SecurityId::Ric(nth(fields, 0, "security_id")?)),
                _ => Err(DecodeError::UnexpectedData(
                    "Invalid security_id type found in STK contract_data_msg",
                )),
            },
        )
        .collect::<Result<_, _>>()?;

    if let Ok(ToWrapper::ContractQuery((query_client, req_id_client))) = rx.try_recv() {
        if let crate::contract::Query::IbContractId(con_id_client, routing_client) = query_client {
            if con_id_client != contract_id {
                return Err(DecodeError::UnexpectedData("Unexpected contract ID"));
            }
            if exchange != routing_client {
                return Err(DecodeError::UnexpectedData("Unexpected routing exchange"));
            }
        }
        if req_id_client != req_id {
            return Err(DecodeError::UnexpectedData("Unexpected request ID"));
        }
        let contract = match sec_type {
            ContractType::Stock => Some(Contract::Stock(Stock {
                symbol,
                exchange,
                currency,
                local_symbol,
                trading_class,
                contract_id,
                min_tick,
                primary_exchange: primary_exchange
                    .parse()
                    .map_err(|e| ("primary_exchange", e))?,
                long_name,
                sector,
                order_types,
                valid_exchanges,
                security_ids,
                stock_type: nth(fields, 5, "stock_type")?,
            })),
            ContractType::SecOption => {
                let inner = SecOptionInner {
                    contract_id,
                    min_tick,
                    symbol,
                    exchange,
                    strike,
                    multiplier: multiplier.parse().map_err(|e| ("multiplier", e))?,
                    expiration_date: NaiveDate::parse_and_remainder(
                        expiration_date.as_str(),
                        "%Y%m%d",
                    )
                    .map_err(|_| ("expiration_date", ParseDateTimeError::Timestamp))?
                    .0,
                    underlying_contract_id,
                    sector,
                    trading_class,
                    currency,
                    local_symbol,
                    long_name,
                    order_types,
                    valid_exchanges,
                };
                match class.as_str() {
                    "C" => Some(Contract::SecOption(SecOption::Call(inner))),
                    "P" => Some(Contract::SecOption(SecOption::Put(inner))),
                    _ => return Err(DecodeError::UnexpectedData("Unexpected option class")),
                }
            }
            ContractType::Crypto => Some(Contract::Crypto(Crypto {
                contract_id,
                min_tick,
                symbol,
                trading_class,
                currency,
                local_symbol,
                long_name,
                order_types,
                valid_exchanges,
            })),
            ContractType::Forex => Some(Contract::Forex(Forex {
                contract_id,
                min_tick,
                symbol,
                exchange,
                trading_class,
                currency,
                local_symbol,
                long_name,
                order_types,
                valid_exchanges,
            })),
            ContractType::Index => Some(Contract::Index(Index {
                contract_id,
                min_tick,
                symbol,
                exchange,
                currency,
                local_symbol,
                long_name,
                order_types,
                valid_exchanges,
            })),
            ContractType::SecFuture => Some(Contract::SecFuture(SecFuture {
                contract_id,
                min_tick,
                symbol,
                exchange,
                multiplier: multiplier.parse().map_err(|e| ("multiplier", e))?,
                expiration_date: NaiveDate::parse_and_remainder(expiration_date.as_str(), "%Y%m%d")
                    .map_err(|_| ("expiration_date", ParseDateTimeError::Timestamp))?
                    .0,
                trading_class,
                underlying_contract_id,
                currency,
                local_symbol,
                long_name,
                order_types,
                valid_exchanges,
            })),
            ContractType::Commodity => Some(Contract::Commodity(Commodity {
                contract_id,
                min_tick,
                symbol,
                exchange,
                trading_class,
                currency,
                local_symbol,
                long_name,
                order_types,
                valid_exchanges,
            })),
        };

        tx.send(ToClient::NewContract(contract.ok_or(
            DecodeError::UnexpectedData("No contract was created"),
        )?))
        .await
        .map_err(Box::new)?;
    }
    Ok(())
}

#[inline]
fn deserialize_contract_proxy<E: crate::contract::ProxyExchange + Clone>(
    fields: &mut Fields,
) -> Result<Proxy<Contract, E>, DecodeError> {
    decode_fields!(
        fields =>
            contract_id @ 0: ContractId,
            symbol @ 0: String,
            sec_type @ 0: ContractType,
            expiration_date @ 0: String,
            strike @ 0: String,
            right @ 0: String,
            multiplier @ 0: String,
            exch_or_priamry @ 0: String,
            currency @ 0: Currency,
            local_symbol @ 0: String,
            trading_class @ 0: String
    );
    let (exchange, primary_exchange) = E::decode(exch_or_priamry)?;

    let inner = match sec_type {
        ContractType::Stock => Contract::Stock(Stock {
            contract_id,
            min_tick: f64::default(),
            symbol,
            exchange,
            primary_exchange,
            stock_type: String::default(),
            security_ids: Vec::default(),
            sector: String::default(),
            trading_class,
            currency,
            local_symbol,
            long_name: String::default(),
            order_types: Vec::default(),
            valid_exchanges: Vec::default(),
        }),
        ContractType::Crypto => Contract::Crypto(Crypto {
            contract_id,
            min_tick: f64::default(),
            symbol,
            trading_class,
            currency,
            local_symbol,
            long_name: String::default(),
            order_types: Vec::default(),
            valid_exchanges: Vec::default(),
        }),
        ContractType::Index => Contract::Index(Index {
            contract_id,
            min_tick: f64::default(),
            symbol,
            exchange,
            currency,
            local_symbol,
            long_name: String::default(),
            order_types: Vec::default(),
            valid_exchanges: Vec::default(),
        }),
        ContractType::Commodity => Contract::Commodity(Commodity {
            contract_id,
            min_tick: f64::default(),
            symbol,
            exchange,
            trading_class,
            currency,
            local_symbol,
            long_name: String::default(),
            order_types: Vec::default(),
            valid_exchanges: Vec::default(),
        }),
        ContractType::Forex => Contract::Forex(Forex {
            contract_id,
            min_tick: f64::default(),
            symbol,
            exchange,
            trading_class,
            currency,
            local_symbol,
            long_name: String::default(),
            order_types: Vec::default(),
            valid_exchanges: Vec::default(),
        }),
        ContractType::SecFuture => Contract::SecFuture(SecFuture {
            contract_id,
            min_tick: f64::default(),
            symbol,
            exchange,
            multiplier: multiplier.parse().map_err(|e| ("multiplier", e))?,
            expiration_date: NaiveDate::parse_and_remainder(expiration_date.as_str(), "%Y%m%d")
                .map_err(|_| ("expiration_date", ParseDateTimeError::Timestamp))?
                .0,
            trading_class,
            underlying_contract_id: contract_id,
            currency,
            local_symbol,
            long_name: String::default(),
            order_types: Vec::default(),
            valid_exchanges: Vec::default(),
        }),
        ContractType::SecOption => {
            let op_inner = SecOptionInner {
                contract_id,
                min_tick: f64::default(),
                symbol,
                exchange,
                strike: strike.parse().map_err(|e| ("strike", e))?,
                multiplier: multiplier.parse().map_err(|e| ("multiplier", e))?,
                expiration_date: NaiveDate::parse_and_remainder(expiration_date.as_str(), "%Y%m%d")
                    .map_err(|_| ("expiration_date", ParseDateTimeError::Timestamp))?
                    .0,
                underlying_contract_id: contract_id,
                sector: String::default(),
                trading_class,
                currency,
                local_symbol,
                long_name: String::default(),
                order_types: Vec::default(),
                valid_exchanges: Vec::default(),
            };
            let op_outer = match right.as_str() {
                "C" => SecOption::Call(op_inner),
                "P" => SecOption::Put(op_inner),
                other => {
                    return Err(DecodeError::Other(format!(
                        "Unexpected option right. Expected \'C\' or \'P\'. Found {other}."
                    )))
                }
            };
            Contract::SecOption(op_outer)
        }
    };

    Ok(Proxy {
        inner,
        _exch: std::marker::PhantomData,
    })
}

#[derive(Debug, Clone, Error)]
pub(crate) enum DecodeError {
    #[error("Missing data for field {field_name}.")]
    /// The data is missing from the API callback
    MissingData { field_name: &'static str },
    #[error("Failed to parse integer field {field_name}. Cause: {int_error}")]
    /// Failed to parse integer field
    ParseIntError {
        field_name: &'static str,
        int_error: std::num::ParseIntError,
    },
    #[error("Failed to parse boolean field {field_name}. Cause: {bool_error}")]
    /// Failed to parse boolean field
    ParseBoolError {
        field_name: &'static str,
        bool_error: std::str::ParseBoolError,
    },
    #[error("Failed to parse float field {field_name}. Cause: {float_error}")]
    /// Failed to parse floating point field
    ParseFloatError {
        field_name: &'static str,
        float_error: std::num::ParseFloatError,
    },
    #[error("Failed to parse currency field {field_name}. Cause: {currency_error}")]
    /// Failed to parse [`Currency`] field
    ParseCurrencyError {
        field_name: &'static str,
        currency_error: crate::currency::ParseCurrencyError,
    },
    #[error("Failed to parse class field {field_name}. Cause: {class_error}")]
    /// Failed to parse [`market::data::live_data::Class`] field
    ParseClassError {
        field_name: &'static str,
        class_error: crate::market_data::live_data::ParseClassError,
    },
    #[error("Failed to parse tag field {field_name}. Cause: {tag_error}")]
    /// Failed to parse [`Tag`] field
    ParseTagError {
        field_name: &'static str,
        tag_error: account::ParseTagError,
    },
    #[error("Failed to parse exchange field {field_name}. Cause: {exchange_error}")]
    /// Failed to parse [`Routing`] or [`Primary`] field
    ParseExchangeError {
        field_name: &'static str,
        exchange_error: crate::exchange::ParseExchangeError,
    },
    #[error("Failed to parse contract ID field {field_name}. Cause: {contract_id_error}")]
    /// Failed to parse [`ContractId`] field
    ParseContractIdError {
        field_name: &'static str,
        contract_id_error: crate::contract::ParseContractIdError,
    },
    #[error("Failed to parse contract type field {field_name}. Cause: {contract_type_error}")]
    /// Failed to parse [`ContractType`] field
    ParseContractTypeError {
        field_name: &'static str,
        contract_type_error: crate::contract::ParseContractTypeError,
    },
    #[error("Failed to parse payload {field_name}. Cause: {payload_error}")]
    /// Failed to parse any value in the [`crate::payload`] module
    ParsePayloadError {
        field_name: &'static str,
        payload_error: ParsePayloadError,
    },
    #[error("Failed to parse attribute: {0}")]
    /// Failed to parse an [`account::Attribute`] value
    ParseAttributeError(ParseAttributeError),
    #[error("Failed to parse datetime field {field_name}. Cause: {datetime_error}")]
    /// Failed to parse an datetime value
    ParseDateTimeError {
        field_name: &'static str,
        datetime_error: ParseDateTimeError,
    },
    #[error("Failed to parse order side field {field_name}")]
    ParseOrderSideError {
        field_name: &'static str,
        order_side_error: ParseOrderSideError,
    },
    #[error("{0}")]
    UnexpectedData(&'static str),
    #[error("Error when sending data {0}")]
    SendError(#[from] Box<tokio::sync::mpsc::error::SendError<ToClient>>),
    #[error("{0}")]
    Other(String),
}

#[derive(Debug, Error)]
#[error("Decode error in function {function_name}. Cause {decode_error}")]
pub(crate) struct DecodeContext {
    decode_error: DecodeError,
    function_name: &'static str,
}

impl DecodeError {
    #[inline]
    pub(crate) fn with_context(self, msg: &'static str) -> DecodeContext {
        DecodeContext {
            decode_error: self,
            function_name: msg,
        }
    }
}

impl From<(&'static str, std::num::ParseIntError)> for DecodeError {
    fn from(value: (&'static str, std::num::ParseIntError)) -> Self {
        Self::ParseIntError {
            field_name: value.0,
            int_error: value.1,
        }
    }
}

impl From<(&'static str, std::str::ParseBoolError)> for DecodeError {
    fn from(value: (&'static str, std::str::ParseBoolError)) -> Self {
        Self::ParseBoolError {
            field_name: value.0,
            bool_error: value.1,
        }
    }
}

impl From<(&'static str, std::num::ParseFloatError)> for DecodeError {
    fn from(value: (&'static str, std::num::ParseFloatError)) -> Self {
        Self::ParseFloatError {
            field_name: value.0,
            float_error: value.1,
        }
    }
}

impl From<(&'static str, crate::currency::ParseCurrencyError)> for DecodeError {
    fn from(value: (&'static str, crate::currency::ParseCurrencyError)) -> Self {
        Self::ParseCurrencyError {
            field_name: value.0,
            currency_error: value.1,
        }
    }
}

impl From<(&'static str, crate::market_data::live_data::ParseClassError)> for DecodeError {
    fn from(value: (&'static str, crate::market_data::live_data::ParseClassError)) -> Self {
        Self::ParseClassError {
            field_name: value.0,
            class_error: value.1,
        }
    }
}

impl From<(&'static str, account::ParseTagError)> for DecodeError {
    fn from(value: (&'static str, account::ParseTagError)) -> Self {
        Self::ParseTagError {
            field_name: value.0,
            tag_error: value.1,
        }
    }
}

impl From<(&'static str, crate::exchange::ParseExchangeError)> for DecodeError {
    fn from(value: (&'static str, crate::exchange::ParseExchangeError)) -> Self {
        Self::ParseExchangeError {
            field_name: value.0,
            exchange_error: value.1,
        }
    }
}

impl From<(&'static str, crate::contract::ParseContractIdError)> for DecodeError {
    fn from(value: (&'static str, crate::contract::ParseContractIdError)) -> Self {
        Self::ParseContractIdError {
            field_name: value.0,
            contract_id_error: value.1,
        }
    }
}

impl From<(&'static str, crate::contract::ParseContractTypeError)> for DecodeError {
    fn from(value: (&'static str, crate::contract::ParseContractTypeError)) -> Self {
        Self::ParseContractTypeError {
            field_name: value.0,
            contract_type_error: value.1,
        }
    }
}

impl From<(&'static str, ParsePayloadError)> for DecodeError {
    fn from(value: (&'static str, ParsePayloadError)) -> Self {
        Self::ParsePayloadError {
            field_name: value.0,
            payload_error: value.1,
        }
    }
}

impl From<(&'static str, ParseOrderSideError)> for DecodeError {
    fn from(value: (&'static str, ParseOrderSideError)) -> Self {
        Self::ParseOrderSideError {
            field_name: value.0,
            order_side_error: value.1,
        }
    }
}

impl From<ParseAttributeError> for DecodeError {
    fn from(value: ParseAttributeError) -> Self {
        Self::ParseAttributeError(value)
    }
}

impl From<(&'static str, ParseDateTimeError)> for DecodeError {
    fn from(value: (&'static str, ParseDateTimeError)) -> Self {
        Self::ParseDateTimeError {
            field_name: value.0,
            datetime_error: value.1,
        }
    }
}

impl From<(&'static str, std::convert::Infallible)> for DecodeError {
    fn from(value: (&'static str, std::convert::Infallible)) -> Self {
        unreachable!()
    }
}

#[derive(Debug, Clone, Error)]
/// An error returned when parsing a datetime fails.
pub enum ParseDateTimeError {
    /// Failed to parse an [`IbTimeZone`]
    #[error("Failed to parse timezone")]
    Timezone(#[from] ParseTimezoneError),
    /// Invalid timestamp
    #[error("Invalid timestamp: out-of-range number of seconds and/or invalid nanosecond")]
    Timestamp,
    /// Failed to resolve single timezone
    #[error("Failed to resolve a single timezone from provided information")]
    Single,
}
