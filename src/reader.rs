use bytes::{Buf, BytesMut};
use crossbeam::queue::SegQueue;
use std::sync::Arc;
use tokio::{io::AsyncReadExt, net::tcp::OwnedReadHalf};

#[derive(Debug)]
pub struct Reader {
    r_reader: OwnedReadHalf,
    r_queue: Arc<SegQueue<Vec<String>>>,
    r_disconnect: tokio_util::sync::CancellationToken,
}

impl Reader {
    pub fn new(
        r_reader: OwnedReadHalf,
        r_queue: Arc<SegQueue<Vec<String>>>,
        r_disconnect: tokio_util::sync::CancellationToken,
    ) -> Self {
        Self {
            r_reader,
            r_queue,
            r_disconnect,
        }
    }

    pub async fn run(mut self) -> Self {
        loop {
            tokio::select! {
                () = self.r_disconnect.cancelled() => {println!("Reader thread: disconnecting"); break self},
                () = async {
                    if let Ok(Ok(len)) = self.r_reader.read_u32().await.map(usize::try_from) {
                        let mut buf = BytesMut::with_capacity(len);
                        if len == self.r_reader.read_buf(&mut buf).await.unwrap_or(0) {
                            let msg = buf.chunk()
                                .split(|b| *b == 0)
                                .map(|s| core::str::from_utf8(s).unwrap_or("").to_owned())
                                .collect::<Vec<String>>();
                            self.r_queue.push(msg);
                        }
                    }
                } => (),
            }
        }
    }
}
