use anyhow::Context;
use chrono::{NaiveDate, NaiveDateTime};

use crate::payload::{
    market_depth::{CompleteEntry, Entry, Operation},
    CalculationResult, ExchangeId, HistogramEntry, HistoricalBar, HistoricalBarCore, Tick,
};
use crate::tick::{
    Accessibility, AuctionData, Class, Dividends, EtfNav, ExtremeValue, Ipo, MarkPrice,
    OpenInterest, Period, Price, PriceFactor, QuotingExchanges, Rate, RealTimeVolume,
    RealTimeVolumeBase, SecOptionCalculationResults, SecOptionCalculationSource,
    SecOptionCalculations, SecOptionVolume, Size, SummaryVolume, TimeStamp, Volatility, Yield,
};
use crate::{
    contract::{
        Commodity, Contract, ContractId, Crypto, Forex, Index, SecFuture, SecOption,
        SecOptionInner, SecurityId, Stock,
    },
    currency::Currency,
    exchange::Routing,
    message::{ToClient, ToWrapper},
    wrapper::Wrapper,
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

#[inline]
pub fn tick_price_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
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
            wrapper.price_data(req_id, Class::Live(price));
            if let Some(sz) = size {
                wrapper.size_data(req_id, Class::Live(sz));
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
            wrapper.extreme_data(req_id, value);
        }
        35 => {
            wrapper.auction(req_id, AuctionData::Price(price));
        }
        37 | 79 => {
            let mark = match tick_type {
                37 => MarkPrice::Standard(price),
                79 => MarkPrice::Slow(price),
                _ => panic!("The impossible occurred"),
            };
            wrapper.mark_price(req_id, mark);
        }
        50..=52 => {
            let yld = match tick_type {
                50 => Yield::Bid(price),
                51 => Yield::Ask(price),
                52 => Yield::Last(price),
                _ => panic!("The impossible occurred"),
            };
            wrapper.yield_data(req_id, yld);
        }
        57 => {
            wrapper.price_data(req_id, Class::Live(Price::LastRthTrade(price)));
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
            wrapper.price_data(req_id, Class::Delayed(price));
            if let Some(sz) = size {
                wrapper.size_data(req_id, Class::Delayed(sz));
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
            wrapper.etf_nav(req_id, nav);
        }
        t => {
            return Err(anyhow::Error::msg(format!(
                "Unexpected price market data request: {t}"
            )))
        }
    };
    Ok(())
}

#[inline]
pub fn tick_size_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    decode_fields!(
        fields =>
            req_id @ 2: i64,
            tick_type @ 0: u16,
            value @ 0: f64
    );
    decode_generic_tick_msg(req_id, tick_type, value, wrapper)
}

#[inline]
pub fn order_status_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
// todo: Implement a proper Error Enum
pub fn err_msg_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    decode_fields!(
        fields =>
            req_id @ 2: i64,
            error_code @ 0: i64,
            error_string @ 0: String,
            advanced_order_reject_json @ 0: String
    );
    wrapper.error(req_id, error_code, error_string, advanced_order_reject_json);
    Ok(())
}

#[inline]
pub fn open_order_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn acct_value_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn portfolio_value_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn acct_update_time_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
#[allow(clippy::redundant_pub_crate)]
pub(crate) async fn next_valid_id_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
    tx: &mut Tx,
    rx: &mut Rx,
) -> anyhow::Result<()> {
    if rx.try_recv() == Ok(ToWrapper::StartApiNextValidId) {
        tx.send(ToClient::StartApiNextValidId(
            nth(fields, 2)
                .with_context(|| "Expected ID, found none")?
                .parse::<i64>()
                .with_context(|| "Invalid value for ID")?,
        ))
        .await
        .with_context(|| "Failure when sending ID")?;
    }
    Ok(())
}

