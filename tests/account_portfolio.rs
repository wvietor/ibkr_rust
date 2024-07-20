use std::future::Future;

use tokio;

use ibapi::account::Tag;
use ibapi::client::{ActiveClient, Builder, Host, Mode};
use ibapi::wrapper::{CancelToken, Initializer, Wrapper};

struct AccountDataWrapper;

impl Wrapper for AccountDataWrapper {}

struct AccountSummaryInitializer;

impl Initializer for AccountSummaryInitializer {
    type Wrap<'c> = AccountDataWrapper;
    type Recur<'c> = ();

    fn build(
        self,
        client: &mut ActiveClient,
        _cancel_loop: CancelToken,
    ) -> impl Future<Output = (Self::Wrap<'_>, ())> + Send {
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
            (AccountDataWrapper, ())
        }
    }
}

#[tokio::test]
async fn account_summary() -> Result<(), Box<dyn std::error::Error>> {
    let discon =
        Builder::from_config_file(Mode::Paper, Host::Gateway, Some("config.toml".as_ref()))?
            .connect(1)
            .await?
            .remote(AccountSummaryInitializer)
            .await;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    discon.cancel();
    Ok(())
}

struct AccountUpdateInitializer;

impl Initializer for AccountUpdateInitializer {
    type Wrap<'c> = AccountDataWrapper;
    type Recur<'c> = ();

    fn build(
        self,
        client: &mut ActiveClient,
        _cancel_loop: CancelToken,
    ) -> impl Future<Output = (Self::Wrap<'_>, ())> + Send {
        async {
            client.req_account_updates(None).await.unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            client.cancel_account_updates(None).await.unwrap();
            (AccountDataWrapper, ())
        }
    }
}

#[tokio::test]
async fn account_update() -> Result<(), Box<dyn std::error::Error>> {
    let discon =
        Builder::from_config_file(Mode::Paper, Host::Gateway, Some("config.toml".as_ref()))?
            .connect(2)
            .await?
            .remote(AccountUpdateInitializer)
            .await;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    discon.cancel();
    Ok(())
}

struct PositionInitializer;

impl Initializer for PositionInitializer {
    type Wrap<'c> = AccountDataWrapper;
    type Recur<'c> = ();
    fn build(
        self,
        client: &mut ActiveClient,
        _cancel_loop: CancelToken,
    ) -> impl Future<Output = (Self::Wrap<'_>, ())> + Send {
        async {
            client.req_positions().await.unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            client.cancel_positions().await.unwrap();
            (AccountDataWrapper, ())
        }
    }
}

#[tokio::test]
async fn positions() -> Result<(), Box<dyn std::error::Error>> {
    let discon =
        Builder::from_config_file(Mode::Paper, Host::Gateway, Some("config.toml".as_ref()))?
            .connect(3)
            .await?
            .remote(PositionInitializer)
            .await;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    discon.cancel();
    Ok(())
}

struct PnlInitializer;

impl Initializer for PnlInitializer {
    type Wrap<'c> = AccountDataWrapper;
    type Recur<'c> = ();
    fn build(
        self,
        client: &mut ActiveClient,
        _cancel_loop: CancelToken,
    ) -> impl Future<Output = (Self::Wrap<'_>, ())> + Send {
        async {
            let id = client
                .req_pnl(&client.get_managed_accounts().iter().next().unwrap().clone())
                .await
                .unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            client.cancel_pnl(id).await.unwrap();
            (AccountDataWrapper, ())
        }
    }
}

#[tokio::test]
async fn pnl() -> Result<(), Box<dyn std::error::Error>> {
    let discon =
        Builder::from_config_file(Mode::Paper, Host::Gateway, Some("config.toml".as_ref()))?
            .connect(4)
            .await
            .unwrap()
            .remote(PnlInitializer)
            .await;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    discon.cancel();
    Ok(())
}
