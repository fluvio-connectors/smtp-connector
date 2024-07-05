mod config;
mod sink;

use anyhow::Result;
use config::SmtpConfig;
use futures::{SinkExt, StreamExt};
use sink::SmtpSink;

use fluvio_connector_common::{connector, consumer::ConsumerStream, tracing, Sink};

const SIGNATURES: &str = concat!("Fluvio Community SMTP Sink Connector ", env!("CARGO_PKG_VERSION"));

#[connector(sink)]
async fn start(config: SmtpConfig, mut stream: impl ConsumerStream) -> Result<()> {
    tracing::debug!(?config);

    let sink = SmtpSink::new(&config)?;
    let mut sink = sink.connect(None).await?;

    tracing::info!("Starting {SIGNATURES}");
    while let Some(item) = stream.next().await {
        tracing::debug!("Received record in consumer");
        let str = String::from_utf8(item?.as_ref().to_vec())?;
        sink.send(str).await?;
    }
    tracing::info!("Consumer loop finished");

    Ok(())
}
