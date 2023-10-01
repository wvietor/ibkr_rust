use ibapi;
use ibapi::client;
use ibapi::contract::{ContractId, Stock};
use ibapi::order::{Limit, Order};

#[tokio::test(flavor="multi_thread")]
async fn place_order_test() -> Result<(), Box<dyn std::error::Error>> {
    let wrapper= ibapi::default_wrapper::DefaultWrapper;
    let mut client = client::Builder::from_config_file(
        client::Mode::Paper,
        client::Host::Gateway,
        Some("./config.toml"))?
        .connect(10, wrapper)
        .await?
        .run().await;
    client.req_current_time().await?;

    let con = ibapi::contract::new::<Stock>(&mut client, ContractId(265598)).await?;
    let ord = Order::Buy {
        security: &con,
        execute_method: &Limit {
            quantity: 10.0,
            price: 165.0,
            time_in_force: Default::default(),
        }
    };
    client.req_place_order(&ord).await?;
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    client.disconnect().await?;
    Ok(())
}