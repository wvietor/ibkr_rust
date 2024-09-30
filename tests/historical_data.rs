use std::future::Future;

use tokio_util::time::FutureExt;

use ibapi::prelude::*;

struct ChannelWrapper {
    tx: tokio::sync::mpsc::Sender<(i64, Vec<Bar>)>,
}

impl Wrapper for ChannelWrapper {
    fn historical_bars(&mut self, req_id: i64, _start_datetime: chrono::DateTime<chrono::Utc>, _end_datetime: chrono::DateTime<chrono::Utc>, bars: Vec<Bar>) -> impl Future + Send {
        async move {
            let _ = self.tx.send((req_id, bars)).await;
        }
    }
}

impl Recurring for ChannelWrapper {
    async fn cycle(&mut self) {}
}

#[tokio::test]
async fn spy_bars() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    let mut client = Builder::from_config_file(Mode::Paper, Host::Gateway, &Some("config.toml"))?
        .connect(8)
        .await?
        .disaggregated(ChannelWrapper { tx })
        .await;

    let spy = contract::new::<Stock>(&mut client, "BBG000BDTBL9".parse()?).await?;
    let id = client
        .req_historical_bar(
            &spy,
            historical_bar::EndDateTime::Present,
            historical_bar::Duration::Week(1),
            historical_bar::Size::Minutes(historical_bar::MinuteSize::Fifteen),
            historical_bar::Trades,
            false,
        )
        .await?;
    if let Some(msg) = rx
        .recv()
        .timeout(std::time::Duration::from_secs(15))
        .await?
    {
        assert_eq!(id, msg.0);
        println!("{:?}", msg.1.first());
    }

    Ok(())
}
