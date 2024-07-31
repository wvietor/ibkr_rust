use ibapi::client::{ActiveClient, Builder, Host, Mode};
use ibapi::prelude::Tag;
use ibapi::wrapper::{CancelToken, Initializer, Wrapper};
// use metadata::LevelFilter;
use std::future::Future;
use std::net::Ipv4Addr;
use std::str::FromStr;
use tokio;
use tracing_subscriber::EnvFilter;
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct ScannerWrapper;

impl ibapi::wrapper::Wrapper for ScannerWrapper {
    fn scanner_parameters(&mut self, req_id: i64, xml: String) -> impl Future + Send {
        println!("scanner_parameters():");
        async move {
            let contains_tags = xml.contains("ScanParameterResponse")
                && xml.contains("InstrumentList")
                && xml.contains("/ScanParameterResponse");
            assert!(contains_tags);
        }
    }
}

impl ibapi::wrapper::Initializer for ScannerWrapper {
    type Wrap<'c> = ScannerWrapper;
    type Recur<'c> = ();

    #[allow(clippy::manual_async_fn)]
    fn build(
        self,
        client: &mut ActiveClient,
        _cancel_loop: CancelToken,
    ) -> impl Future<Output = (Self::Wrap<'_>, Self::Recur<'_>)> + Send {
        async move {
            let req_status = client.req_scanner_parameters().await;
            (self, ())
        }
    }
}

#[tokio::test]
async fn test_req_scanner_parameters() -> Result<(), Box<dyn std::error::Error>> {
    let client = Builder::manual(4002, Ipv4Addr::from_str("127.0.0.1").ok())
        .connect(5)
        .await?
        .remote(ScannerWrapper)
        .await;

    // println!(" client.req_scanner_parameters().await");
    // let req_status = client.req_scanner_parameters().await;
    // println!(" client.req_scanner_parameters().await");
    // println!("req_status2:{:?}", &req_status);

    tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    client.cancel(); //.await?;
    Ok(())
}
