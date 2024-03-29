use core::future::Future;

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use thiserror::Error;

use crate::account::{self, Tag, TagValue};
use crate::contract::{
    Commodity, Contract, ContractId, ContractType, Crypto, Forex, Index, Proxy, SecFuture,
    SecOption, SecOptionInner, SecurityId, Stock,
};
use crate::exchange::Primary;
use crate::execution::{Exec, Execution, OrderSide};
use crate::payload::{
    market_depth::{CompleteEntry, Entry, Operation},
    Bar, BarCore, BidAsk, ExchangeId, Fill, HistogramEntry, Last, MarketDataClass, Midpoint,
    OrderStatus, Pnl, PnlSingle, Position, PositionSummary, TickData, Trade,
};
use crate::tick::{
    Accessibility, AuctionData, CalculationResult, Class, Dividends, EtfNav, ExtremeValue, Ipo,
    MarkPrice, OpenInterest, Period, Price, PriceFactor, QuotingExchanges, Rate, RealTimeVolume,
    RealTimeVolumeBase, SecOptionCalculationResults, SecOptionCalculationSource,
    SecOptionCalculations, SecOptionVolume, Size, SummaryVolume, TimeStamp, Volatility, Yield,
};
use crate::timezone::ParseTimezoneError;
use crate::{
    currency::Currency,
    exchange::Routing,
    message::{ToClient, ToWrapper},
    timezone, wrapper,
};

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
            s => Some(s.parse::<$op_f_type>().map_err(|e| (stringify!($f_name), e).into())?)
        };
    };
    ($fields: expr => $f_name: ident @ $ind: literal: $f_type: ty) => {
        let $f_name = nth($fields, $ind, stringify!($f_name))?
            .parse::<$f_type>().map_err(|e| (stringify!($f_name), e).into())?;
    };
    ($fields: expr => $($f_name: ident @ $ind: literal: $f_type: ty ),* $(,)?) => {
        $(
            decode_fields!($fields => $f_name @ $ind: $f_type);
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
                    .map_err(|e| ($name, account::AttributeError(e.to_string())))?,
            ),
            concat!($root_name, "-C") => account::Segment::Commodity(
                $value
                    .parse()
                    .map_err(|e| ($name, account::AttributeError(e.to_string())))?,
            ),
            concat!($root_name, "-P") => account::Segment::Paxos(
                $value
                    .parse()
                    .map_err(|e| ($name, account::AttributeError(e.to_string())))?,
            ),
            concat!($root_name, "-S") => account::Segment::Security(
                $value
                    .parse()
                    .map_err(|e| ($name, account::AttributeError(e.to_string())))?,
            ),
            _ => {
                return Err(DecodeError::Other(format!(
                    "Could not match {} in {} segment parsing",
                    $name, $root_name
                )))
            }
        }
    };
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
                price @ 0: f64,
                size @ 0: Option<f64>,
                _attr_mask @ 0: u8
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
                    client_id @ 0: i64,
                    why_held @ 0: Option<crate::payload::Locate>,
                    market_cap_price @ 0: f64
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
                .order_status((status.as_str(), core).try_into()?)
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
            let proxy = deserialize_contract_proxy(fields)?;
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
                "AccountOrGroup" => match value.as_str() {
                    "All" => account::Attribute::AccountOrGroup(
                        account::Group::All,
                        currency.parse().map_err(|e| {
                            ("AccountOrGroup", account::AttributeError(e.to_string())).into()
                        })?,
                    ),
                    name => account::Attribute::AccountOrGroup(
                        account::Group::Name(name.to_owned()),
                        currency.parse().map_err(|e| {
                            (name.as_str(), account::AttributeError(e.to_string())).into()
                        })?,
                    ),
                },
                "AccountReady" => account::Attribute::AccountReady(value.parse().map_err(|e| {
                    ("AccountReady", account::AttributeError(e.to_string())).into()
                })?),
                "AccountType" => account::Attribute::AccountType(value),
                expand_seg_variants!("AccruedCash") => account::Attribute::AccruedCash(
                    impl_seg_variants!("AccruedCash", name, value),
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                expand_seg_variants!("AccruedDividend") => account::Attribute::AccruedDividend(
                    impl_seg_variants!("AccruedDividend", name, value),
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                expand_seg_variants!("AvailableFunds") => account::Attribute::AvailableFunds(
                    impl_seg_variants!("AvailableFunds", name, value),
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                expand_seg_variants!("Billable") => account::Attribute::Billable(
                    impl_seg_variants!("Billable", name, value),
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                "BuyingPower" => account::Attribute::BuyingPower(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })??,
                ),
                "CashBalance" => account::Attribute::CashBalance(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                expand_seg_variants!("ColumnPrio") => {
                    account::Attribute::ColumnPrio(impl_seg_variants!("ColumnPrio", name, value))
                }
                "CorporateBondValue" => account::Attribute::CorporateBondValue(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                "Cryptocurrency" => account::Attribute::Cryptocurrency(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                "Currency" => {
                    account::Attribute::Currency(value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?)
                }
                "Cushion" => {
                    account::Attribute::Cushion(value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?)
                }
                "DayTradesRemaining" => {
                    account::Attribute::DayTradesRemaining(value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?)
                }
                "DayTradesRemainingT+1" => {
                    account::Attribute::DayTradesRemainingTPlus1(value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?)
                }
                "DayTradesRemainingT+2" => {
                    account::Attribute::DayTradesRemainingTPlus2(value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?)
                }
                "DayTradesRemainingT+3" => {
                    account::Attribute::DayTradesRemainingTPlus3(value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?)
                }
                "DayTradesRemainingT+4" => {
                    account::Attribute::DayTradesRemainingTPlus4(value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?)
                }
                "DayTradingStatus-S" => account::Attribute::DayTradingStatus(value),
                expand_seg_variants!("EquityWithLoanValue") => {
                    account::Attribute::EquityWithLoanValue(
                        impl_seg_variants!("EquityWithLoanValue", name, value),
                        currency.parse().map_err(|e| {
                            (name.as_str(), account::AttributeError(e.to_string())).into()
                        })?,
                    )
                }
                expand_seg_variants!("ExcessLiquidity") => account::Attribute::ExcessLiquidity(
                    impl_seg_variants!("ExcessLiquidity", name, value),
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                "ExchangeRate" => account::Attribute::ExchangeRate(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                expand_seg_variants!("FullAvailableFunds") => {
                    account::Attribute::FullAvailableFunds(
                        impl_seg_variants!("FullAvailableFunds", name, value),
                        currency.parse().map_err(|e| {
                            (name.as_str(), account::AttributeError(e.to_string())).into()
                        })?,
                    )
                }
                expand_seg_variants!("FullExcessLiquidity") => {
                    account::Attribute::FullExcessLiquidity(
                        impl_seg_variants!("FullExcessLiquidity", name, value),
                        currency.parse().map_err(|e| {
                            (name.as_str(), account::AttributeError(e.to_string())).into()
                        })?,
                    )
                }
                expand_seg_variants!("FullInitMarginReq") => account::Attribute::FullInitMarginReq(
                    impl_seg_variants!("FullInitMarginReq", name, value),
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                expand_seg_variants!("FullMaintMarginReq") => {
                    account::Attribute::FullMaintenanceMarginReq(
                        impl_seg_variants!("FullMaintMarginReq", name, value),
                        currency.parse().map_err(|e| {
                            (name.as_str(), account::AttributeError(e.to_string())).into()
                        })?,
                    )
                }
                "FundValue" => account::Attribute::FundValue(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                "FutureOptionValue" => account::Attribute::FutureOptionValue(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                "FuturesPNL" => account::Attribute::FuturesPnl(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                "FxCashBalance" => account::Attribute::FxCashBalance(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                "GrossPositionValue" => account::Attribute::GrossPositionValue(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                "GrossPositionValue-S" => account::Attribute::GrossPositionValueSecurity(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                expand_seg_variants!("Guarantee") => account::Attribute::Guarantee(
                    impl_seg_variants!("Guarantee", name, value),
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                expand_seg_variants!("IndianStockHaircut") => {
                    account::Attribute::IndianStockHaircut(
                        impl_seg_variants!("IndianStockHaircut", name, value),
                        currency.parse().map_err(|e| {
                            (name.as_str(), account::AttributeError(e.to_string())).into()
                        })?,
                    )
                }
                expand_seg_variants!("InitMarginReq") => account::Attribute::InitMarginReq(
                    impl_seg_variants!("InitMarginReq", name, value),
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                "IssuerOptionValue" => account::Attribute::IssuerOptionValue(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                "Leverage-S" => {
                    account::Attribute::LeverageSecurity(value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?)
                }
                expand_seg_variants!("LookAheadAvailableFunds") => {
                    account::Attribute::LookAheadAvailableFunds(
                        impl_seg_variants!("LookAheadAvailableFunds", name, value),
                        currency.parse().map_err(|e| {
                            (name.as_str(), account::AttributeError(e.to_string())).into()
                        })?,
                    )
                }
                expand_seg_variants!("LookAheadExcessLiquidity") => {
                    account::Attribute::LookAheadExcessLiquidity(
                        impl_seg_variants!("LookAheadExcessLiquidity", name, value),
                        currency.parse().map_err(|e| {
                            (name.as_str(), account::AttributeError(e.to_string())).into()
                        })?,
                    )
                }
                expand_seg_variants!("LookAheadInitMarginReq") => {
                    account::Attribute::LookAheadInitMarginReq(
                        impl_seg_variants!("LookAheadInitMarginReq", name, value),
                        currency.parse().map_err(|e| {
                            (name.as_str(), account::AttributeError(e.to_string())).into()
                        })?,
                    )
                }
                expand_seg_variants!("LookAheadMaintMarginReq") => {
                    account::Attribute::LookAheadMaintenanceMarginReq(
                        impl_seg_variants!("LookAheadMaintMarginReq", name, value),
                        currency.parse().map_err(|e| {
                            (name.as_str(), account::AttributeError(e.to_string())).into()
                        })?,
                    )
                }
                "LookAheadNextChange" => account::Attribute::LookAheadNextChange(value.parse()?),
                expand_seg_variants!("MaintMarginReq") => account::Attribute::MaintenanceMarginReq(
                    impl_seg_variants!("MaintMarginReq", name, value),
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                "MoneyMarketFundValue" => account::Attribute::MoneyMarketFundValue(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                "MutualFundValue" => account::Attribute::MutualFundValue(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                "NLVAndMarginInReview" => {
                    account::Attribute::NlvAndMarginInReview(value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?)
                }
                "NetDividend" => account::Attribute::NetDividend(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                expand_seg_variants!("NetLiquidation") => account::Attribute::NetLiquidation(
                    impl_seg_variants!("NetLiquidation", name, value),
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                "NetLiquidationByCurrency" => account::Attribute::NetLiquidationByCurrency(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                "NetLiquidationUncertainty" => account::Attribute::NetLiquidationUncertainty(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                "OptionMarketValue" => account::Attribute::OptionMarketValue(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                expand_seg_variants!("PASharesValue") => account::Attribute::PaSharesValue(
                    impl_seg_variants!("PASharesValue", name, value),
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                expand_seg_variants!("PhysicalCertificateValue") => {
                    account::Attribute::PhysicalCertificateValue(
                        impl_seg_variants!("PhysicalCertificateValue", name, value),
                        currency.parse().map_err(|e| {
                            (name.as_str(), account::AttributeError(e.to_string())).into()
                        })?,
                    )
                }
                expand_seg_variants!("PostExpirationExcess") => {
                    account::Attribute::PostExpirationExcess(
                        impl_seg_variants!("PostExpirationExcess", name, value),
                        currency.parse().map_err(|e| {
                            (name.as_str(), account::AttributeError(e.to_string())).into()
                        })?,
                    )
                }
                expand_seg_variants!("PostExpirationMargin") => {
                    account::Attribute::PostExpirationMargin(
                        impl_seg_variants!("PostExpirationMargin", name, value),
                        currency.parse().map_err(|e| {
                            (name.as_str(), account::AttributeError(e.to_string())).into()
                        })?,
                    )
                }
                "PreviousDayEquityWithLoanValue" => {
                    account::Attribute::PreviousDayEquityWithLoanValue(
                        value.parse().map_err(|e| {
                            (name.as_str(), account::AttributeError(e.to_string())).into()
                        })?,
                        currency.parse().map_err(|e| {
                            (name.as_str(), account::AttributeError(e.to_string())).into()
                        })?,
                    )
                }
                "PreviousDayEquityWithLoanValue-S" => {
                    account::Attribute::PreviousDayEquityWithLoanValueSecurity(
                        value.parse().map_err(|e| {
                            (name.as_str(), account::AttributeError(e.to_string())).into()
                        })?,
                        currency.parse().map_err(|e| {
                            (name.as_str(), account::AttributeError(e.to_string())).into()
                        })?,
                    )
                }
                "RealCurrency" => {
                    account::Attribute::RealCurrency(currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?)
                }
                "RealizedPnL" => account::Attribute::RealizedPnL(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                "RegTEquity" => account::Attribute::RegTEquity(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                "RegTEquity-S" => account::Attribute::RegTEquitySecurity(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                "RegTMargin" => account::Attribute::RegTMargin(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                "RegTMargin-S" => account::Attribute::RegTMarginSecurity(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                "SMA" => account::Attribute::Sma(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                "SMA-S" => account::Attribute::SmaSecurity(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                "StockMarketValue" => account::Attribute::StockMarketValue(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                "TBillValue" => account::Attribute::TBillValue(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                "TBondValue" => account::Attribute::TBondValue(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                "TotalCashBalance" => account::Attribute::TotalCashBalance(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                expand_seg_variants!("TotalCashValue") => account::Attribute::TotalCashValue(
                    impl_seg_variants!("TotalCashValue", name, value),
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                expand_seg_variants!("TotalDebitCardPendingCharges") => {
                    account::Attribute::TotalDebitCardPendingCharges(
                        impl_seg_variants!("TotalDebitCardPendingCharges", name, value),
                        currency.parse().map_err(|e| {
                            (name.as_str(), account::AttributeError(e.to_string())).into()
                        })?,
                    )
                }
                "TradingType-S" => account::Attribute::TradingTypeSecurity(value),
                "UnrealizedPnL" => account::Attribute::UnrealizedPnL(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                "WarrantValue" => account::Attribute::WarrantValue(
                    value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                    currency.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?,
                ),
                "WhatIfPMEnabled" => {
                    account::Attribute::WhatIfPMEnabled(value.parse().map_err(|e| {
                        (name.as_str(), account::AttributeError(e.to_string())).into()
                    })?)
                }
                expand_seg_variants!("SegmentTitle") => {
                    if name.ends_with('C') || name.ends_with('P') || name.ends_with('S') {
                        return Ok(());
                    }
                    return Err(DecodeError::Other("Unexpected segment title encountered.  This may mandate an API update: currently-supported values are C, P, and S as outlined in the account::Segment type.".to_owned()));
                }
                _ => {
                    return Err(DecodeError::Other(format!(
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
                .account_attribute_time(NaiveTime::parse_from_str(timestamp.as_str(), "%H:%M")?)
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

            let (dt, tz) = NaiveDateTime::parse_and_remainder(datetime.as_str(), "%Y%m%d %T ")?;
            let datetime = dt
                .and_local_timezone(tz.parse::<timezone::IbTimeZone>()?)
                .single()
                .ok_or(anyhow::anyhow!("Invalid timezone in execution data."))?
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
                    let (date, rem) = NaiveDate::parse_and_remainder(datetime_str, "%Y%m%d")?;
                    let time = if rem.is_empty() {
                        NaiveTime::default()
                    } else {
                        NaiveTime::parse_and_remainder(rem, " %T")?.0
                    };
                    let core = BarCore {
                        datetime: NaiveDateTime::new(date, time).and_utc(),
                        open: open.parse()?,
                        high: high.parse()?,
                        low: low.parse()?,
                        close: close.parse()?,
                    };
                    let (volume, wap, trade_count) =
                        (volume.parse()?, wap.parse()?, trade_count.parse::<i64>()?);
                    let bar = if volume > 0. && wap > 0. && trade_count > 0 {
                        Bar::Trades(Trade {
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
                    let value = value
                        .parse()
                        .with_context(|| "Invalid value in timestamp decode")?;
                    if value == 0 {
                        return Ok(());
                    }
                    let timestamp = match tick_type {
                        45 | 88 => DateTime::from_timestamp(value, 0),
                        85 => DateTime::from_timestamp_millis(value),
                        _ => unreachable!(),
                    }
                    .ok_or_else(|| {
                        anyhow::Error::msg("Invalid timestamp encountered in string message")
                    })?;
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
                        last_time: DateTime::from_timestamp(
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
                        _ => unreachable!(),
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
                    DateTime::from_timestamp(datetime, 0).ok_or_else(|| {
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
                        .ok_or(anyhow::anyhow!("Invalid timestamp."))?,
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
                datetime: NaiveDateTime::parse_and_remainder(datetime_str.as_str(), "%Y%m%d %T")?
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
                        datetime: DateTime::from_timestamp(time.parse()?, 0)
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
                        datetime: DateTime::from_timestamp(time.parse()?, 0)
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
                        datetime: DateTime::from_timestamp(time.parse()?, 0)
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
    ) -> impl Future<Output = DecodeResult> {
        async move {
            decode_fields!(
                fields =>
                    req_id @ 1: i64,
                    tick_type @ 0: u8,
                    timestamp @ 0: i64
            );
            let datetime = DateTime::from_timestamp(timestamp, 0)
                .ok_or_else(|| anyhow::Error::msg("Invalid timestamp"))?;
            let tick = match tick_type {
                1 | 2 => TickData::Last(Last {
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
) -> anyhow::Result<()> {
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
                    .with_context(|| "Invalid exchange in STK primary_exchange")?,
                long_name,
                sector,
                order_types,
                valid_exchanges,
                security_ids,
                stock_type: nth(fields, 5).with_context(|| "Expected stock_type but none found")?,
            })),
            ContractType::SecOption => {
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

        tx.send(ToClient::NewContract(
            contract.ok_or_else(|| anyhow::Error::msg("No contract was created"))?,
        ))
        .await
        .with_context(|| "Failure when sending contract")?;
    }
    Ok(())
}

#[inline]
fn deserialize_contract_proxy(fields: &mut Fields) -> anyhow::Result<Proxy<Contract>> {
    decode_fields!(
        fields =>
            contract_id @ 0: ContractId,
            symbol @ 0: String,
            sec_type @ 0: ContractType,
            expiration_date @ 0: String,
            strike @ 0: String,
            right @ 0: String,
            multiplier @ 0: String,
            primary_exchange @ 0: String,
            currency @ 0: Currency,
            local_symbol @ 0: String,
            trading_class @ 0: String
    );

    let inner = match sec_type {
        ContractType::Stock => Contract::Stock(Stock {
            contract_id,
            min_tick: f64::default(),
            symbol,
            exchange: Routing::Smart,
            primary_exchange: primary_exchange
                .parse()
                .with_context(|| "Invalid primary exchange in STK proxy")?,
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
            exchange: Routing::Smart,
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
            exchange: Routing::Smart,
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
            exchange: Routing::Smart,
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
            exchange: Routing::Smart,
            multiplier: multiplier
                .parse()
                .with_context(|| "Invalid multiplier in FUT proxy")?,
            expiration_date: NaiveDate::parse_and_remainder(expiration_date.as_str(), "%Y%m%d")
                .with_context(|| "Invalid expiration date in OPT proxy")?
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
                exchange: Routing::Smart,
                strike: strike.parse().with_context(|| "Invalid strike")?,
                multiplier: multiplier
                    .parse()
                    .with_context(|| "Invalid multiplier in OPT proxy")?,
                expiration_date: NaiveDate::parse_and_remainder(expiration_date.as_str(), "%Y%m%d")
                    .with_context(|| "Invalid expiration date in OPT proxy")?
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
                        "Unexpected option right. Expected \'C\' or \'P\'. Found {}.",
                        other
                    )))
                }
            };
            Contract::SecOption(op_outer)
        }
    };

    Ok(Proxy { inner })
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
        payload_error: crate::payload::ParsePayloadError,
    },
    #[error("Failed to parse attribute {attribute_name}. Cause: {attribute_error}")]
    /// Failed to parse an [`account::Attribute`] value
    ParseAttributeError {
        attribute_name: &'static str,
        attribute_error: account::ParseAttributeError,
    },
    #[error("Failed to parse datetime field {field_name}. Cause: {datetime_error}")]
    /// Failed to parse an [`account::Attribute`] value
    ParseDateTimeError {
        field_name: &'static str,
        datetime_error: ParseTimezoneError,
    },
    #[error("{0}")]
    Other(String),
}

impl From<(&'static str, std::num::ParseIntError)> for DecodeError {
    fn from(value: (&'static str, std::num::ParseIntError)) -> Self {
        Self::ParseIntError {
            field_name: value.0,
            int_error: value.1,
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

impl From<(&'static str, crate::payload::ParsePayloadError)> for DecodeError {
    fn from(value: (&'static str, crate::payload::ParsePayloadError)) -> Self {
        Self::ParsePayloadError {
            field_name: value.0,
            payload_error: value.1,
        }
    }
}

impl From<(&'static str, account::ParseAttributeError)> for DecodeError {
    fn from(value: (&'static str, account::ParseAttributeError)) -> Self {
        Self::ParseAttributeError {
            attribute_name: value.0,
            attribute_error: value.1,
        }
    }
}

impl From<(&'static str, ParseTimezoneError)> for DecodeError {
    fn from(value: (&'static str, ParseTimezoneError)) -> Self {
        Self::ParseDateTimeError {
            field_name: value.0,
            datetime_error: value.1,
        }
    }
}

impl<E: std::error::Error> From<E> for DecodeError {
    fn from(value: E) -> Self {
        Self::Other(value.to_string())
    }
}
