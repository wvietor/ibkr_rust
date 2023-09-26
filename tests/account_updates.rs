use ibapi::{
    client::{self, Host, Mode},
    default_wrapper::DefaultWrapper,
};
#[tokio::test(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wrapper = DefaultWrapper;
    let mut client =
        client::Builder::from_config_file(Mode::Paper, Host::Gateway, Some("config.toml"))?
            .connect(0, wrapper)
            .await?
            .run()
            .await;

    client.req_current_time().await?;
    client.req_account_updates(None).await?;
    client.req_managed_accounts().await?;
    client.req_positions().await?;
    let acct = client
        .get_managed_accounts()
        .clone()
        .drain()
        .next()
        .ok_or_else(|| anyhow::Error::msg("No account number to get"))?;
    client.req_pnl(acct).await?;

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    client.disconnect().await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn temp() -> Result<(), Box<dyn std::error::Error>> {
    let wrapper = DefaultWrapper;
    let mut client =
        client::Builder::from_config_file(Mode::Paper, Host::Gateway, Some("config.toml"))?
            .connect(1, wrapper)
            .await?
            .run()
            .await;
    client.req_current_time().await?;
    client
        .req_account_summary(&vec![
            ibapi::account::Tag::AccountType,
            ibapi::account::Tag::FullMaintenanceMarginReq,
        ])
        .await?;

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    client.disconnect().await?;
    Ok(())
}
