use std::future::Future;

use ibapi::client::{ActiveClient, Builder, Host, Mode};
use ibapi::wrapper::{
    CancelToken, Initializer, LocalInitializer, LocalRecurring, LocalWrapper, Recurring, Wrapper,
};

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct SendWrapper;

impl Wrapper for SendWrapper {}

impl Recurring for SendWrapper {
    fn cycle(&mut self) -> impl Future<Output = ()> + Send {
        async { () }
    }
}

impl Initializer for SendWrapper {
    type Wrap<'c> = SendWrapper;
    fn build(
        self,
        client: &mut ActiveClient,
        _cancel_loop: CancelToken,
    ) -> impl Future<Output = Self::Wrap<'_>> + Send {
        async move {
            let aapl: ibapi::contract::Stock =
                ibapi::contract::new(client, "BBG000B9XRY4".parse().unwrap())
                    .await
                    .unwrap();
            assert_eq!(aapl.symbol(), "AAPL");
            self
        }
    }
}

#[derive(Debug, Default, Clone)]
struct NonSendWrapper {
    cancel_loop: CancelToken,
    _non_send: std::rc::Rc<i64>,
}

impl LocalWrapper for NonSendWrapper {}

impl LocalRecurring for NonSendWrapper {
    fn cycle(&mut self) -> impl Future<Output = ()> {
        async {
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            self.cancel_loop.cancel();
        }
    }
}

impl LocalInitializer for NonSendWrapper {
    type Wrap<'c> = NonSendWrapper;

    fn build(
        self,
        client: &mut ActiveClient,
        cancel_loop: CancelToken,
    ) -> impl Future<Output = Self::Wrap<'_>> {
        async {
            let aapl: ibapi::contract::Stock =
                ibapi::contract::new(client, "BBG000B9XRY4".parse().unwrap())
                    .await
                    .unwrap();
            assert_eq!(aapl.symbol(), "AAPL");
            NonSendWrapper {
                cancel_loop,
                ..Self::default()
            }
        }
    }
}

#[tokio::test]
async fn disaggregated_remote() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Builder::from_config_file(Mode::Paper, Host::Gateway, &None::<&'static str>)?
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
    let cancel_token =
        Builder::from_config_file(Mode::Paper, Host::Gateway, &None::<&'static str>)?
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
    Builder::from_config_file(Mode::Paper, Host::Gateway, &None::<&'static str>)?
        .connect(7)
        .await?
        .local(NonSendWrapper::default(), None)
        .await?;

    Ok(())
}
