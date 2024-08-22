use std::{
    sync::{atomic::AtomicI32, Arc},
    time::Duration,
};

use ibapi::limits::*;
use tokio::time::{sleep, Instant};

#[tokio::test]
async fn test_limiter_50_rps() {
    let iter_variants = vec![50, 100, 150, 200];
    let delays_between = vec![0];

    start_limiter_thread();

    let start_total = Instant::now();

    test_async(&iter_variants, &delays_between).await;

    for iter_count in iter_variants.into_iter() {
        for delay in delays_between.clone() {
            // let mut rng = StdRng::from_entropy();
            // let delay = rng.gen_range(i * 1..i * 2);
            // iterate(iter_count, delay).await;
        }
    }

    // let total_sleep = Duration::from_millis(
    //     ((iter_variants.into_iter().sum::<i32>() as f32 * delays_between.len() as f32) * 1.2 / 60.0)
    //         as u64,
    // );
    let elapsed_total = start_total.elapsed().as_millis() as f64 / 1000.0;

    println!("Total elapsed: {:?}", elapsed_total);
}

async fn test_async(iter_variants: &Vec<i32>, delays_between: &Vec<i32>) {
    let mut async_handels = vec![];
    let start_total = Instant::now();

    for iter_count in iter_variants.clone() {
        for delay in delays_between.clone().into_iter() {
            async_handels.push(tokio::spawn(iterate(iter_count, delay)));
        }
    }

    for h in async_handels {
        let _ = h.await;
    }

    let elapsed_total = start_total.elapsed().as_millis() as f64 / 1000.0;
    let total_time_for_exec = iter_variants.into_iter().sum::<i32>() as f64 / 50.0;

    assert!(elapsed_total < total_time_for_exec * 1.13);
    assert!(elapsed_total > total_time_for_exec * 0.87);

    println!("Total elapsed: {:?}", elapsed_total);
}

async fn iterate(iter_count: i32, delay: i32) {
    let start = Instant::now();
    for _ in 0..=iter_count {
        increment_message_per_second_count().await;
        sleep(Duration::from_millis(delay as u64)).await;
    }
    let elapsed = start.elapsed().as_millis() as f64 / 1000.0;
    let total_time_for_exec = iter_count as f64 / 50.0;

    println!(
        "Iteration count: {iter_count}, Delay: {delay} => Elapsed: {:?} / {} sec",
        elapsed, total_time_for_exec,
    );
    // assert!(elapsed < total_time_for_exec * 1.13);
    // assert!(elapsed > total_time_for_exec * 0.87);
}