#[inline]
#[allow(clippy::redundant_pub_crate)]
pub(crate) async fn contract_data_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
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
        .collect::<Result<Vec<Routing>, _>>()
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
        .collect::<Result<Vec<SecurityId>, _>>()?;

    if let Ok(ToWrapper::ContractQuery((con_id_client, req_id_client))) = rx.try_recv() {
        if con_id_client != contract_id {
            return Err(anyhow::Error::msg("Unexpected contract ID"));
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

#[inline]
pub fn execution_data_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn market_depth_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
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

    wrapper.update_market_depth(req_id, operation);
    Ok(())
}

#[inline]
pub fn market_depth_l2_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
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

    wrapper.update_market_depth(req_id, operation);
    Ok(())
}

#[inline]
pub fn news_bulletins_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
#[allow(clippy::redundant_pub_crate)]
pub(crate) async fn managed_accts_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
    tx: &mut Tx,
    rx: &mut Rx,
) -> anyhow::Result<()> {
    if rx.try_recv() == Ok(ToWrapper::StartApiManagedAccts) {
        tx.send(ToClient::StartApiManagedAccts(
            fields.skip(2).filter(|v| v.as_str() != "").collect(),
        ))
        .await
        .with_context(|| "Failure when sending managed_accts")?;
    }
    Ok(())
}

#[inline]
pub fn receive_fa_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn historical_data_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    decode_fields!(
        fields =>
            req_id @ 1: i64,
            start_date_str @ 0: String,
            end_date_str @ 0: String,
            count @ 0: usize
    );
    let mut bars = Vec::with_capacity(count);
    for chunk in fields.collect::<Vec<String>>().chunks(8) {
        if let [date, open, high, low, close, volume, wap, trade_count] = chunk {
            let bar;
            let core = HistoricalBarCore {
                datetime: chrono::NaiveDateTime::parse_and_remainder(date, "%Y%m%d %T")?.0,
                open: open.parse()?,
                high: high.parse()?,
                low: low.parse()?,
                close: close.parse()?,
            };
            let (volume, wap, trade_count) =
                (volume.parse()?, wap.parse()?, trade_count.parse::<i64>()?);
            if volume > 0. && wap > 0. && trade_count > 0 {
                bar = HistoricalBar::Trades {
                    bar: core,
                    volume,
                    wap,
                    trade_count: trade_count.try_into()?,
                }
            } else {
                bar = HistoricalBar::Ordinary(core)
            }
            bars.push(bar)
        }
    }
    wrapper.historical_bars(req_id, bars);
    Ok(())
}

#[inline]
pub fn bond_contract_data_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn scanner_parameters_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn scanner_data_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn tick_option_computation_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
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
    wrapper.sec_option_computation(req_id, calc);

    Ok(())
}

#[inline]
pub fn tick_generic_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    decode_fields!(
        fields =>
            req_id @ 2: i64,
            tick_type @ 0: u16,
            value @ 0: f64
    );
    decode_generic_tick_msg(req_id, tick_type, value, wrapper)
}

#[inline]
pub fn tick_string_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
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
            wrapper.quoting_exchanges(req_id, quoting_exchanges);
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
            .ok_or_else(|| anyhow::Error::msg("Invalid timestamp encountered in string message"))?;
            let timestamp = match tick_type {
                45 => Class::Live(TimeStamp::Last(timestamp)),
                85 => Class::Live(TimeStamp::Regulatory(timestamp)),
                88 => Class::Delayed(TimeStamp::Last(timestamp)),
                _ => panic!("The impossible occurred"),
            };
            wrapper.timestamp(req_id, timestamp);
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
                        .with_context(|| "Invalid value in RealTimeVolume last_time decode")?,
                    0,
                )
                .ok_or_else(|| {
                    anyhow::Error::msg("Invalid Unix timestamp found in real time volume message")
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
            wrapper.real_time_volume(req_id, volume);
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
                    .with_context(|| "Invalid value in Dividends next_dividend decode datetime")?
                    .0,
                    divs.next()
                        .ok_or(MissingInputData)
                        .with_context(|| "No next price in dividend message")?
                        .parse()
                        .with_context(|| "Invalid value in Dividends next_dividend decode value")?,
                ),
            };
            wrapper.dividends(req_id, dividends);
        }
        62 => {
            wrapper.news(req_id, value);
        }
        t => {
            return Err(anyhow::Error::msg(format!(
                "Unexpected price market data request: {t}"
            )))
        }
    };
    Ok(())
}

