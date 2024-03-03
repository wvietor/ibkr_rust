use anyhow::Context;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use core::future::Future;

use crate::account::{self, Tag, TagValue};
use crate::contract::{
    Commodity, Contract, ContractId, Crypto, Forex, Index, SecFuture, SecOption, SecOptionInner,
    SecurityId, Stock,
};
use crate::payload::{
    market_depth::{CompleteEntry, Entry, Operation},
    Bar, BarCore, BidAsk, ExchangeId, Fill, HistogramEntry, Last, MarketDataClass, Midpoint,
    OrderStatus, Pnl, PnlSingle, Position, PositionSummary, Tick, Trade,
};
use crate::tick::{
    Accessibility, AuctionData, CalculationResult, Class, Dividends, EtfNav, ExtremeValue, Ipo,
    MarkPrice, OpenInterest, Period, Price, PriceFactor, QuotingExchanges, Rate, RealTimeVolume,
    RealTimeVolumeBase, SecOptionCalculationResults, SecOptionCalculationSource,
    SecOptionCalculations, SecOptionVolume, Size, SummaryVolume, TimeStamp, Volatility, Yield,
};
use crate::{
    currency::Currency,
    exchange::Routing,
    message::{ToClient, ToWrapper},
    wrapper,
};

type Tx = tokio::sync::mpsc::Sender<ToClient>;
type Rx = tokio::sync::mpsc::Receiver<ToWrapper>;
type Fields = std::vec::IntoIter<String>;

