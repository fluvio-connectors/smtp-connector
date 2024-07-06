use fluvio_connector_common::connector;

#[derive(Debug)]
#[connector(config, name = "smtp")]
pub(crate) struct SmtpConfig {
    #[serde(default = "default_host")]
    pub host: String,

    #[serde(default = "default_port")]
    pub port: u32,

    pub explicit_tls: bool,
    pub implicit_tls: bool,

    pub user: Option<String>,
    pub password: Option<String>,
}

#[inline]
fn default_host() -> String {
    "localhost".into()
}

#[inline]
fn default_port() -> u32 {
    32
}
