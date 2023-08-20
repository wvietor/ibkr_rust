use ibapi::{
    client::{Builder, Client, Host, Mode},
    contract::{self, Contract, ContractId, Forex, SecOption, Stock},
    contract_dispatch,
    default_wrapper::DefaultWrapper,
    market_data::live_data,
};

#[tokio::test(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\x1B[34mMain test beginnning!\x1B[0m");
    let mut client = Builder::from_config_file(
        Mode::Paper,
        Host::Gateway,
        Some("../config.toml"),
    )?
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
            contract::new::<SecOption>(&mut client, ContractId(606_333_854))
                .await?
                .into(),
        ),
    ]);

    for con in cons.values() {
        contract_dispatch! {
            con =>
                async (Client::req_market_data)
                (&mut client)
                (vec![live_data::data_types::Empty], live_data::RefreshType::Streaming, false)
        }?;
    }

    std::thread::sleep(std::time::Duration::from_secs(20));
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
