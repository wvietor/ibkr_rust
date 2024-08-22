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
        client: &mut ActiveClient,
        _cancel_loop: CancelToken,
    ) -> impl Future<Output = (Self::Wrap<'_>, Self::Recur<'_>)> + Send {
        async move {
            let aapl: ibapi::contract::Stock =
                ibapi::contract::new(client, "BBG000B9XRY4".parse().unwrap())
                    .await
                    .unwrap();
            assert_eq!(aapl.symbol(), "AAPL");
            (self, ())
        }
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
        client: &mut ActiveClient,
        cancel_loop: CancelToken,
    ) -> impl Future<Output = (Self::Wrap<'_>, Self::Recur<'_>)> {
        async {
            let aapl: ibapi::contract::Stock =
                ibapi::contract::new(client, "BBG000B9XRY4".parse().unwrap())
                    .await
                    .unwrap();
            assert_eq!(aapl.symbol(), "AAPL");
            (self, Recur { cancel_loop })
        }
    }
}

#[derive(Debug, Clone)]
struct Recur {
    cancel_loop: CancelToken,
}

impl ibapi::wrapper::LocalRecurring for Recur {
    fn cycle(&mut self) -> impl Future<Output = ()> {
        async {
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            self.cancel_loop.cancel();
        }
    }
}

#[tokio::test]
async fn disaggregated_remote() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Builder::from_config_file(Mode::Paper, Host::Gateway, None)?
        .connect(5)
        .await?
        .disaggregated(SendWrapper)
        .await;
    client.req_current_time().await?;
    let aapl: ibapi::contract::Stock =
        ibapi::contract::new(&mut client, "BBG000B9XRY4".parse()?).await?;
    assert_eq!(aapl.symbol(), "AAPL");
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    client.disconnect().await?;
    Ok(())
}

#[tokio::test]
async fn remote() -> Result<(), Box<dyn std::error::Error>> {
    let cancel_token = Builder::from_config_file(Mode::Paper, Host::Gateway, None)?
        .connect(6)
        .await?
        .remote(SendWrapper)
        .await;

    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    cancel_token.cancel();
    Ok(())
}

#[tokio::test]
async fn local() -> Result<(), Box<dyn std::error::Error>> {
    Builder::from_config_file(Mode::Paper, Host::Gateway, None)?
        .connect(7)
        .await?
        .local(NonSendWrapper::default(), None)
        .await?;

    Ok(())
}
