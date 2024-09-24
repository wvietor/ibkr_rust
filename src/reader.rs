use bytes::{Buf, BytesMut};
use tokio::{io::AsyncReadExt, net::tcp::OwnedReadHalf};
use tracing::{error, info, warn};

#[derive(Debug)]
pub struct Reader {
    inner: OwnedReadHalf,
    tx: tokio::sync::mpsc::Sender<Vec<String>>,
    disconnect: tokio_util::sync::CancellationToken,
}

impl Reader {
    pub fn new(
        r_reader: OwnedReadHalf,
        tx: tokio::sync::mpsc::Sender<Vec<String>>,
        r_disconnect: tokio_util::sync::CancellationToken,
    ) -> Self {
        Self {
            inner: r_reader,
            tx,
            disconnect: r_disconnect,
        }
    }

    #[tracing::instrument(level = tracing::Level::DEBUG)]
    pub async fn run(mut self) -> Self {
        loop {
            tokio::select! {
                biased;
                () = async {
                    if let Ok(Ok(len)) = self.inner.read_u32().await.map(usize::try_from) {
                        tracing::trace!("Message received.");
                        let mut buf = BytesMut::with_capacity(len);
                        let mut total_read = 0;
                        while total_read < len {
                            match self.inner.read_buf(&mut buf).await {
                                Ok(0) => { warn!("TCP Reader read 0 bytes (this should never happen and is likely an error in message parsing)") },
                                Ok(n) => { total_read += n; },
                                Err(e) => error!(error=%e, "IO Error when receiving message.")
                            }
                        }
                        let msg = buf.chunk()
                        .split(|b| *b == 0)
                        .map(|s| core::str::from_utf8(s).unwrap_or("").to_owned())
                        .collect::<Vec<String>>();
                        tracing::trace!(msg=?&msg, "Message pushed.");
                        match self.tx.send(msg).await {
                            Ok(()) => (),
                            Err(e) => error!(%e, "IO Error when sending message. Client receiver may have dropped."),
                        }
                    }
                } => (),
                () = self.disconnect.cancelled() => { info!("Reader thread: disconnecting"); break self} ,
            }
        }
    }
}
