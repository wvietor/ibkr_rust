use super::*;
use bytes::{Buf, BytesMut};
use crossbeam::queue::SegQueue;
use std::sync::Arc;
use tokio::{io::AsyncReadExt, net::tcp::OwnedReadHalf};

#[derive(Debug)]
pub struct Reader {
    inner: OwnedReadHalf,
    queue: Arc<SegQueue<Vec<String>>>,
    disconnect: tokio_util::sync::CancellationToken,
}

impl Reader {
    pub fn new(
        r_reader: OwnedReadHalf,
        r_queue: Arc<SegQueue<Vec<String>>>,
        r_disconnect: tokio_util::sync::CancellationToken,
    ) -> Self {
        Self {
            inner: r_reader,
            queue: r_queue,
            disconnect: r_disconnect,
        }
    }

    pub async fn run(mut self) -> Self {
        loop {
            tokio::select! {
                () = self.disconnect.cancelled() => {println!("Reader thread: disconnecting"); break self},
                () = async {
                    if let Ok(Ok(len)) = self.inner.read_u32().await.map(usize::try_from) {
                        let mut buf = BytesMut::with_capacity(len);

                        const MAX_MESSAGE_SIZE: usize = 3_000_000;
                        if len > MAX_MESSAGE_SIZE {
                            warn!("Reader:run() -> Received message length ({}) exceeds the maximum allowed size", len);
                        }
                        let mut total_read = 0;
                        while total_read < len {
                            match self.inner.read_buf(&mut buf).await {
                                Ok(0) => {
                                    warn!("Reader:run() -> Socket closed while reading message");
                                    break;
                                }
                                Ok(n) => total_read += n,
                                Err(e) => {
                                    error!("Reader:run() -> Error reading from socket: {:?}", e);
                                    break;
                                }
                            }
                        }

                        if len == total_read {
                            let msg = buf.chunk()
                                .split(|b| *b == 0)
                                .map(|s| core::str::from_utf8(s).unwrap_or("").to_owned())
                                .collect::<Vec<String>>();
                            self.queue.push(msg);
                        } else{
                            error!("Reader:run() -> len != total_read -> len:{len}, total_read:{total_read} ");
                        }
                    }
                } => (),
            }
        }
    }
}
