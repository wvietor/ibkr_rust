use ibapi::client::{ActiveClient, Builder, Host, Mode};
use ibapi::limits::SCANNER_SUBSCRIPTION_MAP;
use ibapi::scanner_subscription::{
    ScannerContract, ScannerSubscription, ScannerSubscriptionIsComplete,
};
use ibapi::wrapper::{CancelToken, Initializer, Wrapper};
use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::sync::atomic::AtomicI32;
use std::sync::LazyLock;
use tokio::sync::RwLock;
use tracing_test::traced_test;

const REQUESTS_COUNT: i32 = 11;
// const COMPLITED_REQUEST_COUNT: AtomicI32 = AtomicI32::new(REQUESTS_COUNT / 2);
const NUMBER_OF_RESULT: usize = 22;

// static TOTAL_REQUESTS: AtomicI32 = AtomicI32::new(0);
static TOTAL_RESPONCES: AtomicI32 = AtomicI32::new(0);

static REQ_IDS: LazyLock<RwLock<HashMap<i64, bool>>> =
    LazyLock::new(|| RwLock::new(HashMap::with_capacity(REQUESTS_COUNT as usize)));

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct ScannerWrapper;
// {
// responces: i32,
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
            TOTAL_RESPONCES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            // result.iter().for_each(|r| println!("{:?}", r));

            REQ_IDS.write().await.insert(req_id, true);

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
    type Recur<'c> = ();

    #[allow(clippy::manual_async_fn)]
    fn build(
        self,
        client: &mut ActiveClient,
        _cancel_loop: CancelToken,
    ) -> impl Future<Output = (Self::Wrap<'_>, Self::Recur<'_>)> + Send {
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
            println!("DONE");
            (self, ())
        }
    }
}

// async fn spawn_it<T: ScannerSubscriptionIsComplete>(client: &mut ActiveClient, subscription: T) {
//     tokio::spawn(async move { client.req_scanner_subscription(&subscription) });
// }

#[traced_test]
#[tokio::test]
async fn req_scanner_subscription_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut client =
        Builder::from_config_file(Mode::Paper, Host::Gateway, Some("config.toml".as_ref()))?
            .connect(5)
            .await?;
    println!("Connected");

    let mut client = client
        .disaggregated(ScannerWrapper)
        // .remote(ScannerWrapper { responces: 0 })
        .await;
    println!("disaggregated");

    for _i in 1..=REQUESTS_COUNT {
        let subscription = ScannerSubscription::asia_stocks()
            .asia_stocks()
            .hot_by_price()
            .number_of_result_rows(NUMBER_OF_RESULT as i32);
        // .price_below(30.0);

        if let Ok(req_id) = client.req_scanner_subscription(&subscription).await {
            println!("req_scanner_subscription: #{req_id}");

            REQ_IDS.write().await.insert(req_id, false);
            // client.cancel_scanner_subscription(req_id).await;
            // if req_id % 2 == 0 {
            //     let _ = client.cancel_scanner_subscription(req_id).await;
            // }
        }

        // let mut w = REQ_IDS.write().await;
        // let to_remove: Vec<i64> = w.iter().filter(|(k, v)| **v).map(|(&k, _)| k).collect();

        // for req_id in to_remove {
        //     client.cancel_scanner_subscription(req_id.clone()).await;
        //     w.remove(&req_id);
        // }
        // println!("NEW LOOP:");

        let mut map_lock = SCANNER_SUBSCRIPTION_MAP.write().await;

        // println!("LOOP:");
        // map_lock.iter().for_each(|e| print!("{:?}, ", e));
        // println!("");

        if map_lock.is_empty() {
            break;
        }

        let to_remove: Vec<i64> = map_lock
            .iter()
            .filter(|(_, v)| v.received)
            .map(|(&k, _)| k)
            .collect();

        if !to_remove.is_empty() {
            println!("to_remove: {:?}", to_remove);
        }
        drop(map_lock);

        for req_id in to_remove {
            println!("1");
            client.cancel_scanner_subscription(req_id.clone()).await;
            println!("2");
            // map_lock.remove(&req_id);
        }
    }

    loop {
        // println!("NEW LOOP:");

        let mut map_lock = SCANNER_SUBSCRIPTION_MAP.write().await;

        // println!("LOOP:");
        // map_lock.iter().for_each(|e| print!("{:?}, ", e));
        // println!("");

        if map_lock.is_empty() {
            break;
        }

        let to_remove: Vec<i64> = map_lock
            .iter()
            .filter(|(_, v)| v.received)
            .map(|(&k, _)| k)
            .collect();

        if !to_remove.is_empty() {
            println!("to_remove: {:?}", to_remove);
        }
        drop(map_lock);

        for req_id in to_remove {
            println!("1");
            client.cancel_scanner_subscription(req_id.clone()).await;
            println!("2");
            // map_lock.remove(&req_id);
        }
    }

    println!("requests done");
    println!("REQ_IDS: {:?}", REQ_IDS);

    println!("go to sleep");
    println!("REQ_IDS: {:?}", REQ_IDS);

    tokio::time::sleep(std::time::Duration::from_secs(33)).await;
    client.disconnect();
    Ok(())
}

// map_lock.retain(|&key, value| value.received == true);

// for (req_id, tracker) in map_lock.iter() {
//     if tracker.received {
//         client.cancel_scanner_subscription(req_id.clone()).await;
//         // map.remove(&req_id);
//     }
// }

// let mut w = REQ_IDS.read().await;
// loop {
//     let x: Vec<i64> = w.iter().filter(|(k, v)| **v).map(|(&k, _)| k).collect();
//     for req_id in &x {
//         client.cancel_scanner_subscription(req_id.clone()).await;
//         // w.remove(&req_id);
//     }
//     if x.is_empty() {
//         break;
//     }
// }
