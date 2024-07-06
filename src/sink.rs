use anyhow::{anyhow, Result};
use async_trait::async_trait;

use async_native_tls::TlsConnector;
use async_std::net::TcpStream;

use fluvio::Offset;
use fluvio_connector_common::{tracing, LocalBoxSink, Sink};

use crate::SmtpConfig;

#[derive(Debug)]
pub(crate) struct SmtpSink {
    config: SmtpConfig,
}

impl SmtpSink {
    pub(crate) fn new(config: SmtpConfig) -> Result<Self> {
        Ok(Self { config } )
    }
}

use crate::record::SmtpRecord;

#[async_trait]
impl Sink<SmtpRecord> for SmtpSink {
    async fn connect(self, _offset: Option<Offset>) -> Result<LocalBoxSink<SmtpRecord>> {
        let config = self.config;
        
        let unfold = futures::sink::unfold(
            config,
            |config: SmtpConfig, record: crate::record::SmtpRecord| async move {
                tracing::info!("Config: {:?}", config);
                tracing::info!("Record = {:?}", record);
                

                Ok(config)
            },
        );

        Ok(Box::pin(unfold))
    }
}