#[inline]
pub fn tick_efp_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    unimplemented!();
}

#[inline]
pub fn current_time_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    decode_fields!(
        fields =>
            datetime @ 2: i64
    );

    wrapper.current_time(
        NaiveDateTime::from_timestamp_opt(datetime, 0).ok_or_else(|| {
            anyhow::Error::msg(
                "Invalid datetime value encountered while parsing the UNIX timestamp!",
            )
        })?,
    );
    Ok(())
}

#[inline]
pub fn real_time_bars_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn fundamental_data_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn contract_data_end_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn open_order_end_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn acct_download_end_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn execution_data_end_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn delta_neutral_validation_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn tick_snapshot_end_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn market_data_type_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn commission_report_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn position_data_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn position_end_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn account_summary_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn account_summary_end_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn verify_message_api_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn verify_completed_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn display_group_list_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn display_group_updated_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn verify_and_auth_message_api_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn verify_and_auth_completed_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn position_multi_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn position_multi_end_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn account_update_multi_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn account_update_multi_end_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn security_definition_option_parameter_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn security_definition_option_parameter_end_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn soft_dollar_tiers_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn family_codes_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn symbol_samples_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn mkt_depth_exchanges_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn tick_req_params_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    decode_fields!(
        fields =>
            req_id @ 1: i64,
            min_tick @ 0: f64,
            exchange_id @ 0: ExchangeId,
            snapshot_permissions @ 0: u32
    );
    wrapper.tick_params(req_id, min_tick, exchange_id, snapshot_permissions);
    Ok(())
}

#[inline]
pub fn smart_components_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn news_article_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn tick_news_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn news_providers_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn historical_news_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn historical_news_end_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn head_timestamp_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    decode_fields!(
        fields =>
            req_id @ 1: i64,
            timestamp @ 0: String
    );
    wrapper.head_timestamp(req_id, chrono::NaiveDateTime::parse_from_str(timestamp.as_str(), "%Y%m%d-%T")?);
    Ok(())
}

#[inline]
pub fn histogram_data_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
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
    wrapper.histogram(req_id, hist);
    Ok(())
}

#[inline]
pub fn historical_data_update_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
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
    let core = HistoricalBarCore {
        datetime: chrono::NaiveDateTime::parse_and_remainder(datetime_str.as_str(), "%Y%m%d %T")?.0,
        open,
        high,
        low,
        close,
    };
    let bar;
    if trade_count > 0 && wap > 0. && volume > 0. {
        bar = HistoricalBar::Trades {
            bar: core,
            volume,
            wap,
            trade_count: trade_count.try_into()?,
        }
    } else {
        bar = HistoricalBar::Ordinary(core)
    }
    wrapper.updating_historical_bar(req_id, bar);
    Ok(())
}

#[inline]
pub fn reroute_mkt_data_req_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn reroute_mkt_depth_req_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn market_rule_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn pnl_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn pnl_single_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn historical_ticks_midpoint_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    decode_fields!(
        fields =>
            req_id @ 1: i64,
            tick_count @ 0: usize
    );
    let mut ticks = Vec::with_capacity(tick_count);
    for chunk in fields.take(tick_count * 4).collect::<Vec<String>>().chunks_exact(4) {
        if let [time, _, price, size] = chunk {
            ticks.push(Tick::Midpoint {
                datetime: NaiveDateTime::from_timestamp_opt(time.parse()?, 0).ok_or_else(|| anyhow::Error::msg("Invalid datetime"))?,
                price: price.parse()?
            })
        }
    }
    wrapper.historical_ticks(req_id, ticks);
    Ok(())
}

