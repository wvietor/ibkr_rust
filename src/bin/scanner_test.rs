use ibapi::client::{ActiveClient, Builder, Host, Mode};
use ibapi::prelude::Tag;
use ibapi::wrapper::{CancelToken, Initializer, Wrapper};
use std::future::Future;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::runtime::Runtime;
use tracing::level_filters::LevelFilter;
use tracing::{info, Level};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct ScannerWrapper;

use ibapi::*;

impl ibapi::wrapper::Wrapper for ScannerWrapper {
    fn scanner_parameters(&mut self, req_id: i64, xml: String) -> impl Future + Send {
        async move {
            info!(
                "WRAPPER:scanner_parameters(): req_id:{req_id},  xml.len(): {}",
                xml.len()
            );
        }
    }
    fn error(
        &mut self,
        req_id: i64,
        error_code: i64,
        error_string: String,
        advanced_order_reject_json: String,
    ) -> impl Future + Send {
        async move {
            if error_code != 2104 && error_code != 2106 && error_code != 2158 {
                info!("error_code: {}, error_string:{}", error_code, error_string);
            }
        }
    }
}

impl ibapi::wrapper::Initializer for ScannerWrapper {
    type Wrap<'c> = ScannerWrapper;
    type Recur<'c> = ();

    fn build(
        self,
        client: &mut ActiveClient,
        _cancel_loop: CancelToken,
    ) -> impl Future<Output = (Self::Wrap<'_>, Self::Recur<'_>)> + Send {
        async move {
            // let id = client
            //     .req_account_summary(&vec![
            //         Tag::AccountType,
            //         Tag::NetLiquidation,
            //         Tag::TotalCashValue,
            //     ])
            //     .await
            //     .unwrap();
            // println!("req_status1:{:?}", &id);

            // let id = client
            //     .req_account_summary(&vec![Tag::AccountType])
            //     .await
            //     .unwrap();
            let req_status = client.req_current_time().await;

            let req_status = client.req_scanner_parameters().await;

            (self, ())
        }
    }
}

//
// ─── TOKIO ──────────────────────────────────────────────────────────────────────
//

pub fn init_tokio() -> Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(6)
        .enable_all()
        // .thread_name("Tokio-Worker")
        .thread_name_fn(|| {
            static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
            let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
            format!("Tokio-{}", id)
        })
        // .thread_stack_size(3 * 1024 * 1024)
        .build()
        .unwrap()
}

//
// ─── TRACING ────────────────────────────────────────────────────────────────────
//
pub fn init_stdout_tracing() {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::DEBUG.into())
        .from_env_lossy()
        .add_directive("hyper=off".parse().unwrap())
        .add_directive("reqwest=off".parse().unwrap());
    // .add_directive("external_crate=off".parse().unwrap());
    // .add_directive("my_crate=debug".parse().unwrap());

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_max_level(Level::DEBUG)
        .with_file(true)
        .with_line_number(true)
        .with_thread_names(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_env_filter(filter)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();
    info!("Tracing initialized");
}

// #[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_stdout_tracing();
    let rt = init_tokio();

    rt.block_on(async {
        let mut client = Builder::manual(4002, Ipv4Addr::from_str("127.0.0.1").ok())
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

        println!("Hello");
        Ok(())
    })
}
