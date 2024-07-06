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
        Ok(Self { config })
    }
}

use crate::record::SmtpRecord;
use async_smtp::{Envelope, SendableEmail, SmtpClient, SmtpTransport};

#[async_trait]
impl Sink<SmtpRecord> for SmtpSink {
    async fn connect(self, _offset: Option<Offset>) -> Result<LocalBoxSink<SmtpRecord>> {
        let config = self.config;

        let unfold = futures::sink::unfold(
            config,
            |config: SmtpConfig, record: crate::record::SmtpRecord| async move {
                tracing::info!("Record = {:?}", record);

                let stream = TcpStream::connect(format!("{}:{}", config.host, config.port)).await?;
                let mut stream = async_native_tls::connect(&config.host, stream).await?;

                let mut client = SmtpClient::new();
                let mut transport = SmtpTransport::new(client, stream).await?;

                if let Some(ref user) = config.user {
                    if let Some(ref pass) = config.password {
                        let credentials = async_smtp::authentication::Credentials::new(
                            user.clone(),
                            pass.clone(),
                        );
                        let mechs = [
                            async_smtp::authentication::Mechanism::Plain,
                            async_smtp::authentication::Mechanism::Login,
                        ];

                        transport.try_login(&credentials, &mechs).await?;
                    }
                }

                let to = async_smtp::EmailAddress::new(record.to.address.clone()).unwrap();
                let from = async_smtp::EmailAddress::new(record.from.address.clone()).unwrap();

                let email = SendableEmail::new(
                    Envelope::new(Some(from), vec![to])?,
                    format!(
                        "From: <{}>\r\nTo: <{}>\r\nSubject: {}\r\n\r\n{}",
                        record.from.address, record.to.address, record.subject, record.body
                    ),
                );

                tracing::info!("Sending email {:?}", email.envelope());

                transport.send(email).await?;

                Ok(config)
            },
        );

        Ok(Box::pin(unfold))
    }
}