#[inline]
pub fn historical_ticks_bid_ask_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    decode_fields!(
        fields =>
            req_id @ 1: i64,
            tick_count @ 0: usize
    );
    let mut ticks = Vec::with_capacity(tick_count);
    for chunk in fields.take(tick_count * 6).collect::<Vec<String>>().chunks_exact(6) {
        if let [time, _, bid_price, ask_price, bid_size, ask_size] = chunk {
            ticks.push(Tick::BidAsk { 
                datetime: NaiveDateTime::from_timestamp_opt(time.parse()?, 0).ok_or_else(|| anyhow::Error::msg("Invalid datetime"))?,
                bid_price: bid_price.parse()?,
                ask_price: ask_price.parse()?, 
                bid_size: bid_size.parse()?, 
                ask_size: ask_size.parse()?
            });
        }
    }
    wrapper.historical_ticks(req_id, ticks);
    Ok(())
}

#[inline]
pub fn historical_ticks_last_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    decode_fields!(
        fields =>
            req_id @ 1: i64,
            tick_count @ 0: usize
    );
    let mut ticks = Vec::with_capacity(tick_count);
    for chunk in fields.take(tick_count * 6).collect::<Vec<String>>().chunks_exact(6) {
        if let [time, _, price, size, exchange, _] = chunk {
            ticks.push(Tick::Last {
                datetime: NaiveDateTime::from_timestamp_opt(time.parse()?, 0).ok_or_else(|| anyhow::Error::msg("Invalid datetime"))?,
                price: price.parse()?,
                size: size.parse()?,
                exchange: exchange.parse()?
            });
        }
    }
    wrapper.historical_ticks(req_id, ticks);
    Ok(())
}

#[inline]
pub fn tick_by_tick_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    decode_fields!(
        fields =>
            req_id @ 1: i64,
            tick_type @ 0: u8,
            timestamp @ 0: i64
    );
    let datetime = NaiveDateTime::from_timestamp_opt(timestamp, 0).ok_or_else(|| anyhow::Error::msg("Invalid timestamp"))?;
    let tick = match tick_type {
        1 | 2 => Tick::Last {
            datetime,
            price: nth(fields, 0)?.parse()?,
            size: nth(fields, 0)?.parse()?,
            exchange: nth(fields, 1)?.parse()?
        },
        3 => {
            decode_fields!(
                fields =>
                    bid_price @ 0: f64,
                    ask_price @ 0: f64,
                    bid_size @ 0: f64,
                    ask_size @ 0: f64
            );
            Tick::BidAsk {
                datetime,
                bid_price,
                ask_price,
                bid_size,
                ask_size,
            }
        }
        4 => Tick::Midpoint { datetime, price: nth(fields, 0)?.parse()? },
        _ => Err(anyhow::Error::msg("Unexpected tick type"))?
    };
    wrapper.live_tick(req_id, tick);
    Ok(())
}

