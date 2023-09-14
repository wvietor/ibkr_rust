use ibapi::{
    client::{Builder, Host, Mode},
    contract::{self, Contract, ContractId, Forex, SecOption, Stock},
    default_wrapper::DefaultWrapper,
};

#[tokio::test(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\x1B[34mMain test beginnning!\x1B[0m");
    let mut client = Builder::from_config_file(Mode::Paper, Host::Gateway, Some("config.toml"))?
        .connect(0, DefaultWrapper)
        .await?
        .run()
        .await;

    client.req_current_time().await?;

    let cons: std::collections::HashMap<&str, Contract> = std::collections::HashMap::from([
        (
            "gbp",
            contract::new::<Forex>(&mut client, ContractId(12_087_797))
                .await?
                .into(),
        ),
        (
            "qqq",
            contract::new::<Stock>(&mut client, ContractId(320_227_571))
                .await?
                .into(),
        ),
        (
            "aapl_opt",
            contract::new::<SecOption>(&mut client, ContractId(621_534_856))
                .await?
                .into(),
        ),
    ]);

    // for con in cons.values() {
    //     contract_dispatch! {
    //         con =>
    //             async (Client::req_market_depth)
    //             (&mut client)
    //             (10)
    //     }?;
    // }
    match cons.get("qqq") {
        Some(Contract::Stock(stk)) => {
            let mut req_id;
            req_id = client
                .req_updating_historical_bar(
                    stk,
                    ibapi::market_data::updating_historical_bar::Duration::Second(300),
                    ibapi::market_data::updating_historical_bar::Size::Minutes(
                        ibapi::market_data::updating_historical_bar::MinuteSize::One,
                    ),
                    ibapi::market_data::updating_historical_bar::data_types::Midpoint,
                    false,
                )
                .await?;
            std::thread::sleep(std::time::Duration::from_secs(10));
            client.cancel_updating_historical_bar(req_id).await?;

            client
                .req_historical_bar(
                    stk,
                    ibapi::market_data::historical_bar::EndDateTime::Present,
                    ibapi::market_data::historical_bar::Duration::Day(3),
                    ibapi::market_data::historical_bar::Size::Hours(
                        ibapi::market_data::historical_bar::HourSize::One,
                    ),
                    ibapi::market_data::historical_bar::data_types::BidAsk,
                    false,
                )
                .await?;
            std::thread::sleep(std::time::Duration::from_secs(2));

            req_id = client
                .req_histogram_data(stk, false, ibapi::market_data::histogram::Duration::Day(1))
                .await?;
            std::thread::sleep(std::time::Duration::from_secs(2));
            client.cancel_histogram_data(req_id).await?;

            client
                .req_historical_ticks(
                    stk,
                    ibapi::market_data::historical_ticks::TimeStamp::EndDateTime(
                        chrono::NaiveDateTime::parse_from_str(
                            "2023-08-31 16:00:00",
                            "%Y-%m-%d %T",
                        )?,
                    ),
                    ibapi::market_data::historical_ticks::NumberOfTicks::new(10),
                    ibapi::market_data::historical_ticks::data_types::BidAsk,
                    false,
                )
                .await?;

            req_id = client
                .req_market_data(
                    stk,
                    vec![ibapi::market_data::live_data::data_types::Empty],
                    ibapi::market_data::live_data::RefreshType::Streaming,
                    false,
                )
                .await?;
            std::thread::sleep(std::time::Duration::from_secs(2));
            client.cancel_market_data(req_id).await?;

            req_id = client
                .req_tick_by_tick_data(
                    stk,
                    ibapi::market_data::live_ticks::data_types::AllLast,
                    ibapi::market_data::live_ticks::NumberOfTicks::new(100),
                    false,
                )
                .await?;
            std::thread::sleep(std::time::Duration::from_secs(2));
            client.cancel_tick_by_tick_data(req_id).await?;

            req_id = client.req_market_depth(stk, 5).await?;
            std::thread::sleep(std::time::Duration::from_secs(2));
            client.cancel_market_depth(req_id).await?;

            req_id = client
                .req_real_time_bars(
                    stk,
                    ibapi::market_data::live_bar::data_types::Midpoint,
                    false,
                )
                .await?;
            std::thread::sleep(std::time::Duration::from_secs(2));
            client.cancel_real_time_bars(req_id).await?;
        }
        _ => (),
    };

    std::thread::sleep(std::time::Duration::from_secs(10));
    match client.disconnect().await {
        Ok(_) => {
            println!("\x1B[32mOk shutdown!\x1B[0m");
            Ok(())
        }
        Err(e) => {
            println!("\x1B[31mBad shutdown: {:?}\x1B[0m", &e);
            Err(e.into())
        }
    }
}
