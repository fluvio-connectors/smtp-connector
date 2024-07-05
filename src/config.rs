#[derive(Debug)]
#[connector(config, name = "smtp")]
pub(crate) struct SmtpConfig {
    #[serde(default = "default_host")]
    pub host: String,

    #[serde(default = "default_port")]
    pub port: u32,

    pub user: Option<String>,
    pub pass: Option<String>,
}


#[inline]
fn default_host() -> String {
    "localhost".into()
}

#[inline]
fn default_port() -> u32 {
    32
}