#[inline]
pub fn order_bound_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn completed_order_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn completed_orders_end_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn replace_fa_end_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn wsh_meta_data_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn wsh_event_data_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn historical_schedule_msg<W: Wrapper>(
    fields: &mut Fields,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[inline]
pub fn user_info_msg<W: Wrapper>(fields: &mut Fields, wrapper: &mut W) -> anyhow::Result<()> {
    println!("{:?}", &fields);
    Ok(())
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct MissingInputData;

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
fn nth(fields: &mut Fields, n: usize) -> Result<String, MissingInputData> {
    fields.nth(n).ok_or(MissingInputData)
}

#[inline]
fn decode_generic_tick_msg<W: Wrapper>(
    req_id: i64,
    tick_type: u16,
    value: f64,
    wrapper: &mut W,
) -> anyhow::Result<()> {
    match tick_type {
        0 | 3 | 5 => {
            let size = Class::Live(match tick_type {
                0 => Size::Bid(value),
                3 => Size::Ask(value),
                5 => Size::Last(value),
                _ => panic!("The impossible occurred"),
            });
            wrapper.size_data(req_id, size);
        }
        8 | 74 => {
            let volume = match tick_type {
                8 => Class::Live(value),
                74 => Class::Delayed(value),
                _ => panic!("The impossible occurred"),
            };
            wrapper.volume(req_id, volume);
        }
        21 | 63 | 64 | 65 => {
            let volume = match tick_type {
                21 => SummaryVolume::NinetyDayAverage(value),
                63 => SummaryVolume::ThreeMinutes(value),
                64 => SummaryVolume::FiveMinutes(value),
                65 => SummaryVolume::TenMinutes(value),
                _ => panic!("The impossible occurred"),
            };
            wrapper.summary_volume(req_id, volume);
        }
        23 | 24 | 58 => {
            let vol = match tick_type {
                23 => Volatility::SecOptionHistorical(value),
                24 => Volatility::SecOptionImplied(value),
                58 => Volatility::RealTimeHistorical(value),
                _ => panic!("The impossible occurred"),
            };
            wrapper.volatility(req_id, vol);
        }
        29 | 30 | 87 => {
            let volume = match tick_type {
                29 => SecOptionVolume::Call(value),
                30 => SecOptionVolume::Put(value),
                87 => SecOptionVolume::Average(value),
                _ => panic!("The impossible occurred"),
            };
            wrapper.sec_option_volume(req_id, volume);
        }
        34 | 36 | 61 => {
            let auction = match tick_type {
                34 => AuctionData::Volume(value),
                36 => AuctionData::Imbalance(value),
                61 => AuctionData::Regulatory(value),
                _ => panic!("The impossible occurred"),
            };
            wrapper.auction(req_id, auction);
        }
        27 | 28 | 86 => {
            let open_interest = match tick_type {
                27 => OpenInterest::SecOptionCall(value),
                28 => OpenInterest::SecOptionPut(value),
                86 => OpenInterest::SecFuture(value),
                _ => panic!("The impossible occurred"),
            };
            wrapper.open_interest(req_id, open_interest);
        }
        31 | 60 => {
            let factor = match tick_type {
                31 => PriceFactor::IndexFuturePremium(value),
                60 => PriceFactor::BondFactorMultiplier(value),
                _ => panic!("The impossible occurred"),
            };
            wrapper.price_factor(req_id, factor);
        }
        46 | 49 | 89 => {
            let access = match tick_type {
                46 => Accessibility::Shortable(value),
                49 => Accessibility::Halted(value),
                89 => Accessibility::ShortableShares(value),
                _ => panic!("The impossible occurred"),
            };
            wrapper.accessibility(req_id, access);
        }
        54 => {
            wrapper.trade_count(req_id, value);
        }
        55 | 56 => {
            let rate = match tick_type {
                55 => Rate::Trade(value),
                56 => Rate::Volume(value),
                _ => panic!("The impossible occurred"),
            };
            wrapper.rate(req_id, rate);
        }
        69..=71 => {
            let size = Class::Delayed(match tick_type {
                69 => Size::Bid(value),
                70 => Size::Ask(value),
                71 => Size::Last(value),
                _ => panic!("The impossible occurred"),
            });
            wrapper.size_data(req_id, size);
        }
        101 | 102 => {
            let ipo = match tick_type {
                101 => Ipo::Estimated(value),
                102 => Ipo::Final(value),
                _ => panic!("The impossible occurred"),
            };
            wrapper.ipo(req_id, ipo);
        }
        t => {
            return Err(anyhow::Error::msg(format!(
                "Unexpected generic market data request: {t}"
            )))
        }
    };

    Ok(())
}
