use ibapi::account::Tag;
use ibapi::client::{ActiveClient, Builder, Host, Mode};
use ibapi::wrapper::{CancelToken, Remote, RemoteInitializer};
use std::future::Future;
use tokio;

struct Wrapper<'c>(&'c mut ActiveClient, CancelToken);

impl Remote for Wrapper<'_> {}

struct AccountSummaryInitializer;

impl RemoteInitializer for AccountSummaryInitializer {
    type Wrap<'c> = Wrapper<'c>;

    fn build(
        self,
        client: &mut ActiveClient,
        cancel_loop: CancelToken,
    ) -> impl Future<Output = Self::Wrap<'_>> + Send {
        async {
            let id = client
                .req_account_summary(&vec![
                    Tag::AccountType,
                    Tag::NetLiquidation,
                    Tag::TotalCashValue,
                    Tag::SettledCash,
                    Tag::AccruedCash,
                    Tag::BuyingPower,
                    Tag::AvailableFunds,
                    Tag::EquityWithLoanValue,
                    Tag::PreviousEquityWithLoanValue,
                    Tag::GrossPositionValue,
                    Tag::RegTEquity,
                    Tag::RegTMargin,
                    Tag::Sma,
                    Tag::InitMarginReq,
                    Tag::MaintenanceMarginReq,
                    Tag::AvailableFunds,
                    Tag::ExcessLiquidity,
                    Tag::Cushion,
                    Tag::FullInitMarginReq,
                    Tag::FullMaintenanceMarginReq,
                    Tag::FullAvailableFunds,
                    Tag::FullExcessLiquidity,
                    Tag::LookAheadNextChange,
                    Tag::LookAheadMaintenanceMarginReq,
                    Tag::LookAheadAvailableFunds,
                    Tag::LookAheadExcessLiquidity,
                    Tag::HighestSeverity,
                    Tag::DayTradesRemaining,
                    Tag::Leverage,
                ])
                .await
                .unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            client.cancel_account_summary(id).await.unwrap();
            Wrapper(client, cancel_loop)
        }
    }
}

#[tokio::test]
async fn account_summary() -> anyhow::Result<()> {
    let discon = Builder::from_config_file(Mode::Paper, Host::Gateway, Some("config.toml"))?
        .connect(1)
        .await?
        .remote(AccountSummaryInitializer)
        .await;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    discon.cancel();
    Ok(())
}

struct AccountUpdateInitializer;

impl RemoteInitializer for AccountUpdateInitializer {
    type Wrap<'c> = Wrapper<'c>;

    fn build(
        self,
        client: &mut ActiveClient,
        cancel_loop: CancelToken,
    ) -> impl Future<Output = Self::Wrap<'_>> + Send {
        async {
            client.req_account_updates(None).await.unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            client.cancel_account_updates(None).await.unwrap();
            Wrapper(client, cancel_loop)
        }
    }
}

#[tokio::test]
async fn account_update() -> anyhow::Result<()> {
    let discon = Builder::from_config_file(Mode::Paper, Host::Gateway, Some("config.toml"))?
        .connect(2)
        .await?
        .remote(AccountUpdateInitializer)
        .await;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    discon.cancel();
    Ok(())
}

struct PositionInitializer;

impl RemoteInitializer for PositionInitializer {
    type Wrap<'c> = Wrapper<'c>;

    fn build(
        self,
        client: &mut ActiveClient,
        cancel_loop: CancelToken,
    ) -> impl Future<Output = Self::Wrap<'_>> + Send {
        async {
            client.req_positions().await.unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            client.cancel_positions().await.unwrap();
            Wrapper(client, cancel_loop)
        }
    }
}

#[tokio::test]
async fn positions() -> anyhow::Result<()> {
    let discon = Builder::from_config_file(Mode::Paper, Host::Gateway, Some("config.toml"))?
        .connect(3)
        .await?
        .remote(PositionInitializer)
        .await;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    discon.cancel();
    Ok(())
}

struct PnlInitializer;

impl RemoteInitializer for PnlInitializer {
    type Wrap<'c> = Wrapper<'c>;

    fn build(
        self,
        client: &mut ActiveClient,
        cancel_loop: CancelToken,
    ) -> impl Future<Output = Self::Wrap<'_>> + Send {
        async {
            let id = client
                .req_pnl(&client.get_managed_accounts().iter().next().unwrap().clone())
                .await
                .unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            client.cancel_pnl(id).await.unwrap();
            Wrapper(client, cancel_loop)
        }
    }
}

#[tokio::test]
async fn pnl() -> anyhow::Result<()> {
    let discon = Builder::from_config_file(Mode::Paper, Host::Gateway, Some("config.toml"))?
        .connect(4)
        .await
        .unwrap()
        .remote(PnlInitializer)
        .await;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    discon.cancel();
    Ok(())
}
