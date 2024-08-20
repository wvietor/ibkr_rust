use ibapi::client::{ActiveClient, Builder};
use ibapi::scanner_subscription::{ScannerContract, ScannerSubscription};
use ibapi::wrapper::CancelToken;
use std::future::Future;
use std::net::Ipv4Addr;
use std::str::FromStr;
use tracing::info;
use tracing_test::traced_test;

const REQUESTS_COUNT: i32 = 14;
const COMPLITED_REQUEST_COUNT: usize = REQUESTS_COUNT as usize / 2;
const NUMBER_OF_RESULT: usize = 10;

#[derive(Debug, Default)]
struct ScannerWrapper {
    reqwests: i32,
    good_responces: Vec<(i64, usize)>,
}

impl ibapi::wrapper::Wrapper for ScannerWrapper {
    fn scanner_data(
        &mut self,
        req_id: i64,
        result: Vec<ScannerContract>,
    ) -> impl Future + Send + Send {
        async move {
            self.reqwests += 1;
            self.good_responces.push((req_id, result.len()));

            // println!(
            //     "scanner req: {req_id}, assert: {}",
            //     result.len() == NUMBER_OF_RESULT
            // );
            // result.iter().for_each(|r| println!("{:?}", r));
            assert_eq!(result.len(), NUMBER_OF_RESULT);
        }
    }
    fn scanner_data_end(&mut self, req_id: i64) -> impl Future + Send + Send {
        async move {
            // self.reqwests -= 1;

            println!(
                "Results with responces: {}/{}",
                self.good_responces.len(),
                COMPLITED_REQUEST_COUNT
            );
            if self.reqwests - req_id as i32 == REQUESTS_COUNT {
                assert_eq!(self.good_responces.len(), COMPLITED_REQUEST_COUNT);
            }

            // println!("scanner req: {req_id} end!");
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
                let subscription = ScannerSubscription::us_stocks()
                    .us_major()
                    .top_perc_gain()
                    .number_of_result_rows(NUMBER_OF_RESULT as i32);
                // .price_below(30.0);

                if let Ok(req_id) = client.req_scanner_subscription(&subscription).await {
                    if req_id % 2 == 0 {
                        let _ = client.cancel_scanner_subscription(req_id).await;
                    }
                }
            }
            (self, ())
        }
    }
}

#[traced_test]
#[tokio::test]
async fn test_req_scanner_subscription() -> Result<(), Box<dyn std::error::Error>> {
    let scanner_wrapper = ScannerWrapper {
        reqwests: 0,
        good_responces: vec![],
    };

    let client = Builder::manual(4002, Ipv4Addr::from_str("127.0.0.1").ok())
        .connect(5)
        .await?
        .remote(scanner_wrapper)
        .await;

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    client.cancel();
    Ok(())
}
