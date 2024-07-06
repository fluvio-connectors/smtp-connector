use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Address {
    pub name: String,
    pub address: String,
}

#[derive(Debug, Deserialize)]
pub struct SmtpRecord {
    pub subject: String,
    pub body: String,
    pub from: Address,
    pub to: Address,
}
