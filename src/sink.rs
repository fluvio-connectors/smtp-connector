use anyhow::Result;
use async_trait::async_trait;

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
use async_native_tls::TlsStream;
use async_smtp::{Envelope, SendableEmail, SmtpClient, SmtpTransport};

#[derive(Debug)]
enum Transport {
    Tls(SmtpTransport<TlsStream<async_std::net::TcpStream>>),
    Cleartext(SmtpTransport<TcpStream>),
}

#[async_trait]
impl Sink<SmtpRecord> for SmtpSink {
    async fn connect(self, _offset: Option<Offset>) -> Result<LocalBoxSink<SmtpRecord>> {
        let config = self.config;

        let unfold = futures::sink::unfold(
            config,
            |config: SmtpConfig, record: crate::record::SmtpRecord| async move {
                tracing::debug!("Record = {:?}", record);

                let stream = TcpStream::connect(format!("{}:{}", config.host, config.port)).await?;
                let client = SmtpClient::new();

                let mut transport = if config.implicit_tls {
                    tracing::debug!("Requiring Implicit TLS");
                    let stream = async_native_tls::connect(&config.host, stream).await?;
                    Transport::Tls(SmtpTransport::new(client, stream).await?)
                } else if config.explicit_tls {
                    tracing::debug!("Requiring Explicit TLS");
                    let cleartext_transport = SmtpTransport::new(client, stream).await?;
                    let cleartext_stream_upgrade = cleartext_transport.starttls().await?;

                    let stream =
                        async_native_tls::connect(&config.host, cleartext_stream_upgrade).await?;
                    let client = SmtpClient::new().without_greeting();
                    Transport::Tls(SmtpTransport::new(client, stream).await?)
                } else {
                    tracing::warn!("Not using TLS");
                    Transport::Cleartext(SmtpTransport::new(client, stream).await?)
                };

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

                        match transport {
                            Transport::Tls(ref mut tls) => {
                                tls.try_login(&credentials, &mechs).await?
                            }
                            Transport::Cleartext(_) => {
                                tracing::error!("Configuration - Login is not allowed over cleartext transport without TLS.");
                                panic!("Configuration - Login is not allowed over cleartext transport without TLS.");
                            }
                        }
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

                tracing::debug!("Sending email {:?}", email.envelope());

                match transport {
                    Transport::Tls(mut tls) => tls.send(email).await?,
                    Transport::Cleartext(mut cleartext) => cleartext.send(email).await?,
                };

                Ok(config)
            },
        );

        Ok(Box::pin(unfold))
    }
}
