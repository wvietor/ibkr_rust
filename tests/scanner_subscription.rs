use ibapi::client::{ActiveClient, Builder, Host, Mode};
use ibapi::scanner_subscription::{ScannerContract, ScannerSubscription};
use ibapi::wrapper::{CancelToken, Initializer, Wrapper};
use std::future::Future;
use std::sync::atomic::AtomicI32;
use std::time::Duration;
use tokio::time::sleep;

// const COMPLITED_REQUEST_COUNT: AtomicI32 = AtomicI32::new(REQUESTS_COUNT / 2);
// static TOTAL_REQUESTS: AtomicI32 = AtomicI32::new(0);

// static TOTAL_RESPONCES: AtomicI32 = AtomicI32::new(0);

#[derive(Debug, Clone)]
struct ScannerWrapper {
    tx_scanner_data:
        tokio::sync::mpsc::Sender<(i64, Vec<ibapi::scanner_subscription::ScannerContract>)>,
    tx_scanner_end: tokio::sync::mpsc::Sender<i64>,
}

// impl Recurring for ScannerWrapper {
//     fn cycle(&mut self) -> impl Future<Output = ()> + Send {
//         async { () }
//     }
// }

impl Wrapper for ScannerWrapper {
    fn error(
        &mut self,
        req_id: i64,
        error_code: i64,
        error_string: String,
        advanced_order_reject_json: String,
    ) -> impl Future + Send {
        async move {
            match error_code {
                492 | 2104 | 2106 | 2107 | 2158 => {}
                _ => {
                    println!("{}, {}, {}", req_id, error_code, error_string);
                }
            }
        }
    }

    fn scanner_data(
        &mut self,
        req_id: i64,
        result: Vec<ScannerContract>,
    ) -> impl Future + Send + Send {
        async move {
            self.tx_scanner_data
                .send((req_id, result))
                .await
                .expect("Sending the scanner subscription should succeed");
        }
    }

    fn scanner_data_end(&mut self, req_id: i64) -> impl Future + Send + Send {
        async move {
            self.tx_scanner_end
                .send(req_id)
                .await
                .expect("Sending the scanner subscription end should succeed");
        }
    }
}

#[tokio::test]
async fn req_scanner_subscription_and_cancel_it() -> Result<(), Box<dyn std::error::Error>> {
    const CLIENT_ID: i64 = 0;
    const CHANNEL_SIZE: usize = 50;
    let (tx_scanner_data, mut rx_scanner_data) = tokio::sync::mpsc::channel(CHANNEL_SIZE);
    let (tx_scanner_end, mut rx_scanner_end) = tokio::sync::mpsc::channel(CHANNEL_SIZE);

    let client = Builder::from_config_file(Mode::Paper, Host::Gateway, &None::<&'static str>)?
        .connect(CLIENT_ID)
        .await?;

    let mut client = client
        .disaggregated(ScannerWrapper {
            tx_scanner_data,
            tx_scanner_end,
        })
        // .remote(ScannerWrapper)
        .await;

    let number_of_result_rows = 5;
    let requests_count: i32 = 9;
    let mut result_count: usize = (requests_count as f64 / 2.0).round() as usize;
    let mut cancel_count: usize = requests_count as usize - result_count;

    let mut prev = 0.0; //it.next().unwrap().clone();

    for _i in 0..=requests_count {
        let new = _i * 5;

        let subscription = ScannerSubscription::asia_stocks()
            .asia_stocks()
            .hot_by_price()
            .number_of_result_rows(number_of_result_rows)
            .avg_usd_volume_below(2_000_000.0);
        // .usd_price_above(prev)
        // .usd_price_below(new as f64);

        prev = new as f64;

        let req_id = client
            .req_scanner_subscription(&subscription)
            .await
            .unwrap();

        if req_id % 2 == 0 {
            let _ = client.cancel_scanner_subscription(req_id).await;
        }
    }

    loop {
        tokio::select! {
            Some((req_id, _result)) = rx_scanner_data.recv() => {
                result_count -= 1;
                println!("Scanner subscription req_id: {req_id} is  done, {result_count}");
                if result_count == 0 && cancel_count == 0 {
                    break;
                }
            },
            Some(req_id) = rx_scanner_end.recv() => {
                cancel_count -= 1;
                println!("Scanner subscription req_id: {req_id} end, {cancel_count}");
                client.cancel_scanner_subscription(req_id).await.unwrap();
                if result_count == 0 && cancel_count == 0 {
                    break;
                }
            },
            _ = sleep(Duration::from_secs(10)) => {
                println!("Таймаут: операция не завершилась за 10 секунд");
            }
        }
    }

    assert!(result_count == 0);
    assert!(cancel_count == 0);

    println!("requests done: {}, {}", result_count, cancel_count);

    let _ = client.disconnect().await;
    Ok(())
}