macro_rules! decode_fields {
    ($fields: expr => $ind: literal: String) => {
        nth($fields, $ind).with_context(|| format!("Expected {:?}, found none", &$fields))?
    };
    ($fields: expr => $ind: literal: $f_type: ty) => {
        nth($fields, $ind).with_context(|| format!("Expected {:?}, found none", &$fields))?
            .parse::<$f_type>().with_context(|| format!("Invalid value for {:?}", $fields))?
    };
    ($fields: expr => $($f_name: ident @ $ind: literal: $f_type: ty ),*) => {
        $(
            let $f_name = decode_fields!($fields => $ind: $f_type);
        )*
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
    ($root_name: literal, $name: expr, $value: expr) => {
        match $name.as_str() {
            $root_name => account::Segment::Total(
                $value
                    .parse()
                    .with_context(|| format!("Name: {}, Root name: {}", $name, $root_name))?,
            ),
            concat!($root_name, "-C") => account::Segment::Commodity(
                $value
                    .parse()
                    .with_context(|| format!("Name: {}, Root name: {}", $name, $root_name))?,
            ),
            concat!($root_name, "-P") => account::Segment::Paxos(
                $value
                    .parse()
                    .with_context(|| format!("Name: {}, Root name: {}", $name, $root_name))?,
            ),
            concat!($root_name, "-S") => account::Segment::Security(
                $value
                    .parse()
                    .with_context(|| format!("Name: {}, Root name: {}", $name, $root_name))?,
            ),
            _ => {
                return Err(anyhow::Error::msg(format!(
                    "Could not match {} in {} segment parsing",
                    $name, $root_name
                )))
            }
        }
    };
}

#[ibapi_macros::make_send(Remote(Send): wrapper::Remote)]
pub trait Local: wrapper::Local {
    #[inline]
    fn tick_price_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            decode_fields!(
            fields =>
                req_id @ 2: i64,
                tick_type @ 0: u16,
                price @ 0: f64,
                size @ 0: String,
                attr_mask @ 0: u8
            );

            let size = if size.is_empty() {
                None
            } else {
                Some(
                    size.as_str()
                        .parse()
                        .with_context(|| "Invalid value for size")?,
                )
            };
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
                        _ => panic!("The impossible occurred"),
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
                        _ => panic!("The impossible occurred"),
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
                        _ => panic!("The impossible occurred"),
                    };
                    wrapper.mark_price(req_id, mark).await;
                }
                50..=52 => {
                    let yld = match tick_type {
                        50 => Yield::Bid(price),
                        51 => Yield::Ask(price),
                        52 => Yield::Last(price),
                        _ => panic!("The impossible occurred"),
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
                        _ => panic!("The impossible occurred"),
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
                        _ => panic!("The impossible occurred"),
                    };
                    wrapper.etf_nav(req_id, nav).await;
                }
                t => {
                    return Err(anyhow::Error::msg(format!(
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
    ) -> impl Future<Output = anyhow::Result<()>> {
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
    ) -> impl Future<Output = anyhow::Result<()>> {
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
                    client_id @ 0: i64,
                    why_held @ 0: String,
                    market_cap_price @ 0: f64
            );

            let why_held = match why_held.as_str() {
                "locate" => Some(crate::payload::Locate),
                "" => None,
                s => {
                    return Err(anyhow::anyhow!(
                        "Invalid Locate string. Expected \"locate\" or \"\", got {}",
                        s
                    ))
                }
            };
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
            let status = match status.as_str() {
                "ApiPending" => OrderStatus::ApiPending(core),
                "PendingSubmit" => OrderStatus::PendingSubmit(core),
                "PendingCancel" => OrderStatus::PendingCancel(core),
                "PreSubmitted" => OrderStatus::PreSubmitted(core),
                "Submitted" => OrderStatus::Submitted(core),
                "ApiCancelled" => OrderStatus::ApiCancelled(core),
                "Cancelled" => OrderStatus::Cancelled(core),
                "Filled" => OrderStatus::Filled(core),
                "Inactive" => OrderStatus::Inactive(core),
                s => return Err(anyhow::anyhow!("Invalid order status string. Expected \"PendingSubmit\", \"PendingCancel\", \"PreSubmitted\", \"Submitted\", \"ApiCancelled\", \"Cancelled\", \"Filled\", or \"Inactive\". Got {}", s)),
            };

            wrapper.order_status(status).await;

            Ok(())
        }
    }

    #[inline]
    // todo: Implement a proper Error Enum
    fn err_msg_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
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
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            decode_fields!(
                fields =>
                    order_id @ 1: i64,
                    contract_id @ 0: ContractId,
                    client_id @ 21: i64,
                    permanent_id @ 0: i64,
                    parent_id @ 32: i64
            );
            let parent_id = if parent_id == 0 {
                None
            } else {
                Some(parent_id)
            };
            wrapper
                .open_order(order_id, contract_id, client_id, parent_id, permanent_id)
                .await;
            // outside: 1,
            // hidden: 1
            // discretion: 1
            // good_after: 1
            // skip_shares: 1
            // fa_params: 4
            // model_code: 1
            // good_til: 1
            // rule_80a: 1
            // percent_offset: 1
            // settling_firm: 1
            // short_sale_params: 3
            // auction_strategy: 1
            // box_order_params: 3
            // peg_to_stk_or_vol: 2
            // display_size: 1
            // block: 1
            // sweep: 1
            // all_or_none: 1
            // min_qty: 1
            // oca_type: 1
            // skip_etrade_only: 1
            // skip_firm_quote_only: 1
            // skip_nbbo_price_cap: 1

            /* !!!parent_id!!! */

            // trigger_method: 1
            // vol_order_params: 6 OR 14
            // trail_params: 2
            // basis_points: 2
            // combo_legs: 1 OR arbitrarily many
            // smart_combo_routing_params: 1 OR arbitrarily many
            // scale_order_params: 3 OR 10
            // hedge_params: 1 OR 2
            // opt_out_smart_routing: 1
            // clearing_params: 2
            // not_held: 1
            // delta_neutral: 1 OR 5
            // algo_params: 1 OR arbitrarily many
            // solicited: 1

            /* !!!whatifinfo!!! */

            // vol_randomize: 2
            // peg_to_bench: 0 OR 5
            // conditions: 1 OR arbitrarily many
            // adjusted_order_params: 8
            // soft_dollar: 3
            // cash_qty: 1
            // dont_use_auto: 1
            // is_oms: 1
            // discretionary_up_to_limit: 1
            // use_price_mgmt: 1
            // duration: 1
            // post_to_ats: 1
            // auto_cancel_parent: 1
            // peg_best_peg_mid: 5

            Ok(())
        }
    }

    #[inline]
    fn acct_value_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
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
                "AccountOrGroup" => match value.as_str() {
                    "All" => {
                        account::Attribute::AccountOrGroup(account::Group::All, currency.parse()?)
                    }
                    name => account::Attribute::AccountOrGroup(
                        account::Group::Name(name.to_owned()),
                        currency.parse()?,
                    ),
                },
                "AccountReady" => account::Attribute::AccountReady(value.parse()?),
                "AccountType" => account::Attribute::AccountType(value),
                expand_seg_variants!("AccruedCash") => account::Attribute::AccruedCash(
                    impl_seg_variants!("AccruedCash", name, value),
                    currency.parse()?,
                ),
                expand_seg_variants!("AccruedDividend") => account::Attribute::AccruedDividend(
                    impl_seg_variants!("AccruedDividend", name, value),
                    currency.parse()?,
                ),
                expand_seg_variants!("AvailableFunds") => account::Attribute::AvailableFunds(
                    impl_seg_variants!("AvailableFunds", name, value),
                    currency.parse()?,
                ),
                expand_seg_variants!("Billable") => account::Attribute::Billable(
                    impl_seg_variants!("Billable", name, value),
                    currency.parse()?,
                ),
                "BuyingPower" => account::Attribute::BuyingPower(value.parse()?, currency.parse()?),
                "CashBalance" => account::Attribute::CashBalance(value.parse()?, currency.parse()?),
                expand_seg_variants!("ColumnPrio") => {
                    account::Attribute::ColumnPrio(impl_seg_variants!("ColumnPrio", name, value))
                }
                "CorporateBondValue" => {
                    account::Attribute::CorporateBondValue(value.parse()?, currency.parse()?)
                }
                "Cryptocurrency" => {
                    account::Attribute::Cryptocurrency(value.parse()?, currency.parse()?)
                }
                "Currency" => account::Attribute::Currency(value.parse()?),
                "Cushion" => account::Attribute::Cushion(value.parse()?),
                "DayTradesRemaining" => account::Attribute::DayTradesRemaining(value.parse()?),
                "DayTradesRemainingT+1" => {
                    account::Attribute::DayTradesRemainingTPlus1(value.parse()?)
                }
                "DayTradesRemainingT+2" => {
                    account::Attribute::DayTradesRemainingTPlus2(value.parse()?)
                }
                "DayTradesRemainingT+3" => {
                    account::Attribute::DayTradesRemainingTPlus3(value.parse()?)
                }
                "DayTradesRemainingT+4" => {
                    account::Attribute::DayTradesRemainingTPlus4(value.parse()?)
                }
                "DayTradingStatus-S" => account::Attribute::DayTradingStatus(value),
                expand_seg_variants!("EquityWithLoanValue") => {
                    account::Attribute::EquityWithLoanValue(
                        impl_seg_variants!("EquityWithLoanValue", name, value),
                        currency.parse()?,
                    )
                }
                expand_seg_variants!("ExcessLiquidity") => account::Attribute::ExcessLiquidity(
                    impl_seg_variants!("ExcessLiquidity", name, value),
                    currency.parse()?,
                ),
                "ExchangeRate" => {
                    account::Attribute::ExchangeRate(value.parse()?, currency.parse()?)
                }
                expand_seg_variants!("FullAvailableFunds") => {
                    account::Attribute::FullAvailableFunds(
                        impl_seg_variants!("FullAvailableFunds", name, value),
                        currency.parse()?,
                    )
                }
                expand_seg_variants!("FullExcessLiquidity") => {
                    account::Attribute::FullExcessLiquidity(
                        impl_seg_variants!("FullExcessLiquidity", name, value),
                        currency.parse()?,
                    )
                }
                expand_seg_variants!("FullInitMarginReq") => account::Attribute::FullInitMarginReq(
                    impl_seg_variants!("FullInitMarginReq", name, value),
                    currency.parse()?,
                ),
                expand_seg_variants!("FullMaintMarginReq") => {
                    account::Attribute::FullMaintenanceMarginReq(
                        impl_seg_variants!("FullMaintMarginReq", name, value),
                        currency.parse()?,
                    )
                }
                "FundValue" => account::Attribute::FundValue(value.parse()?, currency.parse()?),
                "FutureOptionValue" => {
                    account::Attribute::FutureOptionValue(value.parse()?, currency.parse()?)
                }
                "FuturesPNL" => account::Attribute::FuturesPnl(value.parse()?, currency.parse()?),
                "FxCashBalance" => {
                    account::Attribute::FxCashBalance(value.parse()?, currency.parse()?)
                }
                "GrossPositionValue" => {
                    account::Attribute::GrossPositionValue(value.parse()?, currency.parse()?)
                }
                "GrossPositionValue-S" => account::Attribute::GrossPositionValueSecurity(
                    value.parse()?,
                    currency.parse()?,
                ),
                expand_seg_variants!("Guarantee") => account::Attribute::Guarantee(
                    impl_seg_variants!("Guarantee", name, value),
                    currency.parse()?,
                ),
                expand_seg_variants!("IndianStockHaircut") => {
                    account::Attribute::IndianStockHaircut(
                        impl_seg_variants!("IndianStockHaircut", name, value),
                        currency.parse()?,
                    )
                }
                expand_seg_variants!("InitMarginReq") => account::Attribute::InitMarginReq(
                    impl_seg_variants!("InitMarginReq", name, value),
                    currency.parse()?,
                ),
                "IssuerOptionValue" => {
                    account::Attribute::IssuerOptionValue(value.parse()?, currency.parse()?)
                }
                "Leverage-S" => account::Attribute::LeverageSecurity(value.parse()?),
                expand_seg_variants!("LookAheadAvailableFunds") => {
                    account::Attribute::LookAheadAvailableFunds(
                        impl_seg_variants!("LookAheadAvailableFunds", name, value),
                        currency.parse()?,
                    )
                }
                expand_seg_variants!("LookAheadExcessLiquidity") => {
                    account::Attribute::LookAheadExcessLiquidity(
                        impl_seg_variants!("LookAheadExcessLiquidity", name, value),
                        currency.parse()?,
                    )
                }
                expand_seg_variants!("LookAheadInitMarginReq") => {
                    account::Attribute::LookAheadInitMarginReq(
                        impl_seg_variants!("LookAheadInitMarginReq", name, value),
                        currency.parse()?,
                    )
                }
                expand_seg_variants!("LookAheadMaintMarginReq") => {
                    account::Attribute::LookAheadMaintenanceMarginReq(
                        impl_seg_variants!("LookAheadMaintMarginReq", name, value),
                        currency.parse()?,
                    )
                }
                "LookAheadNextChange" => account::Attribute::LookAheadNextChange(value.parse()?),
                expand_seg_variants!("MaintMarginReq") => account::Attribute::MaintenanceMarginReq(
                    impl_seg_variants!("MaintMarginReq", name, value),
                    currency.parse()?,
                ),
                "MoneyMarketFundValue" => {
                    account::Attribute::MoneyMarketFundValue(value.parse()?, currency.parse()?)
                }
                "MutualFundValue" => {
                    account::Attribute::MutualFundValue(value.parse()?, currency.parse()?)
                }
                "NLVAndMarginInReview" => account::Attribute::NlvAndMarginInReview(value.parse()?),
                "NetDividend" => account::Attribute::NetDividend(value.parse()?, currency.parse()?),
                expand_seg_variants!("NetLiquidation") => account::Attribute::NetLiquidation(
                    impl_seg_variants!("NetLiquidation", name, value),
                    currency.parse()?,
                ),
                "NetLiquidationByCurrency" => {
                    account::Attribute::NetLiquidationByCurrency(value.parse()?, currency.parse()?)
                }
                "NetLiquidationUncertainty" => {
                    account::Attribute::NetLiquidationUncertainty(value.parse()?, currency.parse()?)
                }
                "OptionMarketValue" => {
                    account::Attribute::OptionMarketValue(value.parse()?, currency.parse()?)
                }
                expand_seg_variants!("PASharesValue") => account::Attribute::PaSharesValue(
                    impl_seg_variants!("PASharesValue", name, value),
                    currency.parse()?,
                ),
                expand_seg_variants!("PhysicalCertificateValue") => {
                    account::Attribute::PhysicalCertificateValue(
                        impl_seg_variants!("PhysicalCertificateValue", name, value),
                        currency.parse()?,
                    )
                }
                expand_seg_variants!("PostExpirationExcess") => {
                    account::Attribute::PostExpirationExcess(
                        impl_seg_variants!("PostExpirationExcess", name, value),
                        currency.parse()?,
                    )
                }
                expand_seg_variants!("PostExpirationMargin") => {
                    account::Attribute::PostExpirationMargin(
                        impl_seg_variants!("PostExpirationMargin", name, value),
                        currency.parse()?,
                    )
                }
                "PreviousDayEquityWithLoanValue" => {
                    account::Attribute::PreviousDayEquityWithLoanValue(
                        value.parse()?,
                        currency.parse()?,
                    )
                }
                "PreviousDayEquityWithLoanValue-S" => {
                    account::Attribute::PreviousDayEquityWithLoanValueSecurity(
                        value.parse()?,
                        currency.parse()?,
                    )
                }
                "RealCurrency" => account::Attribute::RealCurrency(currency.parse()?),
                "RealizedPnL" => account::Attribute::RealizedPnL(value.parse()?, currency.parse()?),
                "RegTEquity" => account::Attribute::RegTEquity(value.parse()?, currency.parse()?),
                "RegTEquity-S" => {
                    account::Attribute::RegTEquitySecurity(value.parse()?, currency.parse()?)
                }
                "RegTMargin" => account::Attribute::RegTMargin(value.parse()?, currency.parse()?),
                "RegTMargin-S" => {
                    account::Attribute::RegTMarginSecurity(value.parse()?, currency.parse()?)
                }
                "SMA" => account::Attribute::Sma(value.parse()?, currency.parse()?),
                "SMA-S" => account::Attribute::SmaSecurity(value.parse()?, currency.parse()?),
                "StockMarketValue" => {
                    account::Attribute::StockMarketValue(value.parse()?, currency.parse()?)
                }
                "TBillValue" => account::Attribute::TBillValue(value.parse()?, currency.parse()?),
                "TBondValue" => account::Attribute::TBondValue(value.parse()?, currency.parse()?),
                "TotalCashBalance" => {
                    account::Attribute::TotalCashBalance(value.parse()?, currency.parse()?)
                }
                expand_seg_variants!("TotalCashValue") => account::Attribute::TotalCashValue(
                    impl_seg_variants!("TotalCashValue", name, value),
                    currency.parse()?,
                ),
                expand_seg_variants!("TotalDebitCardPendingCharges") => {
                    account::Attribute::TotalDebitCardPendingCharges(
                        impl_seg_variants!("TotalDebitCardPendingCharges", name, value),
                        currency.parse()?,
                    )
                }
                "TradingType-S" => account::Attribute::TradingTypeSecurity(value),
                "UnrealizedPnL" => {
                    account::Attribute::UnrealizedPnL(value.parse()?, currency.parse()?)
                }
                "WarrantValue" => {
                    account::Attribute::WarrantValue(value.parse()?, currency.parse()?)
                }
                "WhatIfPMEnabled" => account::Attribute::WhatIfPMEnabled(value.parse()?),
                expand_seg_variants!("SegmentTitle") => {
                    if name.ends_with('C') || name.ends_with('P') || name.ends_with('S') {
                        return Ok(());
                    }
                    return Err(anyhow::Error::msg("Unexpected segment title encountered.  This may mandate an API update: currently-supported values are C, P, and S as outlined in the account::Segment type."));
                }
                _ => {
                    return Err(anyhow::Error::msg(format!(
                        "Invalid account attribute encountered: {name}"
                    )))
                }
            };
            wrapper.account_attribute(attribute, account_number).await;
            Ok(())
        }
    }

    #[inline]
    fn portfolio_value_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            decode_fields!(
                fields =>
                    contract_id @ 2: ContractId,
                    position @ 10: f64,
                    market_price @ 0: f64,
                    market_value @ 0: f64,
                    average_cost @ 0: f64,
                    unrealized_pnl @ 0: f64,
                    realized_pnl @ 0: f64,
                    account_name @ 0: String
            );
            wrapper
                .position(Position {
                    contract_id,
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
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            decode_fields!(
                fields =>
                    timestamp @ 2: String
            );
            wrapper
                .account_attribute_time(NaiveTime::parse_from_str(timestamp.as_str(), "%H:%M")?)
                .await;
            Ok(())
        }
    }

    #[inline]
    fn next_valid_id_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
        tx: &mut Tx,
        rx: &mut Rx,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move { Ok(()) }
    }

    #[inline]
    fn contract_data_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
        tx: &mut Tx,
        rx: &mut Rx,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move { decode_contract_no_wrapper(fields, tx, rx).await }
    }

    #[inline]
    fn execution_data_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn market_depth_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
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

            let entry = CompleteEntry::Ordinary(Entry::try_from((side, position, price, size))?);
            let operation = Operation::try_from((operation, entry))?;

            wrapper.update_market_depth(req_id, operation).await;
            Ok(())
        }
    }

    #[inline]
    fn market_depth_l2_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
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
            let entry = Entry::try_from((side, position, price, size))?;
            let entry = match is_smart {
                0 => CompleteEntry::MarketMaker {
                    market_maker: market_maker
                        .chars()
                        .take(4)
                        .collect::<Vec<char>>()
                        .try_into()
                        .map_err(|_| anyhow::Error::msg("Invalid Mpid encountered"))?,
                    entry,
                },
                _ => CompleteEntry::SmartDepth {
                    exchange: market_maker.parse()?,
                    entry,
                },
            };
            let operation = Operation::try_from((operation, entry))?;

            wrapper.update_market_depth(req_id, operation).await;
            Ok(())
        }
    }

    #[inline]
    fn news_bulletins_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }
    #[inline]
    fn managed_accts_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
        tx: &mut Tx,
        rx: &mut Rx,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move { Ok(()) }
    }

    #[inline]
    fn receive_fa_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn historical_data_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            decode_fields!(
                fields =>
                    req_id @ 1: i64,
                    start_date_str @ 0: String,
                    end_date_str @ 0: String,
                    count @ 0: usize
            );
            let mut bars = Vec::with_capacity(count);
            for chunk in fields.collect::<Vec<String>>().chunks(8) {
                if let [datetime_str, open, high, low, close, volume, wap, trade_count] = chunk {
                    let (date, rem) = NaiveDate::parse_and_remainder(datetime_str, "%Y%m%d")?;
                    let time = if rem.is_empty() {
                        NaiveTime::default()
                    } else {
                        NaiveTime::parse_and_remainder(rem, " %T")?.0
                    };
                    let core = BarCore {
                        datetime: NaiveDateTime::new(date, time),
                        open: open.parse()?,
                        high: high.parse()?,
                        low: low.parse()?,
                        close: close.parse()?,
                    };
                    let (volume, wap, trade_count) =
                        (volume.parse()?, wap.parse()?, trade_count.parse::<i64>()?);
                    let bar = if volume > 0. && wap > 0. && trade_count > 0 {
                        Bar::Trades(crate::payload::Trade {
                            bar: core,
                            volume,
                            wap,
                            trade_count: trade_count.try_into()?,
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
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn scanner_parameters_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn scanner_data_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn tick_option_computation_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
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
                t => {
                    return Err(anyhow::Error::msg(format!(
                        "Unexpected option calculation base: {t}"
                    )))
                }
            };
            let calc = match tick_type {
                10..=13 | 53 => Class::Live(match tick_type {
                    10 => SecOptionCalculationSource::Bid(calc),
                    11 => SecOptionCalculationSource::Ask(calc),
                    12 => SecOptionCalculationSource::Last(calc),
                    13 => SecOptionCalculationSource::Model(calc),
                    53 => SecOptionCalculationSource::Custom(calc),
                    _ => panic!("The impossible occurred"),
                }),
                80..=83 => Class::Delayed(match tick_type {
                    80 => SecOptionCalculationSource::Bid(calc),
                    81 => SecOptionCalculationSource::Ask(calc),
                    82 => SecOptionCalculationSource::Last(calc),
                    83 => SecOptionCalculationSource::Model(calc),
                    _ => panic!("The impossible occurred"),
                }),
                _ => panic!("The impossible occurred"),
            };
            wrapper.sec_option_computation(req_id, calc).await;

            Ok(())
        }
    }

    #[inline]
    fn tick_generic_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
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
    ) -> impl Future<Output = anyhow::Result<()>> {
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
                        _ => panic!("The impossible occurred"),
                    };
                    wrapper.quoting_exchanges(req_id, quoting_exchanges).await;
                }
                45 | 85 | 88 => {
                    let value = value
                        .parse()
                        .with_context(|| "Invalid value in timestamp decode")?;
                    if value == 0 {
                        return Ok(());
                    }
                    let timestamp = match tick_type {
                        45 | 88 => NaiveDateTime::from_timestamp_opt(value, 0),
                        85 => NaiveDateTime::from_timestamp_millis(value),
                        _ => panic!("The impossible occurred"),
                    }
                    .ok_or_else(|| {
                        anyhow::Error::msg("Invalid timestamp encountered in string message")
                    })?;
                    let timestamp = match tick_type {
                        45 => Class::Live(TimeStamp::Last(timestamp)),
                        85 => Class::Live(TimeStamp::Regulatory(timestamp)),
                        88 => Class::Delayed(TimeStamp::Last(timestamp)),
                        _ => panic!("The impossible occurred"),
                    };
                    wrapper.timestamp(req_id, timestamp).await;
                }
                48 | 77 => {
                    let mut vols = value.split(';');
                    let base = RealTimeVolumeBase {
                        last_price: vols
                            .next()
                            .ok_or(MissingInputData)
                            .with_context(|| "No last price in real time volume message")?
                            .parse()
                            .with_context(|| "Invalid value in RealTimeVolume last_price decode")?,
                        last_size: vols
                            .next()
                            .ok_or(MissingInputData)
                            .with_context(|| "No last size in real time volume message")?
                            .parse()
                            .with_context(|| "Invalid value in RealTimeVolume last_size decode")?,
                        last_time: NaiveDateTime::from_timestamp_opt(
                            vols.next()
                                .ok_or(MissingInputData)
                                .with_context(|| "No last time in real time volume message")?
                                .parse()
                                .with_context(|| {
                                    "Invalid value in RealTimeVolume last_time decode"
                                })?,
                            0,
                        )
                        .ok_or_else(|| {
                            anyhow::Error::msg(
                                "Invalid Unix timestamp found in real time volume message",
                            )
                        })?,
                        day_volume: vols
                            .next()
                            .ok_or(MissingInputData)
                            .with_context(|| "No day volume in real time volume message")?
                            .parse()
                            .with_context(|| "Invalid value in RealTimeVolume day_volume decode")?,
                        vwap: vols
                            .next()
                            .ok_or(MissingInputData)
                            .with_context(|| "No VWAP in real time volume message")?
                            .parse()
                            .with_context(|| "Invalid value in RealTimeVolume vwap decode")?,
                        single_mm: vols
                            .next()
                            .ok_or(MissingInputData)
                            .with_context(|| {
                                "No single market maker parameter in real time volume message"
                            })?
                            .parse()
                            .with_context(|| "Invalid value in RealTimeVolume single_mm decode")?,
                    };
                    let volume = match tick_type {
                        48 => RealTimeVolume::All(base),
                        77 => RealTimeVolume::Trades(base),
                        _ => panic!("The impossible occurred"),
                    };
                    wrapper.real_time_volume(req_id, volume).await;
                }
                59 => {
                    let mut divs = value.split(',');
                    let dividends = Dividends {
                        trailing_year: divs
                            .next()
                            .ok_or(MissingInputData)
                            .with_context(|| "No trailing year in dividend message")?
                            .parse()
                            .with_context(|| "Invalid value in Dividends trailing_year decode")?,
                        forward_year: divs
                            .next()
                            .ok_or(MissingInputData)
                            .with_context(|| "No forward year in dividend message")?
                            .parse()
                            .with_context(|| "Invalid value in Dividends forward_year decode")?,
                        next_dividend: (
                            NaiveDate::parse_and_remainder(
                                divs.next()
                                    .ok_or(MissingInputData)
                                    .with_context(|| "No next dividend date in dividend message")?,
                                "%Y%m%d",
                            )
                            .with_context(|| {
                                "Invalid value in Dividends next_dividend decode datetime"
                            })?
                            .0,
                            divs.next()
                                .ok_or(MissingInputData)
                                .with_context(|| "No next price in dividend message")?
                                .parse()
                                .with_context(|| {
                                    "Invalid value in Dividends next_dividend decode value"
                                })?,
                        ),
                    };
                    wrapper.dividends(req_id, dividends).await;
                }
                62 => {
                    wrapper.news(req_id, value).await;
                }
                t => {
                    return Err(anyhow::Error::msg(format!(
                        "Unexpected price market data request: {t}"
                    )))
                }
            };
            Ok(())
        }
    }

    #[inline]
    fn tick_efp_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            unimplemented!();
        }
    }

    #[inline]
    fn current_time_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            decode_fields!(
                fields =>
                    req_id @ 1: i64,
                    datetime @ 0: i64
            );

            wrapper
                .current_time(
                    NaiveDateTime::from_timestamp_opt(datetime, 0).ok_or_else(|| {
                        anyhow::Error::msg(
                            "Invalid datetime value encountered while parsing the UNIX timestamp!",
                        )
                    })?,
                )
                .await;
            Ok(())
        }
    }

    #[inline]
    fn real_time_bars_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
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
                datetime: NaiveDateTime::from_timestamp_opt(date_time, 0)
                    .ok_or(anyhow::Error::msg("Invalid timestamp"))?,
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
                    trade_count: trade_count.try_into()?,
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
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn contract_data_end_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
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
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            wrapper.open_order_end().await;
            Ok(())
        }
    }

    #[inline]
    fn acct_download_end_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
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
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);

            Ok(())
        }
    }

    #[inline]
    fn delta_neutral_validation_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn tick_snapshot_end_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn market_data_type_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
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
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn position_data_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            decode_fields!(
                fields =>
                    account_number @ 2: String,
                    contract_id @ 0: ContractId,
                    position @ 10: f64,
                    average_cost @ 0: f64
            );
            wrapper
                .position_summary(PositionSummary {
                    contract_id,
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
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            wrapper.position_end().await;
            Ok(())
        }
    }

    #[inline]
    fn account_summary_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
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
                Tag::Cushion => TagValue::Float(Tag::Cushion, value.parse()?),
                Tag::LookAheadNextChange => TagValue::Int(Tag::LookAheadNextChange, value.parse()?),
                Tag::HighestSeverity => TagValue::String(Tag::HighestSeverity, value),
                Tag::DayTradesRemaining => TagValue::Int(Tag::DayTradesRemaining, value.parse()?),
                Tag::Leverage => TagValue::Float(Tag::Leverage, value.parse()?),
                t => TagValue::Currency(t, value.parse()?, currency.parse()?),
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
    ) -> impl Future<Output = anyhow::Result<()>> {
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
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn verify_completed_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn display_group_list_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn display_group_updated_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn verify_and_auth_message_api_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn verify_and_auth_completed_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn position_multi_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn position_multi_end_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn account_update_multi_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn account_update_multi_end_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn security_definition_option_parameter_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn security_definition_option_parameter_end_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn soft_dollar_tiers_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn family_codes_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn symbol_samples_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn mkt_depth_exchanges_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn tick_req_params_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
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
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn news_article_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn tick_news_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn news_providers_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn historical_news_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn historical_news_end_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn head_timestamp_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            decode_fields!(
                fields =>
                    req_id @ 1: i64,
                    timestamp @ 0: String
            );
            wrapper
                .head_timestamp(
                    req_id,
                    NaiveDateTime::parse_from_str(timestamp.as_str(), "%Y%m%d-%T")?,
                )
                .await;
            Ok(())
        }
    }

    #[inline]
    fn histogram_data_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
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
                .collect::<Result<Vec<f64>, _>>()?
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
    ) -> impl Future<Output = anyhow::Result<()>> {
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
                datetime: NaiveDateTime::parse_and_remainder(datetime_str.as_str(), "%Y%m%d %T")?.0,
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
                    trade_count: trade_count.try_into()?,
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
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn reroute_mkt_depth_req_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn market_rule_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn pnl_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
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
    ) -> impl Future<Output = anyhow::Result<()>> {
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
    ) -> impl Future<Output = anyhow::Result<()>> {
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
                    ticks.push(Tick::Midpoint(Midpoint {
                        datetime: NaiveDateTime::from_timestamp_opt(time.parse()?, 0)
                            .ok_or_else(|| anyhow::Error::msg("Invalid datetime"))?,
                        price: price.parse()?,
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
    ) -> impl Future<Output = anyhow::Result<()>> {
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
                    ticks.push(Tick::BidAsk(BidAsk {
                        datetime: NaiveDateTime::from_timestamp_opt(time.parse()?, 0)
                            .ok_or_else(|| anyhow::Error::msg("Invalid datetime"))?,
                        bid_price: bid_price.parse()?,
                        ask_price: ask_price.parse()?,
                        bid_size: bid_size.parse()?,
                        ask_size: ask_size.parse()?,
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
    ) -> impl Future<Output = anyhow::Result<()>> {
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
                    ticks.push(Tick::Last(Last {
                        datetime: NaiveDateTime::from_timestamp_opt(time.parse()?, 0)
                            .ok_or_else(|| anyhow::Error::msg("Invalid datetime"))?,
                        price: price.parse()?,
                        size: size.parse()?,
                        exchange: exchange.parse()?,
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
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            decode_fields!(
                fields =>
                    req_id @ 1: i64,
                    tick_type @ 0: u8,
                    timestamp @ 0: i64
            );
            let datetime = NaiveDateTime::from_timestamp_opt(timestamp, 0)
                .ok_or_else(|| anyhow::Error::msg("Invalid timestamp"))?;
            let tick = match tick_type {
                1 | 2 => Tick::Last(Last {
                    datetime,
                    price: nth(fields, 0)?.parse()?,
                    size: nth(fields, 0)?.parse()?,
                    exchange: nth(fields, 1)?.parse()?,
                }),
                3 => {
                    decode_fields!(
                        fields =>
                            bid_price @ 0: f64,
                            ask_price @ 0: f64,
                            bid_size @ 0: f64,
                            ask_size @ 0: f64
                    );
                    Tick::BidAsk(BidAsk {
                        datetime,
                        bid_price,
                        ask_price,
                        bid_size,
                        ask_size,
                    })
                }
                4 => Tick::Midpoint(Midpoint {
                    datetime,
                    price: nth(fields, 0)?.parse()?,
                }),
                _ => Err(anyhow::Error::msg("Unexpected tick type"))?,
            };
            wrapper.live_tick(req_id, tick).await;
            Ok(())
        }
    }

    #[inline]
    fn order_bound_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn completed_order_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn completed_orders_end_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn replace_fa_end_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn wsh_meta_data_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn wsh_event_data_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn historical_schedule_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            println!("{:?}", &fields);
            Ok(())
        }
    }

    #[inline]
    fn user_info_msg(
        fields: &mut Fields,
        wrapper: &mut Self,
    ) -> impl Future<Output = anyhow::Result<()>> {
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
    ) -> impl Future<Output = anyhow::Result<()>> {
        async move {
            match tick_type {
                0 | 3 | 5 => {
                    let size = Class::Live(match tick_type {
                        0 => Size::Bid(value),
                        3 => Size::Ask(value),
                        5 => Size::Last(value),
                        _ => panic!("The impossible occurred"),
                    });
                    wrapper.size_data(req_id, size).await;
                }
                8 | 74 => {
                    let volume = match tick_type {
                        8 => Class::Live(value),
                        74 => Class::Delayed(value),
                        _ => panic!("The impossible occurred"),
                    };
                    wrapper.volume(req_id, volume).await;
                }
                21 | 63 | 64 | 65 => {
                    let volume = match tick_type {
                        21 => SummaryVolume::NinetyDayAverage(value),
                        63 => SummaryVolume::ThreeMinutes(value),
                        64 => SummaryVolume::FiveMinutes(value),
                        65 => SummaryVolume::TenMinutes(value),
                        _ => panic!("The impossible occurred"),
                    };
                    wrapper.summary_volume(req_id, volume).await;
                }
                23 | 24 | 58 => {
                    let vol = match tick_type {
                        23 => Volatility::SecOptionHistorical(value),
                        24 => Volatility::SecOptionImplied(value),
                        58 => Volatility::RealTimeHistorical(value),
                        _ => panic!("The impossible occurred"),
                    };
                    wrapper.volatility(req_id, vol).await;
                }
                29 | 30 | 87 => {
                    let volume = match tick_type {
                        29 => SecOptionVolume::Call(value),
                        30 => SecOptionVolume::Put(value),
                        87 => SecOptionVolume::Average(value),
                        _ => panic!("The impossible occurred"),
                    };
                    wrapper.sec_option_volume(req_id, volume).await;
                }
                34 | 36 | 61 => {
                    let auction = match tick_type {
                        34 => AuctionData::Volume(value),
                        36 => AuctionData::Imbalance(value),
                        61 => AuctionData::Regulatory(value),
                        _ => panic!("The impossible occurred"),
                    };
                    wrapper.auction(req_id, auction).await;
                }
                27 | 28 | 86 => {
                    let open_interest = match tick_type {
                        27 => OpenInterest::SecOptionCall(value),
                        28 => OpenInterest::SecOptionPut(value),
                        86 => OpenInterest::SecFuture(value),
                        _ => panic!("The impossible occurred"),
                    };
                    wrapper.open_interest(req_id, open_interest).await;
                }
                31 | 60 => {
                    let factor = match tick_type {
                        31 => PriceFactor::IndexFuturePremium(value),
                        60 => PriceFactor::BondFactorMultiplier(value),
                        _ => panic!("The impossible occurred"),
                    };
                    wrapper.price_factor(req_id, factor).await;
                }
                46 | 49 | 89 => {
                    let access = match tick_type {
                        46 => Accessibility::Shortable(value),
                        49 => Accessibility::Halted(value),
                        89 => Accessibility::ShortableShares(value),
                        _ => panic!("The impossible occurred"),
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
                        _ => panic!("The impossible occurred"),
                    };
                    wrapper.rate(req_id, rate).await;
                }
                69..=71 => {
                    let size = Class::Delayed(match tick_type {
                        69 => Size::Bid(value),
                        70 => Size::Ask(value),
                        71 => Size::Last(value),
                        _ => panic!("The impossible occurred"),
                    });
                    wrapper.size_data(req_id, size).await;
                }
                101 | 102 => {
                    let ipo = match tick_type {
                        101 => Ipo::Estimated(value),
                        102 => Ipo::Final(value),
                        _ => panic!("The impossible occurred"),
                    };
                    wrapper.ipo(req_id, ipo).await;
                }
                t => {
                    return Err(anyhow::Error::msg(format!(
                        "Unexpected generic market data request: {t}"
                    )))
                }
            };

            Ok(())
        }
    }
}

impl<W: wrapper::Local> Local for W {}

impl<W: wrapper::Remote> Remote for W {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct MissingInputData;

impl std::fmt::Display for MissingInputData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Missing value encountered while decoding an API callback"
        )
    }
}

impl std::error::Error for MissingInputData {
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

#[inline]
pub(crate) fn nth(fields: &mut Fields, n: usize) -> Result<String, MissingInputData> {
    fields.nth(n).ok_or(MissingInputData)
}

#[inline]
pub(crate) async fn decode_contract_no_wrapper(
    fields: &mut Fields,
    tx: &mut Tx,
    rx: &mut Rx,
) -> anyhow::Result<()> {
    decode_fields!(
        fields =>
            req_id @ 1: i64,
            symbol @ 0: String,
            sec_type @ 0: String,
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
        .with_context(|| "Invalid exchange in valid_exchanges")?;
    let security_ids = (0..security_id_count)
        .map(|_| {
            match nth(fields, 0)
                .with_context(|| "Expected number of security_ids but none found")?
                .to_uppercase()
                .as_str()
            {
                "CUSIP" => Ok(SecurityId::Cusip(
                    nth(fields, 0).with_context(|| "Expected CUSIP but none found")?,
                )),
                "SEDOL" => Ok(SecurityId::Sedol(
                    nth(fields, 0).with_context(|| "Expected SEDOL but none found")?,
                )),
                "ISIN" => Ok(SecurityId::Isin(
                    nth(fields, 0).with_context(|| "Expected ISIN but none found")?,
                )),
                "RIC" => Ok(SecurityId::Ric(
                    nth(fields, 0).with_context(|| "Expected RIC but none found")?,
                )),
                _ => Err(anyhow::Error::msg(
                    "Invalid security_id type found in STK contract_data_msg",
                )),
            }
        })
        .collect::<Result<_, _>>()?;

    if let Ok(ToWrapper::ContractQuery((query_client, req_id_client))) = rx.try_recv() {
        if let crate::contract::Query::IbContractId(con_id_client, routing_client) = query_client {
            if con_id_client != contract_id {
                return Err(anyhow::Error::msg("Unexpected contract ID"));
            }
            if exchange != routing_client {
                return Err(anyhow::Error::msg("Unexpected routing exchange"));
            }
        }
        if req_id_client != req_id {
            return Err(anyhow::Error::msg("Unexpected request ID"));
        }
        let contract = match sec_type.as_str() {
            "STK" => Some(Contract::Stock(Stock {
                symbol,
                exchange,
                currency,
                local_symbol,
                trading_class,
                contract_id,
                min_tick,
                primary_exchange: primary_exchange
                    .parse()
                    .with_context(|| "Invalid exchange in STK primary_exchange")?,
                long_name,
                sector,
                order_types,
                valid_exchanges,
                security_ids,
                stock_type: nth(fields, 5).with_context(|| "Expected stock_type but none found")?,
            })),
            "OPT" => {
                let inner = SecOptionInner {
                    contract_id,
                    min_tick,
                    symbol,
                    exchange,
                    strike,
                    multiplier: multiplier
                        .parse()
                        .with_context(|| "Invalid multiplier in OPT multiplier")?,
                    expiration_date: NaiveDate::parse_and_remainder(
                        expiration_date.as_str(),
                        "%Y%m%d",
                    )
                    .with_context(|| "Invalid date string in OPT expiration_date")?
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
                    _ => return Err(anyhow::Error::msg("Unexpected option class")),
                }
            }
            "CRYPTO" => Some(Contract::Crypto(Crypto {
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
            "CASH" => Some(Contract::Forex(Forex {
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
            "IND" => Some(Contract::Index(Index {
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
            "FUT" => Some(Contract::SecFuture(SecFuture {
                contract_id,
                min_tick,
                symbol,
                exchange,
                multiplier: multiplier
                    .parse()
                    .with_context(|| "Invalid multiplier in FUT multiplier")?,
                expiration_date: NaiveDate::parse_and_remainder(expiration_date.as_str(), "%Y%m%d")
                    .with_context(|| "Invalid date string in OPT expiration_date")?
                    .0,
                trading_class,
                underlying_contract_id,
                currency,
                local_symbol,
                long_name,
                order_types,
                valid_exchanges,
            })),
            "CMDTY" => Some(Contract::Commodity(Commodity {
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
            _ => todo!(),
        };

        tx.send(ToClient::NewContract(
            contract.ok_or_else(|| anyhow::Error::msg("No contract was created"))?,
        ))
        .await
        .with_context(|| "Failure when sending contract")?;
    }
    Ok(())
}
