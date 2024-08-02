use ibapi::client::{ActiveClient, Builder};
use ibapi::wrapper::CancelToken;
use std::future::Future;
use std::net::Ipv4Addr;
use std::str::FromStr;
use tracing_test::traced_test;

const REQUESTS_COUNT: u8 = 10;

#[derive(Debug, Default)]
struct ScannerWrapper(u8);

impl ibapi::wrapper::Wrapper for ScannerWrapper {
    fn scanner_parameters(&mut self, req_id: i64, xml: String) -> impl Future + Send {
        async move {
            let contains_tags =
                xml.contains("ScanParameterResponse") && xml.contains("/ScanParameterResponse");
            assert!(contains_tags);
            self.0 += 1;
            println!(
                "{}/{REQUESTS_COUNT} Wrapper:scanner_parameters(): contains_tags:{contains_tags} req_id:{req_id},  xml.len(): {}",
                self.0,
                xml.len()
            );
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
            for _ in 0..REQUESTS_COUNT {
                let _ = client.req_scanner_parameters().await;
            }
            (self, ())
        }
    }
}
#[traced_test]
#[tokio::test]
async fn test_req_scanner_parameters() -> Result<(), Box<dyn std::error::Error>> {
    let client = Builder::manual(4002, Ipv4Addr::from_str("127.0.0.1").ok())
        .connect(5)
        .await?
        .remote(ScannerWrapper(0))
        .await;

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    client.cancel();
    Ok(())
}
