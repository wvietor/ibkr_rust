use std::future::Future;
use ibapi::prelude::*;


enum ExecutionMessage {
    Response(i64, Execution),
    Finished(i64),
}

struct ExecutionWrapper {
    tx: tokio::sync::mpsc::Sender<ExecutionMessage>,
}

impl Wrapper for ExecutionWrapper {
    fn execution(&mut self, req_id: i64, execution: Execution) -> impl Future + Send {
        async move {
            self.tx.send(ExecutionMessage::Response(req_id, execution)).await.expect("sending the execution details should succeed");
        }
    }

    fn execution_details_end(&mut self, req_id: i64) -> impl Future + Send {
        async move {
            self.tx.send(ExecutionMessage::Finished(req_id)).await.expect("sending the req_id should succeed")
        }
    }
}

#[tokio::test]
async fn execs() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    let mut client = Builder::from_config_file(Mode::Paper, Host::Gateway, &None::<std::path::PathBuf>)?
        .connect(9)
        .await?
        .disaggregated(ExecutionWrapper { tx })
        .await;

    let id = client.req_executions(Filter { side: Some(OrderSide::Buy), ..Default::default() }).await?;
    while let Some(msg) = rx.recv().await {
        match msg {
            ExecutionMessage::Response(req_id, execution) => {
                assert_eq!(req_id, id);
                println!("One execution was: {:?}", &execution);
            },
            ExecutionMessage::Finished(req_id) => {
                assert_eq!(req_id, id);
                println!("No more executions to receiver for ID {id}");
                break;
            }
        }
    }
    client.disconnect().await.expect("disconnect should succeed.");

    Ok(())
}
