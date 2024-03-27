use std::future::Future;

use ibapi::client::{ActiveClient, Builder, Host, Mode};
use ibapi::wrapper::CancelToken;

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct SendWrapper;

impl ibapi::wrapper::Wrapper for SendWrapper {}

impl ibapi::wrapper::Initializer for SendWrapper {
    type Wrap<'c> = SendWrapper;
    type Recur<'c> = ();

    fn build(
        self,
        _client: &mut ActiveClient,
        _cancel_loop: CancelToken,
    ) -> impl Future<Output = (Self::Wrap<'_>, Self::Recur<'_>)> + Send {
        async move { (self, ()) }
    }
}

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct NonSendWrapper {
    non_send: std::rc::Rc<i64>,
}

impl ibapi::wrapper::LocalWrapper for NonSendWrapper {}

impl ibapi::wrapper::LocalInitializer for NonSendWrapper {
    type Wrap<'c> = NonSendWrapper;
    type Recur<'c> = Recur;

    fn build(
        self,
        _client: &mut ActiveClient,
        cancel_loop: CancelToken,
    ) -> impl Future<Output = (Self::Wrap<'_>, Self::Recur<'_>)> {
        async { (self, Recur { cancel_loop }) }
    }
}

#[derive(Debug, Clone)]
struct Recur {
    cancel_loop: CancelToken,
}

impl ibapi::wrapper::LocalRecurring for Recur {
    fn cycle(&mut self) -> impl Future<Output = ()> + Send {
        async {
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            self.cancel_loop.cancel();
        }
    }
}

#[tokio::test]
async fn disaggregated_remote() -> anyhow::Result<()> {
    let mut client = Builder::from_config_file(Mode::Paper, Host::Gateway, None)?
        .connect(5)
        .await?
        .disaggregated(SendWrapper)
        .await;
    client.req_current_time().await?;

    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    client.disconnect().await?;
    Ok(())
}

#[tokio::test]
async fn disaggregated_local() -> anyhow::Result<()> {
    let mut client = Builder::from_config_file(Mode::Paper, Host::Gateway, None)?
        .connect(6)
        .await?
        .disaggregated_local(NonSendWrapper::default())
        .await;
    client.req_current_time().await?;

    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    client.disconnect().await?;
    Ok(())
}

#[tokio::test]
async fn remote() -> anyhow::Result<()> {
    let cancel_token = Builder::from_config_file(Mode::Paper, Host::Gateway, None)?
        .connect(7)
        .await?
        .remote(SendWrapper)
        .await;

    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    cancel_token.cancel();
    Ok(())
}

#[tokio::test]
async fn local() -> anyhow::Result<()> {
    Builder::from_config_file(Mode::Paper, Host::Gateway, None)?
        .connect(8)
        .await?
        .local(NonSendWrapper::default(), None)
        .await?;

    Ok(())
}
