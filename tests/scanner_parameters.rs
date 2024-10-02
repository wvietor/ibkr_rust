use ibapi::client::{ActiveClient, Builder, Host, Mode};
use ibapi::wrapper::{CancelToken, Initializer, Recurring, Wrapper};
use std::future::Future;
use std::net::Ipv4Addr;
use std::str::FromStr;

#[derive(Debug, Default)]
struct ScannerWrapper;

impl Recurring for ScannerWrapper {
    fn cycle(&mut self) -> impl Future<Output = ()> + Send {
        async { () }
    }
}

impl Wrapper for ScannerWrapper {
    fn scanner_parameters(&mut self, req_id: i64, xml: String) -> impl Future + Send {
        async move {
            let contains_tags =
                xml.contains("ScanParameterResponse") && xml.contains("/ScanParameterResponse");
            assert!(contains_tags);

            // self.0 += 1;
            println!("scanner_parameters: {}", xml);
        }
    }
}

impl Initializer for ScannerWrapper {
    type Wrap<'c> = ScannerWrapper;

    #[allow(clippy::manual_async_fn)]
    fn build(
        self,
        client: &mut ActiveClient,
        _cancel_loop: CancelToken,
    ) -> impl Future<Output = Self::Wrap<'_>> + Send {
        async move {
            let _ = client.req_scanner_parameters().await;
            ScannerWrapper
        }
    }
}

#[tokio::test]
async fn req_scanner_parameters() -> Result<(), Box<dyn std::error::Error>> {
    let client = Builder::from_config_file(Mode::Paper, Host::Gateway, &None::<&'static str>)?
        .connect(5)
        .await?
        .remote(ScannerWrapper)
        .await;

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    client.cancel();
    Ok(())
}
