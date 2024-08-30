// #![feature(concat_idents)]
use super::*;
// use paste;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, LazyLock,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use tokio::{sync::RwLock, time::sleep};

// ?
pub enum LimitError {
    LimitReached(u8),
}

#[derive(Debug)]
pub struct ScannerTracker {
    pub received: bool,
    // callback ?
}

/// Active scanner subscriptions
pub static ACTIVE_SCANNER_SUBSCRIPTION: LazyLock<Arc<RwLock<HashMap<i64, ScannerTracker>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(HashMap::with_capacity(10))));

// pub async fn

static SCANNER_SUBSCRIPTION_COUNTER: LazyLock<Arc<AtomicU32>> =
    LazyLock::new(|| Arc::new(AtomicU32::new(0)));
static SCANNER_SUBSCRIPTION_LIMIT: u32 = 10;

static SLEEP_FOR_INCREMENT: u64 = 20;

// === TODO: ===
// write macro for impl increment/decrement ?

// macro_rules! create_limiting_fn {
// 	[ $name:ident ] => {
// 		paste! {
//         static [<$name _COUNTER2>]: String = String::new();
//     	}
//         // $(
//         // )+
//     };
// }
// create_limiting_fn![SCANNER_SUBSCRIPTION];

pub async fn increment_req_scanner_subscription() {
    loop {
        println!("increment_req_scanner_subscription");

        let counter = SCANNER_SUBSCRIPTION_COUNTER.load(Ordering::SeqCst);
        // println!("try inc! {}", counter);

        if counter >= SCANNER_SUBSCRIPTION_LIMIT - 1 {
            sleep(Duration::from_millis(SLEEP_FOR_INCREMENT)).await;
        } else {
            break;
        }
    }

    increment_message_per_second().await;
    SCANNER_SUBSCRIPTION_COUNTER.fetch_add(1, Ordering::SeqCst);
}

pub fn decrement_req_scanner_subscription() {
    SCANNER_SUBSCRIPTION_COUNTER.fetch_sub(1, Ordering::SeqCst);
}

static SLEEPS_FOR_50_RPS: [f64; 101] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 0.2, 0.8, 1.8, 3.2, 5.0, 7.2, 9.8, 12.8, 16.2, 20.0, 24.2, 28.8, 33.8, 39.2,
    45.0, 51.2, 57.8, 64.8, 72.2, 80.0, 88.2, 96.8, 105.8, 115.2, 125.0, 135.2, 145.8, 156.8,
    168.2, 180.0, 192.2, 204.8, 217.8, 231.2, 245.0, 259.2, 273.8, 288.8, 304.2, 320.0, 336.2,
    352.8, 369.8, 387.2, 405.0, 423.2, 441.8, 460.8, 480.2, 500.0, 520.2, 540.8, 561.8, 583.2,
    605.0, 627.2, 649.8, 672.8, 696.2, 720.0,
];

static COUNTER: LazyLock<Arc<AtomicU32>> = LazyLock::new(|| Arc::new(AtomicU32::new(0)));
static RATE: LazyLock<Arc<AtomicU32>> = LazyLock::new(|| Arc::new(AtomicU32::new(0)));
static RATE_CALCULATION_HANDLE: LazyLock<Arc<RwLock<Option<JoinHandle<()>>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(None)));

pub async fn increment_message_per_second() {
    let mut rate;

    loop {
        rate = RATE.load(Ordering::SeqCst);
        let sleep_time = if rate >= SLEEPS_FOR_50_RPS.len() as u32 {
            SLEEPS_FOR_50_RPS.last().unwrap()
        } else {
            &SLEEPS_FOR_50_RPS[rate as usize]
        };

        // println!("Rate: {:?}, Sleep: {}", RATE, sleep_time);

        sleep(Duration::from_secs_f64(sleep_time / 1000.0)).await;
        rate = RATE.load(Ordering::SeqCst);
        if rate <= 50 {
            break;
        }
    }

    COUNTER.fetch_add(1, Ordering::SeqCst);
}

pub async fn start_rate_calculation_thread() {
    let counter_clone = COUNTER.clone();
    let rate_clone = RATE.clone();

    let mut write_lock = RATE_CALCULATION_HANDLE.write().await;

    if write_lock.is_none() || write_lock.is_some() && write_lock.as_ref().unwrap().is_finished() {
        let handle = thread::spawn(move || {
            const SLEEP_TIME: u64 = 10; // ms between measures; 10ms - best
            const MAX_SIZE: usize = 1000 / SLEEP_TIME as usize;

            let mut new;
            let mut prev = 0;
            let mut dif;

            let mut vec = [0; MAX_SIZE];

            for i in 0..vec.len() {
                if i % 2 == 0 {
                    vec[i] = 1;
                }
            }

            let mut vec_index = 0;
            let mut sum;

            loop {
                sum = 0;
                new = counter_clone.load(Ordering::SeqCst);
                dif = new - prev;
                prev = new;
                vec_index += 1;

                if vec_index == MAX_SIZE - 1 {
                    vec_index = 0;
                }

                if new == u32::MAX - 1 {
                    counter_clone.store(0, Ordering::SeqCst);
                }

                vec[vec_index] = dif;

                sum += vec.iter().sum::<u32>();
                rate_clone.store(sum, Ordering::SeqCst);

                // println!("{},{} - {},{} - {}", new, prev, dif, sum, vec_index);

                thread::sleep(Duration::from_millis(SLEEP_TIME));
            }
        });

        *write_lock = Some(handle);
    }
}
