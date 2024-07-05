use anyhow::{anyhow, Result};
use async_trait::async_trait;
use reqwest::{Client, RequestBuilder};

use fluvio::Offset;
use fluvio_connector_common::{tracing, LocalBoxSink, Sink};

use crate::SmtpConfig;

#[derive(Debug)]
pub(crate) struct SmtpSink {

}

impl SmtpSink {
    pub(crate) fn new(config: &SmtpConfig) -> Result<Self> {

        Ok(Self { request })
    }
}

#[async_trait]
impl Sink<String> for SmtpSink {
    async fn connect(self, _offset: Option<Offset>) -> Result<LocalBoxSink<String>> {
        let request = self.request;
        let unfold = futures::sink::unfold(
            request,
            |mut request: RequestBuilder, record: String| async move {
                tracing::trace!("{:?}", request);



                Ok::<_, anyhow::Error>(request)
            },
        );

        Ok(Box::pin(unfold))
    }
}
