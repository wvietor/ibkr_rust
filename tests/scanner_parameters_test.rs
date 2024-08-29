use ibapi::client::{ActiveClient, Builder, Host, Mode};
use ibapi::wrapper::{CancelToken, Initializer, Recurring, Wrapper};
use std::future::Future;
use std::net::Ipv4Addr;
use std::str::FromStr;
use tracing_test::traced_test;

const REQUESTS_COUNT: u8 = 10;

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

            println!(
                "?/{REQUESTS_COUNT} Wrapper:scanner_parameters(): contains_tags:{contains_tags} req_id:{req_id},  xml.len(): {}",
                xml.len()
            );
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
            for _ in 0..REQUESTS_COUNT {
                let _ = client.req_scanner_parameters().await;
            }
            ScannerWrapper
        }
    }
}

#[traced_test]
#[tokio::test]
async fn test_req_scanner_parameters() -> Result<(), Box<dyn std::error::Error>> {
    let client = Builder::from_config_file(Mode::Paper, Host::Gateway, &None::<&'static str>)?
        .connect(5)
        .await?
        .remote(ScannerWrapper)
        .await;

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    client.cancel();
    Ok(())
}
