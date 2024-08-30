use ibapi::client::{ActiveClient, Builder, Host, Mode};
use ibapi::limits::ACTIVE_SCANNER_SUBSCRIPTION;
use ibapi::scanner_subscription::{ScannerContract, ScannerSubscription};
use ibapi::wrapper::{CancelToken, Initializer, Recurring, Wrapper};
use std::future::Future;
use std::sync::atomic::AtomicI32;
use tracing_test::traced_test;

const REQUESTS_COUNT: i32 = 21;
// const COMPLITED_REQUEST_COUNT: AtomicI32 = AtomicI32::new(REQUESTS_COUNT / 2);
const NUMBER_OF_RESULT: usize = 50;

// static TOTAL_REQUESTS: AtomicI32 = AtomicI32::new(0);
static TOTAL_RESPONCES: AtomicI32 = AtomicI32::new(0);

// static REQ_IDS: LazyLock<RwLock<HashMap<i64, bool>>> =
//     LazyLock::new(|| RwLock::new(HashMap::with_capacity(REQUESTS_COUNT as usize)));

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct ScannerWrapper;

impl Recurring for ScannerWrapper {
    fn cycle(&mut self) -> impl Future<Output = ()> + Send {
        async { () }
    }
}

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
            TOTAL_RESPONCES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            // result.iter().for_each(|r| println!("{:?}", r));

            // REQ_IDS.write().await.insert(req_id, true);

            assert_eq!(result.len(), NUMBER_OF_RESULT);

            println!(
                "Results with responces: {}/{}, len: {}",
                TOTAL_RESPONCES.load(std::sync::atomic::Ordering::SeqCst),
                REQUESTS_COUNT, // COMPLITED_REQUEST_COUNT.load(std::sync::atomic::Ordering::SeqCst)
                result.len()
            );
        }
    }

    fn scanner_data_end(&mut self, req_id: i64) -> impl Future + Send + Send {
        async move {}
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
            // for _i in 0..REQUESTS_COUNT {
            //     let subscription = ScannerSubscription::us_stocks()
            //         .us_major()
            //         .hot_by_volume()
            //         .number_of_result_rows(NUMBER_OF_RESULT as i32);
            //     // .price_below(30.0);

            //     println!("req_scanner_subscription: {_i}");

            //     if let Ok(req_id) = client.req_scanner_subscription(&subscription).await {
            //         // if req_id % 2 == 0 {
            //         //     let _ = client.cancel_scanner_subscription(req_id).await;
            //         // }
            //     }
            // }
            //
            //

            let mut subsctiptions = vec![];

            let arr = (0..=150).step_by(5).map(|x| x as f64).collect::<Vec<f64>>();
            let mut it = arr.iter();
            let mut prev = it.next().unwrap().clone();

            for _i in 1..=REQUESTS_COUNT {
                let new = it.next().unwrap().clone();
                let subs = ScannerSubscription::asia_stocks()
                    .asia_stocks()
                    .hot_by_price()
                    .number_of_result_rows(NUMBER_OF_RESULT as i32)
                    .usd_price_above(prev)
                    .usd_price_below(new);
                prev = new;
                subsctiptions.push(subs);
            }
            println!("WORK?");

            let _ = client.req_scanner_subscription_once(subsctiptions).await;
            tokio::time::sleep(std::time::Duration::from_secs(8)).await;

            // Ok(())
            println!("DONE");
            ScannerWrapper
        }
    }
}

// async fn spawn_it<T: ScannerSubscriptionIsComplete>(client: &mut ActiveClient, subscription: T) {
//     tokio::spawn(async move { client.req_scanner_subscription(&subscription) });
// }

#[traced_test]
#[tokio::test]
async fn req_scanner_subscription_and_cancel_test() -> Result<(), Box<dyn std::error::Error>> {
    let client = Builder::from_config_file(Mode::Paper, Host::Gateway, &None::<&'static str>)?
        .connect(5)
        .await?;

    let mut client = client
        .disaggregated(ScannerWrapper)
        // .remote(ScannerWrapper)
        .await;

    for _i in 1..=REQUESTS_COUNT {
        let subscription = ScannerSubscription::asia_stocks()
            .asia_stocks()
            .hot_by_price()
            .number_of_result_rows(NUMBER_OF_RESULT as i32)
            .usd_price_below(30.0);

        if let Ok(req_id) = client.req_scanner_subscription(&subscription).await {
            println!("req_scanner_subscription: #{req_id}");

            // REQ_IDS.write().await.insert(req_id, false);
            // client.cancel_scanner_subscription(req_id).await;
            // if req_id % 2 == 0 {
            //     let _ = client.cancel_scanner_subscription(req_id).await;
            // }
        }

        cancel_complited_subscription(&mut client).await;
    }

    loop {
        if cancel_complited_subscription(&mut client).await == 0 {
            break;
        }
    }

    println!("requests done");
    // println!("REQ_IDS: {:?}", REQ_IDS);
    println!(
        "SCANNER_SUBSCRIPTION_MAP: {:?}",
        ACTIVE_SCANNER_SUBSCRIPTION.read().await
    );

    tokio::time::sleep(std::time::Duration::from_secs(33)).await;
    let _ = client.disconnect().await;
    Ok(())
}

// ? ? ? ? ?
async fn cancel_complited_subscription(client: &mut ActiveClient) -> usize {
    let map_lock = ACTIVE_SCANNER_SUBSCRIPTION.read().await;
    let map_len = map_lock.len();

    let to_remove: Vec<i64> = map_lock
        .iter()
        .filter(|(_, v)| v.received)
        .map(|(&k, _)| k)
        .collect();

    println!("cancel: {:?}", to_remove);
    drop(map_lock);
    for req_id in to_remove {
        let _ = client.cancel_scanner_subscription(req_id).await;
    }
    map_len
}

#[tokio::test]
async fn req_scanner_subscription_once_test() -> Result<(), Box<dyn std::error::Error>> {
    let client = Builder::from_config_file(Mode::Paper, Host::Gateway, &None::<&'static str>)?
        .connect(5)
        .await?;

    let mut client = client
        // .disaggregated(ScannerWrapper)
        .remote(ScannerWrapper)
        .await;
    tokio::time::sleep(std::time::Duration::from_secs(17)).await;

    // let mut subsctiptions = vec![];

    // let arr = (0..=150).step_by(5).map(|x| x as f64).collect::<Vec<f64>>();
    // let mut it = arr.iter();
    // let mut prev = it.next().unwrap().clone();

    // for _i in 1..=REQUESTS_COUNT {
    //     let new = it.next().unwrap().clone();
    //     let subs = ScannerSubscription::asia_stocks()
    //         .asia_stocks()
    //         .hot_by_price()
    //         .number_of_result_rows(NUMBER_OF_RESULT as i32)
    //         .usd_price_above(prev)
    //         .usd_price_below(new);
    //     prev = new;
    //     subsctiptions.push(subs);
    // }

    // let _ = client.req_scanner_subscription_once(subsctiptions).await;

    Ok(())
}
